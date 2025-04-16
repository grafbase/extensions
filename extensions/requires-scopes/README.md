# Requires Scopes extension

Provides the `@requiresScopes` directive which prevents access to elements in the query if the user doesn't have the right OAuth scopes. It expects the authentication token to be in JSON, as provided by the [jwt](https://grafbase.com/extensions/jwt) extension, and have the scopes in OAuth2 format. So a `scope` claim with a list of scopes as a string separated by a space.

## Install

```toml
# grafbase.toml
[extension.requires-scopes]
version = "1.0"
```

Install the extensions before starting the gateway:

```bash
grafbase extension install
```

## Usage

```graphql
extend schema
  @link(url: "https://grafbase.com/extensions/requires-scopes/1.0.5", import: ["@requiresScopes"])

type Query {
  public: String!
  hasReadScope: String @requiresScopes(scopes: "read")
  hasReadAndWriteScope: String @requiresScopes(scopes: [["read", "write"]])
  hasReadOrWriteScope: String @requiresScopes(scopes: [["read"], ["write"]])
}
```

| Claims                    | Field                | Access granted? |
| ------------------------- | -------------------- | --------------- |
| `{"scope": ""}`           | public               | yes             |
| `{"scope": ""}`           | hasReadScope         | no              |
| `{"scope": "read"}`       | hasReadScope         | yes             |
| `{"scope": "read"}`       | hasWriteScope        | no              |
| `{"scope": "read"}`       | hasReadOrWriteScope  | yes             |
| `{"scope": "read,write"}` | hasReadAndWriteScope | yes             |
