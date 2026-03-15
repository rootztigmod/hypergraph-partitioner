# Full Benchmark Results

## Test Configuration

| Parameter | Value |
|-----------|-------|
| **Solver** | sigma_freud_v6 |
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
| 10,000 hyperedges | **80% (8-1-1)** | **-0.83%** | 0.3x |
| 20,000 hyperedges | 50% (5-5) | **~0.00%** | 0.8x |
| 50,000 hyperedges | **90% (9-1)** | **-2.33%** | **2.1x** |
| 100,000 hyperedges | **90% (9-1)** | **-2.27%** | **4.5x** |
| 200,000 hyperedges | **100% (10-0)** | **-3.01%** | **8.6x** |

*Gap: negative = sigma_freud better, positive = Mt-KaHyPar better*

---

## Track: 10,000 Hyperedges

**Summary:** 8 wins / 1 ties / 1 losses | Mean gap: -0.83% | Speedup: 0.3x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 00edca03 | 8,445 | 14,068 | 14,125 | -57 (-0.40%) | **YOU** | 1.69s | 0.64s |
| 09b48ee3 | 8,497 | 14,080 | 14,264 | -184 (-1.29%) | **YOU** | 1.59s | 0.47s |
| 0c50a55a | 8,607 | 14,120 | 14,287 | -167 (-1.17%) | **YOU** | 1.73s | 0.53s |
| 145cad6e | 8,832 | 14,226 | 14,292 | -66 (-0.46%) | **YOU** | 1.88s | 0.52s |
| 1d649f9e | 8,751 | 14,478 | 14,438 | +40 (+0.28%) | KaHyPar | 1.69s | 0.55s |
| 2b1ec682 | 8,675 | 14,378 | 14,560 | -182 (-1.25%) | **YOU** | 1.65s | 0.52s |
| aa34801d | 8,605 | 14,400 | 14,618 | -218 (-1.49%) | **YOU** | 1.66s | 0.52s |
| ab24379d | 8,505 | 14,230 | 14,437 | -207 (-1.43%) | **YOU** | 1.62s | 0.50s |
| b78e414a | 8,673 | 14,419 | 14,419 | 0 (0.00%) | Tie | 1.82s | 0.54s |
| fe4dd86f | 8,550 | 13,988 | 14,133 | -145 (-1.03%) | **YOU** | 1.77s | 0.50s |

**Averages:** sigma_freud 1.71s, Mt-KaHyPar 0.53s

---

## Track: 20,000 Hyperedges

**Summary:** 5 wins / 0 ties / 5 losses | Mean gap: ~0.00% | Speedup: 0.8x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 18ae1744 | 17,192 | 28,902 | 28,438 | +464 (+1.63%) | KaHyPar | 2.55s | 1.87s |
| 1fd223a8 | 17,242 | 27,660 | 27,894 | -234 (-0.84%) | **YOU** | 2.36s | 1.83s |
| 3a348b50 | 17,151 | 28,233 | 28,377 | -144 (-0.51%) | **YOU** | 2.18s | 1.83s |
| 59b53abc | 17,013 | 27,319 | 27,395 | -76 (-0.28%) | **YOU** | 2.34s | 1.64s |
| 6d3c8d28 | 17,282 | 28,876 | 28,760 | +116 (+0.40%) | KaHyPar | 2.42s | 2.14s |
| a2abba57 | 16,755 | 27,341 | 27,307 | +34 (+0.12%) | KaHyPar | 2.16s | 1.55s |
| a7dbccb5 | 17,078 | 27,063 | 27,694 | -631 (-2.28%) | **YOU** | 2.27s | 1.84s |
| ad4d13c2 | 16,863 | 26,589 | 26,718 | -129 (-0.48%) | **YOU** | 2.27s | 1.87s |
| b6422f89 | 16,312 | 27,003 | 26,481 | +522 (+1.97%) | KaHyPar | 2.00s | 1.51s |
| ea3bb269 | 17,147 | 28,386 | 28,171 | +215 (+0.76%) | KaHyPar | 2.25s | 1.93s |

**Averages:** sigma_freud 2.28s, Mt-KaHyPar 1.80s

---

## Track: 50,000 Hyperedges

