import sys

from setuptools import setup
from setuptools.command.test import test as TestCommand

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install the setuptools-rust package.")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding

setup_requires = [
    'setuptools',
    'setuptools-rust>=0.10.1'
]
install_requires = []

setup(
    name="pythoncurrent",
    version="0.1.0",
    author="Juan Gomez",
    rust_extensions=[
        RustExtension(
            'pythoncurrent.pythoncurrent',
            'Cargo.toml',
            binding=Binding.PyO3)
        ],
    packages=['pythoncurrent'],
    package_dir={'':'package'},
    install_requires=install_requires,
    setup_requires=setup_requires,
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False
)
