use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use hg_bench::hgr;

#[derive(Parser)]
#[command(name = "run_sigma_freud")]
#[command(about = "Run sigma_freud solver on a folder of .hgr files")]
#[command(version)]
struct Cli {
    /// Input folder containing .hgr files
    hgr_folder: PathBuf,

    /// Output folder for partition files
    output_folder: PathBuf,

    /// Number of partitions
    #[arg(short, default_value = "64")]
    k: u32,

    /// Balance epsilon
    #[arg(short, default_value = "0.03")]
    e: f64,

    /// Refinement rounds
    #[arg(short, default_value = "2000")]
    r: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    fs::create_dir_all(&cli.output_folder)?;

    let mut hgr_files: Vec<PathBuf> = fs::read_dir(&cli.hgr_folder)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "hgr"))
        .collect();

    hgr_files.sort();

    if hgr_files.is_empty() {
        return Err(anyhow!("No .hgr files found in {}", cli.hgr_folder.display()));
    }

    println!("Found {} .hgr files in {}", hgr_files.len(), cli.hgr_folder.display());
    println!("Output folder: {}", cli.output_folder.display());
    println!("Settings: k={}, epsilon={}, refinement={}", cli.k, cli.e, cli.r);
    println!();

    let mut total_time = 0.0;
    let mut total_connectivity = 0u32;

    for (i, hgr_path) in hgr_files.iter().enumerate() {
        let filename = hgr_path.file_stem().unwrap().to_string_lossy();
        print!("[{}/{}] {}... ", i + 1, hgr_files.len(), filename);
        std::io::stdout().flush()?;

        let hypergraph = hgr::read_hgr(&hgr_path)?;
        let max_part_size = ((hypergraph.num_nodes as f64 / cli.k as f64) * (1.0 + cli.e)).ceil() as u32;

        let start = Instant::now();
        let partition = hg_bench::solver::solve(&hypergraph, cli.k, max_part_size, 2, Some(cli.r))?;
        let elapsed = start.elapsed().as_secs_f64();

        let connectivity = hgr::compute_connectivity(&hypergraph, &partition);

        // Output with same name but .partition extension
        let partition_path = cli.output_folder.join(format!("{}.partition", filename));
        hgr::write_partition(&partition_path, &partition)?;

        // Write timing file
        let timing_path = cli.output_folder.join(format!("{}.time", filename));
        let mut timing_file = File::create(&timing_path)?;
        writeln!(timing_file, "{:.3}", elapsed)?;

        println!("KM1={}, time={:.2}s", connectivity, elapsed);

        total_time += elapsed;
        total_connectivity += connectivity;
    }

    println!();
    println!("=== Summary ===");
    println!("Instances: {}", hgr_files.len());
    println!("Total connectivity: {}", total_connectivity);
    println!("Average connectivity: {:.1}", total_connectivity as f64 / hgr_files.len() as f64);
    println!("Total time: {:.2}s", total_time);
    println!("Average time: {:.2}s", total_time / hgr_files.len() as f64);

    Ok(())
}
