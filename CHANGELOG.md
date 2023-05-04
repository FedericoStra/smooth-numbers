# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!-- next-header -->
## [Unreleased]

### Added

- Added function `with_primes` to generate smooth numbers from an arbitrary set or primes.

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
[Unreleased]: https://github.com/FedericoStra/smooth-numbers/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/FedericoStra/smooth-numbers/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/FedericoStra/smooth-numbers/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/FedericoStra/smooth-numbers/releases/tag/v0.1.0
