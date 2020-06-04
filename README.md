# Writing a Python library in Rust

This is a very simple bloom filter Python library written in Rust. Purpose of this library is to demonstrate extending Python with Rust.

# Installation

1. Clone the repo
2. Install Rust and switch to Rust nightly (`$ rustup default nightly`)
3. Create a new virtualenv. (`$ python3.8 -m venv .virtualenvs/ubloom-filter`)
4. Compile the Rust code (`$ cargo build --release`)
4. Run `setup.py` (`$ python setup.py install`)

# Usage

Sample code:

```python

import ubloom_filter


def main():
    bloom = ubloom_filter.ubloom.BloomFilter(10_000, 0.02)
    bloom.insert("hello")
    bloom.insert("world")
    print(bloom.has("hello"))
    print(bloom.has("world"))
    print(bloom.has("NickiMinaj"))

if __name__ == '__main__':
    main()

```
