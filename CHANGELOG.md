# Changelog

## [0.4.0-rc.1] - 2021-12-30

### Added

- Logging with `log` crate.
- Basic bulb discovery functionality.

### Changed

- Switch `tokio` to version 1.
- All duration parameters now use `std::time::Duration`.
- `start_music` no longer needs `port` parameter.
- Fix `start_music` listening address (#10)
- Improved documentation.

### Removed

- `QuotedString`

[0.4.0-rc.1]: https://github.com/leixb/yeelight/releases/tag/v0.4.0-rc.1
