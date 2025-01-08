#![no_std]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate axhal_plat_macros;

pub mod console;
pub mod init;
pub mod irq;
pub mod mem;
pub mod power;
pub mod time;

pub use axhal_plat_macros::{main, secondary_main};
pub use crate_interface::impl_interface as impl_plat_interface;

#[doc(hidden)]
pub mod __priv {
    pub use crate_interface::{call_interface, def_interface};
}

/// Call the function decorated by [`crate::main`] for the primary core.
pub fn call_main(cpu_id: usize, dtb: usize) -> ! {
    unsafe { __axhal_plat_main(cpu_id, dtb) }
}

/// Call the function decorated by [`crate::secondary_main`] for secondary cores.
pub fn call_secondary_main(cpu_id: usize) -> ! {
    unsafe { __axhal_plat_secondary_main(cpu_id) }
}

unsafe extern "Rust" {
    fn __axhal_plat_main(cpu_id: usize, dtb: usize) -> !;
    fn __axhal_plat_secondary_main(cpu_id: usize) -> !;
}
