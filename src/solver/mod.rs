use crate::cuda::GpuContext;
use crate::hgr::Hypergraph;
use anyhow::Result;
use cudarc::driver::CudaSlice;
use serde_json::{Map, Value};
use tig_challenges::hypergraph::{Challenge, Solution};

pub mod track_10k;
pub mod track_20k;
pub mod track_50k;
pub mod track_100k;
pub mod track_200k;

pub fn solve(
    hypergraph: &Hypergraph,
    k: u32,
    max_part_size: u32,
    effort: u32,
    refinement: Option<u32>,
) -> Result<Vec<u32>> {
    let ctx = GpuContext::new()?;
    
    let challenge = hypergraph_to_challenge(hypergraph, k, max_part_size, &ctx)?;
    
    let mut hyperparameters: Map<String, Value> = Map::new();
    hyperparameters.insert("effort".to_string(), Value::Number(effort.into()));
    if let Some(r) = refinement {
        hyperparameters.insert("refinement".to_string(), Value::Number(r.into()));
    }
    let hyperparameters = Some(hyperparameters);
    
    let final_partition: std::cell::RefCell<Vec<u32>> = std::cell::RefCell::new(Vec::new());
    
    let save_solution = |solution: &Solution| -> anyhow::Result<()> {
        *final_partition.borrow_mut() = solution.partition.clone();
        Ok(())
    };
    
    match hypergraph.num_hyperedges {
        0..=15000 => track_10k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
        15001..=30000 => track_20k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
        30001..=75000 => track_50k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
        75001..=150000 => track_100k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
        _ => track_200k::solve(&challenge, &save_solution, &hyperparameters, ctx.module.clone(), ctx.stream.clone(), &ctx.prop)?,
    }
    
    let result = final_partition.borrow().clone();
    Ok(result)
}

fn hypergraph_to_challenge(
    hg: &Hypergraph,
    k: u32,
    max_part_size: u32,
    ctx: &GpuContext,
) -> Result<Challenge> {
    let d_hyperedge_offsets: CudaSlice<i32> = ctx.stream.memcpy_stod(&hg.hyperedge_offsets)?;
    let d_hyperedge_nodes: CudaSlice<i32> = ctx.stream.memcpy_stod(&hg.hyperedge_nodes)?;
    let d_node_offsets: CudaSlice<i32> = ctx.stream.memcpy_stod(&hg.node_offsets)?;
    let d_node_hyperedges: CudaSlice<i32> = ctx.stream.memcpy_stod(&hg.node_hyperedges)?;
    
    let mut hyperedge_sizes: Vec<i32> = Vec::with_capacity(hg.num_hyperedges as usize);
    for i in 0..hg.num_hyperedges as usize {
        let size = hg.hyperedge_offsets[i + 1] - hg.hyperedge_offsets[i];
        hyperedge_sizes.push(size);
    }
    let d_hyperedge_sizes: CudaSlice<i32> = ctx.stream.memcpy_stod(&hyperedge_sizes)?;
    
    let mut node_degrees: Vec<i32> = Vec::with_capacity(hg.num_nodes as usize);
    for i in 0..hg.num_nodes as usize {
        let degree = hg.node_offsets[i + 1] - hg.node_offsets[i];
        node_degrees.push(degree);
    }
    let d_node_degrees: CudaSlice<i32> = ctx.stream.memcpy_stod(&node_degrees)?;
    
    let total_connections = hg.hyperedge_nodes.len() as u32;
    
    Ok(Challenge {
        seed: [0u8; 32],
        num_hyperedges: hg.num_hyperedges,
        num_nodes: hg.num_nodes,
        num_parts: k,
        max_part_size,
        total_connections,
        d_hyperedge_sizes,
        d_hyperedge_offsets,
        d_hyperedge_nodes,
        d_node_degrees,
        d_node_offsets,
        d_node_hyperedges,
        greedy_baseline_connectivity_metric: 0,
    })
}
