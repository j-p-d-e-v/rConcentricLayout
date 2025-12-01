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
| 2 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges |  0 s |   1   ms |   1048   us |
| 2 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges |  0 s |   0   ms |    757   us |
| 2 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges |  0 s |   0   ms |    704   us |
| 2 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges |  0 s |   10  ms |   10461  us |
| 2 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges |  0 s |   0   ms |    770   us |
| 2 threads | sample-data.json                                       |   56   nodes |   76   edges |  0 s |   0   ms |    421   us |
| 2 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges |  0 s |   80  ms |   80565  us |
| 2 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges |  0 s |   27  ms |   27975  us |
| 2 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges |  0 s |   68  ms |   68902  us |
| 2 threads | graph_10000.json                                       | 10000  nodes | 25029  edges |  5 s |  5642 ms |  5642332 us |
| 2 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 31 s | 31840 ms | 31840654 us |
| 2 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 195s | 195112ms | 195112749us |
| 2 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 739s | 739740ms | 739740024us |
| 2 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges |  0 s |   73  ms |   73935  us |
| 4 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges |  0 s |   1   ms |   1027   us |
| 4 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges |  0 s |   0   ms |    949   us |
| 4 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges |  0 s |   0   ms |    820   us |
| 4 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges |  0 s |   8   ms |   8394   us |
| 4 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges |  0 s |   0   ms |    935   us |
| 4 threads | sample-data.json                                       |   56   nodes |   76   edges |  0 s |   0   ms |    502   us |
| 4 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges |  0 s |   67  ms |   67926  us |
| 4 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges |  0 s |   25  ms |   25134  us |
| 4 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges |  0 s |   50  ms |   50396  us |
| 4 threads | graph_10000.json                                       | 10000  nodes | 25029  edges |  3 s |  3959 ms |  3959879 us |
| 4 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 15 s | 15381 ms | 15381327 us |
| 4 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 98 s | 98191 ms | 98191377 us |
| 4 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 401s | 401377ms | 401377066us |
| 4 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges |  0 s |   39  ms |   39412  us |
| 8 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges |  0 s |   0   ms |    905   us |
| 8 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges |  0 s |   1   ms |   1119   us |
| 8 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges |  0 s |   0   ms |    883   us |
| 8 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges |  0 s |   5   ms |   5372   us |
| 8 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges |  0 s |   1   ms |   1154   us |
| 8 threads | sample-data.json                                       |   56   nodes |   76   edges |  0 s |   0   ms |    613   us |
| 8 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges |  0 s |   37  ms |   37627  us |
| 8 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges |  0 s |   16  ms |   16281  us |
| 8 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges |  0 s |   33  ms |   33106  us |
| 8 threads | graph_10000.json                                       | 10000  nodes | 25029  edges |  2 s |  2263 ms |  2263896 us |
| 8 threads | graph_20000.json                                       | 20000  nodes | 50066  edges |  8 s |  8874 ms |  8874563 us |
| 8 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 54 s | 54987 ms | 54987819 us |
| 8 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 216s | 216510ms | 216510349us |
| 8 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges |  0 s |   26  ms |   26294  us |
| 16 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges |  0 s |   1   ms |   1372   us |
| 16 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges |  0 s |   1   ms |   1245   us |
| 16 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges |  0 s |   1   ms |   1033   us |
| 16 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges |  0 s |   3   ms |   3697   us |
| 16 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges |  0 s |   1   ms |   1508   us |
| 16 threads | sample-data.json                                       |   56   nodes |   76   edges |  0 s |   0   ms |    727   us |
| 16 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges |  0 s |   25  ms |   25389  us |
| 16 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges |  0 s |   14  ms |   14434  us |
| 16 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges |  0 s |   23  ms |   23839  us |
| 16 threads | graph_10000.json                                       | 10000  nodes | 25029  edges |  1 s |  1246 ms |  1246199 us |
| 16 threads | graph_20000.json                                       | 20000  nodes | 50066  edges |  4 s |  4596 ms |  4596094 us |
| 16 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 28 s | 28842 ms | 28842062 us |
| 16 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 113s | 113897ms | 113897465us |
| 16 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges |  0 s |   18  ms |   18900  us |
| 32 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 0 s |   2  ms |   2151  us |
| 32 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 0 s |   1  ms |   1958  us |
| 32 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 0 s |   1  ms |   1971  us |
| 32 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 0 s |   4  ms |   4232  us |
| 32 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 0 s |   2  ms |   2124  us |
| 32 threads | sample-data.json                                       |   56   nodes |   76   edges | 0 s |   1  ms |   1423  us |
| 32 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 0 s |  25  ms |  25318  us |
| 32 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 0 s |  15  ms |  15116  us |
| 32 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 0 s |  23  ms |  23367  us |
| 32 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 0 s |  955 ms |  955288 us |
| 32 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 3 s | 3541 ms | 3541574 us |
| 32 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 21s | 21655ms | 21655496us |
| 32 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 86s | 86740ms | 86740632us |
| 32 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 0 s |  20  ms |  20268  us |

