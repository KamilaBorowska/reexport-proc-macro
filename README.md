# `reexport-proc-macro`

Allows re-exporting a procedural macro while adding additional public items.
This is unnecessary in Rust edition 2018, as it's possible to use `pub use`
syntax for that purpose. However, this crate still exists for Rust versions
before 1.31.

## Examples

This makes a crate export all serde derives.

```rust
#[macro_use]
extern crate reexport_proc_macro;
reexport_proc_macro!(serde_derive);
```
