use anyhow::Result;
use clap::Parser;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use hg_bench::cuda::GpuContext;
use tig_challenges::hypergraph::{Challenge, Track};

#[derive(Parser)]
#[command(name = "gen_hgr")]
#[command(about = "Generate TIG hypergraph instances and export as .hgr files")]
#[command(version)]
struct Cli {
    /// Instance size (number of hyperedges): 10000, 20000, 50000, 100000, or 200000
    size: u32,

    /// Output directory for .hgr files
    output_folder: PathBuf,

    /// Number of instances to generate
    #[arg(short, default_value = "1")]
    n: u32,

    /// Starting seed/nonce (instances use seed, seed+1, seed+2, ...)
    #[arg(short, default_value = "0")]
    s: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let tig_track = Track { n_h_edges: cli.size };

    fs::create_dir_all(&cli.output_folder)?;

    println!("Generating {} instances ({} hyperedges)", cli.n, cli.size);
    println!("Output directory: {}", cli.output_folder.display());
    println!();

    let ctx = GpuContext::new()?;

    for i in 0..cli.n {
        let nonce = cli.s + i as u64;
        let seed = generate_tig_seed(cli.size, nonce);
        let seed_hex = format!("{:02x}{:02x}{:02x}{:02x}", seed[0], seed[1], seed[2], seed[3]);

        print!("[{}/{}] Generating instance {}... ", i + 1, cli.n, seed_hex);
        std::io::stdout().flush()?;

        let challenge = Challenge::generate_instance(
            &seed,
            &tig_track,
            ctx.module.clone(),
            ctx.stream.clone(),
            &ctx.prop,
        )?;

        // Output format: <size>_<seed_hex>_<i>.hgr
        let hgr_path = cli.output_folder.join(format!("{}_{}_{}.hgr", cli.size, seed_hex, i));
        export_challenge_to_hgr(&challenge, &hgr_path, &ctx)?;

        println!(
            "nodes={}, hyperedges={}, k={}, max_part_size={}",
            challenge.num_nodes,
            challenge.num_hyperedges,
            challenge.num_parts,
            challenge.max_part_size
        );
    }

    println!();
    println!("Done. Generated {} .hgr files in {}", cli.n, cli.output_folder.display());

    Ok(())
}

fn generate_tig_seed(track: u32, nonce: u64) -> [u8; 32] {
    let track_id = match track {
        0..=15000 => "track_10k",
        15001..=30000 => "track_20k",
        30001..=75000 => "track_50k",
        75001..=150000 => "track_100k",
        _ => "track_200k",
    };

    let settings_json = format!(
        r#"{{"player_id":"benchmark_player","block_id":"benchmark_block","challenge_id":"c004","algorithm_id":"sigma_freud_v5","track_id":"{}"}}"#,
        track_id
    );

    let rand_hash = "0000000000000000000000000000000000000000000000000000000000000000";
    let seed_input = format!("{}_{}_{}", settings_json, rand_hash, nonce);
    blake3::hash(seed_input.as_bytes()).into()
}

fn export_challenge_to_hgr(
    challenge: &Challenge,
    path: &PathBuf,
    ctx: &GpuContext,
) -> Result<()> {
    let hyperedge_offsets: Vec<i32> = ctx.stream.memcpy_dtov(&challenge.d_hyperedge_offsets)?;
    let hyperedge_nodes: Vec<i32> = ctx.stream.memcpy_dtov(&challenge.d_hyperedge_nodes)?;

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{} {}", challenge.num_hyperedges, challenge.num_nodes)?;

    for i in 0..challenge.num_hyperedges as usize {
        let start = hyperedge_offsets[i] as usize;
        let end = hyperedge_offsets[i + 1] as usize;

        let nodes: Vec<String> = hyperedge_nodes[start..end]
            .iter()
            .map(|&n| (n + 1).to_string())
            .collect();

        writeln!(writer, "{}", nodes.join(" "))?;
    }

    writer.flush()?;
    Ok(())
}
