# Sreyeesh Project README

## Project Overview

This project integrates Rust with Python using the `pyo3` crate and provides a Python package that utilizes Rust code. It includes a Python package `randomos` that uses Rust to provide a function for getting a random operating system name.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── __pycache__
│   └── randomos.cpython-310.pyc
├── build
│   └── lib.linux-x86_64-cpython-310
│       └── randomos
├── hello_world
│   └── src
│       └── main.rs
├── pyproject.toml
├── randomos
│   └── __init__.py
├── randomos.py
├── requirements.txt
├── setup.py
├── src
│   └── lib.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   │   ├── build
│   │   ├── deps
│   │   ├── examples
│   │   ├── incremental
│   │   ├── librandomos.d
│   │   ├── librandomos.so
│   │   └── maturin
│   ├── release
│   │   ├── build
│   │   ├── deps
│   │   ├── examples
│   │   ├── incremental
│   │   ├── librandomos.d
│   │   └── librandomos.so
│   └── wheels
│       └── randomos-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl
└── test_randomos.py
```

## Setup Instructions

### 1. Rust Setup

Ensure Rust is installed on your system. You can install Rust using `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Python Setup

Create a Python virtual environment and activate it:

```sh
python -m venv .venv
source .venv/bin/activate
```

Install required Python packages:

```sh
pip install maturin
```

### 3. Project Configuration

#### `Cargo.toml`

The `Cargo.toml` file configures the Rust package. Ensure it looks like this:

```toml
[package]
name = "randomos"
version = "0.1.0"
edition = "2021"

[dependencies]
pyo3 = { version = "0.18", features = ["extension-module"] }
rand = "0.8"

[lib]
crate-type = ["cdylib"]
```

#### `pyproject.toml`

Ensure `pyproject.toml` includes:

```toml
[build-system]
requires = ["maturin"]
build-backend = "maturin"
```

#### `setup.py`

Ensure `setup.py` is configured for `setuptools-rust`:

```python
from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="randomos",
    version="0.1.0",
    rust_extensions=[RustExtension("randomos.randomos_rust")],
    zip_safe=False,
)
```

### 4. Building and Installing

Build the Rust extension and install it:

```sh
maturin build
maturin develop
```

Or use `setup.py` for installation:

```sh
python setup.py build
pip install .
```

### 5. Running Tests

Run the Python tests to ensure everything is working correctly:

```sh
python test_randomos.py
```

### 6. Directory and File Descriptions

- **`Cargo.toml`**: Rust package configuration.
- **`pyproject.toml`**: Build configuration for Python packaging.
- **`setup.py`**: Python setup script for `setuptools-rust`.
- **`src/lib.rs`**: Rust source code.
- **`randomos/__init__.py`**: Python package initialization file.
- **`test_randomos.py`**: Python test script.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- [pyo3](https://pyo3.rs) for Python bindings in Rust.
- [maturin](https://maturin.rs) for building and publishing Rust extensions for Python.

---

Let me know if you need any modifications or additional details!