# Contributing

Thank you for considering contributing to `battery_monitor`! Any improvement — bug fix, new feature, documentation update, or platform support — is welcome.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- A machine with a battery (for testing)

### Setup

```sh
git clone https://github.com/AnshulOP/battery_monitor
cd battery_monitor
cargo build
```

---

## How to Contribute

### Reporting a Bug

Open an [issue](https://github.com/AnshulOP/battery_monitor/issues) and include:

- Your OS and version
- Output of `battery_monitor --version`
- Steps to reproduce
- What you expected vs what actually happened

### Suggesting a Feature

Open an issue with the `enhancement` label. Check the **Future Scope** section in the README first — it may already be planned.

### Submitting a Pull Request

1. Fork the repository and create a branch from `main`:
   ```sh
   git checkout -b feature/your-feature-name
   ```

2. Make your changes. Keep commits focused — one logical change per commit.

3. Ensure the project builds cleanly with no warnings:
   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

4. Format your code before committing:
   ```sh
   cargo fmt
   ```

5. Update `CHANGELOG.md` under an `[Unreleased]` section describing what you changed.

6. Open a pull request against `main`. Describe what the PR does and why.

---

## Code Style

- Follow standard Rust idioms — `clippy` is the authority
- No `unwrap()` in paths that can realistically fail; use proper error propagation
- Keep functions small and single-purpose
- No comments that describe *what* the code does — only *why*, when the reason is non-obvious

---

## Platform Testing

If you can only test on one platform, say so in your PR. CI will catch compilation failures on other platforms, but runtime behaviour on Linux/macOS battery APIs may need manual verification.

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
