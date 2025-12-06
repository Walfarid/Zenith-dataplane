# Contributing to Zenith Data Plane

Thank you for your interest in contributing to Zenith! We welcome contributions from the community.

## Getting Started

1.  **Fork** the repository on GitHub.
2.  **Clone** your fork locally.
3.  **Install Prerequisites**:
    *   Rust (latest stable)
    *   Python 3.10+
    *   Go 1.20+ (for SDK)
    *   Wasm runtime target: `rustup target add wasm32-wasip1`

## Development Workflow

1.  Create a feature branch: `git checkout -b feature/my-new-feature`
2.  Make your changes.
3.  Run tests:
    *   Core: `cd core && cargo test`
    *   Integration: `python3 demo_app.py`
4.  Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/).
    *   Example: `feat(core): Add new ring buffer strategy`
5.  Push to your fork and submit a **Pull Request**.

## Code Style

*   **Rust**: Run `cargo fmt` and `cargo clippy` before committing.
*   **Python**: Use `black` and `pylint`.

## Reporting Issues

Please use the GitHub Issues tracker to report bugs or request features. Provide as much detail as possible.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
