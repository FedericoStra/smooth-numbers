# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

<!-- next-header -->
## [Unreleased]

### Changed

- Refined the implementation of `pratt` and `smooth` so that they do not
  perform unnecessary computations to check whether to increase the counters.

  Incidentally, this allows to produce possibly more terms before overflow
  (one more term in the case of `pratt`).

## [0.1.0] - 2023-05-03

### Added

- Initial implementation of `pratt` and `smooth`.

<!-- next-url -->
[Unreleased]: https://github.com/FedericoStra/smooth-numbers/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/FedericoStra/smooth-numbers/releases/tag/v0.1.0
