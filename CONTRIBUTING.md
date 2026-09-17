# Contributing to GODKILLER ZERO

Thank you for your interest in contributing to GODKILLER ZERO! We welcome contributions that maintain the architectural discipline and rigor of the project.

## Development Workflow

### Prerequisites

- **Rust:** Latest stable toolchain (1.80+)
- **.NET SDK:** .NET 9.0 SDK (for Windows GUI)
- **Git**

### Building and Testing

1. **Clone the repository:**
   ```bash
   git clone https://github.com/taurus42119-stack/godkiller-zero.git
   cd godkiller-zero
   ```

2. **Run test suite:**
   ```bash
   cargo test
   ```

3. **Check formatting and lints:**
   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   ```

4. **Build release binary:**
   ```bash
   cargo build --release
   ```

### Code Style Guidelines

- **Zero Vibe Names:** Avoid generic variables (`data`, `res`, `req`, `item`, `temp`, `val`, `payload`). Use descriptive domain identifiers.
- **Complexity Discipline:** Keep functions concise with Cyclomatic Complexity <= 7 and span <= 70 lines.
- **Error Handling:** Avoid raw `.unwrap()` or `.expect()` in production paths; use structured `Result` and explicit error types.
- **Commit Messages:** Follow Conventional Commits format (e.g., `feat(core):`, `fix(gate):`, `docs(readme):`, `perf(pruner):`).

## Submitting Pull Requests

1. Fork the repository and create your feature branch (`git checkout -b feat/your-feature`).
2. Verify all tests pass (`cargo test`) and lints pass (`cargo clippy --all-targets -- -D warnings`).
3. Commit your changes with conventional commit messages.
4. Push to your branch and open a Pull Request against `main`.
5. Describe the motivation, changes made, and verification steps in the PR description.
