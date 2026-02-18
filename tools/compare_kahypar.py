#!/usr/bin/env python3
"""
Compare TIG hypergraph solutions against Mt-KaHyPar (SOTA).

Prerequisites:
    pip install mtkahypar

Usage:
    python compare_kahypar.py challenge.hgr --partition your_partition.txt
    python batch_compare.py /tmp/challenge_50000_*.hgr  # batch mode
"""

import sys
import time
import argparse
import glob
import os
import math

try:
    import mtkahypar
except ImportError:
    print("ERROR: mtkahypar not installed. Run: pip install mtkahypar")
    sys.exit(1)

# Global Mt-KaHyPar instance
_mtk_instance = None

def get_mtkahypar(num_threads=1):
    """Get or initialize Mt-KaHyPar instance (singleton to avoid warnings)."""
    global _mtk_instance
    if _mtk_instance is None:
        _mtk_instance = mtkahypar.initialize(num_threads)
    return _mtk_instance


def read_partition(filepath):
    """Read partition file (one part ID per line)."""
    with open(filepath, 'r') as f:
        return [int(line.strip()) for line in f]


def read_hgr_header(filepath):
    """Read .hgr header to get num_hyperedges and num_nodes."""
    with open(filepath, 'r') as f:
        header = f.readline().strip().split()
        return int(header[0]), int(header[1])


def compute_connectivity(filepath, partition):
    """Read .hgr file and compute connectivity for given partition."""
    connectivity = 0
    with open(filepath, 'r') as f:
        header = f.readline().strip().split()
        num_hyperedges = int(header[0])
        
        for _ in range(num_hyperedges):
            nodes = [int(x) - 1 for x in f.readline().strip().split()]  # 0-indexed as per literature
            parts_in_edge = set(partition[node] for node in nodes)
            connectivity += len(parts_in_edge) - 1
    
    return connectivity


def check_feasibility(partition, num_nodes, num_parts, epsilon, use_tig_convention=True):
    """
    Check if partition is feasible under balance constraint.
    
    Two conventions:
    - TIG: max_part_size = ceil((num_nodes / num_parts) * (1 + epsilon))
    - Mt-KaHyPar: max_block_weight = ceil(num_nodes / num_parts) * (1 + epsilon)
    
    Returns (is_feasible, max_imbalance, max_part_size, allowed_max, min_size, msg).
    """
    part_sizes = [0] * num_parts
    for p in partition:
        if 0 <= p < num_parts:
            part_sizes[p] += 1
    
    if use_tig_convention:
        # Same as TIG: ceil((num_nodes / num_parts) * 1.03)
        allowed_max = math.ceil((num_nodes / num_parts) * (1 + epsilon))
    else:
        # Mt-KaHyPar: ceil(num_nodes / num_parts) * (1 + epsilon) - from literature
        base = math.ceil(num_nodes / num_parts)
        allowed_max = int(base * (1 + epsilon))
    
    avg_size = num_nodes / num_parts
    max_size = max(part_sizes)
    min_size = min(part_sizes)
    
    # Check all parts have at least 1 node
    if min_size < 1:
        return False, float('inf'), max_size, allowed_max, min_size, "Empty partition found"
    
    # Check max size 
    if max_size > allowed_max:
        imbalance = (max_size - avg_size) / avg_size
        return False, imbalance, max_size, allowed_max, min_size, f"Max size {max_size} > allowed {allowed_max}"
    
    imbalance = (max_size - avg_size) / avg_size
    return True, imbalance, max_size, allowed_max, min_size, "OK"


def run_kahypar(hgr_file, num_parts=64, epsilon=0.03, num_threads=1, preset="quality"):
    """Run Mt-KaHyPar on .hgr file and return connectivity + times."""
    
    # Get the Mt-KaHyPar instance
    mtk = get_mtkahypar(num_threads)
    
    # Set up presets
    if preset == "highest_quality":
        context = mtk.context_from_preset(mtkahypar.PresetType.HIGHEST_QUALITY)
    elif preset == "quality":
        context = mtk.context_from_preset(mtkahypar.PresetType.QUALITY)
    else:
        context = mtk.context_from_preset(mtkahypar.PresetType.DEFAULT)
    
    # Set teh partitioning parameters
    context.set_partitioning_parameters(
        num_parts,
        epsilon,
        mtkahypar.Objective.KM1  # Connectivity objective (lambda-1)
    )
    
    # Load hypergraph from file
    t0 = time.perf_counter()
    hypergraph = mtk.hypergraph_from_file(hgr_file, context)
    load_time = max(0.0, time.perf_counter() - t0)  # Ensure it's non-negative
    num_nodes = hypergraph.num_nodes()
    
    # Run the partitioning 
    t1 = time.perf_counter()
    partitioned_hg = hypergraph.partition(context)
    partition_time = max(0.0, time.perf_counter() - t1)  # Ensure it's non-negative
    
    total_time = load_time + partition_time
    
    # Get connectivity metric directly from Mt-KaHyPar
    connectivity = partitioned_hg.km1()
    
    # Extract partition for verifying 
    partition = [partitioned_hg.block_id(i) for i in range(num_nodes)]
    
    return partition, connectivity, load_time, partition_time, total_time, num_nodes


