#!/bin/bash

# Build Rust
cd rust
cargo build --release
cd ..

# Build Cython
cd python
python3 setup.py build_ext --inplace
mv python_helpers.cpython-311-aarch64-linux-gnu.so ../build/python_helpers.so
cd ..