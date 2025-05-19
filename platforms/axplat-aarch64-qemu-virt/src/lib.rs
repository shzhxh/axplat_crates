#![no_std]

#[macro_use]
extern crate axplat;

#[macro_use]
extern crate memory_addr;

mod boot;
mod init;
mod mem;
mod power;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

axplat_aarch64_common::console_if_impl!(ConsoleIfImpl);
axplat_aarch64_common::time_if_impl!(TimeIfImpl);

#[cfg(feature = "irq")]
axplat_aarch64_common::irq_if_impl!(IrqIfImpl);

#[allow(unused_imports)]
use self::config::devices::{RTC_PADDR, UART_PADDR};
use self::config::plat::PSCI_METHOD;
use self::mem::phys_to_virt;

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    axplat::mem::clear_bss();
    axcpu::init::init_cpu(cpu_id);
    axplat_aarch64_common::pl011::init_early(phys_to_virt(pa!(UART_PADDR)));
    axplat_aarch64_common::psci::init(PSCI_METHOD);
    axplat_aarch64_common::generic_timer::init_early();
    #[cfg(feature = "rtc")]
    axplat_aarch64_common::pl031::init_early(phys_to_virt(pa!(RTC_PADDR)));
    axplat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axcpu::init::init_cpu(cpu_id);
    axplat::call_secondary_main(cpu_id);
}
