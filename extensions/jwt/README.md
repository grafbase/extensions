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
[extension.jwt.config]
# == Required ==
# URL to download the JWKS for signature validation.
url = "https://example.com/.well-known/jwks.json"

# == Optional ==
# Expected `iss` claim. By default it is NOT validated.
# issuer = "example.com"

# Expected `aud` claim. By default it is NOT validated.
# audience = "my-project"

# How long the JWKS will be cached, in seconds.
poll_interval = 60

# Header name from which to retrieve the JWT token.
# header_name = "Authorization"
# Header value prefix to remove before parsing the JWT token.
# header_value_prefix = "Bearer "

# Or use a Cookie name
# cookie_name = "my-cookie"
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

## Validation mechanism

This extension validates JWT ([RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)) tokens and verifies signatures using JWKs ([RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517)) from `jwks.url`. The validation follows these steps:

1. One of the specified JWK must match the JWT signature.
2. If present, the `exp` claim must be a future timestamp, with a 60-second leeway.
3. If present, the `nbf` claim must be a past timestamp, with a 60-second leeway.
4. With a configured `issuer`, the `iss` claim must match the specified `issuer`.
5. With a configured `audience`, the `aud` claim must match the specified `audience`. If the `aud` claim is an array, at least one of the audience `audience` must match.

**Important**: Be sure to check with your authentication provider whether you must check `audience` and/or the `issuer`, you may accept JWT tokens that weren't intended for you service otherwise.
