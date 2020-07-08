from setuptools import setup
import toml

with open("README.rst", "r") as fh:
    long_description = fh.read()

with open("Cargo.toml") as fp:
    f = toml.load(fp)
    version = f["package"]["version"]
    description = f["package"]["description"]



setup(
    name="startin",
    version=version,
    description=description,
    long_description=long_description,
    long_description_content_type="text/x-rst",
    url='https://github.com/hugoledoux/startin_python',
    author='Hugo Ledoux',
    author_email='h.ledoux@tudelft.nl',
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["startin"],
    python_requires='>=3.6',
)



