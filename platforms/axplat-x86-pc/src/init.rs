use axplat::init::InitIf;

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// Initializes the platform devices for the primary core.
    fn platform_init() {
        crate::apic::init_primary();
        crate::time::init_primary();
    }

    /// Initializes the platform devices for secondary cores.
    fn platform_init_secondary() {
        #[cfg(feature = "smp")]
        {
            crate::apic::init_secondary();
            crate::time::init_secondary();
        }
    }
}