### GPU Based Parallel Computing (A little bit of CPU for remapping the node information)
|Number of Threads|Sample Data File|Nodes|Edges|Seconds|Milliseconds| Microseconds|
|----|----|  ----|---- |---- |----|----|
| 2 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 1 s | 1219 ms | 1219008 us |
| 2 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 1 s | 1069 ms | 1069875 us |
| 2 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 1 s | 1362 ms | 1362740 us |
| 2 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 1 s | 1057 ms | 1057240 us |
| 2 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 1 s | 1242 ms | 1242183 us |
| 2 threads | sample-data.json                                       |   56   nodes |   76   edges | 1 s | 1099 ms | 1099599 us |
| 2 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 1 s | 1489 ms | 1489326 us |
| 2 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 1 s | 1205 ms | 1205392 us |
| 2 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 1 s | 1741 ms | 1741561 us |
| 2 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 2 s | 2513 ms | 2513226 us |
| 2 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 4 s | 4156 ms | 4156093 us |
| 2 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 9 s | 9546 ms | 9546352 us |
| 2 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 17s | 17700ms | 17700680us |
| 2 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 1 s | 1282 ms | 1282350 us |
| 4 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 1 s | 1477 ms | 1477515 us |
| 4 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 1 s | 1251 ms | 1251386 us |
| 4 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 1 s | 1208 ms | 1208177 us |
| 4 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 1 s | 1182 ms | 1182923 us |
| 4 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 1 s | 1164 ms | 1164896 us |
| 4 threads | sample-data.json                                       |   56   nodes |   76   edges | 1 s | 1112 ms | 1112349 us |
| 4 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 1 s | 1300 ms | 1300568 us |
| 4 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 1 s | 1784 ms | 1784317 us |
| 4 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 1 s | 1315 ms | 1315933 us |
| 4 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 2 s | 2809 ms | 2809604 us |
| 4 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 4 s | 4514 ms | 4514689 us |
| 4 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 9 s | 9174 ms | 9174567 us |
| 4 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 16s | 16416ms | 16416125us |
| 4 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 1 s | 1275 ms | 1275049 us |
| 8 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 1 s | 1073 ms | 1073476 us |
| 8 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 1 s | 1061 ms | 1061512 us |
| 8 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 1 s | 1067 ms | 1067230 us |
| 8 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 1 s | 1621 ms | 1621397 us |
| 8 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 1 s | 1106 ms | 1106416 us |
| 8 threads | sample-data.json                                       |   56   nodes |   76   edges | 1 s | 1040 ms | 1040033 us |
| 8 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 1 s | 1663 ms | 1663099 us |
| 8 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 1 s | 1189 ms | 1189126 us |
| 8 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 1 s | 1192 ms | 1192988 us |
| 8 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 2 s | 2563 ms | 2563217 us |
| 8 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 4 s | 4104 ms | 4104215 us |
| 8 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 9 s | 9273 ms | 9273028 us |
| 8 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 18s | 18040ms | 18040169us |
| 8 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 1 s | 1361 ms | 1361142 us |
| 16 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 1 s | 1186 ms | 1186004 us |
| 16 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 1 s | 1196 ms | 1196876 us |
| 16 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 1 s | 1162 ms | 1162025 us |
| 16 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 1 s | 1214 ms | 1214323 us |
| 16 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 1 s | 1206 ms | 1206640 us |
| 16 threads | sample-data.json                                       |   56   nodes |   76   edges | 1 s | 1331 ms | 1331843 us |
| 16 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 1 s | 1349 ms | 1349037 us |
| 16 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 1 s | 1434 ms | 1434317 us |
| 16 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 1 s | 1339 ms | 1339267 us |
| 16 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 2 s | 2808 ms | 2808110 us |
| 16 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 4 s | 4699 ms | 4699166 us |
| 16 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 8 s | 8927 ms | 8927781 us |
| 16 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 17s | 17860ms | 17860431us |
| 16 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 1 s | 1352 ms | 1352460 us |
| 32 threads | concentric_nonmesh_star_100.json                       |  100   nodes |   99   edges | 1 s | 1183 ms | 1183142 us |
| 32 threads | sample-data-100-nodes-full-mesh-15-rings-neighbor.json |  100   nodes |  113   edges | 1 s | 1163 ms | 1163816 us |
| 32 threads | sample-data-100-nodes-full-mesh-15-rings.json          |  100   nodes |   81   edges | 1 s | 1150 ms | 1150901 us |
| 32 threads | sample-data-100-nodes-full-mesh.json                   |  100   nodes |  4950  edges | 1 s | 1208 ms | 1208727 us |
| 32 threads | sample-data-cytoscape.json                             |  101   nodes |  154   edges | 1 s | 1203 ms | 1203547 us |
| 32 threads | sample-data.json                                       |   56   nodes |   76   edges | 1 s | 1207 ms | 1207315 us |
| 32 threads | sample_graph_1000.json                                 |  1000  nodes |  3544  edges | 1 s | 1739 ms | 1739234 us |
| 32 threads | sample_tree_1000.json                                  |  1000  nodes |  999   edges | 1 s | 1354 ms | 1354423 us |
| 32 threads | sample_scalefree_1000.json                             |  1000  nodes |  2995  edges | 1 s | 1321 ms | 1321812 us |
| 32 threads | graph_10000.json                                       | 10000  nodes | 25029  edges | 2 s | 2736 ms | 2736781 us |
| 32 threads | graph_20000.json                                       | 20000  nodes | 50066  edges | 4 s | 4435 ms | 4435429 us |
| 32 threads | graph_50000.json                                       | 50000  nodes | 124939 edges | 9 s | 9454 ms | 9454897 us |
| 32 threads | graph_100000.json                                      | 100000 nodes | 250342 edges | 17s | 17831ms | 17831648us |
| 32 threads | telco_realistic_1000_nodes.json                        |  1000  nodes |  1971  edges | 1 s | 1296 ms | 1296482 us |
---

## 🤝 Contributions

PRs and feedback are welcome!
Open an issue if you'd like to request features or propose improvements.

---


MIT or Apache-2.0 (choose your preferred license)
