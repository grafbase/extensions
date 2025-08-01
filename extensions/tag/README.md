# Tag extension

Build a contract with the `@tag` extension defined as:

```graphql
directive @tag(
  name: String!
) repeatable on FIELD_DEFINITION | INTERFACE | OBJECT | UNION | ARGUMENT_DEFINITION | SCALAR | ENUM | ENUM_VALUE | INPUT_OBJECT | INPUT_FIELD_DEFINITION
```

Subgraphs can use it as follows:

```graphql
extend schema
  @link(url: "https://grafbase.com/extensions/tag/1.0.0", import: ["@tag"])

type Accounts @tag(name: "private") {
  id: ID!
}
```

The contract key[^1] is a JSON with two optional keys `includedTags` and `excludedTags`:

[^1]: The contract key can be defined statically in the [gateway configuration](/docs/gateway/configuration/graph) or dynamically with the `on_request` hook.

```json
{
  "includedTags": ["a"],
  "excludedTags": ["b"]
}
```

If the `includedTags` list is empty, the contract schema includes each type and object/interface field unless it's tagged with an excluded tag.

If the `includedTags` list is non-empty, the contract schema excludes each union type and object/interface field unless it's tagged with an included tag.

Unreachable types can be hidden with the following configuration:

```toml
[extensions.tag.config]
hide_unreachable_types = true
```
