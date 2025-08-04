## Rust code conventions

- Always use named interpolation in `format!()` and related invocations. GOOD: `format!("Hello, {name}!")`, BAD: `format!("Hello, {}!", name)`.
