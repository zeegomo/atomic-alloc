#![no_std]
#[allow(hidden_glob_reexports)]
extern crate alloc;
pub use alloc::*;

#[cfg(not(target_has_atomic))]
pub mod sync {
    pub use portable_atomic_util::*;
}
