# Full Benchmark Results

## Test Configuration

| Parameter | Value |
|-----------|-------|
| **Solver** | sigma_freud_v5 |
| **Baseline** | Mt-KaHyPar (highest_quality preset) |
| **Partitions (k)** | 64 |
| **Epsilon (ε)** | 0.03 |
| **Objective** | Connectivity (KM1: Σ(λ-1)) |
| **Vertex weights** | Unit (unweighted) |
| **Refinement rounds** | 2000 |
| **Instances per track** | 10 |

### Test Environment

| Component | Specification |
|-----------|---------------|
| **GPU** | NVIDIA RTX 5070 Ti Laptop (12GB VRAM) |
| **CPU** | Intel Core Ultra 9 275HX (24 cores, 16 threads used for Mt-KaHyPar) |
| **OS** | Ubuntu 24.04 (WSL2) |
| **CUDA** | 12.0 |
| **Rust** | 1.90.0 |

---

## Summary by Track

| Track | Win Rate | Mean Gap | Speedup |
|-------|----------|----------|---------|
| 10,000 hyperedges | 60% (6-4) | -0.21% | 0.4x |
| 20,000 hyperedges | 10% (1-9) | +1.18% | 1.0x |
| 50,000 hyperedges | **90% (9-1)** | **-1.72%** | **2.4x** |
| 100,000 hyperedges | **80% (8-2)** | **-1.71%** | **5.1x** |
| 200,000 hyperedges | **100% (10-0)** | **-2.80%** | **8.7x** |

*Gap: negative = sigma_freud better, positive = Mt-KaHyPar better*

---

## Track: 10,000 Hyperedges

**Summary:** 6 wins / 0 ties / 4 losses | Mean gap: -0.21% | Speedup: 0.4x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 00edca03 | 8,445 | 14,170 | 14,074 | +96 (+0.68%) | KaHyPar | 1.48s | 0.65s |
| 09b48ee3 | 8,497 | 14,131 | 14,314 | -183 (-1.28%) | **YOU** | 1.49s | 0.51s |
| 0c50a55a | 8,607 | 14,172 | 14,267 | -95 (-0.67%) | **YOU** | 1.58s | 0.54s |
| 145cad6e | 8,832 | 14,284 | 14,379 | -95 (-0.66%) | **YOU** | 1.69s | 0.56s |
| 1d649f9e | 8,751 | 14,687 | 14,519 | +168 (+1.16%) | KaHyPar | 1.56s | 0.53s |
| 2b1ec682 | 8,675 | 14,755 | 14,575 | +180 (+1.23%) | KaHyPar | 1.55s | 0.53s |
| aa34801d | 8,605 | 14,448 | 14,615 | -167 (-1.14%) | **YOU** | 1.44s | 0.52s |
| ab24379d | 8,505 | 14,291 | 14,349 | -58 (-0.40%) | **YOU** | 1.51s | 0.63s |
| b78e414a | 8,673 | 14,474 | 14,453 | +21 (+0.15%) | KaHyPar | 1.61s | 0.59s |
| fe4dd86f | 8,550 | 14,031 | 14,190 | -159 (-1.12%) | **YOU** | 1.55s | 0.55s |

**Averages:** sigma_freud 1.55s, Mt-KaHyPar 0.56s

---

## Track: 20,000 Hyperedges

**Summary:** 1 wins / 0 ties / 9 losses | Mean gap: +1.18% | Speedup: 1.0x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 18ae1744 | 17,192 | 29,159 | 28,379 | +780 (+2.75%) | KaHyPar | 2.19s | 2.16s |
| 1fd223a8 | 17,242 | 27,816 | 27,475 | +341 (+1.24%) | KaHyPar | 2.06s | 2.17s |
| 3a348b50 | 17,151 | 28,394 | 28,364 | +30 (+0.11%) | KaHyPar | 2.01s | 1.99s |
| 59b53abc | 17,013 | 27,538 | 27,361 | +177 (+0.65%) | KaHyPar | 2.13s | 1.92s |
| 6d3c8d28 | 17,282 | 29,010 | 28,786 | +224 (+0.78%) | KaHyPar | 2.18s | 2.25s |
| a2abba57 | 16,755 | 28,125 | 27,244 | +881 (+3.23%) | KaHyPar | 1.92s | 1.92s |
| a7dbccb5 | 17,078 | 27,155 | 27,325 | -170 (-0.62%) | **YOU** | 2.04s | 2.26s |
| ad4d13c2 | 16,863 | 26,719 | 26,653 | +66 (+0.25%) | KaHyPar | 2.11s | 1.93s |
| b6422f89 | 16,312 | 27,185 | 26,435 | +750 (+2.84%) | KaHyPar | 1.82s | 1.65s |
| ea3bb269 | 17,147 | 28,482 | 28,303 | +179 (+0.63%) | KaHyPar | 2.05s | 2.02s |

**Averages:** sigma_freud 2.05s, Mt-KaHyPar 2.03s

---

## Track: 50,000 Hyperedges

