# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## [0.1.6] - 2025-03-10

### Changed

- Upgraded `wasmtime` from v26 to v30.0.2.
- Updated `Cargo.lock` to resolve yanked versons of indirect dependencies.

## [0.1.5] - 2024-10-28

### Added

- Any imports can be mocked without depending on `ic0`.
- Support memory64 Wasm modules.

## [0.1.4] - 2024-05-10

### Added

- Includes new system API `in_replicated_execution`.

## [0.1.3] - 2024-04-22

### Added

- Use `clap` to support `-V`/`--version`  and `-h`/`--help`. (#485)

## [0.1.1] - 2023-10-11

### Added

- Includes new system API `cycles_burn128`. (#434)

## [0.1.1] - 2023-09-19

### Added

- Release from the [CI workflow](../../.github/workflows/release-candid-extractor.yml). (#427)

## [0.1.0] - 2023-09-18

### Added

- The first release. (#424)
