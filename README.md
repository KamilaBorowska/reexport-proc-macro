# `reexport-proc-macro`

Allows re-exporting a procedural macro while adding additional public items.

## Examples

This makes a crate export all serde derives.

```rust
#[macro_use]
extern crate reexport_proc_macro;
reexport_proc_macro!(serde_derive);
```
