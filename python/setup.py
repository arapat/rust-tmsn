from setuptools import setup
from setuptools import find_packages
from setuptools_rust import Binding, RustExtension

setup(
    name="tmsn-py",
    version="0.2",
    rust_extensions=[RustExtension("tmsn.tmsn", binding=Binding.PyO3)],
    packages=find_packages(),
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
