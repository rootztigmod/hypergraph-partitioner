#!/bin/bash
# Benchmark script for 20,000 hyperedge instances
set -e

SIZE=20000
N=${1:-10}
REFINEMENT=${2:-2000}
THREADS=${3:-16}

BENCH_DIR="/tmp/bench_${SIZE}"
HGR_DIR="${BENCH_DIR}/hgr"
SIGMA_DIR="${BENCH_DIR}/sigma"
KAHYPAR_DIR="${BENCH_DIR}/kahypar"

echo "=============================================="
echo "Benchmark: ${SIZE} hyperedges"
echo "Instances: ${N}"
echo "Refinement: ${REFINEMENT}"
echo "Threads: ${THREADS}"
echo "=============================================="
echo

# Clean up
rm -rf "${BENCH_DIR}"
mkdir -p "${HGR_DIR}" "${SIGMA_DIR}" "${KAHYPAR_DIR}"

# 1. Generate instances
echo "=== Step 1: Generating instances ==="
./target/release/gen_hgr ${SIZE} "${HGR_DIR}" -n ${N}
echo

# 2. Run sigma_freud
echo "=== Step 2: Running sigma_freud ==="
./target/release/run_sigma_freud "${HGR_DIR}" "${SIGMA_DIR}" -r ${REFINEMENT}
echo

# 3. Run Mt-KaHyPar
echo "=== Step 3: Running Mt-KaHyPar (highest_quality) ==="
python3 tools/run_kahypar.py "${HGR_DIR}" "${KAHYPAR_DIR}" -t ${THREADS} -p highest_quality
echo

# 4. Compare results
echo "=== Step 4: Comparing results ==="
python3 tools/compare_results.py "${HGR_DIR}" "${SIGMA_DIR}" "${KAHYPAR_DIR}"

echo "=============================================="
echo "Benchmark complete. Results in ${BENCH_DIR}"
echo "=============================================="
