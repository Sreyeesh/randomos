from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="randomos",
    version="0.1.0",
    rust_extensions=[RustExtension("randomos.randomos_rust")],
    packages=["randomos"],
    zip_safe=False,
)
