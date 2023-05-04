# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!-- next-header -->
## [Unreleased]

### Added

- Added examples to print 3-, 5- and 7-smooth numbers.

## [0.4.0] - 2023-05-04

### Changed

- ⚠️ Changed the meaning of `k` in `smooth(k, n)`.
  
  Previously `smooth(k, n)` generated numbers whose prime factors were smaller
  than the *k*-th prime, now it generates numbers whose prime factors are less
  than or equal to *k*, in accordance with the definition of *k*-smooth numbers.

- Internal refactor to reduce code duplication.

## [0.3.1] - 2023-05-04

### Fixed

- Optimized the generation of primes in the function `smooth`, using
  `primal::Sieve` instead of `primal::Primes`.

## [0.3.0] - 2023-05-04

### Added

- Added function `with_primes` to generate smooth numbers from an arbitrary
  set or primes.
- Added benchmark to compare computing 3-smooth numbers with `pratt(n)` and
  `with_primes(&[2,3], n)`.

## [0.2.1] - 2023-05-04

### Added

- Added example that prints the first 1344 3-smooth numbers
  (all the 3-smooth numbers less than $2^64$, i.e. that fit in a `u64`).

## [0.2.0] - 2023-05-04

### Changed

- Refined the implementation of `pratt` and `smooth` so that they do not
  perform unnecessary computations to check whether to increase the counters.

  Incidentally, this allows to produce possibly more terms before overflow
  (one more term in the case of `pratt`).

## [0.1.0] - 2023-05-03

### Added

- Initial implementation of `pratt` and `smooth`.

<!-- next-url -->
[Unreleased]: https://github.com/FedericoStra/smooth-numbers/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/FedericoStra/smooth-numbers/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/FedericoStra/smooth-numbers/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/FedericoStra/smooth-numbers/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/FedericoStra/smooth-numbers/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/FedericoStra/smooth-numbers/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/FedericoStra/smooth-numbers/releases/tag/v0.1.0
