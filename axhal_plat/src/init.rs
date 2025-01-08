//! Platform initialization.

/// Platform initialization interface.
#[def_plat_interface]
pub trait InitIf {
    /// Initializes the platform devices for the primary CPU.
    fn platform_init();

    /// Initializes the platform devices for secondary CPUs.
    fn platform_init_secondary();
}
