# `reexport-proc-macro`

[![Build Status](https://travis-ci.org/xfix/reexport-proc-macro.svg?branch=master)](https://travis-ci.org/xfix/reexport-proc-macro)
[![codecov](https://codecov.io/gh/xfix/reexport-proc-macro/branch/master/graph/badge.svg)](https://codecov.io/gh/xfix/reexport-proc-macro)

Allows re-exporting a procedural macro while adding additional public items.

## Examples

This makes a crate export all serde derives.

```
#[macro_use]
extern crate reexport_proc_macro;
reexport_proc_macro!(serde_derive);
```
