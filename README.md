# (WorkInProgress) rConcentricLayout

Rust-powered, GPU-optional concentric layout engine for Cytoscape.js.
Offload layout computation from the browser to a fast Rust backend, WebAssembly module, or GPU-accelerated WGPU pipeline.

Built for graph visualization, network topology tools, and high-performance layout workloads.

## ✨ Features (Planned & Ongoing)

- ✅ Pure Rust concentric layout core
- ✅ Supports single-ring and multi-ring computations
- ✅ Ready for integration with Cytoscape.js
- ✅ Designed for CPU parallelism (Rayon / custom threading)
- ✅ Optional GPU acceleration via WGPU
- ✅ WASM target for browser-based execution
- ✅ Clean Rust API for layout computation
- ✅ Can be embedded in backend services or compiled to WASM
---

## 🚀 Purpose

`rConcentricLayout` allows you to:

- Offload Cytoscape.js layout computation to a Rust backend
- Run layout logic in WASM for fast, client-side graph visualization
- Use GPU acceleration when performing large or complex layout calculations
- Integrate with visualization platforms and network dashboards efficiently

---

## 📦 Installation
(Coming soon)

```
cargo add rconcentriclayout
```

WASM and NPM bindings will be added later.

---

## 🧠 How It Works

`rConcentricLayout` computes node positions using:

1. Ring levels (based on degree, user-defined, or algorithmic criteria)
2. Angular spacing
3. Radius scaling
4. Parallel CPU passes or GPU passes
5. Outputting `(x, y)` positions in a JSON structure compatible with Cytoscape.js

The layout can be computed in:

- Rust (synchronous)
- Rust with multi-threaded CPU parallelism
- Rust with GPU acceleration (via WGPU compute shader)
- WASM for browser usage

---

## 🗺️ Roadmap

1. ✅ Develop one-ring calculation
2. ✅ Develop multiple-ring calculation
3. 🔄 Develop CPU-based parallel computation
4. 🔄 Test in Cytoscape.js
5. ✅ Develop GPU-based computation (WGPU)
6. 🔄 Test GPU mode in Cytoscape.js
7. 🔜 Develop WASM version
8. 🔜 Test WASM version in Cytoscape.js
9. 🔜 Create performance benchmarks

> ✅ = done • 🔄 = in progress • 🔜 = upcoming

---

## 📊 Benchmarking

### CPU Based Parallel Computing