def read_timing(partition_file):
    """Read timing from companion _timing.txt file."""
    timing_file = partition_file.replace('.txt', '_timing.txt')
    if os.path.exists(timing_file):
        with open(timing_file, 'r') as f:
            return float(f.readline().strip())
    return None


def compare_single(hgr_file, partition_file, num_parts=64, epsilon=0.03, 
                   num_threads=1, preset="quality", verbose=True):
    """Compare a single instance. Returns dict with results."""
    
    num_hyperedges, num_nodes = read_hgr_header(hgr_file)
    
    # Run Mt-KaHyPar
    kahypar_partition, kahypar_conn, kp_load_time, kp_partition_time, kp_total_time, _ = run_kahypar(
        hgr_file, num_parts, epsilon, num_threads, preset
    )
    
    # Check Mt-KaHyPar feasibility
    kp_feasible, kp_imbalance, kp_max, kp_allowed, kp_min, kp_msg = check_feasibility(
        kahypar_partition, num_nodes, num_parts, epsilon
    )
    
    result = {
        'hgr_file': hgr_file,
        'num_nodes': num_nodes,
        'num_hyperedges': num_hyperedges,
        'kahypar_conn': kahypar_conn,
        'kahypar_load_time': kp_load_time,
        'kahypar_partition_time': kp_partition_time,
        'kahypar_total_time': kp_total_time,
        'kahypar_feasible': kp_feasible,
        'kahypar_imbalance': kp_imbalance,
        'kahypar_max_size': kp_max,
        'allowed_max': kp_allowed,
    }
    
    if partition_file and os.path.exists(partition_file):
        your_partition = read_partition(partition_file)
        your_conn = compute_connectivity(hgr_file, your_partition)
        your_time = read_timing(partition_file)
        
        # Check sigma_freud_v5 feasibility
        your_feasible, your_imbalance, your_max, your_allowed, your_min, your_msg = check_feasibility(
            your_partition, num_nodes, num_parts, epsilon
        )
        
        diff = your_conn - kahypar_conn
        pct = (diff / kahypar_conn) * 100 if kahypar_conn > 0 else 0
        
        result.update({
            'your_conn': your_conn,
            'your_time': your_time,
            'your_feasible': your_feasible,
            'your_imbalance': your_imbalance,
            'your_max_size': your_max,
            'diff': diff,
            'pct': pct,
            'winner': 'YOU' if your_conn < kahypar_conn else ('TIE' if your_conn == kahypar_conn else 'KaHyPar'),
        })
        
        if verbose:
            print(f"  Nodes: {num_nodes}, Hyperedges: {num_hyperedges}, MaxBlockAllowed: {kp_allowed}")
            time_str = f", time={your_time:.2f}s" if your_time else ""
            print(f"  Mt-KaHyPar ({preset}): conn={kahypar_conn}, load={kp_load_time:.2f}s, partition={kp_partition_time:.2f}s, total={kp_total_time:.2f}s, maxBlk={kp_max}/{kp_allowed}")
            print(f"  Your algo: conn={your_conn}{time_str}, maxBlk={your_max}/{your_allowed}")
            if not your_feasible:
                print(f"  WARNING: Your partition is INFEASIBLE - {your_msg}")
            if not kp_feasible:
                print(f"  WARNING: Mt-KaHyPar partition is INFEASIBLE - {kp_msg}")
            print(f"  Diff: {diff:+d} ({pct:+.2f}%) -> {result['winner']}")
    else:
        if verbose:
            print(f"  Nodes: {num_nodes}, Hyperedges: {num_hyperedges}")
            print(f"  Mt-KaHyPar ({preset}): conn={kahypar_conn}, time={kahypar_time:.2f}s")
    
    return result


