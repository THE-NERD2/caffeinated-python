from setuptools import setup, Extension
from Cython.Build import cythonize

setup(
    ext_modules = cythonize([Extension(
        "python_helpers",
        ["python_helpers.pyx"],
        include_dirs = ["../rust/include"],
        extra_objects = ["../rust/target/release/libcaffeinatedpython.a"]
    )], force = True)
)