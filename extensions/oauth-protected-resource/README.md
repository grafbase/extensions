# OAuth 2.0 Protected Resource Metadata extension

This extension implements [RFC 9728](https://datatracker.ietf.org/doc/html/rfc9728) to provide a standard endpoint for OAuth 2.0 protected resources to publish their metadata. The endpoint allows OAuth clients and authorization servers to discover information needed to interact with protected resources.

## Install

```toml
# grafbase.toml
[extensions.oauth-protected-resource]
version = "0.1"
```

Run the install command before starting the gateway:

```bash
grafbase extension install
```

## Configuration

### Basic Configuration

```toml
# grafbase.toml
[extensions.oauth-protected-resource.config.metadata]
# Required - The resource identifier URL for this protected resource
resource = "https://api.example.com"
```

### Advanced Configuration

```toml
# grafbase.toml
[extensions.oauth-protected-resource.config]
# Optional - Override the default path (defaults to "/.well-known/oauth-protected-resource")
path = "/.well-known/oauth-protected-resource"

[extensions.oauth-protected-resource.config.metadata]
# Required - The resource identifier URL for this protected resource
resource = "https://api.example.com"

# Optional - List of authorization servers that can issue tokens for this resource
authorization_servers = ["https://auth.example.com", "https://auth-backup.example.com"]

# Optional - List of supported scopes
scopes_supported = ["read", "write", "admin"]

# Optional - Supported methods for presenting bearer tokens
bearer_methods_supported = ["header", "body"]

# Optional - JWKS URI for the resource server's signing keys
jwks_uri = "https://api.example.com/.well-known/jwks.json"

# Optional - Human-readable information
resource_name = "Example API"
resource_documentation = "https://docs.example.com/api"
resource_policy_uri = "https://example.com/api/policy"
resource_tos_uri = "https://example.com/api/terms"

# Optional - Security features
tls_client_certificate_bound_access_tokens = true
dpop_signing_alg_values_supported = ["RS256"]
dpop_bound_access_tokens_required = false
```

## How It Works

This extension exposes a standardized metadata endpoint (by default at `/.well-known/oauth-protected-resource`) that contains information about your protected resource in JSON format. OAuth clients can use this endpoint to discover:

- Which authorization servers to use
- What scopes to request
- How to present access tokens
- Other security requirements and capabilities

The extension doesn't perform authentication itself - it simply provides discovery information according to the RFC 9728 standard.

## Related Extensions

- [jwt](https://grafbase.com/extensions/jwt) - For JWT token validation

## More Information

For more details on the OAuth Protected Resource Metadata standard, see [RFC 9728](https://datatracker.ietf.org/doc/html/rfc9728).
