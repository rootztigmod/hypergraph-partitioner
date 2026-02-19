#!/bin/bash
# Master benchmark script - runs all sizes
set -e

N=${1:-10}
REFINEMENT=${2:-2000}
THREADS=${3:-16}

echo "=============================================="
echo "FULL BENCHMARK SUITE"
echo "Instances per size: ${N}"
echo "Refinement: ${REFINEMENT}"
echo "Threads: ${THREADS}"
echo "=============================================="
echo

./bench_10000.sh ${N} ${REFINEMENT} ${THREADS}
./bench_20000.sh ${N} ${REFINEMENT} ${THREADS}
./bench_50000.sh ${N} ${REFINEMENT} ${THREADS}
./bench_100000.sh ${N} ${REFINEMENT} ${THREADS}
./bench_200000.sh ${N} ${REFINEMENT} ${THREADS}

echo
echo "=============================================="
echo "ALL BENCHMARKS COMPLETE"
echo "=============================================="
