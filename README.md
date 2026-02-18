# sigma_freud: GPU-Accelerated Hypergraph Partitioner

A high-performance CUDA-based hypergraph partitioner that achieves **state-of-the-art partition quality** on large-scale instances while maintaining competitive runtime performance.

## Overview

This repository provides a standalone benchmark harness for `sigma_freud_v5`, a GPU-accelerated hypergraph partitioning algorithm developed for [TIG (The Innovation Game)](https://github.com/tig-foundation/tig-monorepo). The algorithm consistently outperforms Mt-KaHyPar—the current state-of-the-art parallel hypergraph partitioner—on large instances (50k+ hyperedges).

### Key Results

| Instance Size | Win Rate | Quality Improvement | Speedup vs Mt-KaHyPar |
|---------------|----------|--------------------|-----------------------|
| 10,000 hyperedges | 50% (5-5) | -0.28%* | 0.3x |
| 50,000 hyperedges | 60% (6-4) | **+0.41%** | 0.9x |
| 100,000 hyperedges | **70% (7-3)** | **+1.70%** | **1.2x faster** |
| 200,000 hyperedges | **100% (10-0)** | **+2.35%** | **1.6x faster** |

*Quality improvement = reduction in connectivity (KM1 metric). Positive means sigma_freud produces better partitions.*

**The algorithm's advantage grows with problem size**, achieving a perfect 10/10 win rate on 200k hyperedge instances with 2.35% better partition quality and 1.6x faster execution time.

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
| **CPU** | Intel Core i9 (16 threads used for Mt-KaHyPar) |
| **OS** | Ubuntu 24.04 (WSL2) |
| **CUDA** | 12.0 |
| **Rust** | 1.90.0 |

### Comparison Setup

- **sigma_freud_v5**: Single NVIDIA GPU, `effort=3` (600 refinement rounds)
- **Mt-KaHyPar**: 16 CPU threads, `quality` preset, connectivity objective

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
# Ubuntu/Debian
sudo apt-get install libboost-program-options-dev libhwloc-dev libtbb-dev

# Build Mt-KaHyPar from source
git clone https://github.com/kahypar/mt-kahypar.git
cd mt-kahypar
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
sudo make install
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
for track in 10000 50000 100000 200000; do
    echo "=== Track: $track hyperedges ==="
    
    # Generate and solve
    ./target/release/hg_bench gen \
        --track $track \
        --nonces 10 \
        --out /tmp/bench_${track} \
        --effort 3
    
    # Compare against Mt-KaHyPar
    python3 tools/compare_kahypar.py \
        "/tmp/bench_${track}/challenge_${track}_*.hgr" \
        --batch \
        --threads 16 \
        --preset quality
    
    echo ""
done
```

**Expected runtime:** ~10 minutes on RTX 5070 Ti + 16-thread CPU

## Algorithm Description

sigma_freud_v5 is a GPU-accelerated hypergraph partitioner that combines multiple optimization techniques:

### Phase 1: Initial Partitioning
- **Hyperedge clustering**: Groups similar hyperedges to guide initial node placement
- **Preference-based assignment**: Assigns nodes to partitions based on hyperedge connectivity patterns

### Phase 2: Refinement
- **GPU-parallel move computation**: Evaluates millions of potential node moves simultaneously
- **Gain-based selection**: Prioritizes moves that reduce connectivity
- **Tabu search**: Prevents cycling by temporarily forbidding recent moves

### Phase 3: Iterated Local Search (ILS)
- **Perturbation**: Escapes local optima by randomly relocating nodes
- **Intensification**: Deep refinement around promising solutions
- **Best-solution tracking**: Maintains the best partition found across iterations

### Phase 4: Balance Repair
- **Overflow handling**: Moves nodes from overweight blocks
- **Connectivity-aware**: Prefers moves that don't increase cut

### Key Optimizations
- **Coalesced GPU memory access**: Maximizes memory bandwidth utilization
- **Warp-level primitives**: Uses CUDA warp shuffle for fast reductions
- **Adaptive parameters**: Tunes refinement intensity based on instance characteristics

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
