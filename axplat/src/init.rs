//! Platform initialization.

/// Platform initialization interface.
#[def_plat_interface]
pub trait InitIf {
    /// Initializes the platform devices for the primary core.
    fn platform_init();

    /// Initializes the platform devices for secondary cores.
    fn platform_init_secondary();
}
