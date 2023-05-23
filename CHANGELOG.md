# Changelog

## [0.4.1] - 2023-05-23

- cli: Added json output for get command
- cli: Show output of commands when using `all` (compatible with `get --json`)

## [0.4.0] - 2022-06-30

### Changed

- Updated all dependencies
- Add nix build

### Added

- Binary target providing a cli (`yeelight-cli` is now part of this crate)
- cli: Discovery of lamps
- cli: Run commands by lamp name instead of address
- cli: Run commands on all discovered bulbs
- cli: Timeout connections

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
[0.4.0]: https://github.com/leixb/yeelight/releases/tag/v0.4.0
