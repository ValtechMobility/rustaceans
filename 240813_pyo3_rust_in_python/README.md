# PyO3 - Rust Bindings for Python
**Disclaimer**: This will not be executable in this folder because it starts with a number. Just follow the tutorial and everything should work fine ;)

## Steps

### 1. Virtual Environment setup

```
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin init
```

### 2. Package installieren

For the development part, `maturin develop` is sufficient. To publish, simply use `maturin publish` to push the packages directly to PyPI.

### 3. Project-Files

- `pyproject.toml` packaging information
- `lib.rs` code which will be packaged for python
- `rustaceans.pyi` contains the type definition

### 4. Done
To test run `maturin develop` and use in a python file in the same directory/terminal session, publish using `maturin publish`