**Summary:** 9 wins / 0 ties / 1 losses | Mean gap: -1.72% | Speedup: 2.4x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 06281a36 | 42,506 | 71,875 | 71,110 | +765 (+1.08%) | KaHyPar | 3.64s | 8.27s |
| 062b2da4 | 41,802 | 68,059 | 69,135 | -1,076 (-1.56%) | **YOU** | 3.53s | 7.79s |
| 0fcdac45 | 42,493 | 70,506 | 71,181 | -675 (-0.95%) | **YOU** | 3.65s | 8.55s |
| 111ec9a6 | 42,256 | 70,201 | 71,337 | -1,136 (-1.59%) | **YOU** | 3.69s | 8.62s |
| 30a8f3dd | 41,601 | 68,567 | 70,867 | -2,300 (-3.25%) | **YOU** | 3.53s | 8.04s |
| 9d9aba79 | 42,224 | 69,412 | 69,977 | -565 (-0.81%) | **YOU** | 3.45s | 8.10s |
| c547f80e | 42,059 | 68,554 | 70,357 | -1,803 (-2.56%) | **YOU** | 3.48s | 8.71s |
| d28e8f9a | 42,237 | 68,438 | 70,913 | -2,475 (-3.49%) | **YOU** | 3.62s | 9.03s |
| ec31b55e | 42,330 | 67,504 | 70,019 | -2,515 (-3.59%) | **YOU** | 3.43s | 8.88s |
| ee59e329 | 42,091 | 68,678 | 69,025 | -347 (-0.50%) | **YOU** | 3.44s | 9.05s |

**Averages:** sigma_freud 3.55s, Mt-KaHyPar 8.50s

---

## Track: 100,000 Hyperedges

**Summary:** 8 wins / 0 ties / 2 losses | Mean gap: -1.71% | Speedup: 5.1x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 061c00f9 | 84,927 | 136,957 | 142,471 | -5,514 (-3.87%) | **YOU** | 6.15s | 30.43s |
| 5e53df80 | 83,358 | 135,130 | 138,037 | -2,907 (-2.11%) | **YOU** | 5.98s | 28.75s |
| 639d2cbb | 84,137 | 144,932 | 142,876 | +2,056 (+1.44%) | KaHyPar | 6.17s | 31.55s |
| 9e769adb | 84,406 | 138,527 | 140,391 | -1,864 (-1.33%) | **YOU** | 6.17s | 32.07s |
| b8b07601 | 85,419 | 143,958 | 142,842 | +1,116 (+0.78%) | KaHyPar | 6.40s | 32.89s |
| bc9db8da | 84,300 | 138,866 | 143,570 | -4,704 (-3.28%) | **YOU** | 6.14s | 32.31s |
| c742393e | 83,851 | 140,838 | 145,238 | -4,400 (-3.03%) | **YOU** | 6.00s | 31.40s |
| cd7767c5 | 83,514 | 141,622 | 141,625 | -3 (-0.00%) | **YOU** | 6.06s | 30.29s |
| d4694bde | 85,199 | 139,753 | 145,711 | -5,958 (-4.09%) | **YOU** | 6.15s | 36.31s |
| f056283d | 84,548 | 140,824 | 143,158 | -2,334 (-1.63%) | **YOU** | 7.01s | 31.25s |

**Averages:** sigma_freud 6.22s, Mt-KaHyPar 31.73s

---

## Track: 200,000 Hyperedges

**Summary:** 10 wins / 0 ties / 0 losses | Mean gap: -2.80% | Speedup: 8.7x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 008a9780 | 168,330 | 273,946 | 277,638 | -3,692 (-1.33%) | **YOU** | 10.48s | 95.66s |
| 5f6ac1b7 | 166,894 | 276,186 | 279,459 | -3,273 (-1.17%) | **YOU** | 11.36s | 96.21s |
| 609ae189 | 168,254 | 282,134 | 292,578 | -10,444 (-3.57%) | **YOU** | 10.86s | 101.08s |
| 6773cdbc | 168,200 | 280,904 | 293,319 | -12,415 (-4.23%) | **YOU** | 10.87s | 101.62s |
| 8e79d3e3 | 167,622 | 278,412 | 283,318 | -4,906 (-1.73%) | **YOU** | 10.95s | 98.86s |
| 9c13a1aa | 168,552 | 272,489 | 282,035 | -9,546 (-3.38%) | **YOU** | 11.66s | 95.16s |
| 9edc7fdc | 167,495 | 265,769 | 272,137 | -6,368 (-2.34%) | **YOU** | 12.57s | 93.64s |
| ab4e832a | 168,666 | 274,725 | 283,854 | -9,129 (-3.22%) | **YOU** | 10.95s | 102.37s |
| d4e7d3f0 | 168,671 | 268,217 | 278,866 | -10,649 (-3.82%) | **YOU** | 11.81s | 96.18s |
| e9c060cc | 167,688 | 284,125 | 293,543 | -9,418 (-3.21%) | **YOU** | 11.63s | 103.27s |

**Averages:** sigma_freud 11.31s, Mt-KaHyPar 98.41s

---

## Reproduction Commands

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

---

## Notes

- All partitions produced by sigma_freud are **FEASIBLE** (satisfy balance constraint)
- Mt-KaHyPar uses `highest_quality` preset which prioritizes solution quality over speed
- Speedup is computed as Mt-KaHyPar partition time / sigma_freud time
- KM1 (connectivity) = Σ(λ(e) - 1) where λ(e) = number of parts connected by hyperedge e
- Lower KM1 is better
