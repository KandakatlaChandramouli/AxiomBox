# AxiomBox

**A Research-Grade, OCI-Compatible Container Runtime in Rust**

> *Constructing Linux container primitives from first principles — with correctness, auditability, and systems research as primary objectives.*

---

## Overview

AxiomBox is a research-oriented implementation of the [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec), written in Rust, targeting Linux. It is not a production container runtime. It is a deliberately constructed research artifact: every subsystem is built from scratch, every abstraction is explicit, and every design decision is traceable to a specification, a kernel interface, or a security primitive.

The project's purpose is threefold:

1. **To produce a correct, auditable reference implementation** of the OCI lifecycle model that systems researchers and engineers can read, extend, and reason about.
2. **To serve as an experimental platform** for studying namespace orchestration, cgroup v2 scheduling, rootless container isolation, and runtime security enforcement in a controlled, well-instrumented environment.
3. **To generate a publication-quality research artifact** that documents the engineering of a minimal, verifiable OCI runtime — suitable for systems venues such as OSDI, EuroSys, USENIX ATC, and SOSP.

AxiomBox occupies the same conceptual space as [runc](https://github.com/opencontainers/runc), [crun](https://github.com/containers/crun), and [youki](https://github.com/containers/youki), but with an explicitly different set of priorities: **not performance at the cost of clarity, not compatibility at the cost of correctness, and not features at the cost of auditability**.

---

## Motivation

### The Container Runtime Problem

The container ecosystem has matured rapidly around a core abstraction: the OCI Runtime Specification. Yet the implementations of that specification — runc, crun, youki — are production systems carrying substantial operational complexity. Their source code reflects years of accumulated edge cases, compatibility constraints, and platform-specific workarounds.

This complexity is appropriate for production deployments. It is a hindrance for research.

A researcher who wants to study how namespace isolation is established, how capability bounding sets interact with `execve(2)`, how cgroup v2 hierarchies propagate resource constraints, or how bundle validation affects security posture — faces a significant barrier. The production runtimes are correct, but they are not legible.

AxiomBox is built to be legible.

### Linux Isolation Primitives

Modern Linux container isolation rests on a small set of kernel mechanisms:

- **Namespaces** (`clone(2)`, `unshare(2)`, `setns(2)`): providing isolated views of the PID tree, network stack, mount hierarchy, IPC objects, UTS hostname, user identity, cgroup hierarchy, and time.
- **cgroups v2**: the unified hierarchy for resource accounting and enforcement across CPU, memory, I/O, and PIDs.
- **Capabilities** (`cap_set_proc(3)`, `prctl(2)`): fine-grained decomposition of root privilege into enumerable, droppable units.
- **Seccomp-BPF**: a programmable syscall filter mechanism that allows per-process enforcement of a restricted syscall policy.
- **`pivot_root(2)` / `mount(2)`**: constructing a new filesystem view for the container's root.

No container runtime invents these mechanisms. Every container runtime assembles them. The question is: *in what order, under what invariants, and with what validation?*

AxiomBox makes these assembly decisions explicit, documented, and testable.

### Why Rust

Rust's ownership model eliminates a class of memory safety vulnerabilities that have historically affected C-based systems software. For a security-critical runtime that handles privilege escalation paths, namespace transitions, and filesystem operations, this is not a secondary concern. Additionally, Rust's type system enables encoding of OCI specification invariants at the type level — invalid states become unrepresentable rather than merely runtime-detectable.

---

## Design Philosophy

### Correctness Before Completeness

AxiomBox prioritises being *correct for what it implements* over implementing everything. Each phase of development is considered complete only when the implementation is consistent with the specification, validated by tests, and free of known security defects. Incomplete but correct is preferable to complete but unsound.

### Security as a Structural Property

Security in AxiomBox is not a layer applied after the fact. Path traversal protection, capability validation, namespace safety checks, and bundle integrity enforcement are structural components of the system, present from Phase 1. Subsequent phases inherit and extend this foundation rather than retrofitting it.

### Explicit Architecture

No implicit global state. No hidden control flow. Every subsystem has a defined interface, defined inputs, defined outputs, and defined failure modes. The architecture is intended to be fully visible to a reader working from the source — a property that matters both for research reproducibility and for security auditing.

### Reproducibility

A research artifact must be reproducible. AxiomBox enforces consistent formatting (`rustfmt`), zero-warning compilation (`clippy -D warnings`), and a deterministic test suite. Build reproducibility is a first-class engineering constraint.

### Specification Fidelity

Where the OCI Runtime Specification mandates behaviour, AxiomBox implements that behaviour. Where the specification is ambiguous or underspecified, AxiomBox documents the interpretation taken. Deviations from the specification, if any, are treated as defects.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          AxiomBox CLI                               │
│              (create / start / kill / delete / state)               │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────────────┐
│                       Lifecycle Engine                              │
│        Container state machine · Hook dispatch · PID tracking       │
└──────┬──────────────────┬────────────────────┬───────────────────────┘
       │                  │                    │
┌──────▼──────┐   ┌───────▼───────┐   ┌────────▼────────┐
│  OCI Layer  │   │  Runtime Core │   │  Security Layer │
│             │   │               │   │                 │
│ Spec models │   │ Process spawn │   │ Capabilities    │
│ State model │   │ Sync protocol │   │ Seccomp-BPF     │
│ Bundle load │   │ Error model   │   │ no_new_privs    │
│ Validation  │   │               │   │                 │
└──────┬──────┘   └───────┬───────┘   └────────┬────────┘
       │                  │                    │
┌──────▼──────────────────▼────────────────────▼────────────────────┐
│                     Namespace Layer                                │
│    clone(2) · unshare(2) · setns(2) · user namespace mapping      │
└───────────────────────────┬────────────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────────────┐
│                     Mount / RootFS Layer                           │
│       pivot_root(2) · bind mounts · tmpfs · overlay preparation   │
└───────────────────────────┬────────────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────────────┐
│                     cgroup v2 Layer                                │
│    Hierarchy traversal · Resource controller application          │
│    CPU · memory · pids · io · unified cgroup path management      │
└────────────────────────────────────────────────────────────────────┘
```

Each layer is implemented as an independent Rust module with a defined public interface. Dependencies between layers flow strictly downward. The lifecycle engine orchestrates layer interactions; layers do not invoke each other directly.

---

## Repository Structure

```
axiombox/
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Crate root and public API surface
│   │
│   ├── oci/                     # OCI Layer
│   │   ├── spec.rs              # OCI runtime specification models
│   │   ├── state.rs             # Container state machine (creating/created/running/stopped)
│   │   ├── bundle.rs            # Bundle discovery and loader
│   │   └── validate/
│   │       ├── mod.rs           # Validation orchestration
│   │       ├── semantic.rs      # Spec-level semantic validation
│   │       ├── security.rs      # Security-focused validation
│   │       ├── path.rs          # Path traversal protection
│   │       ├── capabilities.rs  # Capability set validation
│   │       ├── namespaces.rs    # Namespace configuration validation
│   │       ├── hostname.rs      # UTS/hostname validation
│   │       └── annotations.rs  # Annotation key/value validation
│   │
│   ├── lifecycle/               # Lifecycle Engine (Phase 2)
│   ├── runtime/                 # Runtime Core (Phase 3)
│   ├── namespaces/              # Namespace Layer (Phase 4)
│   ├── rootfs/                  # RootFS and Mount Subsystem (Phase 5)
│   ├── cgroup/                  # cgroup v2 Resource Management (Phase 6)
│   └── security/                # Security Hardening (Phase 7)
│
├── tests/
│   ├── oci_unit/                # 54 OCI unit tests
│   └── integration/             # 10 OCI integration tests
│
├── docs/
│   ├── architecture.md          # Architectural decisions and rationale
│   ├── security-model.md        # Security properties and threat model
│   └── oci-conformance.md       # OCI specification compliance notes
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## Completed Milestones

| Phase | Status | Description |
|-------|--------|-------------|
| **Phase 1: OCI Specification Layer** | ✅ Complete | Full OCI runtime spec model in Rust. Container state machine. Bundle discovery and loading. Semantic and security validation. Path traversal protection. Capability, namespace, hostname, and annotation validation. 64 tests passing. |

---

## Roadmap

| Phase | Status | Technical Contribution |
|-------|--------|------------------------|
| **Phase 1** — OCI Specification Layer | ✅ Complete | Correct Rust models of OCI spec. Bundle validation. Security-aware input validation. |
| **Phase 2** — Lifecycle Engine | 🔲 Planned | Container state machine implementation. `create`, `start`, `kill`, `delete` command semantics. OCI hook dispatch. Sync protocol between parent and container init. |
| **Phase 3** — Runtime Core | 🔲 Planned | Process spawning across namespace boundaries. Error propagation across `fork(2)` / `exec(2)` boundaries. PID 1 behaviour in container context. |
| **Phase 4** — Namespace Isolation | 🔲 Planned | Programmatic namespace creation via `clone(2)`. `unshare(2)` and `setns(2)` semantics. User namespace UID/GID mapping. Namespace ordering invariants. |
| **Phase 5** — RootFS and Mount Subsystem | 🔲 Planned | `pivot_root(2)` implementation. Bind mount propagation. `tmpfs` and `devpts` setup. `/proc` and `/sys` masking. OCI mount specification application. |
| **Phase 6** — cgroup v2 Resource Management | 🔲 Planned | Unified cgroup v2 hierarchy traversal. CPU, memory, pids, and I/O controller application. Cgroup path lifecycle management. |
| **Phase 7** — Security Hardening | 🔲 Planned | Capability bounding set application via `prctl(2)`. Seccomp-BPF profile application. `no_new_privs` enforcement. Ambient capability set management. |
| **Phase 8** — OCI Runtime Compatibility | 🔲 Planned | End-to-end compatibility with OCI-compliant container managers (containerd, Podman). `state(1)` command correctness. Integration with standard container image tooling. |
| **Phase 9** — Benchmarking and Evaluation | 🔲 Planned | Container startup latency measurement. Memory footprint analysis. Namespace setup overhead. Comparative evaluation against runc, crun, youki. |
| **Phase 10** — Research Publication Artifact | 🔲 Planned | Publication-ready artifact with reproducible benchmarks, documented design decisions, and research paper supporting material. |

---

## Testing and Validation

### Test Suite

| Category | Count |
|----------|-------|
| OCI unit tests | 54 |
| OCI integration tests | 10 |
| **Total** | **64** |

### Validation Strategy

The test suite for Phase 1 is structured around the following categories:

**Specification Conformance Tests**
Verify that the OCI spec models correctly deserialize and represent all mandatory and optional fields defined in the OCI Runtime Specification. Invalid configurations must be rejected; valid configurations must be accepted.

**Semantic Validation Tests**
Verify that invalid semantic combinations — configurations that are individually syntactically valid but collectively unsound — are detected and reported. Examples include conflicting namespace entries, invalid capability names, and malformed annotation keys.

**Security Validation Tests**
Verify that the security validation layer correctly identifies dangerous configurations: host namespace sharing without explicit intent, overly permissive capability sets, and absence of required security constraints.

**Path Traversal Protection Tests**
Verify that no path accepted by the bundle loader or mount configuration can escape the container root. Directory traversal sequences (`../`), symlink races, and absolute path injections are tested as rejection cases.

### CI Validation

All patches must pass:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Zero warnings are enforced at the compiler level. Formatting is non-negotiable.

---

## Security Model

AxiomBox treats security as a structural property of the runtime, not an operational concern. The following security properties are implemented and tested in Phase 1:

### Path Traversal Protection

Bundle paths and container root paths are canonicalized and validated before use. Path components that traverse parent directories are rejected. Symbolic link resolution is handled with awareness of TOCTOU race conditions.

### Capability Validation

The OCI spec permits configuration of Linux capability sets: `bounding`, `effective`, `inheritable`, `permitted`, and `ambient`. AxiomBox validates that all named capabilities correspond to known Linux capability constants, that capability sets are internally consistent, and that configurations do not inadvertently expand privilege beyond specification intent.

### Namespace Validation

Namespace configurations are validated for internal consistency. Duplicate namespace entries, invalid namespace types, and path-based namespace references that do not exist are rejected at validation time rather than at runtime.

### Bundle Integrity

Bundle structure is validated before any lifecycle operation is attempted. The `config.json` must be present, parseable, and semantically valid. The rootfs path must be resolvable relative to the bundle root. Validation failures produce structured, actionable error messages.

### Threat Model

The threat model for AxiomBox assumes:
- An untrusted bundle may be provided as input.
- The container manager invoking AxiomBox is trusted.
- The host kernel is trusted.
- Privilege escalation from within the container must not be achievable through runtime defects.

Formal threat model documentation is tracked in `docs/security-model.md`.

---

## Engineering Metrics

| Metric | Value |
|--------|-------|
| Implementation language | Rust (stable) |
| Target platform | Linux (kernel ≥ 5.4, cgroup v2) |
| OCI spec version | v1.1.x |
| Total tests | 64 |
| Unit tests | 54 |
| Integration tests | 10 |
| Compiler warnings | 0 (enforced) |
| Clippy lints violated | 0 (enforced) |
| Lines of code (Phase 1) | *measured at release* |
| Modules | *tracked per phase* |
| Test coverage (line) | *instrumented at Phase 3* |
| Container startup latency | *benchmarked at Phase 9* |
| Build status | passing |

---

## Research Directions

AxiomBox is designed to support the following research directions. Each represents an open question in the systems literature with tractable experimental methodology.

### OCI Runtime Architecture and Correctness

The OCI Runtime Specification defines a lifecycle model but leaves substantial implementation latitude. A systematic comparison of how runc, crun, youki, and AxiomBox interpret ambiguous specification language — and what divergent runtime behaviours result — has direct implications for container interoperability and correctness guarantees.

### Rootless Container Isolation

Rootless containers, enabled by user namespaces, trade the need for root privilege against increased complexity in UID/GID mapping and capability handling. The security properties of rootless runtimes — particularly the boundary between user namespace capabilities and host-visible privilege — remain an active research area.

### Namespace Orchestration

The ordering and sequencing of namespace creation is not fully specified by the OCI standard. Different orderings produce different isolation properties and different failure modes. A formal model of namespace creation sequencing, verified against the Linux kernel's namespace implementation, would constitute a novel contribution.

### Runtime Security Policy Enforcement

Seccomp-BPF profile construction is currently ad hoc in most runtimes. A principled methodology for deriving minimal-privilege seccomp profiles from container workload specifications — and verifying those profiles against known syscall attack surfaces — is an open research problem.

### cgroup v2 Resource Scheduling Fidelity

The relationship between OCI resource specifications, cgroup v2 controller parameters, and observed container resource consumption is not well characterised. Experimental measurement of scheduling fidelity under resource pressure, across different workload profiles, would provide practically useful data and a methodological template.

---

## Current Status

> **AxiomBox is under active development and is not production-ready.**

**What exists today (Phase 1 complete):**

- A complete Rust implementation of the OCI runtime specification data model.
- A correct container state machine covering the `creating`, `created`, `running`, and `stopped` states.
- Bundle discovery and loading with full path validation.
- A multi-stage validation pipeline covering semantic correctness, security properties, path safety, capabilities, namespaces, hostname, and annotations.
- 64 passing tests with enforced zero-warning compilation.

**What does not yet exist:**

- Any Linux system call integration (no `clone`, `unshare`, `mount`, `pivot_root`, or `execve`).
- Container process spawning.
- cgroup v2 integration.
- Seccomp-BPF application.
- A functional CLI.
- OCI runtime compatibility with container managers.

The project is at the foundation layer. The structural decisions made in Phase 1 — the type system design, the validation architecture, the error model — establish the substrate on which all subsequent phases are built.

---

## Development Principles

**Specification before implementation.** No subsystem is implemented until its specification-level semantics are understood and documented. Ambiguities in the OCI specification are resolved by reading the spec text, the reference implementation, and the Linux kernel source — in that order.

**Every decision is a tradeoff.** Performance, simplicity, and generality exist in tension. Each tradeoff is made explicitly and documented. An undocumented tradeoff is a design debt.

**Tests encode invariants.** A test is not a verification that the current code produces the current output. A test is a machine-checkable encoding of an invariant the system must satisfy. Tests are written before or alongside the code they validate, not after.

**Security failures are not runtime errors.** A configuration that violates a security invariant is rejected at validation time. It does not produce a runtime failure. The distinction between *invalid input* and *runtime fault* is maintained throughout.

**Warnings are defects.** Compiler warnings and linter warnings indicate code whose behaviour the author cannot fully reason about. They are treated as defects and are not permitted in the codebase.

**Legibility is a design constraint.** Code that is correct but unreadable fails one of AxiomBox's primary objectives. Where performance and legibility conflict, legibility takes precedence until benchmarks identify a specific, measured regression.

---

## References

| Resource | Notes |
|----------|-------|
| [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec) | Primary specification document. All OCI-compliant behaviour in AxiomBox is derived from this specification. |
| [Linux Namespaces — `namespaces(7)`](https://man7.org/linux/man-pages/man7/namespaces.7.html) | Authoritative Linux kernel documentation for all namespace types. |
| [Linux cgroup v2 — `cgroups(7)`](https://man7.org/linux/man-pages/man7/cgroups.7.html) | Kernel documentation for the unified cgroup v2 hierarchy. |
| [Linux Capabilities — `capabilities(7)`](https://man7.org/linux/man-pages/man7/capabilities.7.html) | Capability enumeration and semantics. |
| [runc](https://github.com/opencontainers/runc) | Reference OCI runtime implementation in Go. Production runtime for containerd and Docker. |
| [crun](https://github.com/containers/crun) | High-performance OCI runtime in C. Notable for minimal overhead and rootless container support. |
| [youki](https://github.com/containers/youki) | OCI runtime in Rust. The most directly comparable implementation in terms of language choice. |
| [containerd](https://github.com/containerd/containerd) | Industry-standard container manager. Primary consumer of OCI-compliant runtimes. |
| [gVisor](https://github.com/google/gvisor) | Application kernel for containers. Relevant for its alternative isolation model. |
| [Firecracker](https://github.com/firecracker-microvm/firecracker) | MicroVM monitor from AWS. Relevant for VM-based isolation research and comparison. |
| [Kata Containers](https://github.com/kata-containers/kata-containers) | OCI-compatible runtime using hardware virtualisation for isolation. |
| Kerrisk, M. — *The Linux Programming Interface* | Comprehensive reference for Linux system calls, namespaces, and process semantics. |

---

## Closing

The container runtime is one of the most security-critical components in the modern cloud software stack. It is the process that negotiates the boundary between a workload and the host kernel — establishing the namespace context, constructing the filesystem view, applying the resource policy, and enforcing the privilege constraints that determine what a container can and cannot do.

Despite this criticality, the runtime layer is often treated as infrastructure — a black box operated rather than understood. AxiomBox is built on the premise that this black box should be opened: that a correct, legible, well-tested implementation of the OCI lifecycle model is a contribution in its own right, and that the act of building one from first principles is the most direct path to understanding what container isolation actually means.

The system described here is incomplete. That is the point. It is being built the way all serious systems software should be built: one verified layer at a time, on a foundation that holds.

---

*AxiomBox is developed as a research artifact. Feedback from systems researchers, runtime engineers, and security practitioners is welcomed.*
