# Changelog

All notable changes to the Postgres extension will be documented in this file.

## [0.6.0] - 2025-07-02

### Added
- Updated to use new logging macros for improved debugging
- New SDK resolver API integration

### Changed
- Updated SDK dependencies to latest version

## [0.5.0] - 2025-05-30

### Added
- Schema and table allow/denylist configuration support
- Empty allowlist now properly filters everything out

### Fixed
- Fixed behavior when empty allowlist is specified

## [0.4.9] - 2025-05-21

### Fixed
- Fixed issue with aliases in queries and mutations

## [0.4.8] - 2025-05-21

### Added
- Support for `@derive` and `@is` directives in introspection
- Added ability to define cross-database joins

## [0.4.7] - 2025-05-16

### Fixed
- Fixed typo in composite-schemas spec URL
- Fixed cursor generation to not generate cursors when not needed

## [0.4.6] - 2025-05-15

### Fixed
- Fixed naming issue in pagination filters
- Fixed ordering for next/previous cursors when moving backwards
- Fixed newlines with long base64 cursors
- Improved population of hasNextPage and hasPreviousPage

## [0.4.5] - 2025-05-14

### Added
- Added more scalar types for Postgres
- Reduced the amount of unused types in introspection

## [0.4.4] - 2025-05-09

### Added
- Added support for proper cursor-based pagination

## [0.4.3] - 2025-05-07

### Fixed
- Fixed enable/disable mutations functionality

## [0.4.2] - 2025-05-07

### Fixed
- Fixed Windows build issues

## [0.4.1] - 2025-05-06

### Added
- Added PostgreSQL mTLS validation

## [0.4.0] - 2025-05-05

### Added
- Added support for PostgreSQL views
- Configuration options for enabling/disabling queries or mutations

## [0.3.0] - 2025-04-28

### Added
- Added lookup support for federation
- Improved extension description

## [0.2.0] - 2025-04-25

### Changed
- Various tweaks for consistency in API design

## [0.1.1] - 2025-04-04

### Added
- Initial public release with basic functionality

## [0.1.0] - 2025-04-04

### Added
- Initial implementation of the Postgres extension
