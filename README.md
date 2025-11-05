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

## 🧩 Integration Example (Planned)

**Rust Backend → Cytoscape.js**

```js
const response = await fetch("/layout", {
  method: "POST",
  body: JSON.stringify(cy.elements().json()),
});

const layoutPositions = await response.json();

cy.nodes().positions((node) => layoutPositions[node.id()]);
```

**WASM → Cytoscape.js** (future)

```js
import init, { compute_concentric } from "rconcentriclayout-wasm";

await init();

const result = compute_concentric(elements);
cy.nodes().positions((n) => result[n.id()]);
```

---

## 🧪 Testing

All generated positions will be tested directly inside Cytoscape.js with a demo graph to ensure correctness and visual consistency.

---

## 📊 Benchmarking (Planned)

Benchmarks will compare:

- CPU (single-thread)
- CPU (parallel)
- GPU (WGPU compute)
- WASM browser execution

Metrics:

- Throughput  
- Latency per layout computation  
- Memory cost  
- Stability across large datasets (1K–50K nodes)  

---

## 🤝 Contributions

PRs and feedback are welcome!  
Open an issue if you'd like to request features or propose improvements.

---

## 📜 License

MIT or Apache-2.0 (choose your preferred license)
