# KOSMOWONDA (Wonders in Cosmos)

> A CPU-based N-body gravitational physics simulator written in Rust.

## Core Idea
Simulates gravitational interaction between bodies using Newtonian physics:

```text
F = G * (m1 * m2) / r²
```

**Integration method:** Symplectic Euler (stable for orbital motion)

## Architecture
| Module | Description |
|---|---|
| `kosmo_core` | Main simulation loop |
| `kosmo_physics` | Gravity, integration, parallelism |
| `kosmo_math` | `Vec2` math operations (local vector)|
| `kosmo_render` | CPU pixel buffer renderer (`minifb`) |
| `kosmo_benchmarks`| Criterion performance tests |

## Features
### Physics
- O(n²) pairwise gravitational simulation
- Softened gravity for stability
- Symplectic Euler integration

### Performance
- Sequential implementation (baseline)
- Rayon parallel implementation
- Adaptive switching (sequential vs parallel)

### Rendering
- CPU pixel buffer (`Vec<u32>`)
- Filled circle rendering
- World => screen transformation
- Trail rendering (fade-based)

## Benchmark Results (Ryzen 5 7530U)
*Hardware: AMD Ryzen 5 7530U @ 2.00 GHz, 16 GB RAM*

| N Bodies | Sequential | Parallel (Rayon) | Adaptive (Auto Switch) |
| :--- | :--- | :--- | :--- |
| **10** | ~0.81 µs | ~24 µs | ~0.81 µs |
| **100** | ~78 µs | ~47 µs | ~58 µs |
| **500** | ~1.93 ms | ~0.60 ms | ~0.59 ms |

> **Key Observation:** Parallelization becomes beneficial after ~ N ≈ 50–100 bodies. The system achieves a **~ 3.3× speedup** at N=500.

## Complexity
- **Naive gravity computation:** `O(n²)`
- **Pair interactions:** `n(n-1)/2`

## Design Insights
- Parallelism can outperform more "optimal" sequential execution.
- Runtime adaptation is critical for real-world performance.
- Memory + scheduling overhead matters as much as Big-O.
- CPU utilization > theoretical efficiency at scale.

## Future Work
- [ ] Barnes–Hut optimization (O(n log n))
- [ ] Quad-tree spatial partitioning
- [ ] Better integrators (Velocity Verlet / RK4)
- [ ] Collision detection and merging

## Status
This project is a learning-grade physics engine exploring:
* Numerical stability
* Parallel systems design
* Performance benchmarking
* Real-time simulation architecture