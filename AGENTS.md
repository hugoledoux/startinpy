# AGENTS.md

## Build

Always use **maturin**, never `cargo build`. The `cdylib` crate-type with the `extension-module` PyO3 feature requires maturin to link against the Python shared library.

```bash
# Dev install (editable, fast iteration)
source .venv/bin/activate
maturin develop

# Or without activating:
uv run maturin develop
```

## Python environment

- Managed by **uv** (`.venv/`, `uv.lock` at root).
- Activate with `source .venv/bin/activate` or prefix commands with `uv run`.
- Minimum Rust version: **1.83** (pyo3 0.29 requirement). `rust-toolchain.toml` pins to `stable`.

## Tests

```bash
source .venv/bin/activate
uv pip install ".[test]"    # installs pytest, numpy, laspy, laszip
python -m pytest tests/ -v
```

## Docs

```bash
source .venv/bin/activate
uv pip install ".[docs]"    # installs sphinx, furo, myst-parser, etc.
uv run sphinx-build docs docs/_build/html
```

## LAZ file support

`lazrs` cannot build on Python 3.13 (uses pyo3 0.20). Use **laszip** instead. `[project.optional-dependencies] test` already includes `laszip`, not `lazrs`.

## Linting

- **Python**: Ruff in pyproject.toml (`ruff check`, `ruff format`)
- **Rust**: `cargo fmt` and `cargo clippy` (not yet enforced in CI)

## Architecture

- **Single Rust file**: `src/lib.rs` — all PyO3 bindings (`#[pyclass]`, `#[pymethods]`, `#[pymodule]`)
- **Python package**: `startinpy/__init__.py` re-exports `from startinpy.startinpy import *`
- **Core Rust library**: `startin = "0.8.3"` (the triangulation engine, external crate)
- `.so` files under `startinpy/` are build artifacts (gitignored)

## macOS linker

`.cargo/config.toml` handles the macOS `-undefined dynamic_lookup` flags for both x86_64 and aarch64 via `cfg(target_os = "macos")`.

## CI

- **build.yml**: PR/push/ weekly cron. Runs maturin + pytest on linux/windows/macos (Python 3.12 only).
- **publish.yml**: Triggered by GitHub Release. Builds wheels for Python 3.10–3.14 on linux/windows/macos. No sdist upload.
