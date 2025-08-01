# Authenticated extension

Provides an `@authenticated` directive, for subgraphs, which prevents access to elements in the query when the user is not authenticated.

## Install

```toml
# grafbase.toml
[extensions.authenticated]
version = "1.0"
```

Run the install command before starting the gateway

```bash
grafbase extension install
```

## Usage

```graphql
# subgraph schema
extend schema
  @link(
    url: "https://grafbase.com/extensions/authenticated/1.0.0"
    import: ["@authenticated"]
  )

type Query {
  public: String!
  mustBeAuthenticated: String! @authenticated
}
```
