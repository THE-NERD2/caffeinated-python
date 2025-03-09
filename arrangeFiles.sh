#!/bin/bash

mkdir -p build/files
cp native/build/caffeinated_python.so build/files/caffeinated_python.so
cp jvm/build/libs/caffeinated-python-$1-all.jar build/files/caffeinated-python-$1-all.jar