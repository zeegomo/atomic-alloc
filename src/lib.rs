#![no_std]

extern crate alloc as __allocc;
pub use __allocc::*;

#[cfg(not(target_has_atomic = "ptr"))]
pub mod sync {
    pub use portable_atomic_util::*;
}
