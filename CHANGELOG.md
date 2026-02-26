# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.9](https://github.com/ansg191-lab/arr-backup/compare/v0.1.8...v0.1.9) - 2025-04-26

### Fixed

- *(deps)* update rust crate ureq to v3.0.11
- *(deps)* update rust crate time to v0.3.41
- *(deps)* update rust crate zip to v2.4.2
- *(deps)* update rust crate ureq to v3.0.9
- *(deps)* update rust crate http to v1.3.1

### Other

- *(deps)* update dependency cargo-zigbuild to v0.20.0
- Merge pull request #113 from ansg191-lab/renovate/anyhow-1.x-lockfile
- *(deps)* update rust docker tag to v1.86.0
- *(deps)* update actions/create-github-app-token action to v2
- *(deps)* update actions/create-github-app-token action to v1.12.0
- Merge pull request #101 from ansg191-lab/renovate/ureq-3.x-lockfile
- Merge pull request #100 from ansg191-lab/renovate/crate-zip-vulnerability
- *(deps)* update rust docker tag to v1.85.1
- *(deps)* update docker/login-action digest to 74a5d14

## [0.1.8](https://github.com/ansg191-lab/arr-backup/compare/v0.1.7...v0.1.8) - 2025-03-04

### Fixed

- *(deps)* update rust crate anyhow to v1.0.97
- *(deps)* update rust crate ureq to v3.0.8
- *(deps)* update rust crate ureq to v3.0.7

### Other

- Merge pull request [#82](https://github.com/ansg191-lab/arr-backup/pull/82) from ansg191-lab/renovate/docker-build-push-action-digest
- Merge pull request [#83](https://github.com/ansg191-lab/arr-backup/pull/83) from ansg191-lab/renovate/docker-metadata-action-digest
- Merge pull request [#84](https://github.com/ansg191-lab/arr-backup/pull/84) from ansg191-lab/renovate/docker-setup-buildx-action-digest
- Merge pull request [#81](https://github.com/ansg191-lab/arr-backup/pull/81) from ansg191-lab/renovate/zip-2.x-lockfile

## [0.1.7](https://github.com/ansg191-lab/arr-backup/compare/v0.1.6...v0.1.7) - 2025-02-22

### Fixed

- *(ci)* add riscv platform to docker build

## [0.1.6](https://github.com/ansg191-lab/arr-backup/compare/v0.1.5...v0.1.6) - 2025-02-22

### Fixed

- *(ci)* remove riscv64gc-unknown-linux-musl

## [0.1.5](https://github.com/ansg191-lab/arr-backup/compare/v0.1.4...v0.1.5) - 2025-02-22

### Added

- add riscv binary release
- add riscv64 docker image

### Fixed

- specify zigbuild version for renovate
- *(docker)* fix rust image digest
- *(deps)* update rust crate serde to v1.0.218

### Other

- Merge pull request [#74](https://github.com/ansg191-lab/arr-backup/pull/74) from ansg191-lab/renovate/docker-build-push-action-digest
- *(deps)* update docker/setup-buildx-action digest to f7ce87c
- *(deps)* update docker/setup-qemu-action digest to 4574d27
- Revert "ci: switch to multi-runner docker build"
- *(deps)* update rust docker tag to v1.85.0
- Merge pull request [#55](https://github.com/ansg191-lab/arr-backup/pull/55) from ansg191-lab/renovate/ureq-3.x-lockfile
- Merge pull request [#70](https://github.com/ansg191-lab/arr-backup/pull/70) from ansg191-lab/renovate/anyhow-1.x-lockfile
- *(deps)* remove unneeded options from renovate
- Fix alpine package versions
- Use renovate to manage alpine packages

## [0.1.4](https://github.com/ansg191-lab/arr-backup/compare/v0.1.3...v0.1.4) - 2025-02-15

### Other

- fix release CI due to repo transfer

## [0.1.3](https://github.com/ansg191-lab/arr-backup/compare/v0.1.2...v0.1.3) - 2025-02-15

### Fixed

- *(deps)* update rust crate serde to v1.0.216

### Other

- *(deps)* update ureq to 3.0.5
- Merge pull request [#58](https://github.com/ansg191-lab/arr-backup/pull/58) from ansg191-lab/renovate/alpine-3.21
- Merge pull request [#52](https://github.com/ansg191-lab/arr-backup/pull/52) from ansg191-lab/renovate/docker-setup-buildx-action-3.x
- Merge pull request [#56](https://github.com/ansg191-lab/arr-backup/pull/56) from ansg191-lab/renovate/zip-2.x-lockfile
- Merge pull request [#59](https://github.com/ansg191-lab/arr-backup/pull/59) from ansg191-lab/renovate/swatinem-rust-cache-digest
- Merge pull request [#60](https://github.com/ansg191-lab/arr-backup/pull/60) from ansg191-lab/renovate/anyhow-1.x-lockfile
- add missing `merge_group` to rust tests
- modify release-plz to use Github App
- switch to multi-runner docker build
- remove renovate action
- Merge pull request [#33](https://github.com/ansg191-lab/arr-backup/pull/33) from ansg191/renovate/rust-1.83.0-alpine
- *(deps)* update alpine docker tag to v3.21

## [0.1.2](https://github.com/ansg191/arr-backup/compare/v0.1.1...v0.1.2) - 2024-12-03

### Other

- *(deps)* update rust docker tag to v1.83.0

## [0.1.1](https://github.com/ansg191/arr-backup/compare/v0.1.0...v0.1.1) - 2024-12-01

### Fixed

- *(deps)* update rust crate tracing-subscriber to v0.3.19

### Other

- enable semantic commits for renovate
