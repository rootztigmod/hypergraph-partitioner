use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod cuda;
mod hgr;
mod solver;

#[derive(Parser)]
#[command(name = "hg_bench")]
#[command(about = "GPU-accelerated hypergraph partitioner - benchmarking against Mt-KaHyPar")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate TIG instances, solve them, and export results
    Gen {
        /// Track size (number of hyperedges): 10000, 20000, 50000, 100000, or 200000
        #[arg(short, long)]
        track: u32,

        /// Number of instances to generate and solve
        #[arg(short, long, default_value = "10")]
        nonces: u32,

        /// Output directory for .hgr and partition files
        #[arg(short, long)]
        out: PathBuf,

        /// Effort level (0-5, higher = better quality, slower)
        #[arg(short, long, default_value = "2")]
        effort: u32,

        /// Refinement rounds (overrides effort-based default if specified)
        #[arg(short, long)]
        refinement: Option<u32>,
    },

    /// Solve an existing .hgr file
    File {
        /// Path to input .hgr file
        #[arg(long)]
        hgr: PathBuf,

        /// Output path for partition file
        #[arg(short, long)]
        out: PathBuf,

        /// Number of partitions (default: 64)
        #[arg(short, long, default_value = "64")]
        k: u32,

        /// Balance epsilon (default: 0.03)
        #[arg(short, long, default_value = "0.03")]
        epsilon: f64,

        /// Effort level (0-5, higher = better quality, slower)
        #[arg(long, default_value = "2")]
        effort: u32,

        /// Refinement rounds (overrides effort-based default if specified)
        #[arg(long)]
        refinement: Option<u32>,
    },

    /// Verify a partition and compute metrics
    Score {
        /// Path to .hgr file
        #[arg(long)]
        hgr: PathBuf,

        /// Path to partition file
        #[arg(long)]
        partition: PathBuf,

        /// Number of partitions (default: 64)
        #[arg(short, long, default_value = "64")]
        k: u32,

        /// Balance epsilon (default: 0.03)
        #[arg(short, long, default_value = "0.03")]
        epsilon: f64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen {
            track,
            nonces,
            out,
            effort,
            refinement,
        } => {
            use tig_challenges::hypergraph::{Challenge, Solution, Track};
            use serde_json::{Map, Value};
            
            println!("Generating {} instances for track {} hyperedges", nonces, track);
            println!("Output directory: {}", out.display());
            println!("Effort: {}, Refinement: {:?}", effort, refinement);
            
            fs::create_dir_all(&out)?;
            
            let ctx = cuda::GpuContext::new()?;
            let tig_track = Track { n_h_edges: track };
            
            let mut hyperparameters: Map<String, Value> = Map::new();
            hyperparameters.insert("effort".to_string(), Value::Number(effort.into()));
            if let Some(r) = refinement {
                hyperparameters.insert("refinement".to_string(), Value::Number(r.into()));
            }
            let hyperparameters = Some(hyperparameters);
            
            let mut total_connectivity = 0u64;
            let mut total_time = 0.0f64;
            
            for nonce in 0..nonces {
                let seed = generate_tig_seed(track, nonce as u64);
                let seed_hex = format!("{:02x}{:02x}{:02x}{:02x}", seed[0], seed[1], seed[2], seed[3]);
                
                println!("\n[{}/{}] Generating instance with seed {}...", nonce + 1, nonces, seed_hex);
                
                let challenge = Challenge::generate_instance(
                    &seed,
                    &tig_track,
                    ctx.module.clone(),
                    ctx.stream.clone(),
                    &ctx.prop,
                )?;
                
                println!("  Nodes: {}, Hyperedges: {}, k: {}, max_part_size: {}", 
                    challenge.num_nodes, challenge.num_hyperedges, challenge.num_parts, challenge.max_part_size);
                
                let hgr_path = out.join(format!("challenge_{}_{}.hgr", track, seed_hex));
                export_challenge_to_hgr(&challenge, &hgr_path, &ctx)?;
                println!("  Exported .hgr to: {}", hgr_path.display());
                
                let final_partition: std::cell::RefCell<Vec<u32>> = std::cell::RefCell::new(Vec::new());
                let save_solution = |solution: &Solution| -> anyhow::Result<()> {
                    *final_partition.borrow_mut() = solution.partition.clone();
                    Ok(())
                };
                
                let start = Instant::now();
                
                match track {
                    0..=15000 => solver::track_10k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
                    15001..=30000 => solver::track_20k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
                    30001..=75000 => solver::track_50k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
                    75001..=150000 => solver::track_100k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
                    _ => solver::track_200k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
                }
                
                let elapsed = start.elapsed().as_secs_f64();
                total_time += elapsed;
                
                let partition = final_partition.borrow();
                let partition_path = out.join(format!("partition_{}_{}.txt", track, seed_hex));
                hgr::write_partition(&partition_path, &partition)?;
                
                // Write timing file for comparison script
                let timing_path = out.join(format!("partition_{}_{}_timing.txt", track, seed_hex));
                fs::write(&timing_path, format!("{:.3}\n", elapsed))?;
                
                let hg = hgr::read_hgr(&hgr_path)?;
                let connectivity = hgr::compute_connectivity(&hg, &partition);
                total_connectivity += connectivity as u64;
                
                println!("  Connectivity (KM1): {}", connectivity);
                println!("  Time: {:.2}s", elapsed);
                println!("  Partition saved to: {}", partition_path.display());
            }
            
            println!("\n=== Summary ===");
            println!("Instances: {}", nonces);
            println!("Track: {} hyperedges", track);
            println!("Total connectivity: {}", total_connectivity);
            println!("Average connectivity: {:.1}", total_connectivity as f64 / nonces as f64);
            println!("Total time: {:.2}s", total_time);
            println!("Average time: {:.2}s", total_time / nonces as f64);
        }

        Commands::File {
            hgr,
            out,
            k,
            epsilon,
            effort,
            refinement,
        } => {
            println!("Solving: {}", hgr.display());
            println!("Output: {}", out.display());
            println!("k={}, epsilon={}, effort={}, refinement={:?}", k, epsilon, effort, refinement);

            let hypergraph = hgr::read_hgr(&hgr)?;
            println!(
                "Loaded hypergraph: {} nodes, {} hyperedges",
                hypergraph.num_nodes, hypergraph.num_hyperedges
            );

            let max_part_size = ((hypergraph.num_nodes as f64 / k as f64) * (1.0 + epsilon)).ceil() as u32;
            println!("Max partition size: {}", max_part_size);

            let start = Instant::now();
            let partition = solver::solve(
                &hypergraph,
                k,
                max_part_size,
                effort,
                refinement,
            )?;
            let elapsed = start.elapsed().as_secs_f64();

            hgr::write_partition(&out, &partition)?;
            println!("Partition written to: {}", out.display());

            let connectivity = hgr::compute_connectivity(&hypergraph, &partition);
            println!("Connectivity (KM1): {}", connectivity);
            println!("Time: {:.2}s", elapsed);
        }

        Commands::Score {
            hgr,
            partition,
            k,
            epsilon,
        } => {
            println!("Scoring partition...");
            println!("Hypergraph: {}", hgr.display());
            println!("Partition: {}", partition.display());

            let hypergraph = hgr::read_hgr(&hgr)?;
            let partition_vec = hgr::read_partition(&partition)?;

            let max_part_size = ((hypergraph.num_nodes as f64 / k as f64) * (1.0 + epsilon)).ceil() as u32;

            let connectivity = hgr::compute_connectivity(&hypergraph, &partition_vec);
            let (is_feasible, max_size, min_size) = hgr::check_feasibility(&partition_vec, k, max_part_size);

            println!("\n=== Results ===");
            println!("Nodes: {}", hypergraph.num_nodes);
            println!("Hyperedges: {}", hypergraph.num_hyperedges);
            println!("Partitions (k): {}", k);
            println!("Epsilon: {}", epsilon);
            println!("Max allowed size: {}", max_part_size);
            println!("Connectivity (KM1): {}", connectivity);
            println!("Max partition size: {}", max_size);
            println!("Min partition size: {}", min_size);
            println!("Feasible: {}", if is_feasible { "YES" } else { "NO" });
        }
    }

    Ok(())
}

