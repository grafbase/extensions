# JWT extension

This extension provides JWT authentication for the Grafbase Gateway.

## Installing

Add the following to your Grafbase Gateway configuration file:

```toml
# grafbase.toml
[extensions.jwt]
version = "1"
```

Run the install command before starting the gateway

```bash
grafbase extension install
```

## Configuration

```toml
# grafbase.toml
[extensions.jwt.config]
# == Required ==
# URL to download the JWKS for signature validation.
url = "https://example.com/.well-known/jwks.json"

# == Optional ==
# Expected `iss` claim. By default it is NOT validated.
# issuer = "example.com"

# Expected `aud` claim. By default it is NOT validated.
# If a list is provided, only one of those audience must match.
# Note that `aud` claim can be an array in all cases in the JWT and only one of the `aud` claims
# must match an audience defined here.
# audience = "my-project"
# audience = ["my-project", "my-other-name"]

# How long the JWKS will be cached, in seconds.
poll_interval = 60

# Header name from which to retrieve the JWT token.
# header_name = "Authorization"
# Header value prefix to remove before parsing the JWT token.
# header_value_prefix = "Bearer "

# Or use a Cookie name
# cookie_name = "my-cookie"

# Optional - OAuth Protected Resource Metadata configuration (RFC 9728)
# [extensions.jwt.config.oauth.protected_resource]
# metadata_path = "/.well-known/oauth-protected-resource"  # Optional custom path
# [extensions.jwt.config.oauth.protected_resource.metadata]
# resource = "https://api.example.com"  # Required to enable the metadata endpoint
# authorization_servers = ["https://auth.example.com"]
# scopes_supported = ["read", "write"]

# Optional - Static headers that are returned with 401 Unauthorized responses
# [[extensions.jwt.config.unauthenticated_headers]]
# name = "WWW-Authenticate"
# value = 'Bearer realm="grafbase", error="invalid_token", error_description="Invalid JWT token" resource_metadata="https://api.grafbase.com/.well-known/oauth-protected-resource"'
```

If neither a header nor a cookie is specified, the extension will default to the following:

```toml
header_name = "Authorization"
header_value_prefix = "Bearer "
```

## Usage

Once installed, the authentication extension will be automatically used by the Grafbase Gateway and reject non-authenticated requests.
If you want anonymous users you should change the default authentication in you `grafbase.toml` to:

```toml
# grafbase.toml
[authentication]
default = "anonymous"
```

## OAuth Protected Resource Metadata

The JWT extension can optionally expose an OAuth Protected Resource Metadata endpoint according to [RFC 9728](https://datatracker.ietf.org/doc/html/rfc9728). This allows OAuth clients to discover information about your protected resource.

To enable this feature, add the `oauth.protected_resource.metadata.resource` configuration inside your extension configuration:

```toml
# grafbase.toml
[extensions.jwt.config.oauth.protected_resource.metadata]
resource = "https://api.example.com"  # Required to enable the metadata endpoint
```

## Validation mechanism

This extension validates JWT ([RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)) tokens and verifies signatures using JWKs ([RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517)) from `jwks.url`. The validation follows these steps:

1. One of the specified JWK must match the JWT signature.
2. If present, the `exp` claim must be a future timestamp, with a 60-second leeway.
3. If present, the `nbf` claim must be a past timestamp, with a 60-second leeway.
4. With a configured `issuer`, the `iss` claim must match the specified `issuer`.
5. With a configured `audience`, the `aud` claim must match the specified `audience`. If the `aud` claim is an array, at least one of the audience `audience` must match.

**Important**: Be sure to check with your authentication provider whether you must check `audience` and/or the `issuer`, you may accept JWT tokens that weren't intended for you service otherwise.

## OAuth Protected Resource Metadata

The JWT extension can optionally expose an OAuth Protected Resource Metadata endpoint according to [RFC 9728](https://datatracker.ietf.org/doc/html/rfc9728). This allows OAuth clients to discover information about your protected resource.

To enable this feature, add the `oauth.protected_resource.metadata.resource` configuration inside your extension configuration:

```toml
# grafbase.toml
[extensions.jwt.config.oauth.protected_resource.metadata]
resource = "https://api.example.com"  # Required to enable the metadata endpoint
```

### Full OAuth Metadata Configuration

```toml
# grafbase.toml
[extensions.jwt.config.oauth]
# Optional - Override the default path (defaults to "/.well-known/oauth-protected-resource")
metadata_path = "/.well-known/oauth-protected-resource"

[extensions.jwt.config.oauth.protected_resource.metadata]
# Required - The resource identifier URL for this protected resource
resource = "https://api.example.com"

# Optional - List of authorization servers that can issue tokens for this resource
authorization_servers = ["https://auth.example.com", "https://auth-backup.example.com"]

# Optional - List of supported scopes
scopes_supported = ["read", "write", "admin"]

# Optional - Supported methods for presenting bearer tokens
bearer_methods_supported = ["header", "body"]

# Optional - JWKS URI for the resource server's signing keys (defaults to jwt.config.url)
jwks_uri = "https://api.example.com/.well-known/jwks.json"

# Optional - Human-readable information
resource_name = "Example API"
resource_documentation = "https://docs.example.com/api"
resource_policy_uri = "https://example.com/api/policy"
resource_tos_uri = "https://example.com/api/terms"

# Optional - Security features
tls_client_certificate_bound_access_tokens = true
```

When at least the resource is defined, the JWT extension will serve the OAuth Protected Resource Metadata at the configured path (defaulting to `/.well-known/oauth-protected-resource`). This eliminates the need to install the separate `oauth-protected-resource` extension if you're already using JWT authentication.
