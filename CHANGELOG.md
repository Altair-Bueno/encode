# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.1](https://github.com/Altair-Bueno/encode/compare/v1.0.0...v1.0.1) - 2025-11-25

### Other

- *(deps)* bump actions/checkout in the github-actions-updates group ([#30](https://github.com/Altair-Bueno/encode/pull/30))
- *(deps)* bump actions/checkout in the github-actions-updates group ([#28](https://github.com/Altair-Bueno/encode/pull/28))

## [1.0.0](https://github.com/Altair-Bueno/encode/compare/v0.2.4...v1.0.0) - 2025-06-29

### Fixed

- Update msrv to the smallest supported ([#26](https://github.com/Altair-Bueno/encode/pull/26))

### Other

- [**breaking**] Stabilize API for 1.0.0 ([#20](https://github.com/Altair-Bueno/encode/pull/20))

## [0.2.4](https://github.com/Altair-Bueno/encode/compare/v0.2.3...v0.2.4) - 2025-04-19

### Added

- Added IoEncoder to adapt std::io::Write ([#24](https://github.com/Altair-Bueno/encode/pull/24))

## [0.2.3](https://github.com/Altair-Bueno/encode/compare/v0.2.2...v0.2.3) - 2025-04-19

### Added

- bool arrays can now be used to encode bitflags ([#23](https://github.com/Altair-Bueno/encode/pull/23))

### Other

- Remove markdownlint in favor of prettier ([#21](https://github.com/Altair-Bueno/encode/pull/21))

## [0.2.2](https://github.com/Altair-Bueno/encode/compare/v0.2.1...v0.2.2) - 2025-04-13

### Added

- Break Encoder into StrEncoder and ByteEncoder ([#16](https://github.com/Altair-Bueno/encode/pull/16))

### Fixed

- Update the deprecated attribute with the right version ([#18](https://github.com/Altair-Bueno/encode/pull/18))
- Add missing implementations and attributes ([#17](https://github.com/Altair-Bueno/encode/pull/17))

### Other

- Switch to nightly toolchain so coverage works always ([#15](https://github.com/Altair-Bueno/encode/pull/15))
- Improve test coverage ([#10](https://github.com/Altair-Bueno/encode/pull/10))

## [0.2.1](https://github.com/Altair-Bueno/encode/compare/v0.2.0...v0.2.1) - 2025-03-26

### Added

- Add support for CString ([#12](https://github.com/Altair-Bueno/encode/pull/12))
- Implement `Encodable<E>` for more `Cow`s ([#11](https://github.com/Altair-Bueno/encode/pull/11))

### Other

- Restrict CI to main/master branches
- Cleanup ci and add codecov bot comments on PR ([#9](https://github.com/Altair-Bueno/encode/pull/9))
- *(deps)* bump the github-actions-updates group with 2 updates ([#8](https://github.com/Altair-Bueno/encode/pull/8))
- Combine coverage and doc coverage tarpaulin steps
- Remove cache as it is not needed
- Replace cargo install with taiki-e/install-action so runner doesn't
- Fix README link
- Include codecov badge
- Ensure all tests run
- Add code coverage reporting to CI workflow
- Update repository owner in release-plz workflow
- Replace GitHub Actions workflows with new CI setup based on release-plz ([#5](https://github.com/Altair-Bueno/encode/pull/5))
