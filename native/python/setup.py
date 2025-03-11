from setuptools import setup, Extension
from Cython.Build import cythonize

setup(
    ext_modules = cythonize([Extension(
        "caffeinated_python",
        ["caffeinated_python.pyx"],
        include_dirs = ["../rust/include"],
        extra_objects = ["../rust/target/release/libcaffeinatedpython.a"]
    )], force = True)
)