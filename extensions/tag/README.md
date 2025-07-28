# Tag extension

Build a contract with the `@tag` extension defined as:

```graphql
directive @tag(
  name: String!
) repeatable on FIELD_DEFINITION | INTERFACE | OBJECT | UNION | ARGUMENT_DEFINITION | SCALAR | ENUM | ENUM_VALUE | INPUT_OBJECT | INPUT_FIELD_DEFINITION
```

The contract key is a JSON with two optional keys `includedTags` and `excludedTags`:

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
