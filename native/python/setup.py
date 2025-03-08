from setuptools import setup
from Cython.Build import cythonize

setup(
    ext_modules = cythonize("caffeinated_python.pyx"),
    include_dirs = ["../rust/include"],
    libraries = ["rust"],
    library_dirs = ["../rust/target/release"]
)