|Number of Threads|Sample Data File|Nodes|Edges|Seconds|Milliseconds| Microseconds|
|----|----|  ----|---- |---- |----|----|
| 2 threads | nodes_10_full_mesh.json  |   10   nodes |   90   edges |  0 s |   0   ms |    508   us |
| 2 threads | nodes_100_full_mesh.json |  100   nodes |  9900  edges |  0 s |   6   ms |   6008   us |
| 2 threads | nodes_1000_random.json   |  1000  nodes |  4000  edges |  0 s |   22  ms |   22653  us |
| 2 threads | nodes_2000_random.json   |  2000  nodes |  8000  edges |  0 s |   89  ms |   89221  us |
| 2 threads | nodes_5000_random.json   |  5000  nodes | 20000  edges |  0 s |  866  ms |  866871  us |
| 2 threads | nodes_10000_random.json  | 10000  nodes | 40000  edges |  3 s |  3282 ms |  3282113 us |
| 2 threads | nodes_50000_random.json  | 50000  nodes | 200000 edges | 73 s | 73045 ms | 73045235 us |
| 2 threads | nodes_100000_random.json | 100000 nodes | 400000 edges | 295s | 295919ms | 295919804us |
| 2 threads | telco_sample.json        |   52   nodes |  220   edges |  0 s |   0   ms |    348   us |
| 4 threads | nodes_10_full_mesh.json  |   10   nodes |   90   edges |  0 s |   0   ms |    193   us |
| 4 threads | nodes_100_full_mesh.json |  100   nodes |  9900  edges |  0 s |   5   ms |   5407   us |
| 4 threads | nodes_1000_random.json   |  1000  nodes |  4000  edges |  0 s |   19  ms |   19844  us |
| 4 threads | nodes_2000_random.json   |  2000  nodes |  8000  edges |  0 s |   72  ms |   72126  us |
| 4 threads | nodes_5000_random.json   |  5000  nodes | 20000  edges |  0 s |  422  ms |  422170  us |
| 4 threads | nodes_10000_random.json  | 10000  nodes | 40000  edges |  1 s |  1704 ms |  1704142 us |
| 4 threads | nodes_50000_random.json  | 50000  nodes | 200000 edges | 41 s | 41012 ms | 41012659 us |
| 4 threads | nodes_100000_random.json | 100000 nodes | 400000 edges | 168s | 168856ms | 168856157us |
| 4 threads | telco_sample.json        |   52   nodes |  220   edges |  0 s |   0   ms |    408   us |
| 8 threads | nodes_10_full_mesh.json  |   10   nodes |   90   edges | 0 s |   0  ms |   269   us |
| 8 threads | nodes_100_full_mesh.json |  100   nodes |  9900  edges | 0 s |   3  ms |   3203  us |
| 8 threads | nodes_1000_random.json   |  1000  nodes |  4000  edges | 0 s |  12  ms |  12979  us |
| 8 threads | nodes_2000_random.json   |  2000  nodes |  8000  edges | 0 s |  42  ms |  42199  us |
| 8 threads | nodes_5000_random.json   |  5000  nodes | 20000  edges | 0 s |  219 ms |  219733 us |
| 8 threads | nodes_10000_random.json  | 10000  nodes | 40000  edges | 0 s |  908 ms |  908255 us |
| 8 threads | nodes_50000_random.json  | 50000  nodes | 200000 edges | 22s | 22105ms | 22105210us |
| 8 threads | nodes_100000_random.json | 100000 nodes | 400000 edges | 87s | 87454ms | 87454680us |
| 8 threads | telco_sample.json        |   52   nodes |  220   edges | 0 s |   0  ms |   394   us |
| 16 threads | nodes_10_full_mesh.json  |   10   nodes |   90   edges | 0 s |   0  ms |   398   us |
| 16 threads | nodes_100_full_mesh.json |  100   nodes |  9900  edges | 0 s |   2  ms |   2181  us |
| 16 threads | nodes_1000_random.json   |  1000  nodes |  4000  edges | 0 s |   9  ms |   9709  us |
| 16 threads | nodes_2000_random.json   |  2000  nodes |  8000  edges | 0 s |  26  ms |  26499  us |
| 16 threads | nodes_5000_random.json   |  5000  nodes | 20000  edges | 0 s |  118 ms |  118519 us |
| 16 threads | nodes_10000_random.json  | 10000  nodes | 40000  edges | 0 s |  472 ms |  472412 us |
| 16 threads | nodes_50000_random.json  | 50000  nodes | 200000 edges | 11s | 11630ms | 11630157us |
| 16 threads | nodes_100000_random.json | 100000 nodes | 400000 edges | 45s | 45904ms | 45904075us |
| 16 threads | telco_sample.json        |   52   nodes |  220   edges | 0 s |   0  ms |   499   us |
| 32 threads | nodes_10_full_mesh.json  |   10   nodes |   90   edges | 0 s |   0  ms |   710   us |
| 32 threads | nodes_100_full_mesh.json |  100   nodes |  9900  edges | 0 s |   4  ms |   4072  us |
| 32 threads | nodes_1000_random.json   |  1000  nodes |  4000  edges | 0 s |  10  ms |  10042  us |
| 32 threads | nodes_2000_random.json   |  2000  nodes |  8000  edges | 0 s |  29  ms |  29730  us |
| 32 threads | nodes_5000_random.json   |  5000  nodes | 20000  edges | 0 s |  96  ms |  96942  us |
| 32 threads | nodes_10000_random.json  | 10000  nodes | 40000  edges | 0 s |  397 ms |  397866 us |
| 32 threads | nodes_50000_random.json  | 50000  nodes | 200000 edges | 8 s | 8436 ms | 8436697 us |
| 32 threads | nodes_100000_random.json | 100000 nodes | 400000 edges | 34s | 34646ms | 34646886us |
| 32 threads | telco_sample.json        |   52   nodes |  220   edges | 0 s |   1  ms |   1287  us |

### GPU Based Parallel Computing (A little bit of CPU for remapping the node information)
| Dispatch Workgroup | WorkGroup Size | Total Threads ( Dispatch Workgroup x WorkGroup Size) |Sample Data File|Nodes|Edges|Seconds|Milliseconds| Microseconds|
|----|----|----|----|----|----|----|----|----|
| 64 | 64 | 4096 Threads  | nodes_10_full_mesh.json  |   10   nodes |   90   edges | 1 s | 1262 ms | 1262898 us |
| 64 | 64 | 4096 Threads  | nodes_100_full_mesh.json |  100   nodes |  9900  edges | 1 s | 1626 ms | 1626835 us |
| 64 | 64 | 4096 Threads  | nodes_1000_random.json   |  1000  nodes |  4000  edges | 1 s | 1100 ms | 1100852 us |
| 64 | 64 | 4096 Threads  | nodes_2000_random.json   |  2000  nodes |  8000  edges | 1 s | 1220 ms | 1220404 us |
| 64 | 64 | 4096 Threads  | nodes_5000_random.json   |  5000  nodes | 20000  edges | 1 s | 1643 ms | 1643311 us |
| 64 | 64 | 4096 Threads  | nodes_10000_random.json  | 10000  nodes | 40000  edges | 2 s | 2415 ms | 2415705 us |
| 64 | 64 | 4096 Threads  | nodes_50000_random.json  | 50000  nodes | 200000 edges | 7 s | 7937 ms | 7937554 us |
| 64 | 64 | 4096 Threads  | nodes_100000_random.json | 100000 nodes | 400000 edges | 14s | 14982ms | 14982001us |
| 64 | 64 | 4096 Threads  | telco_sample.json        |   52   nodes |  220   edges | 0 s |  928 ms |  928486 us |

---

## 🤝 Contributions

PRs and feedback are welcome!
Open an issue if you'd like to request features or propose improvements.

---


MIT or Apache-2.0 (choose your preferred license)
