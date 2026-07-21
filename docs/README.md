# Building the docs locally

```bash
source .venv/bin/activate
uv pip install ".[docs]"
uv run sphinx-build . _build/html
```

Open `_build/html/index.html` in a browser, or use the live-reload server:

```bash
uv run sphinx-autobuild . _build/html
```
