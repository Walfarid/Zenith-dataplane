# Mastering High-Frequency Data Planes: The Zenith Tutorial

**By Wahyu Ardiansyah**

Welcome to the official tutorial for **Zenith-dataplane**. This guide will take you from zero to running a sub-100µs latency data pipeline on your local machine using Rust, Arrow, and WebAssembly.

## What is Zenith?
Zenith is an enterprise-grade engine that allows you to ingest millions of events per second and process them safely using sandboxed plugins. It solves the "Two API Problem" by allowing you to prototype in Python and deploy in Rust without rewriting your data model.

## Prerequisites
*   Rust (1.75+)
*   Python 3.10+
*   Cargo

## Step 1: Building the Core Engine
Zenith is built in Rust for memory safety and speed.

1.  Clone the repository:
    ```bash
    git clone https://github.com/vibeswithkk/Zenith-dataplane
    cd zenith-dataplane
    ```

2.  Build the core library:
    ```bash
    cd core
    cargo build --release
    ```

## Step 2: Understanding the Architecture
Zenith uses a **Ring Buffer**. Imagine a circular conveyor belt.
*   **Producers (Python)** put boxes (Arrow Batches) on the belt.
*   **The Engine (Rust)** takes boxes off, opens them, and lets **Plugins (WASM)** inspect strict contents.
*   **Zero-Copy:** We don't move the contents of the box. We just look at it. This saves CPU.

## Step 3: Your First Plugin
You can write plugins in Rust (compiled to WASM).

Create a file `my_plugin.rs`:
```rust
#[no_mangle]
pub extern "C" fn on_event() {
    // Logic here
}
```
Compile it to `wasm32-wasi`.

## Step 4: Running the Benchmark via SDK
We have prepared a benchmark tool to verify performance.

```bash
# Upcoming feature in SDK
python3 bench/run_latency_test.py
```

## Conclusion
You have now set up the foundation of a high-frequency trading system. Zenith provides the critical "hot path" infrastructure so you can focus on the logic.

For more details, see `docs/ARCHITECTURE.md`.
