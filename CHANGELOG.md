# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- README with screenshots, architecture notes, Code of Conduct, security policy, pull request
  template, CODEOWNERS and Dependabot configuration.

### Fixed
- Smoke test race when switching Explore tabs, which failed CI on fast runners.

## [0.1.0] - 2026-10-05

### Added
- Parallax: per-issue USDC bounties, escrowed and released on merge, with a receipt per payout.
- Maintainer area: connect and verify a repository, post funded issues, assign one contributor,
  merge and release.
- Contributor area: assignments, pull request submission, receipts, Stellar payout address.
- Soroban escrow contract with `fund`, `assign`, `release` and `refund`, and unit tests.
- CI for the web build, end-to-end smoke test, and contract format, lint, tests and wasm build.

### Changed
- Rebranded from Surge. Replaced the wave, pool and points model with per-issue escrow.

[Unreleased]: https://github.com/zeemscript/parallax/compare/main...HEAD
[0.1.0]: https://github.com/zeemscript/parallax/commits/main
