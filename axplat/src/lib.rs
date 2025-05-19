#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate axplat_macros;

pub mod console;
pub mod init;
pub mod irq;
pub mod mem;
pub mod power;
pub mod time;

pub use axplat_macros::{main, secondary_main};
pub use crate_interface::impl_interface as impl_plat_interface;

#[doc(hidden)]
pub mod __priv {
    pub use crate_interface::{call_interface, def_interface};
}

/// Call the function decorated by [`axplat::main`][main] for the primary core.
pub fn call_main(cpu_id: usize, dtb: usize) -> ! {
    unsafe { __axplat_main(cpu_id, dtb) }
}

/// Call the function decorated by [`axplat::secondary_main`][secondary_main] for secondary cores.
pub fn call_secondary_main(cpu_id: usize) -> ! {
    unsafe { __axplat_secondary_main(cpu_id) }
}

unsafe extern "Rust" {
    fn __axplat_main(cpu_id: usize, dtb: usize) -> !;
    fn __axplat_secondary_main(cpu_id: usize) -> !;
}
