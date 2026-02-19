#!/usr/bin/env python3
"""
Compare sigma_freud and Mt-KaHyPar results and print comparison.

Usage:
    python3 compare_results.py <hgr_folder> <sigma_folder> <kahypar_folder>
"""

import argparse
import sys
from pathlib import Path


def read_hgr_header(path: str) -> tuple:
    """Read .hgr file header and return (num_nodes, num_hyperedges)"""
    with open(path, 'r') as f:
        header = f.readline().strip().split()
    return int(header[1]), int(header[0])


def read_partition(path: str) -> list:
    """Read partition file"""
    with open(path, 'r') as f:
        return [int(line.strip()) for line in f if line.strip()]


def read_time(path: str) -> float:
    """Read timing file"""
    with open(path, 'r') as f:
        return float(f.readline().strip())


def compute_km1(hgr_path: str, partition: list) -> int:
    """Compute KM1 connectivity"""
    with open(hgr_path, 'r') as f:
        lines = f.readlines()
    
    km1 = 0
    for line in lines[1:]:
        line = line.strip()
        if line:
            nodes = [int(x) - 1 for x in line.split()]
            parts = set(partition[n] for n in nodes)
            if len(parts) > 1:
                km1 += len(parts) - 1
    return km1


def main():
    parser = argparse.ArgumentParser(description='Compare solver results')
    parser.add_argument('hgr_folder', help='Folder containing .hgr files')
    parser.add_argument('sigma_folder', help='Folder containing sigma_freud partitions')
    parser.add_argument('kahypar_folder', help='Folder containing kahypar partitions')
    
    args = parser.parse_args()
    
    hgr_folder = Path(args.hgr_folder)
    sigma_folder = Path(args.sigma_folder)
    kahypar_folder = Path(args.kahypar_folder)
    
    hgr_files = sorted(hgr_folder.glob('*.hgr'))
    
    if not hgr_files:
        print(f"Error: No .hgr files found in {hgr_folder}")
        sys.exit(1)
    
    results = []
    
    for hgr_path in hgr_files:
        name = hgr_path.stem
        
        sigma_part = sigma_folder / f"{name}.partition"
        sigma_time = sigma_folder / f"{name}.time"
        kahypar_part = kahypar_folder / f"{name}.partition"
        kahypar_time = kahypar_folder / f"{name}.time"
        
        if not all(p.exists() for p in [sigma_part, sigma_time, kahypar_part, kahypar_time]):
            continue
        
        sigma_partition = read_partition(str(sigma_part))
        kahypar_partition = read_partition(str(kahypar_part))
        
        sigma_km1 = compute_km1(str(hgr_path), sigma_partition)
        kahypar_km1 = compute_km1(str(hgr_path), kahypar_partition)
        
        sigma_t = read_time(str(sigma_time))
        kahypar_t = read_time(str(kahypar_time))
        
        results.append({
            'name': name,
            'sigma_km1': sigma_km1,
            'kahypar_km1': kahypar_km1,
            'sigma_time': sigma_t,
            'kahypar_time': kahypar_t,
        })
    
    if not results:
        print("No matching results found")
        sys.exit(1)
    
    # Print comparison table
    print()
    print("=" * 90)
    print("COMPARISON: sigma_freud vs Mt-KaHyPar")
    print("=" * 90)
    print(f"{'Instance':<30} {'sigma KM1':>12} {'kahypar KM1':>12} {'winner':>10} {'gap':>10}")
    print("-" * 90)
    
    sigma_wins = 0
    ties = 0
    
    for r in results:
        if r['sigma_km1'] < r['kahypar_km1']:
            winner = "sigma"
            sigma_wins += 1
        elif r['sigma_km1'] > r['kahypar_km1']:
            winner = "kahypar"
        else:
            winner = "tie"
            ties += 1
        
        gap = (r['sigma_km1'] - r['kahypar_km1']) / r['kahypar_km1'] * 100
        gap_str = f"{gap:+.2f}%"
        
        # Truncate name if too long
        name = r['name'][-28:] if len(r['name']) > 28 else r['name']
        
        print(f"{name:<30} {r['sigma_km1']:>12} {r['kahypar_km1']:>12} {winner:>10} {gap_str:>10}")
    
    # Summary
    total = len(results)
    kahypar_wins = total - sigma_wins - ties
    
    total_sigma_km1 = sum(r['sigma_km1'] for r in results)
    total_kahypar_km1 = sum(r['kahypar_km1'] for r in results)
    avg_gap = (total_sigma_km1 - total_kahypar_km1) / total_kahypar_km1 * 100
    
    avg_sigma_time = sum(r['sigma_time'] for r in results) / total
    avg_kahypar_time = sum(r['kahypar_time'] for r in results) / total
    speedup = avg_kahypar_time / avg_sigma_time if avg_sigma_time > 0 else 0
    
    print("-" * 90)
    print()
    print("SUMMARY")
    print(f"  Instances: {total}")
    print(f"  sigma_freud wins: {sigma_wins}/{total}")
    print(f"  kahypar wins: {kahypar_wins}/{total}")
    print(f"  ties: {ties}/{total}")
    print()
    print(f"  Total KM1 - sigma: {total_sigma_km1}, kahypar: {total_kahypar_km1}")
    print(f"  Average gap: {avg_gap:+.2f}% (negative = sigma better)")
    print()
    print(f"  Avg time - sigma: {avg_sigma_time:.2f}s, kahypar: {avg_kahypar_time:.2f}s")
    print(f"  Speedup: {speedup:.1f}x")
    print()


if __name__ == '__main__':
    main()
