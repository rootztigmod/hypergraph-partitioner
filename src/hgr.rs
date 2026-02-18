use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub struct Hypergraph {
    pub num_nodes: u32,
    pub num_hyperedges: u32,
    pub hyperedge_offsets: Vec<i32>,
    pub hyperedge_nodes: Vec<i32>,
    pub node_offsets: Vec<i32>,
    pub node_hyperedges: Vec<i32>,
}

pub fn read_hgr(path: &Path) -> Result<Hypergraph> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let header = lines
        .next()
        .ok_or_else(|| anyhow!("Empty .hgr file"))??;
    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid .hgr header"));
    }

    let num_hyperedges: u32 = parts[0].parse()?;
    let num_nodes: u32 = parts[1].parse()?;

    let mut hyperedge_offsets: Vec<i32> = Vec::with_capacity(num_hyperedges as usize + 1);
    let mut hyperedge_nodes: Vec<i32> = Vec::new();

    hyperedge_offsets.push(0);

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        for node_str in line.split_whitespace() {
            let node: i32 = node_str.parse()?;
            hyperedge_nodes.push(node - 1);
        }
        hyperedge_offsets.push(hyperedge_nodes.len() as i32);
    }

    if hyperedge_offsets.len() != num_hyperedges as usize + 1 {
        return Err(anyhow!(
            "Expected {} hyperedges, found {}",
            num_hyperedges,
            hyperedge_offsets.len() - 1
        ));
    }

    let (node_offsets, node_hyperedges) = build_node_to_hyperedge(
        num_nodes as usize,
        &hyperedge_offsets,
        &hyperedge_nodes,
    );

    Ok(Hypergraph {
        num_nodes,
        num_hyperedges,
        hyperedge_offsets,
        hyperedge_nodes,
        node_offsets,
        node_hyperedges,
    })
}

fn build_node_to_hyperedge(
    num_nodes: usize,
    hyperedge_offsets: &[i32],
    hyperedge_nodes: &[i32],
) -> (Vec<i32>, Vec<i32>) {
    let mut node_degrees = vec![0i32; num_nodes];

    let num_hyperedges = hyperedge_offsets.len() - 1;
    for hedge in 0..num_hyperedges {
        let start = hyperedge_offsets[hedge] as usize;
        let end = hyperedge_offsets[hedge + 1] as usize;
        for &node in &hyperedge_nodes[start..end] {
            if (node as usize) < num_nodes {
                node_degrees[node as usize] += 1;
            }
        }
    }

    let mut node_offsets = vec![0i32; num_nodes + 1];
    for i in 0..num_nodes {
        node_offsets[i + 1] = node_offsets[i] + node_degrees[i];
    }

    let total_connections = node_offsets[num_nodes] as usize;
    let mut node_hyperedges = vec![0i32; total_connections];
    let mut node_current = vec![0i32; num_nodes];

    for hedge in 0..num_hyperedges {
        let start = hyperedge_offsets[hedge] as usize;
        let end = hyperedge_offsets[hedge + 1] as usize;
        for &node in &hyperedge_nodes[start..end] {
            let n = node as usize;
            if n < num_nodes {
                let pos = node_offsets[n] + node_current[n];
                node_hyperedges[pos as usize] = hedge as i32;
                node_current[n] += 1;
            }
        }
    }

    (node_offsets, node_hyperedges)
}

#[allow(dead_code)]
pub fn write_hgr(path: &Path, hypergraph: &Hypergraph) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{} {}", hypergraph.num_hyperedges, hypergraph.num_nodes)?;

    for i in 0..hypergraph.num_hyperedges as usize {
        let start = hypergraph.hyperedge_offsets[i] as usize;
        let end = hypergraph.hyperedge_offsets[i + 1] as usize;

        let nodes: Vec<String> = hypergraph.hyperedge_nodes[start..end]
            .iter()
            .map(|&n| (n + 1).to_string())
            .collect();

        writeln!(writer, "{}", nodes.join(" "))?;
    }

    writer.flush()?;
    Ok(())
}

pub fn read_partition(path: &Path) -> Result<Vec<u32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut partition = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            partition.push(line.parse()?);
        }
    }

    Ok(partition)
}

pub fn write_partition(path: &Path, partition: &[u32]) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for &part in partition {
        writeln!(writer, "{}", part)?;
    }

    writer.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn write_partition_with_timing(path: &Path, partition: &[u32], elapsed_secs: f64) -> Result<()> {
    write_partition(path, partition)?;

    let timing_path = path.with_extension("").to_string_lossy().to_string() + "_timing.txt";
    let mut timing_file = File::create(&timing_path)?;
    writeln!(timing_file, "{:.3}", elapsed_secs)?;

    Ok(())
}

pub fn compute_connectivity(hypergraph: &Hypergraph, partition: &[u32]) -> u32 {
    let mut connectivity = 0u32;

    for i in 0..hypergraph.num_hyperedges as usize {
        let start = hypergraph.hyperedge_offsets[i] as usize;
        let end = hypergraph.hyperedge_offsets[i + 1] as usize;

        let mut parts_in_edge: HashSet<u32> = HashSet::new();
        for &node in &hypergraph.hyperedge_nodes[start..end] {
            if (node as usize) < partition.len() {
                parts_in_edge.insert(partition[node as usize]);
            }
        }

        if parts_in_edge.len() > 1 {
            connectivity += (parts_in_edge.len() - 1) as u32;
        }
    }

    connectivity
}

pub fn check_feasibility(partition: &[u32], k: u32, max_part_size: u32) -> (bool, u32, u32) {
    let mut part_sizes = vec![0u32; k as usize];

    for &p in partition {
        if (p as usize) < part_sizes.len() {
            part_sizes[p as usize] += 1;
        }
    }

    let max_size = *part_sizes.iter().max().unwrap_or(&0);
    let min_size = *part_sizes.iter().min().unwrap_or(&0);

    let is_feasible = min_size >= 1 && max_size <= max_part_size;

    (is_feasible, max_size, min_size)
}
