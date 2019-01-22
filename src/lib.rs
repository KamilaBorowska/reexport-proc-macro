//! A crate providing a macro to re-export a procedural macro.
//!
//! This allows for adding additional public items in addition to
//! re-exported derives. This is unnecessary in Rust edition 2018,
//! as it's possible to use `pub use` syntax for that purpose.

#![no_std]

/// Re-exports a procedural macro so that all its derives are visible
/// publicly while allowing to add additional public items.
///
/// Intended to be only used with proc-macro crates.
///
/// # Examples
///
/// This makes a crate export all serde derives.
///
/// ```
/// #[macro_use]
/// extern crate reexport_proc_macro;
/// reexport_proc_macro!(serde_derive);
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! reexport_proc_macro {
    ($crate_name:ident) => {
        #[macro_use]
        #[allow(unused_imports)]
        extern crate $crate_name;
        #[doc(hidden)]
        pub use self::$crate_name::*;
    };
}