/// Generate seed using TIG's exact method:
/// seed = blake3(jsonify(BenchmarkSettings) + "_" + rand_hash + "_" + nonce)
/// 
/// We use fixed, reproducible values for BenchmarkSettings so anyone can verify:
/// - player_id: "benchmark_player"
/// - block_id: "benchmark_block"  
/// - challenge_id: "c004" (hypergraph challenge)
/// - algorithm_id: "sigma_freud_v5"
/// - track_id: derived from track size
fn generate_tig_seed(track: u32, nonce: u64) -> [u8; 32] {
    let track_id = match track {
        0..=15000 => "track_10k",
        15001..=30000 => "track_20k",
        30001..=75000 => "track_50k",
        75001..=150000 => "track_100k",
        _ => "track_200k",
    };
    
    // BenchmarkSettings JSON format (matching TIG's serde serialization order)
    let settings_json = format!(
        r#"{{"player_id":"benchmark_player","block_id":"benchmark_block","challenge_id":"c004","algorithm_id":"sigma_freud_v5","track_id":"{}"}}"#,
        track_id
    );
    
    // Use a fixed rand_hash for reproducibility
    let rand_hash = "0000000000000000000000000000000000000000000000000000000000000000";
    
    // TIG's seed formula: blake3(settings_json + "_" + rand_hash + "_" + nonce)
    let seed_input = format!("{}_{}_{}", settings_json, rand_hash, nonce);
    let hash: [u8; 32] = blake3::hash(seed_input.as_bytes()).into();
    
    hash
}

fn export_challenge_to_hgr(
    challenge: &tig_challenges::hypergraph::Challenge,
    path: &PathBuf,
    ctx: &cuda::GpuContext,
) -> Result<()> {
    use std::io::{BufWriter, Write};
    use std::fs::File;
    
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
