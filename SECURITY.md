## Security Model

`Invalru` operates under a multi-layered security model designed to safeguard the host infrastructure while performing high-fidelity vulnerability research and dynamic exploit verification.

```
       [ Non-Root Space ]                      [ Isolated Runtime Space ]
+------------------------------+             +----------------------------+
|  Decision Layer (Python)     |             |  gVisor Container Sandbox  |
|  - Payload Synthesis         | --(gRPC)--> |  - Ephemeral State         |
|  - Multi-Agent Orchestration |             |  - PoC Verification Execution|
+------------------------------+             +----------------------------+
               |                                           |
               | (Unix Domain Sockets)                     | (Network Only)
               v                                           v
+------------------------------+                           |
|  Analysis Layer (Rust)       |                           |
|  - SWC / Tree-sitter Parsing  |                           |
|  - WASM Sandboxed Hooks      |                           |
+------------------------------+                           |
               ^                                           |
               | (Shared Memory / Ring Buffers)            |
               |                                           v
+-------------------------------------------------------------------------+
|  INGESTION LAYER (Go Core) - [ PRIVILEGED KERNEL SPACE (eBPF / XDP) ]   |
+-------------------------------------------------------------------------+

```

### 1. Privilege Separation & Boundary Isolation

* **Minimal Kernel Interface:** The Go ingestion engine requires `CAP_SYS_ADMIN` (or `CAP_BPF` / `CAP_NET_ADMIN` on modern kernels) exclusively to load and attach eBPF programs to the XDP driver. Once the ring buffers are allocated, raw socket processing stays isolated.
* **Deterministic Static Analysis:** The Rust plane processes AST structures, untrusted third-party transpiled code, and syntax mutations inside a deterministic runtime. Custom analysis hooks are explicitly executed within sandboxed WebAssembly (`wasmer`) environments to prevent host environment subversion via malicious target code.
* **Ephemeral Containment:** Any active vulnerability verification or dynamic execution of generated Proof of Concepts (PoC) by the Python orchestration agents is strictly restricted to ephemeral `gVisor` (`runsc`) container spaces. Network interactions are confined to prevent side-channel targeting or accidental impacts on production networks outside the target configuration scope.

### 2. Memory & In-Transit Telemetry Protections

* **Zero-Copy Bounds Validation:** Telemetry data mapping from kernel space to user space relies on fixed-size memory-mapped ring buffers. Strict bounds verification is enforced in both the Go receiver and the Rust consumer to prevent memory exhaustion, ring-buffer corruption, or buffer overflow attacks.
* **Local IPC Access Control:** In-system communication between components utilizes gRPC over Unix Domain Sockets (`/var/run/invalru.sock`). File permissions are locked down down to the executing user group to mitigate unauthorized pipeline hijacking or rogue telemetry injection.

### 3. Supply Chain & Model Guardrails

* **Local-Only Semantic Inference:** The execution engine communicates strictly with a local inference endpoint (`mistral-nemo-12b`). No source trees, AST fragments, target IPs, or payload profiles are transmitted to external third-party APIs.

---

## Supported Versions

Only the latest release version or active commits on the `main` branch receive security patches and updates.

---

## Reporting a Vulnerability

If you discover a vulnerability within the eBPF kernel loader, IPC transit mechanics, WASM sandbox escape vectors, or gVisor orchestration layer, do not open a public GitHub issue.

Email detailed security reports directly to: **[bugs@ventie.dev]**

### Please include:

* A thorough description of the bug or design flaw (e.g., memory corruption, escape vector, privilege escalation).
* A minimal, reproducible Proof of Concept (PoC) code or step-by-step reproduction instructions.
* An evaluation of potential impacts on the host operating system or neighboring network stacks.

An acknowledgement will be issued within 48 hours, followed by a coordinated disclosure schedule.