def batch_compare(pattern, num_parts=64, epsilon=0.03, num_threads=1, preset="quality"):
    """Batch compare all files matching pattern."""
    
    hgr_files = sorted(glob.glob(pattern))
    if not hgr_files:
        print(f"No files found matching: {pattern}")
        return
    
    print(f"Found {len(hgr_files)} hypergraph files")
    print(f"Settings: k={num_parts}, epsilon={epsilon}, threads={num_threads}, preset={preset}")
    print(f"Objective: connectivity (lambda-1 metric)")
    print("=" * 70)
    
    results = []
    your_wins = 0
    your_ties = 0
    your_losses = 0
    your_total = 0
    
    for hgr_file in hgr_files:
        # Derive partition file from hgr file
        # /tmp/challenge_50000_abc123.hgr -> /tmp/partition_50000_abc123.txt
        base = os.path.basename(hgr_file)
        seed = base.replace('challenge_', '').replace('.hgr', '')
        partition_file = os.path.join(os.path.dirname(hgr_file), f"partition_{seed}.txt")
        
        print(f"\n{base}:")
        result = compare_single(hgr_file, partition_file, num_parts, epsilon, num_threads, preset)
        results.append(result)
        
        if 'your_conn' in result:
            your_total += 1
            if result['winner'] == 'YOU':
                your_wins += 1
            elif result['winner'] == 'TIE':
                your_ties += 1
            else:
                your_losses += 1
    
    # Results summary
    print("\n" + "=" * 70)
    print("SUMMARY")
    print("=" * 70)
    
    if your_total > 0:
        pcts = [r['pct'] for r in results if 'pct' in r]
        avg_pct = sum(pcts) / your_total
        median_pct = sorted(pcts)[len(pcts) // 2]
        
        avg_kp_load = sum(r['kahypar_load_time'] for r in results) / len(results)
        avg_kp_partition = sum(r['kahypar_partition_time'] for r in results) / len(results)
        avg_kp_total = sum(r['kahypar_total_time'] for r in results) / len(results)
        
        # Calculate the average time if available
        your_times = [r['your_time'] for r in results if r.get('your_time') is not None]
        avg_your_time = sum(your_times) / len(your_times) if your_times else None
        
        print(f"Instances: {your_total}")
        print(f"Record: {your_wins} wins / {your_ties} ties / {your_losses} losses")
        print(f"Gap: mean={avg_pct:+.2f}%, median={median_pct:+.2f}% (negative = you're better)")
        print(f"Avg Mt-KaHyPar time: load={avg_kp_load:.2f}s, partition={avg_kp_partition:.2f}s, total={avg_kp_total:.2f}s")
        if avg_your_time:
            speedup_partition = avg_kp_partition / avg_your_time if avg_your_time > 0 else 0
            speedup_total = avg_kp_total / avg_your_time if avg_your_time > 0 else 0
            print(f"Avg Your time: {avg_your_time:.2f}s (partition speedup: {speedup_partition:.1f}x, total speedup: {speedup_total:.1f}x)")
            
            # Quality efficiency: how much better vs Mt-KaHyPar per second of compute
            if avg_pct < 0:  # Only show if sigma_freud_v5 is winning on quality
                quality_per_sec = abs(avg_pct) / avg_your_time
                print(f"Quality efficiency: {quality_per_sec:.3f}% improvement vs Mt-KaHyPar per second of your compute")
        
        # Check feasibility
        your_infeasible = sum(1 for r in results if 'your_feasible' in r and not r['your_feasible'])
        kp_infeasible = sum(1 for r in results if not r.get('kahypar_feasible', True))
        
        if your_infeasible > 0:
            print(f"WARNING: {your_infeasible} of your partitions are INFEASIBLE!")
        else:
            print(f"All your partitions are FEASIBLE")
        
        if kp_infeasible > 0:
            print(f"WARNING: {kp_infeasible} Mt-KaHyPar partitions are INFEASIBLE!")
        
        # Print verification
        print(f"\nConstraint verification:")
        print(f"  k = {num_parts} partitions")
        print(f"  epsilon = {epsilon}")
        print(f"  Balance (TIG): max_block <= ceil((n/k) * (1+epsilon)) = ceil(({results[0]['num_nodes']}/{num_parts}) * {1+epsilon:.2f}) = {results[0]['allowed_max']}")
        print(f"  Objective = connectivity (KM1: sum of lambda-1)")
        print(f"  Vertex weights = unit (unweighted)")
        print(f"\nNote: We report both Mt-KaHyPar load and partition times; speedups are computed vs partition time (load time is negligible here).")
    
    return results


def main():
    parser = argparse.ArgumentParser(description='Compare hypergraph partitioning against Mt-KaHyPar')
    parser.add_argument('hgr_file', help='Path to .hgr file or glob pattern for batch mode')
    parser.add_argument('--partition', '-p', help='Path to your partition file (single mode)')
    parser.add_argument('--parts', '-k', type=int, default=64, help='Number of partitions (default: 64)')
    parser.add_argument('--epsilon', '-e', type=float, default=0.03, help='Balance constraint (default: 0.03)')
    parser.add_argument('--threads', '-t', type=int, default=1, help='Number of threads (default: 1)')
    parser.add_argument('--preset', choices=['default', 'quality', 'highest_quality'], 
                        default='quality', help='Mt-KaHyPar preset (default: quality)')
    parser.add_argument('--batch', '-b', action='store_true', help='Batch mode: treat hgr_file as glob pattern')
    args = parser.parse_args()
    
    if args.batch or '*' in args.hgr_file:
        batch_compare(args.hgr_file, args.parts, args.epsilon, args.threads, args.preset)
    else:
        print(f"Comparing: {args.hgr_file}")
        compare_single(args.hgr_file, args.partition, args.parts, args.epsilon, 
                      args.threads, args.preset, verbose=True)
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
