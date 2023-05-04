# smooth-numbers

> Algorithms to generate smooth numbers

[![crates.io](https://img.shields.io/crates/v/smooth-numbers?logo=rust)](https://crates.io/crates/smooth-numbers)
[![docs.rs](https://img.shields.io/docsrs/smooth-numbers?logo=docsdotrs)](https://docs.rs/smooth-numbers)
[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/smooth-numbers&color=brightgreen&logo=github)](https://github.com/FedericoStra/smooth-numbers)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/smooth-numbers/rust.yml?logo=githubactions&logoColor=white)](https://github.com/FedericoStra/smooth-numbers/actions/workflows/rust.yml)
[![Dependencies status](https://deps.rs/repo/github/FedericoStra/smooth-numbers/status.svg)](https://deps.rs/repo/github/FedericoStra/smooth-numbers)
[![MIT license](https://img.shields.io/crates/l/smooth-numbers)](https://choosealicense.com/licenses/mit/)

See the definition of *smooth number* on
[Wikipedia](https://en.wikipedia.org/wiki/Smooth_number) and
[MathWorld](https://mathworld.wolfram.com/SmoothNumber.html).

## Examples

Compute the first 10 3-smooth numbers, i.e. numbers of the form `2^i * 3^j`:

```rust
use smooth_numbers::*;

assert_eq!(
    smooth(3, 10),
    [1, 2, 3, 4, 6, 8, 9, 12, 16, 18]
);
```

Compute the first 10 numbers of the form `2^i * 5^j`:

```rust
use smooth_numbers::*;

assert_eq!(
    with_primes(&[2, 5], 10),
    [1, 2, 4, 5, 8, 10, 16, 20, 25, 32]
);
```
