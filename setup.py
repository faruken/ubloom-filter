from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="ubloom-filter",
    version="0.0.1",
    author="faruken",
    author_email="faruken@users.noreply.github.com",
    rust_extensions=[RustExtension("ubloom_filter", binding=Binding.PyO3)],
    install_requires=["setuptools_rust"],
    zip_safe=False
)
