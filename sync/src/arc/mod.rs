// This module is an userspace implementation of kernel::sync [1].
// Lacking of origin kernel's support, this module differs the origin
// kerne's impelementation in the following ways:
// * Use "Arc" in Rust's standard library instead of self-implemented
// 
// [1]: https://origin.kernel.org/doc/rustdoc/v6.8/kernel/sync/


mod arc;

pub use arc::Arc;