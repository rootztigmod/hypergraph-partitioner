# sigma_freud: GPU-Accelerated Hypergraph Partitioner

A high-performance CUDA-based hypergraph partitioner that achieves **state-of-the-art partition quality** on large-scale instances while maintaining competitive runtime performance.

## Overview

This repository provides a standalone benchmark harness for `sigma_freud_v5`, a GPU-accelerated hypergraph partitioning algorithm developed for [TIG (The Innovation Game)](https://github.com/tig-foundation/tig-monorepo). The algorithm consistently outperforms Mt-KaHyPar—the current state-of-the-art parallel hypergraph partitioner—on large instances (50k+ hyperedges).

### Key Results (vs Mt-KaHyPar `highest_quality` preset)

| Instance Size | Win Rate | Quality Improvement | Speedup |
|---------------|----------|---------------------|---------|
| 10,000 hyperedges | 70% (7-3) | +0.08% | 0.4x |
| 20,000 hyperedges | 50% (5-5) | -0.56% | 0.6x |
| 50,000 hyperedges | **80% (8-2)** | **+1.66%** | **2.4x faster** |
| 100,000 hyperedges | **80% (8-2)** | **+1.54%** | **4.8x faster** |
| 200,000 hyperedges | **100% (10-0)** | **+2.64%** | **8.9x faster** |

*Quality improvement = mean reduction in connectivity (KM1 metric). Positive means sigma_freud produces better partitions.*

**The algorithm's advantage emerges at scale.** On smaller instances (10k-20k), Mt-KaHyPar is competitive or slightly better. At 50k+ hyperedges, sigma_freud consistently outperforms on both quality and speed, achieving a perfect 10/10 win rate on 200k instances with 2.64% better partition quality while running 8.9x faster.

## Problem Definition

Given a hypergraph H = (V, E) where:
- V = set of vertices (nodes)
- E = set of hyperedges (each hyperedge connects 2+ vertices)

Find a k-way partition of V into blocks {V₁, V₂, ..., Vₖ} that:
1. **Minimizes connectivity** (KM1 metric): Σₑ (λ(e) - 1), where λ(e) is the number of blocks connected by hyperedge e
2. **Satisfies balance constraint**: |Vᵢ| ≤ ⌈(|V|/k) × (1 + ε)⌉ for all blocks

### Benchmark Parameters

- **k = 64** partitions
- **ε = 0.03** (3% balance tolerance)
- **Objective**: Minimize connectivity (λ-1 / KM1 metric)
- **Vertex weights**: Unit (unweighted)

## Benchmark Methodology

### Instance Generation

Instances are generated using TIG's hypergraph challenge specification with deterministic seeding for reproducibility:

```
seed = blake3(jsonify(BenchmarkSettings) + "_" + rand_hash + "_" + nonce)
```

This ensures anyone can regenerate identical instances for verification.

### Test Environment

| Component | Specification |
|-----------|---------------|
| **GPU** | NVIDIA RTX 5070 Ti Laptop (12GB VRAM) |
| **CPU** | Intel Core Ultra 9 275HX (24 cores, 16 threads used for Mt-KaHyPar) |
| **OS** | Ubuntu 24.04 (WSL2) |
| **CUDA** | 12.0 |
| **Rust** | 1.90.0 |

### Comparison Setup

- **sigma_freud_v5**: Single NVIDIA GPU, `refinement=2000` (2000 refinement rounds)
- **Mt-KaHyPar**: 16 CPU threads, `highest_quality` preset, connectivity objective

Both solvers receive identical .hgr format hypergraphs and are measured on partition time only (excluding I/O).

## Installation

### Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| NVIDIA GPU | Turing+ (RTX 20 series or newer) | Compiled for `sm_75` architecture |
| CUDA Toolkit | 12.0+ | For GPU acceleration |
| Rust | 1.70+ | Build toolchain |
| Python | 3.8+ | Comparison scripts |
| Mt-KaHyPar | Latest | For baseline comparison |

### Building from Source

```bash
# Clone the repository
git clone https://github.com/rootztigmod/hypergraph-partitioner.git
cd hypergraph-partitioner

# Build release binary
cargo build --release

# Verify CUDA is accessible (WSL2 users)
export LD_LIBRARY_PATH=/usr/lib/wsl/lib:$LD_LIBRARY_PATH
./target/release/hg_bench gen --track 10000 --nonces 1 --out /tmp/test
```

### Installing Mt-KaHyPar (for comparison)

```bash
pip install mtkahypar
```

## Usage

### Command Overview

```
hg_bench <COMMAND>

Commands:
  gen     Generate TIG instances, solve them, and export results
  file    Solve an existing .hgr file
  score   Verify a partition and compute metrics
```

### Generate and Benchmark

Generate instances using TIG's specification, solve with sigma_freud, and export for comparison:

```bash
./target/release/hg_bench gen \
    --track 100000 \
    --nonces 10 \
    --out /tmp/benchmark \
    --effort 3
```

**Output:**
- `challenge_100000_<seed>.hgr` - Hypergraph in hMETIS format
- `partition_100000_<seed>.txt` - Partition assignment (one block ID per line)
- `partition_100000_<seed>_timing.txt` - Solver runtime in seconds

### Compare Against Mt-KaHyPar

```bash
python3 tools/compare_kahypar.py \
    "/tmp/benchmark/challenge_100000_*.hgr" \
    --batch \
    --threads 16 \
    --preset quality
```

**Sample Output:**
```
======================================================================
SUMMARY
======================================================================
Instances: 10
Record: 7 wins / 0 ties / 3 losses
Gap: mean=-1.70%, median=-2.09% (negative = you're better)
Avg Mt-KaHyPar time: partition=3.26s
Avg Your time: 2.74s (speedup: 1.2x)
All your partitions are FEASIBLE
```

### Solve Existing .hgr Files

Partition any hypergraph in hMETIS .hgr format:

```bash
./target/release/hg_bench file \
    --hgr /path/to/graph.hgr \
    --out /path/to/partition.txt \
    --k 64 \
    --epsilon 0.03 \
    --effort 3
```

### Verify Partition Quality

```bash
./target/release/hg_bench score \
    --hgr /path/to/graph.hgr \
    --partition /path/to/partition.txt \
    --k 64 \
    --epsilon 0.03
```

**Output:**
```
=== Results ===
Nodes: 84137
Hyperedges: 100000
Partitions (k): 64
Epsilon: 0.03
Max allowed size: 1355
Connectivity (KM1): 141234
Max partition size: 1355
Min partition size: 1298
Feasible: YES
```

## CLI Reference

### `gen` Command

| Option | Description | Default |
|--------|-------------|---------|
| `-t, --track <N>` | Target hyperedge count (10000, 20000, 50000, 100000, 200000) | Required |
| `-n, --nonces <N>` | Number of instances to generate | 10 |
| `-o, --out <DIR>` | Output directory | Required |
| `-e, --effort <0-5>` | Quality/speed tradeoff (higher = better quality, slower) | 2 |
| `-r, --refinement <N>` | Override refinement iterations | Auto (based on effort) |

**Effort Levels:**
| Level | Refinement Rounds | Use Case |
|-------|------------------|----------|
| 0 | 300 | Quick testing |
| 1 | 400 | Fast results |
| 2 | 500 | Balanced (default) |
| 3 | 600 | Quality focus |
| 4 | 800 | High quality |
| 5 | 1000 | Maximum quality |

**Note:** The `--refinement` flag overrides effort-based defaults. Published benchmarks use `--refinement 2000`. For maximum quality, values up to 5000 can be used at the cost of increased runtime.

### `file` Command

| Option | Description | Default |
|--------|-------------|---------|
| `--hgr <FILE>` | Input hypergraph (.hgr format) | Required |
| `-o, --out <FILE>` | Output partition file | Required |
| `-k <N>` | Number of partitions | 64 |
| `-e, --epsilon <F>` | Balance tolerance (0.0-1.0) | 0.03 |
| `--effort <0-5>` | Quality level | 2 |
| `--refinement <N>` | Override refinement iterations | Auto |

### `score` Command

| Option | Description | Default |
|--------|-------------|---------|
| `--hgr <FILE>` | Input hypergraph | Required |
| `--partition <FILE>` | Partition to verify | Required |
| `-k <N>` | Expected partition count | 64 |
| `-e, --epsilon <F>` | Balance tolerance | 0.03 |

## Reproducing Published Results

To reproduce the benchmark results from this README:

```bash
# Ensure CUDA is accessible
export LD_LIBRARY_PATH=/usr/lib/wsl/lib:$LD_LIBRARY_PATH

# Run complete benchmark suite
for track in 10000 20000 50000 100000 200000; do
    echo "=== Track: $track hyperedges ==="
    
    # Clean output directory before each run
    rm -rf /tmp/bench_${track}
    
    # Generate and solve with refinement=2000
    ./target/release/hg_bench gen \
        --track $track \
        --nonces 10 \
        --out /tmp/bench_${track} \
        --refinement 2000
    
    # Compare against Mt-KaHyPar highest_quality preset
    python3 tools/compare_kahypar.py \
        "/tmp/bench_${track}/challenge_${track}_*.hgr" \
        --batch \
        --threads 16 \
        --preset highest_quality
    
    echo ""
done
```

**Expected runtime:** ~45 minutes on RTX 5070 Ti + 16-thread CPU (Mt-KaHyPar `highest_quality` is compute-intensive)

## Algorithm Description

sigma_freud_v5 is a GPU-accelerated hypergraph partitioner that combines multiple optimization techniques:

### Novel Contribution: Dual Bitmask KM1 Gain Model

The core algorithmic innovation is a constant-time move gain computation for the KM1 (connectivity) objective using two precomputed bitmasks per hyperedge:

- **`edge_flags_all`**: Bitmask indicating which partitions have *any* node in this hyperedge
- **`edge_flags_double`**: Bitmask indicating which partitions have *two or more* nodes in this hyperedge

This allows O(1) detection of whether moving a node will add or remove a partition label from an incident hyperedge—the exact quantity KM1 measures. Traditional FM-style implementations maintain gain tables updated incrementally, requiring iteration over hyperedge members. The bitmask approach is particularly well-suited to GPU execution where bitwise operations are cheap and memory access patterns can be coalesced.

### Additional Techniques

#### Capacity-Aware Move Selection

Rather than greedily selecting top-gain moves (which tends to overfill attractive partitions), the solver enforces per-destination quotas derived from remaining balance slack. This systematically exploits the allowed imbalance budget (ε) while distributing improvements across partitions.

#### Tabu Search with Aspiration

The refinement loop uses tabu search to prevent cycling, with an aspiration criterion allowing high-gain moves to override tabu status. This is a standard metaheuristic technique adapted for GPU batch processing.

#### Deterministic GPU Pipeline

The solver achieves full reproducibility through parallel scoring on GPU followed by serial commit on host. This avoids atomic race conditions and ensures identical results across runs.

### Algorithm Phases

#### Phase 1: Initial Partitioning
- **Size-bucketed hyperedge clustering**: Groups hyperedges by size and hash signature to derive node-to-partition priors, emphasizing small-edge coherence
- **Preference-based assignment**: Nodes assigned to partitions based on weighted voting from incident hyperedge clusters

#### Phase 2: Refinement
- **GPU-parallel move computation**: Evaluates millions of potential moves using the dual bitmask gain model
- **Quota-constrained selection**: Distributes moves across partitions respecting capacity limits
- **Adaptive move limits**: Move batch sizes vary by refinement phase (larger early, smaller late)

#### Phase 3: Iterated Local Search (ILS)
- **Controlled perturbation**: Escapes local optima by relocating a fraction of nodes
- **Quick refinement**: Short refinement bursts after perturbation to evaluate new basins
- **Best-solution tracking**: Maintains the globally best partition across all ILS iterations

#### Phase 4: Balance Repair
- **Overflow handling**: Moves nodes from overweight blocks prioritizing low-connectivity-impact moves
- **Final polish**: Light refinement rounds to recover any quality lost during balance repair

## File Formats

### Input: hMETIS .hgr Format

```
<num_hyperedges> <num_nodes>
<nodes in hyperedge 1 (1-indexed, space-separated)>
<nodes in hyperedge 2>
...
```

**Example:**
```
3 5
1 2 3
2 3 4
4 5
```

### Output: Partition Format

One line per node, containing the block ID (0 to k-1):

```
0
0
1
1
2
...
```

## Limitations

- Requires NVIDIA GPU (Turing architecture or newer: RTX 20/30/40/50 series, GTX 16 series)
- Optimized for k=64 partitions (TIG challenge specification)
- Best performance on instances with 50k+ hyperedges
- Single-GPU implementation (no multi-GPU support)

## Citation

If you use this work in academic research, please cite:

```
@software{sigma_freud_v5,
  author = {rootztigmod},
  title = {sigma_freud: GPU-Accelerated Hypergraph Partitioner},
  year = {2026},
  url = {https://github.com/rootztigmod/hypergraph-partitioner}
}
```

## Acknowledgments

- [TIG Foundation](https://github.com/tig-foundation) for the hypergraph challenge specification
- [Mt-KaHyPar](https://github.com/kahypar/mt-kahypar) team for the excellent baseline partitioner
- [cudarc](https://github.com/coreylowman/cudarc) for Rust CUDA bindings

## License

MIT License - see [LICENSE](LICENSE) for details.
