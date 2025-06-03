use axplat::power::PowerIf;

struct PowerImpl;

#[impl_plat_interface]
impl PowerIf for PowerImpl {
    /// Bootstraps the given CPU core with the given initial stack (in physical
    /// address).
    ///
    /// Where `cpu_id` is the logical CPU ID (0, 1, ..., N-1, N is the number of
    /// CPU cores on the platform).
    fn cpu_boot(cpu_id: usize, stack_top_paddr: usize) {
        #[cfg(feature = "smp")]
        crate::mp::start_secondary_cpu(cpu_id, pa!(stack_top_paddr));
        #[cfg(not(feature = "smp"))]
        {
            let _ = (cpu_id, stack_top_paddr);
            log::warn!(
                "feature `smp` is not enabled for crate `{}`!",
                env!("CARGO_CRATE_NAME")
            );
        }
    }

    /// Shutdown the whole system.
    fn system_off() -> ! {
        log::info!("Shutting down...");
        // TODO
        loop {
            axcpu::asm::halt();
        }
    }
}
