# WWW-Authenticate extension

This is a very simple hook extension for the Grafbase Gateway that inserts the `WWW-Authenticate` header into all responses with status code Unauthorized (401).

The [`WWW-Authenticate` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/WWW-Authenticate) is used to provide information on the authentication process to clients after a failed request. It is a [general HTTP concept](https://httpwg.org/specs/rfc9110.html#field.www-authenticate), and notably useful if you want to make the Gateway act as an [OAuth 2.1 protected resource](https://datatracker.ietf.org/doc/html/rfc9728#name-www-authenticate-response), for example when exposing a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/specification/draft/basic/authorization) server.

## Usage

Add this to your configuration:

```toml
[extensions]
www-authenticate.version = "0.1"

[extensions.www-authenticate.config]
www_authenticate_header_value = """Bearer resource_metadata="https://api.grafbase.com/.well-known/oauth-protected-resource""""
```

The value of `www_authenticate_header_value` will be used as the value of the `WWW-Authenticate` header in all responses with status code Unauthorized (401).
