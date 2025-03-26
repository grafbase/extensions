# JWT

This extension provides JWT authentication for the Grafbase Gateway.

## Installing

Add the following to your Grafbase Gateway configuration file:

```toml
# grafbase.toml

[extensions.jwt]
version = "0.2"
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
header_name = "Authorization"
# Header value prefix to remove before parsing the JWT token.
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
