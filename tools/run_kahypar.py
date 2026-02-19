#!/usr/bin/env python3
"""
Run Mt-KaHyPar on a folder of .hgr files and output partitions.

Usage:
    python3 run_kahypar.py <hgr_folder> <output_folder> [options]

Example:
    python3 run_kahypar.py /tmp/instances /tmp/kahypar_partitions -t 16 -p highest_quality
"""

import argparse
import sys
import time
from pathlib import Path

try:
    import mtkahypar
except ImportError:
    print("Error: mtkahypar not installed. Run: pip install mtkahypar")
    sys.exit(1)

# Global Mt-KaHyPar instance
_mtk_instance = None

def get_mtkahypar(num_threads=1):
    """Get or initialize Mt-KaHyPar instance (singleton to avoid warnings)."""
    global _mtk_instance
    if _mtk_instance is None:
        _mtk_instance = mtkahypar.initialize(num_threads)
    return _mtk_instance


def write_partition(path: str, partition: list):
    """Write partition to file (one part ID per line)"""
    with open(path, 'w') as f:
        for part in partition:
            f.write(f"{part}\n")


def run_kahypar(hgr_path: str, k: int, epsilon: float, num_threads: int, preset: str) -> tuple:
    """
    Run Mt-KaHyPar on a single .hgr file.
    Returns (partition, km1_connectivity, elapsed_time)
    """
    mtk = get_mtkahypar(num_threads)
    
    # Set up presets
    if preset == "highest_quality":
        context = mtk.context_from_preset(mtkahypar.PresetType.HIGHEST_QUALITY)
    elif preset == "quality":
        context = mtk.context_from_preset(mtkahypar.PresetType.QUALITY)
    else:
        context = mtk.context_from_preset(mtkahypar.PresetType.DEFAULT)
    
    # Set partitioning parameters
    context.set_partitioning_parameters(
        k,
        epsilon,
        mtkahypar.Objective.KM1  # Connectivity objective (lambda-1)
    )
    
    # Load hypergraph from file
    hypergraph = mtk.hypergraph_from_file(hgr_path, context)
    num_nodes = hypergraph.num_nodes()
    
    # Run partitioning
    start = time.perf_counter()
    partitioned_hg = hypergraph.partition(context)
    elapsed = time.perf_counter() - start
    
    # Get connectivity metric directly from Mt-KaHyPar
    km1 = partitioned_hg.km1()
    
    # Extract partition
    partition = [partitioned_hg.block_id(i) for i in range(num_nodes)]
    
    return partition, km1, elapsed


def main():
    parser = argparse.ArgumentParser(description='Run Mt-KaHyPar on .hgr files')
    parser.add_argument('hgr_folder', help='Input folder containing .hgr files')
    parser.add_argument('output_folder', help='Output folder for partition files')
    parser.add_argument('-k', type=int, default=64, help='Number of partitions (default: 64)')
    parser.add_argument('-e', type=float, default=0.03, help='Balance epsilon (default: 0.03)')
    parser.add_argument('-t', type=int, default=16, help='Number of threads (default: 16)')
    parser.add_argument('-p', default='highest_quality', 
                        choices=['default', 'quality', 'highest_quality'],
                        help='Mt-KaHyPar preset (default: highest_quality)')
    
    args = parser.parse_args()
    
    # Find .hgr files
    hgr_folder = Path(args.hgr_folder)
    output_folder = Path(args.output_folder)
    output_folder.mkdir(parents=True, exist_ok=True)
    
    hgr_files = sorted(hgr_folder.glob('*.hgr'))
    
    if not hgr_files:
        print(f"Error: No .hgr files found in {hgr_folder}")
        sys.exit(1)
    
    print(f"Found {len(hgr_files)} .hgr files in {hgr_folder}")
    print(f"Output folder: {output_folder}")
    print(f"Settings: k={args.k}, epsilon={args.e}, threads={args.t}, preset={args.p}")
    print()
    
    total_time = 0.0
    total_km1 = 0
    
    for i, hgr_path in enumerate(hgr_files):
        filename = hgr_path.stem
        print(f"[{i+1}/{len(hgr_files)}] {filename}... ", end='', flush=True)
        
        partition, km1, elapsed = run_kahypar(
            str(hgr_path), args.k, args.e, args.t, args.p
        )
        
        # Output with same name but .partition extension
        partition_path = output_folder / f"{filename}.partition"
        write_partition(str(partition_path), partition)
        
        # Write timing
        timing_path = output_folder / f"{filename}.time"
        with open(timing_path, 'w') as f:
            f.write(f"{elapsed:.3f}\n")
        
        print(f"KM1={km1}, time={elapsed:.2f}s")
        
        total_time += elapsed
        total_km1 += km1
    
    print()
    print("=== Summary ===")
    print(f"Instances: {len(hgr_files)}")
    print(f"Total connectivity: {total_km1}")
    print(f"Average connectivity: {total_km1 / len(hgr_files):.1f}")
    print(f"Total time: {total_time:.2f}s")
    print(f"Average time: {total_time / len(hgr_files):.2f}s")


if __name__ == '__main__':
    main()
