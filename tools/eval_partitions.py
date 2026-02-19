#!/usr/bin/env python3
"""
Evaluate partitions against hypergraphs and compute KM1 connectivity.

Usage:
    python3 eval_partitions.py <hgr_folder> <partition_folder> [options]

Example:
    python3 eval_partitions.py /tmp/instances /tmp/sigma_partitions -v
"""

import argparse
import sys
from pathlib import Path


def read_hgr(path: str) -> tuple:
    """Read .hgr file and return (num_nodes, num_hyperedges, hyperedges)"""
    with open(path, 'r') as f:
        lines = f.readlines()
    
    header = lines[0].strip().split()
    num_hyperedges = int(header[0])
    num_nodes = int(header[1])
    
    hyperedges = []
    for line in lines[1:]:
        line = line.strip()
        if line:
            nodes = [int(x) - 1 for x in line.split()]  # Convert to 0-indexed
            hyperedges.append(nodes)
    
    return num_nodes, num_hyperedges, hyperedges


def read_partition(path: str) -> list:
    """Read partition file (one part ID per line)"""
    with open(path, 'r') as f:
        return [int(line.strip()) for line in f if line.strip()]


def compute_km1(hyperedges: list, partition: list) -> int:
    """
    Compute KM1 connectivity metric.
    KM1 = sum over all hyperedges of (lambda(e) - 1)
    where lambda(e) = number of distinct parts in hyperedge e
    """
    km1 = 0
    for he in hyperedges:
        parts = set(partition[node] for node in he)
        if len(parts) > 1:
            km1 += len(parts) - 1
    return km1


def compute_balance(partition: list, k: int) -> tuple:
    """
    Compute balance statistics.
    Returns (max_size, min_size, sizes_dict)
    """
    sizes = {}
    for part in partition:
        sizes[part] = sizes.get(part, 0) + 1
    
    all_sizes = [sizes.get(i, 0) for i in range(k)]
    return max(all_sizes), min(all_sizes), all_sizes


def validate_partition(partition: list, num_nodes: int, k: int) -> tuple:
    """
    Validate partition.
    Returns (is_valid, error_message)
    """
    if len(partition) != num_nodes:
        return False, f"Length mismatch: expected {num_nodes}, got {len(partition)}"
    
    for i, p in enumerate(partition):
        if p < 0 or p >= k:
            return False, f"Invalid label at node {i}: {p} (must be 0 to {k-1})"
    
    return True, None


def main():
    parser = argparse.ArgumentParser(description='Evaluate partitions and compute KM1')
    parser.add_argument('hgr_folder', help='Folder containing .hgr files')
    parser.add_argument('partition_folder', help='Folder containing partition files (.partition)')
    parser.add_argument('-k', type=int, default=64, help='Number of partitions (default: 64)')
    parser.add_argument('-e', type=float, default=0.03, help='Balance epsilon (default: 0.03)')
    parser.add_argument('-v', action='store_true', help='Show per-instance details')
    
    args = parser.parse_args()
    
    hgr_folder = Path(args.hgr_folder)
    partition_folder = Path(args.partition_folder)
    
    # Find .hgr files
    hgr_files = sorted(hgr_folder.glob('*.hgr'))
    
    if not hgr_files:
        print(f"Error: No .hgr files found in {hgr_folder}")
        sys.exit(1)
    
    print(f"Evaluating partitions")
    print(f"  HGR folder: {hgr_folder}")
    print(f"  Partition folder: {partition_folder}")
    print(f"  k={args.k}, epsilon={args.e}")
    print()
    
    total_km1 = 0
    valid_count = 0
    feasible_count = 0
    results = []
    
    for hgr_path in hgr_files:
        # Derive partition filename (same name, .partition extension)
        filename = hgr_path.stem
        partition_path = partition_folder / f"{filename}.partition"
        
        if not partition_path.exists():
            print(f"Warning: No partition for {filename}")
            continue
        
        # Read files
        num_nodes, num_hyperedges, hyperedges = read_hgr(str(hgr_path))
        partition = read_partition(str(partition_path))
        
        # Compute max allowed size
        max_allowed = int((num_nodes / args.k) * (1 + args.e) + 0.999999)
        
        # Validate
        is_valid, error = validate_partition(partition, num_nodes, args.k)
        if not is_valid:
            print(f"INVALID: {filename} - {error}")
            continue
        
        valid_count += 1
        
        # Compute metrics
        km1 = compute_km1(hyperedges, partition)
        max_size, min_size, _ = compute_balance(partition, args.k)
        is_feasible = max_size <= max_allowed
        
        if is_feasible:
            feasible_count += 1
        
        total_km1 += km1
        
        results.append({
            'name': filename,
            'nodes': num_nodes,
            'hyperedges': num_hyperedges,
            'km1': km1,
            'max_size': max_size,
            'max_allowed': max_allowed,
            'feasible': is_feasible
        })
        
        if args.v:
            status = "FEASIBLE" if is_feasible else "INFEASIBLE"
            print(f"{filename}: KM1={km1}, max_block={max_size}/{max_allowed}, {status}")
    
    print()
    print("=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Instances evaluated: {len(results)}")
    print(f"Valid partitions: {valid_count}")
    print(f"Feasible partitions: {feasible_count}")
    print(f"Total connectivity (KM1): {total_km1}")
    if results:
        print(f"Average connectivity: {total_km1 / len(results):.1f}")
    print()
    
    if feasible_count == len(results):
        print("All partitions are FEASIBLE")
    else:
        print(f"WARNING: {len(results) - feasible_count} partitions are INFEASIBLE")
    
    # Exit with error if any invalid/infeasible
    if feasible_count < len(results):
        sys.exit(2)
    if valid_count < len(hgr_files):
        sys.exit(1)


if __name__ == '__main__':
    main()
