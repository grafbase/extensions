# Changelog

## [1.3.0] - 2025-07-15

- Added support for static headers returned with 401 responses.
- Renamed the `[extensions.jwt.config.oauth]` section of the configuration to `[extensions.jwt.config.oauth.protected_resource.metadata]`.

## [1.2.0] - 2025-06-25

- Added support for [OAuth 2.0 Protected Resource Metadata](https://datatracker.ietf.org/doc/html/rfc9728) and returning the `WWW-Authenticate` header on unsuccessful authentication. See the README for the new configuration options.

## [1.1.0] - 2025-05-01

- Support for an array in `audience` configuration.
- Better configuration de-serialization errors.

## [1.0.0] - 2025-05-01

- No changes.

## [0.2.8] - 2025-04-31

- Adding `cookie_name` parameter to retrieve a JWT token from a cookie.