**Summary:** 9 wins / 0 ties / 1 losses | Mean gap: -2.33% | Speedup: 2.1x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 06281a36 | 42,506 | 71,293 | 70,964 | +329 (+0.46%) | KaHyPar | 4.48s | 8.96s |
| 062b2da4 | 41,802 | 67,630 | 69,879 | -2,249 (-3.22%) | **YOU** | 3.74s | 8.87s |
| 0fcdac45 | 42,493 | 69,949 | 71,325 | -1,376 (-1.93%) | **YOU** | 3.91s | 9.02s |
| 111ec9a6 | 42,256 | 69,786 | 70,750 | -964 (-1.36%) | **YOU** | 3.98s | 8.81s |
| 30a8f3dd | 41,601 | 68,175 | 70,609 | -2,434 (-3.45%) | **YOU** | 3.67s | 7.73s |
| 9d9aba79 | 42,224 | 68,968 | 69,549 | -581 (-0.84%) | **YOU** | 3.64s | 7.59s |
| c547f80e | 42,059 | 68,181 | 70,303 | -2,122 (-3.02%) | **YOU** | 3.73s | 8.34s |
| d28e8f9a | 42,237 | 67,997 | 71,446 | -3,449 (-4.83%) | **YOU** | 4.26s | 7.97s |
| ec31b55e | 42,330 | 67,207 | 70,105 | -2,898 (-4.13%) | **YOU** | 3.74s | 8.06s |
| ee59e329 | 42,091 | 68,232 | 68,915 | -683 (-0.99%) | **YOU** | 3.72s | 8.00s |

**Averages:** sigma_freud 3.89s, Mt-KaHyPar 8.33s

---

## Track: 100,000 Hyperedges

**Summary:** 9 wins / 0 ties / 1 losses | Mean gap: -2.27% | Speedup: 4.5x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 061c00f9 | 84,927 | 136,119 | 142,519 | -6,400 (-4.49%) | **YOU** | 6.36s | 29.26s |
| 5e53df80 | 83,358 | 134,330 | 138,060 | -3,730 (-2.70%) | **YOU** | 6.61s | 27.47s |
| 639d2cbb | 84,137 | 144,073 | 144,525 | -452 (-0.31%) | **YOU** | 7.10s | 28.42s |
| 9e769adb | 84,406 | 137,773 | 140,103 | -2,330 (-1.66%) | **YOU** | 6.44s | 28.27s |
| b8b07601 | 85,419 | 143,020 | 142,766 | +254 (+0.18%) | KaHyPar | 6.52s | 30.59s |
| bc9db8da | 84,300 | 137,960 | 141,834 | -3,874 (-2.73%) | **YOU** | 6.48s | 29.73s |
| c742393e | 83,851 | 139,764 | 145,166 | -5,402 (-3.72%) | **YOU** | 6.32s | 28.76s |
| cd7767c5 | 83,514 | 140,654 | 141,568 | -914 (-0.65%) | **YOU** | 6.25s | 28.02s |
| d4694bde | 85,199 | 138,885 | 145,286 | -6,401 (-4.41%) | **YOU** | 6.38s | 30.50s |
| f056283d | 84,548 | 139,854 | 142,928 | -3,074 (-2.15%) | **YOU** | 6.35s | 29.60s |

**Averages:** sigma_freud 6.48s, Mt-KaHyPar 29.06s

---

## Track: 200,000 Hyperedges

**Summary:** 10 wins / 0 ties / 0 losses | Mean gap: -3.01% | Speedup: 8.6x

| Instance | Nodes | sigma_freud KM1 | Mt-KaHyPar KM1 | Diff | Winner | sigma_freud Time | Mt-KaHyPar Time |
|----------|-------|-----------------|----------------|------|--------|------------------|-----------------|
| 008a9780 | 168,330 | 272,709 | 277,549 | -4,840 (-1.74%) | **YOU** | 11.69s | 99.64s |
| 5f6ac1b7 | 166,894 | 273,270 | 281,738 | -8,468 (-3.01%) | **YOU** | 11.49s | 87.77s |
| 609ae189 | 168,254 | 279,964 | 289,222 | -9,258 (-3.20%) | **YOU** | 11.02s | 101.08s |
| 6773cdbc | 168,200 | 279,474 | 293,229 | -13,755 (-4.69%) | **YOU** | 11.54s | 95.00s |
| 8e79d3e3 | 167,622 | 276,919 | 280,131 | -3,212 (-1.15%) | **YOU** | 10.82s | 95.83s |
| 9c13a1aa | 168,552 | 270,904 | 282,027 | -11,123 (-3.94%) | **YOU** | 10.88s | 95.99s |
| 9edc7fdc | 167,495 | 264,345 | 272,644 | -8,299 (-3.04%) | **YOU** | 10.89s | 97.49s |
| ab4e832a | 168,666 | 273,052 | 283,319 | -10,267 (-3.62%) | **YOU** | 10.84s | 92.04s |
| d4e7d3f0 | 168,671 | 266,770 | 275,304 | -8,534 (-3.10%) | **YOU** | 10.94s | 93.55s |
| e9c060cc | 167,688 | 282,604 | 289,779 | -7,175 (-2.48%) | **YOU** | 10.85s | 99.57s |

**Averages:** sigma_freud 11.10s, Mt-KaHyPar 95.80s

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
