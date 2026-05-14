# Sigma Freud V8: GPU-Accelerated Hypergraph Partitioner

A CUDA/Rust hypergraph partitioner that achieves lower KM1 connectivity than Mt-KaHyPar `highest_quality` on the tested TIG-style benchmark instances, while showing increasingly strong runtime scaling on larger tracks.

## Overview

This repository provides a standalone benchmark harness for Sigma Freud V8, a CUDA/Rust implementation of a GPU-accelerated hypergraph partitioning method developed for [TIG (The Innovation Game)](https://github.com/tig-foundation/tig-monorepo). The current harness uses the solver from `tig-algorithms/src/hypergraph/test`, with the same TIG challenge generation and local KM1 verification pipeline used for comparison against Mt-KaHyPar.

The benchmark claims in this README are specific to the TIG-style generated hypergraph instances and protocol described below. They should not be read as universal state-of-the-art claims across all public hypergraph partitioning benchmark suites.

## Advance Evidence Method

The candidate Advance method is **Deterministic Hyperedge-Consensus and Quota-Replayed Refinement for Balanced k-Way Hypergraph Partitioning**.

The submitted method is an abstract algorithmic method for balanced k-way hypergraph partitioning. It combines:

1. compact dual-bitmask KM1 gain estimation;
2. deterministic quota-bounded move selection and host replay;
3. balance-preserving swap and cycle refinement;
4. hyperedge-guided perturbation;
5. deterministic consensus/relinking against retained high-quality partitions or best-known assignments.

The CUDA/Rust implementation in this repository is one embodiment of that method, with track-specific parameterisation for different TIG instance sizes.

### Key Results (vs Mt-KaHyPar `highest_quality` preset)

Latest default-refinement results on 10 instances per measured track:

| Instance Size | Win Rate | Quality Improvement | Speedup |
|---------------|----------|---------------------|---------|
| 10,000 hyperedges | **100% (10-0)** | **+2.37%** | 0.05x speedup (Mt-KaHyPar faster) |
| 20,000 hyperedges | **100% (10-0)** | **+1.53%** | 0.9x speedup (near parity) |
| 50,000 hyperedges | **100% (10-0)** | **+3.79%** | **1.9x faster** |
| 100,000 hyperedges | **100% (10-0)** | **+3.64%** | **3.3x faster** |
| 200,000 hyperedges | **100% (10-0)** | **+4.52%** | **5.9x faster** |

*Quality improvement = mean reduction in connectivity (KM1 metric). Positive means Sigma Freud produces better partitions. Speedup is calculated against Mt-KaHyPar partition time; values below 1.0x mean Mt-KaHyPar is faster.*

**The latest recorded benchmark summary shows Sigma Freud at 50/50 quality wins against Mt-KaHyPar `highest_quality` across all TIG hypergraph tracks.** Full raw logs should be retained with any evidence package for auditability. The 10k and 20k tracks are quality wins with slower or near-parity runtime. At 50k and above, Sigma Freud wins on both quality and speed, reaching a 5.9x speedup on 200k.

Raw logs for each track should be stored under `results/raw_logs/` or an equivalent audit directory when this repository is used as an evidence package.

### Aggregate Results

| Metric | Result |
|--------|--------|
| Total tested instances | 50 |
| Record vs Mt-KaHyPar `highest_quality` | 50 wins / 0 ties / 0 losses |
| Feasible Sigma Freud partitions | 50 / 50 |
| Objective | KM1 / connectivity |
| Balance constraint | k = 64, epsilon = 0.03 |

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

Instances are generated using `tig_challenges::hypergraph::Challenge::generate_instance` directly from the TIG crate, ensuring identical instance generation to the TIG platform.

Seed derivation follows TIG's method:
```
seed = blake3(jsonify(BenchmarkSettings) + "_" + rand_hash + "_" + nonce)
```

This ensures anyone can regenerate identical instances for verification.

### Verification Pipeline

| Stage | Method |
|-------|--------|
| **Generation** | `tig-challenges` crate (`Challenge::generate_instance`) |
| **Export** | GPU buffers exported to standard hMETIS `.hgr` format |
| **Scoring** | Local KM1 computation from `.hgr` + partition files |
| **Feasibility** | Local balance check (max block ≤ allowed) |

The local scorer matches TIG's KM1 definition: `Σ(λ(e) - 1)` where `λ(e)` is the number of parts connected by hyperedge `e`.

**Note on TIG verification**: `tig-challenges` also exposes `evaluate_connectivity_metric()` for GPU-based scoring. If a cross-check against TIG's internal scorer is required, the instance can be regenerated from seed and evaluated directly. The local scorer provided here is transparent, auditable, and produces identical results.

### Test Environment

| Component | Specification |
|-----------|---------------|
| **GPU** | NVIDIA RTX 5070 Ti Laptop (12GB VRAM) |
| **CPU** | Intel Core Ultra 9 275HX; Mt-KaHyPar invoked with `--threads 16` |
| **OS** | Ubuntu 24.04 (WSL2) |
| **CUDA** | 12.0 |
| **Rust** | 1.90.0 |

### Comparison Setup

- **Sigma Freud V8**: Single NVIDIA GPU, default per-track refinement from the Rust track files
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

Record the installed Mt-KaHyPar package details alongside benchmark logs:

```bash
pip show mtkahypar
python3 - <<'PY'
import mtkahypar
print(mtkahypar.__version__ if hasattr(mtkahypar, "__version__") else mtkahypar)
PY
```

## Scoring / Validation

The built-in scorer validates partitions and computes KM1 connectivity. It is designed for transparent third-party verification.

```bash
./target/release/hg_bench score \
    --hgr /tmp/challenge_10000_<seed>.hgr \
    --partition /tmp/partition_10000_<seed>.txt \
    --k 64 \
    --epsilon 0.03
```

The scorer is intentionally strict:

- Partition length **must equal** the number of nodes
- Every label must be in the range **`[0, k-1]`**
- Balance constraint: `max_block <= ceil((n/k) * (1 + epsilon))`

**Exit Codes:**

| Code | Meaning |
|------|---------|
| 0 | Valid and feasible partition |
| 1 | Invalid input (wrong length, labels out of range) |
| 2 | Valid but infeasible (balance constraint violated) |

### Validation Examples

```bash
# Valid partition - exit 0
./target/release/hg_bench score --hgr /tmp/test.hgr --partition /tmp/valid.txt --k 64 --epsilon 0.03
# Output: Feasible: YES
# Exit code: 0

# Wrong length - exit 1
echo "0" > /tmp/bad.txt
./target/release/hg_bench score --hgr /tmp/test.hgr --partition /tmp/bad.txt --k 64 --epsilon 0.03
# ERROR: Invalid partition - Partition length mismatch: expected 8607 nodes, got 1
# Exit code: 1

# Invalid label - exit 1
./target/release/hg_bench score --hgr /tmp/test.hgr --partition /tmp/bad_label.txt --k 64 --epsilon 0.03
# ERROR: Invalid partition - Invalid partition label at node 100: 999 (must be < 64)
# Exit code: 1
```

## Standalone Tools

For verification and reproducibility, the pipeline is split into standalone components that can be audited independently.

### 1. Generate Instances (`gen_hgr`)

Generate TIG hypergraph instances as .hgr files:

```bash
# gen_hgr <size> <output_folder> [-n <count>] [-s <seed>]
./target/release/gen_hgr 100000 /tmp/instances -n 10 -s 0
```

**Output files:** `<size>_<seed_hex>_<i>.hgr` (e.g., `100000_a1b2c3d4_0.hgr`)

### 2. Run Sigma Freud (`run_sigma_freud`)

Solve a folder of .hgr files:

```bash
# run_sigma_freud <hgr_folder> <output_folder> [-k <partitions>] [-e <epsilon>] [-r <refinement>]
./target/release/run_sigma_freud /tmp/instances /tmp/sigma -k 64 -e 0.03 -r 2000
```

**Output files:** Same name as input, `.partition` extension (e.g., `100000_a1b2c3d4_0.partition`)

For default per-track refinement, prefer `hg_bench gen` without `--refinement`. The `run_sigma_freud` helper currently accepts `-r` and passes it as an explicit refinement override.

### 3. Run Mt-KaHyPar (`run_kahypar.py`)

Solve the same instances with Mt-KaHyPar:

```bash
# run_kahypar.py <hgr_folder> <output_folder> [-k <partitions>] [-e <epsilon>] [-t <threads>] [-p <preset>]
python3 tools/run_kahypar.py /tmp/instances /tmp/kahypar -t 16 -p highest_quality
```

**Output files:** Same name as input, `.partition` extension

### 4. Evaluate Partitions (`eval_partitions.py`)

Compute KM1 connectivity for any partition folder:

```bash
# eval_partitions.py <hgr_folder> <partition_folder> [-k <partitions>] [-e <epsilon>] [-v]
python3 tools/eval_partitions.py /tmp/instances /tmp/sigma -v
```

### 5. Compare Results (`compare_results.py`)

Compare sigma_freud and Mt-KaHyPar results side-by-side:

```bash
# compare_results.py <hgr_folder> <sigma_folder> <kahypar_folder>
python3 tools/compare_results.py /tmp/instances /tmp/sigma /tmp/kahypar
```

**Output:**
```
==========================================================================================
COMPARISON: Sigma Freud vs Mt-KaHyPar
==========================================================================================
Instance                          sigma KM1  kahypar KM1     winner        gap
------------------------------------------------------------------------------------------
100000_a1b2c3d4_0                    14523        14891      sigma     -2.47%
...

SUMMARY
  Instances: 10
  Sigma Freud wins: 8/10
  Average gap: -1.57% (negative = sigma better)
  Speedup: 4.9x
```

### 6. Benchmark Scripts

Convenience scripts that run the full pipeline for each instance size:

```bash
# Make scripts executable
chmod +x bench_*.sh

# Run a specific size (default: 10 instances)
./bench_100000.sh

# Run with custom settings: <n_instances> <refinement> <threads>
./bench_100000.sh 20 3000 16

# Run all sizes
./bench_all.sh 10 2000 16
```

Each `bench_<size>.sh` script:
1. Generates instances with `gen_hgr`
2. Solves with `run_sigma_freud`
3. Solves with `run_kahypar.py`
4. Evaluates both sets of partitions
5. Prints comparative results

This allows third parties to:
- Generate identical instances from the same seeds
- Run either solver independently
- Verify KM1 scores with simple, auditable Python code

The benchmark scripts pass an explicit refinement count. To reproduce the default-refinement results in this README, use the `hg_bench gen` commands in the "Reproducing Published Results" section instead.

---

## Usage (Combined Tool)

The `hg_bench` binary combines all functionality for convenience:

### Command Overview

```
hg_bench <COMMAND>

Commands:
  gen     Generate TIG instances, solve them, and export results
  file    Solve an existing .hgr file
  score   Verify a partition and compute metrics
```

### Generate and Benchmark

Generate instances using TIG's specification, solve with Sigma Freud, and export for comparison:

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
    --preset highest_quality
```

**Sample Output:**
```
======================================================================
SUMMARY
======================================================================
Instances: 10
Record: 10 wins / 0 ties / 0 losses
Gap: mean=-3.64%, median=-4.29% (negative = you're better)
Avg Mt-KaHyPar time: partition=31.42s
Avg Your time: 9.60s (speedup: 3.3x)
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
=== Results (local scorer) ===
Nodes: 84137
Hyperedges: 100000
Partitions (k): 64
Epsilon: 0.03
Max allowed size: 1355
Connectivity (KM1): 141234
Max partition size: 1355
Min partition size: 1298
Feasible: YES

Note: KM1 = Σ(λ(e)-1) where λ(e) = parts connected by hyperedge e
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
| Level | Use Case |
|-------|----------|
| 0 | Quick testing |
| 1 | Fast results |
| 2 | Balanced |
| 3 | Quality focus (latest benchmark default) |
| 4 | High quality |
| 5 | Maximum quality |

**Note:** Refinement budgets are track-specific and selected inside the solver's `track_*.rs` files from the chosen `effort`. The `--refinement` flag overrides those per-track defaults. Latest README results omit `--refinement`.

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

# Run all tracks with default per-track refinement
for track in 10000 20000 50000 100000 200000; do
    echo "=== Track: $track hyperedges ==="
    
    # Clean output directory before each run
    rm -rf /tmp/bench_${track}
    
    # Generate and solve with default per-track refinement at effort=3
    ./target/release/hg_bench gen \
        --track $track \
        --nonces 10 \
        --out /tmp/bench_${track} \
        --effort 3
    
    # Compare against Mt-KaHyPar highest_quality preset
    python3 tools/compare_kahypar.py \
        "/tmp/bench_${track}/challenge_${track}_*.hgr" \
        --batch \
        --threads 16 \
        --preset highest_quality
    
    echo ""
done
```

Do not pass `--refinement` if you want the solver to use the defaults embedded in each track file.

**Expected runtime:** varies by track on RTX 5070 Ti + 16-thread CPU; Mt-KaHyPar `highest_quality` dominates runtime for larger tracks.

## Algorithm Description

Sigma Freud V8 is a CUDA/Rust implementation of a deterministic capacity-aware method for balanced k-way hypergraph partitioning under the KM1/connectivity objective.

The implementation should not be read as claiming that hypergraph partitioning, GPU execution, FM-style refinement, tabu search, ILS, recombination, or swap moves are individually new. Those are known techniques. The claimed contribution is the specific method composition used here: compact dual-mask KM1 gain estimation, quota-bounded deterministic move replay, balance-preserving exchange refinement, and hyperedge-guided perturbation, combined in a pipeline tuned for the TIG k=64 balanced KM1 objective.

### Candidate Advance Method: Deterministic Hyperedge-Consensus and Quota-Replayed Refinement

The method combines compact dual-bitmask KM1 gain estimation, deterministic quota-bounded move selection and host replay, balance-preserving swap and cycle refinement, hyperedge-guided perturbation, and deterministic consensus/relinking against retained high-quality partitions or best-known assignments. The CUDA/Rust code in this repository is one implementation of that method, with track-specific parameterisation for different TIG instance sizes.

### Component: Dual Bitmask KM1 Gain Model

One core implementation component is a constant-time move gain computation for the KM1 (connectivity) objective using two precomputed bitmasks per hyperedge:

- **`edge_flags_all`**: Bitmask indicating which partitions have *any* node in this hyperedge
- **`edge_flags_double`**: Bitmask indicating which partitions have *two or more* nodes in this hyperedge

This allows O(1) detection of whether moving a node will add or remove a partition label from an incident hyperedge—the exact quantity KM1 measures. Traditional FM-style implementations maintain gain tables updated incrementally, requiring iteration over hyperedge members. The bitmask approach is particularly well-suited to GPU execution where bitwise operations are cheap and memory access patterns can be coalesced.

### Additional Techniques

#### Capacity-Aware Move Selection

Rather than greedily selecting top-gain moves (which tends to overfill attractive partitions), the solver enforces per-destination quotas derived from remaining balance slack. This systematically exploits the allowed imbalance budget (ε) while distributing improvements across partitions.

#### Tabu Search with Aspiration

The refinement loop uses tabu search to prevent cycling, with an aspiration criterion allowing high-gain moves to override tabu status. This is a standard metaheuristic technique adapted for GPU batch processing.

#### Balance-Neutral Swap Moves

Beyond single-node relocations, the solver uses balance-neutral swap moves where pairs of nodes in different partitions are exchanged simultaneously. This escapes local optima unreachable by standard single-node moves while preserving the balance constraint exactly. A 3-way cycle extension further generalises this to triplets (A→B, B→C, C→A), with best-gain tracking and early-break pruning for efficiency.

#### Hyperedge-Guided Perturbation

The ILS perturbation phase identifies high-connectivity hyperedges and preferentially relocates their nodes toward the majority partition. This focuses disruption on the most costly connectivity regions of the current solution, improving the quality of ILS restarts compared to uniform random perturbation.

#### Consensus / Relinking Against High-Quality Assignments

The solver retains high-quality assignments and uses deterministic consensus or relinking steps to bias new candidate partitions toward locally successful structures while preserving the balance constraint. In the current implementation, this includes per-hyperedge elite voting/relinking logic that reuses useful hyperedge-local structure without relying on nondeterministic crossover.

#### Deterministic GPU Pipeline

The solver achieves full reproducibility through parallel scoring on GPU followed by serial commit on host. This avoids atomic race conditions and ensures identical results across runs.

### Algorithm Phases

#### Phase 1: Initial Partitioning
- **Size-bucketed hyperedge clustering**: Groups hyperedges by size and hash signature to derive node-to-partition priors, emphasising small-edge coherence
- **Preference-based assignment**: Nodes assigned to partitions based on weighted voting from incident hyperedge clusters

#### Phase 2: Refinement
- **GPU-parallel move computation**: Evaluates millions of potential moves using the dual bitmask gain model
- **Quota-constrained selection**: Distributes moves across partitions respecting capacity limits
- **Adaptive move limits**: Move batch sizes vary by refinement phase (larger early, smaller late)
- **Swap phase**: Balance-neutral 2-way and 3-way cycle moves applied each round to escape local optima

#### Phase 3: Iterated Local Search (ILS)
- **Hyperedge-guided perturbation**: Disrupts high-connectivity regions to explore new solution basins
- **Quick refinement**: Short refinement bursts after perturbation to evaluate new basins
- **Best-solution tracking**: Maintains the globally best partition across all ILS iterations

#### Phase 4: Balance Repair
- **Overflow handling**: Moves nodes from overweight blocks prioritising low-connectivity-impact moves
- **Final polish**: Full swap phase followed by light refinement rounds to recover quality after balance repair

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
- Runtime advantage appears from 50k+ hyperedges under the published protocol, reaching 5.9x faster than Mt-KaHyPar on the 200k track while also improving KM1 quality.
- Single-GPU implementation (no multi-GPU support)

## Citation

If you use this work in academic research, please cite:

```
@software{sigma_freud_v8,
  author = {rootztigmod},
  title = {Sigma Freud V8: GPU-Accelerated Hypergraph Partitioner},
  year = {2026},
  url = {https://github.com/rootztigmod/hypergraph-partitioner}
}
```

## Related Work

The following prior work informed the design of Sigma Freud:

- **Fiduccia & Mattheyses (1982)** — "A Linear-Time Heuristic for Improving Network Partitions." *19th ACM/IEEE Design Automation Conference.* The foundational move-based refinement framework underlying the refinement loop.

- **Glover, F. (1989/1990)** — "Tabu Search — Part I & II." *ORSA Journal on Computing.* The tabu search mechanism with aspiration criterion used in the main refinement loop.

- **Lourenço, H.R., Martin, O.C. & Stützle, T. (2003)** — "Iterated Local Search." In *Handbook of Metaheuristics*, Springer. The ILS framework underpinning the perturbation and re-optimisation strategy.

- **Schlag et al. — "High-Quality Hypergraph Partitioning"**: describes KaHyPar, a high-quality multilevel hypergraph partitioner for cut and λ−1/connectivity objectives.

- **Gottesbüren et al. — "Scalable High-Quality Hypergraph Partitioning"**: describes Mt-KaHyPar, the shared-memory high-quality hypergraph partitioning framework used as the principal baseline in this repository.

- **KaHyPar-E and memetic hypergraph partitioning work**: prior work explores evolutionary, recombination, and mutation-style search for high-quality hypergraph partitioning. Sigma Freud does not claim novelty merely from using elite solutions, consensus/recombination, or perturbation concepts.

- **Wu et al. — "gHyPart: GPU-friendly End-to-End Hypergraph Partitioner"**: demonstrates that GPU hypergraph partitioning is known prior art. Sigma Freud does not claim novelty merely from executing hypergraph partitioning on a GPU.

The claimed novelty is not that hypergraph partitioning, FM-style refinement, tabu search, ILS, recombination, swap moves, or GPU execution are individually new. The claimed novelty is the specific combination and sequencing used here: compact dual-mask KM1 gain estimation, deterministic quota-bounded move replay, balance-preserving exchange refinement, and hyperedge-guided perturbation applied to the TIG balanced k-way KM1 objective.

## Acknowledgments

- [TIG Foundation](https://github.com/tig-foundation) for the hypergraph challenge specification
- [Mt-KaHyPar](https://github.com/kahypar/mt-kahypar) team for the excellent baseline partitioner
- [cudarc](https://github.com/coreylowman/cudarc) for Rust CUDA bindings

## License

MIT License - see [LICENSE](LICENSE) for details.
