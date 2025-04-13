# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
