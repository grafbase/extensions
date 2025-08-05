## General guidelines

- Do not write comments just describing the code. Only write comments describing _why_ that code is written in a certain way, or to point out non-obvious details or caveats. Or to help break up long blocks of code into logical chunks.

## Rust code conventions

- Always use named interpolation in `format!()` and related invocations. GOOD: `format!("Hello, {name}!")`, BAD: `format!("Hello, {}!", name)`.
