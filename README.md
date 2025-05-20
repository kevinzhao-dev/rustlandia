# rustlandia

## Month 1
| Week  | Focus                                 | Targets                                                                                                          | Hands-on & Deliverable                                                                                   |
| ----- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| **1** | Development Environment & Rust Basics | • Install `rustup`, `cargo`, `clippy`, `rust-analyzer`  <br>• Master basic types, control flow, pattern matching | **Hello Cargo** – build a first CLI that prints system info; enable CI with GitHub Actions               |
| **2** | Ownership, Borrowing, Lifetimes       | • Grasp `move`, borrowing rules, lifetime annotations <br>• Drill with *rustlings*                               | Implement a **generic LRU cache** (uses interior mutability); test-coverage ≥ 90 %                       |
| **3** | Generics, Traits, Type System         | • Trait objects, `impl Trait`, higher-rank trait bounds <br>• Review common std collections                      | Build a **multithreaded work queue (thread pool)** exposing a generic task interface                     |
| **4** | Modules, Crates, Ecosystem & Workflow | • Cargo workspaces, feature flags, profiles <br>• Versioning & release flow                                      | Package and publish a first crate (e.g., simple `rate-limiter`) to crates.io with semver docs on docs.rs |

## Month 2
| Week  | Focus                                 | Targets                                                                                            | Hands-on & Deliverable                                                                                            |
| ----- | ------------------------------------- | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **5** | Error Handling, Testing, Benchmarking | • `Result`, `thiserror`, `anyhow` <br>• Unit/integration tests, `criterion` benchmarks             | Add full test + bench pipeline to earlier crates; export HTML benchmark report                                    |
| **6** | Concurrency (Sync) & Multithreading   | • `Send`/`Sync`, `Arc`/`RwLock`/`mpsc` <br>• Data-parallelism with **Rayon**                       | Use Rayon to batch-process image convolution; measure and document speed-up                                       |
| **7** | Asynchronous (Async) & Networking     | • `async/await`, `Future`, `Pin`/`Unpin` <br>• Ecosystem: **tokio**, **reqwest**, **tonic** (gRPC) | Build an **async HTTP server** (echo + JSON API) and stress-test to 10 k RPS                                      |
| **8** | Macros & Code Generation              | • Declarative vs. procedural macros <br>• `syn` / `quote` toolchain                                | Create a **custom derive macro** (e.g., auto-implement `Display`/`FromStr` for enums) and publish it to crates.io |

## Month 3
| Week   | Focus                                     | Targets                                                                                                                 | Hands-on & Deliverable                                                                                                                                   |
| ------ | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **9**  | Systems Programming & `unsafe`            | • `unsafe`, FFI, `no_std`, memory layout <br>• SIMD, `#[repr(C)]`                                                       | Re-implement part of the LRU cache as a **C-callable static library**; call it from a C++ example                                                        |
| **10** | CLI & Dev-Prod Best Practices             | • **clap v4** builder API, **miette** error reports <br>• Cross-compilation, release profiles                           | Ship a **cross-platform CLI** (macOS/Linux/Windows) supporting `--generate-completions`                                                                  |
| **11** | Embedded Rust (ARM Cortex-M)              | • `embedded-hal`, `rtic`, `probe-rs` <br>• Drive I²C/SPI/GPIO                                                           | On STM32 or RP2040 build an **environment sensor** (temperature + OLED); CI runs `cargo-embedded-lint`                                                   |
| **12** | Capstone: Distributed-System Mini-Project | • Raft consensus or key-value store (inspired by MIT 6.5840) <br>• gRPC + tokio, property-based tests with **proptest** | **Mini-KV**: <br>1️⃣ Raft protocol  2️⃣ Client SDK  3️⃣ Fault-tolerance test scripts. <br>Write a 3-page README/post-mortem explaining design trade-offs |


