# Invalru

An autonomous, agentic Bug Bounty and vulnerability orchestration pipeline powered by eBPF telemetry, high-fidelity Abstract Syntax Tree (AST) mutation, and reinforcement learning-guided execution.

Invalru operates not as a sequence of fragile automation scripts, but as a closed-loop, low-latency concurrent system. It bridges the gap between kernel-level traffic instrumentation, deep semantic code comprehension, and autonomous validation.

---

## 1. Architectural Blueprint: "Eyes, Hands, Brain"

Invalru decouples data ingestion, structural analysis, and exploit orchestration into three distinct layers interconnected via low-latency communication channels.


```

+------------------------------------------------------------+
|                  INGESTION LAYER (Go)                      |
|  - eBPF / XDP Network Ingestion                            |
|  - Raw TCP/HTTP Stream Capture                             |
|  - Zero-Copy Ring Buffers                                  |
+-----------------------------+------------------------------+
|
| Shared Memory / Raw Bytes
v
+------------------------------------------------------------+
|                   ANALYSIS LAYER (Rust)                    |
|  - tree-sitter AST & CFG/DFG Construction                  |
|  - WASM-Isolated Execution Sandbox                         |
|  - Speed Web Compiler (SWC) Indexing                       |
+-----------------------------+------------------------------+
|
| gRPC over Unix Domain Sockets
v
+------------------------------------------------------------+
|                   DECISION LAYER (Python)                  |
|  - Multi-Agent Orchestration (LangGraph)                   |
|  - Fine-Tuned Mistral-Nemo-12B Semantic Audit              |
|  - gVisor Containerized Sandbox Exploitation               |
+------------------------------------------------------------+

```

### Eyes: Ingestion Layer (Go)
Engineered for ultra-low CPU overhead. Utilizing `eBPF` at the `XDP` (eXpress Data Path) layer, it intercepts and filters raw network traffic directly inside the kernel space. It maps hidden API endpoints, records microservices interaction, and passes data upstream without context switching overhead.

### Hands: Analysis Layer (Rust)
Built for deterministic speed. It ingests runtime code artifacts and transpiled JS/TS bundles, converting them into Abstract Syntax Trees (AST), Control Flow Graphs (CFG), and Data Flow Graphs (DFG). It tracks state flow from user-controlled inputs down to critical code execution points (`sinks`).

### Brain: Decision Layer (Python)
An asynchronous multi-agent system built on top of `LangGraph`. It leverages specialized LLMs and reinforcement learning to synthesize findings, evaluate potential exploit paths, mutate payloads, and safely execute Proof of Concepts (PoC).

---

## 2. Core Technical Stack

| Layer | Technologies | Functional Responsibility |
| :--- | :--- | :--- |
| **Data Ingestion (Go)** | `gopacket`, `ebpf-go`, `NATS JetStream` | Kernel-space telemetry, high-throughput event streaming. |
| **Static & Dataflow Analysis (Rust)** | `swc_core`, `tree-sitter`, `wasmer` | AST parsing, DFG/CFG mapping, safe execution of untrusted hooks. |
| **Orchestration & AI (Python)** | `LangGraph`, `vLLM`, `Pydantic` | Agent execution trees, LLM serving, payload validation logic. |
| **Intelligence Core** | `Fine-tuned Mistral-Nemo-12B` | Local inference specialized in vulnerability patterns & semantics. |
| **Interconnect** | `gRPC` over `Unix Domain Sockets` | Zero-copy shared memory boundaries and structured protobuf streams. |

---

## 3. Engineering Requirements & Guarantees

* **Immutability:** Every payload, telemetry log, and intermediate AST state is versioned and cryptographically indexed to guarantee strict exploit reproducibility.
* **Zero-Copy Memory Boundary:** Telemetry transport from the Go kernel space to the Rust analysis plane utilizes memory-mapped ring buffers, minimizing latency and memory overhead.
* **Strict Isolation:** Dynamic evaluation of code and validation of exploits are executed within hard-isolated `cgroup` spaces and ephemeral `gVisor` runtimes, preventing side-channel contamination or defensive evasions from targeting the scanner itself.
* **Asynchronous Concurrency:** System resource quotas are enforced per scan-thread using native asynchronous task-scheduling mechanics (`goroutines` in Go, `tokio` loop in Rust).

---

## 4. Development Roadmap & Phases

### Phase I: Semantic Filtering & Flow Mapping
* **Objective:** Minimize the Signal-to-Noise Ratio (SNR) in high-throughput environments.
* **Mechanics:** The Go layer watches ingress pipelines. The Rust layer parses JS/TS payloads on the fly to detect mutation patterns. The Python agent maintains a vector space via `FAISS`, locating semantic clusters of code resembling dangerous deserialization, object injections, or weak access controls.

### Phase II: RL-Guided Fuzzing (The Evolution Engine)
* **Objective:** Optimize automated payload generation to bypass modern Web Application Firewalls (WAF).
* **Mechanics:** Implements a Proximal Policy Optimization (PPO) reinforcement learning loop. The mutation engine alters successful exploitation primitives based on live feedback metrics (server response timing, HTTP status shifts, and runtime code coverage).

### Phase III: Autonomous Exploitation & Verification
* **Objective:** Eliminate false positives via deterministic verification.
* **Mechanics:** The Python executor spawns an ephemeral `gVisor` sandbox container replicating the target state. It executes a dynamically synthesized PoC, verifying whether the vulnerability triggers state corruption, data exfiltration, or logical bypasses, without degrading production target availability.

---

## 5. Getting Started

### Prerequisites

* Linux Kernel >= 5.15 with eBPF enabled (`CONFIG_DEBUG_INFO_BTF=y`)
* Go 1.22+
* Rust (Edition 2021)
* Python 3.11+
* Docker & gVisor (`runsc`) runtime

### Installation

Clone the repository and submodules:

```bash
git clone --recursive [https://github.com/itsVentie/Invalru.git](https://github.com/itsVentie/Invalru.git)
cd Invalru

```

Compile the eBPF drivers and Ingestion Engine:

```bash
make build-ingestion

```

Compile the Rust Analysis Plane:

```bash
make build-analysis

```

Initialize the Python Orchestration Environment:

```bash
make init-brain

```

---

## 6. Configuration

Invalru is configured using a unified topology file.

```yaml
engine:
  interface: eth0
  shared_memory_buffer_mb: 512
  grpc_socket_path: /var/run/Invalru.sock

analysis:
  max_ast_depth: 128
  enable_wasm_hooks: true

agents:
  llm_endpoint: http://localhost:8000/v1
  model_name: mistral-nemo-12b-bounty
  max_thought_iterations: 5
  sandbox_runtime: runsc

```

---

## 7. Operational Workflow

To spin up the continuous passive/active pipeline against an authorized target environment:

```bash
sudo ./bin/invalru --config ./config.yaml --live

```

### Stream Pipeline Controls

You can interact with the running orchestration loop using the native CLI client:

```bash
./bin/invalctl status --socket /var/run/invalru.sock

```

```bash
./bin/invalctl agents dump-graph --session-id current

```

```bash
./bin/invalctl export-poc --vuln-id VULN-2026-0912

```

---

## 8. License

Distributed under the Apache License 2.0. See `LICENSE` for details.

