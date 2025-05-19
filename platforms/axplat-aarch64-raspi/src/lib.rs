#![no_std]
#![feature(naked_functions)]

#[macro_use]
extern crate axplat;

#[macro_use]
extern crate memory_addr;

mod boot;
mod init;
mod mem;
mod power;

#[cfg(feature = "smp")]
mod mp;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

axplat_aarch64_common::console_if_impl!(ConsoleIfImpl);
axplat_aarch64_common::time_if_impl!(TimeIfImpl);

#[cfg(feature = "irq")]
axplat_aarch64_common::irq_if_impl!(IrqIfImpl);

use self::config::devices::UART_PADDR;
use self::mem::phys_to_virt;

unsafe extern "C" {
    fn exception_vector_base();
}

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    axplat::mem::clear_bss();
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat_aarch64_common::pl011::init_early(phys_to_virt(pa!(UART_PADDR)));
    axplat_aarch64_common::generic_timer::init_early();
    axplat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat::call_secondary_main(cpu_id);
}
