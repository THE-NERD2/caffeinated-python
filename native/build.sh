#!/bin/bash

# Build Rust
cd rust
cargo build --release
cd ..

# Build Cython
cd python
python3 setup.py build_ext --inplace
mv caffeinated_python.so ../build
cd ..