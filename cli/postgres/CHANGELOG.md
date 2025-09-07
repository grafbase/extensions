# Changelog

All notable changes to the Grafbase PostgreSQL CLI tool will be documented in this file.

## [0.3.10] - 2025-09-07

- Fix an install script issue with 0.3.9. The binary should be identical to 0.3.9.

## [0.3.9] - 2025-09-05

- Generate `@lookup` fields without non-null wrappers, since per the composite schemas spec, lookup fields should be nullable.

## [0.3.7] - 2025-05-22

### Added
- Schema and table filtering with allowlist/denylist options
  - Added `schema_allowlist` and `schema_denylist` for global schema filtering
  - Added `table_allowlist` and `table_denylist` for filtering tables within a schema
  - Denylist takes precedence over allowlist when both are specified

## [0.3.6] - 2025-05-21

### Added
- Support for `@derive` and `@is` directives in introspection
- Added ability to define cross-database joins

## [0.3.5] - 2025-05-21

### Fixed
- Fixed installer URL in documentation
- Fixed typo in composite-schemas spec URL in postgres-introspection

## [0.3.4] - 2025-05-15

### Changed
- Improved documentation clarity
- Removed "you must build" text from documentation

## [0.3.3] - 2025-05-14

### Added
- Reduced the amount of unused types in introspection output
- Added more scalar types for PostgreSQL in the introspection library

## [0.3.2] - 2025-05-09

### Added
- Support for cursor-based pagination in the generated schema

## [0.3.1] - 2025-05-07

### Added
- Installation script for easier setup of the CLI tool

### Fixed
- Fixed enable/disable mutations functionality
- Fixed installation documentation links

## [0.3.0] - 2025-05-07

### Added
- Support for PostgreSQL views
- Configuration options for enabling/disabling queries or mutations
- Added PostgreSQL mTLS validation support

### Fixed
- Fixed CLI behavior when running without .env and config files

## [0.2.0] - 2025-05-05

### Added
- Comprehensive README with usage examples
- Release workflow for CLI binaries
- Improved command-line interface
- Support for lookup functionality for federation

## [0.1.0] - 2025-04-04

### Added
- Initial implementation of the PostgreSQL CLI tool
- Basic introspection capabilities for PostgreSQL databases
- GraphQL SDL generation for tables and relationships
