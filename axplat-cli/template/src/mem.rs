use axhal_plat::mem::{MemIf, PhysMemRegion};

struct MemIfImpl;

#[impl_plat_interface]
impl MemIf for MemIfImpl {
    /// Returns all normal memory (RAM) regions on the platform.
    fn ram_regions() -> &'static [PhysMemRegion] {
        todo!()
    }

    /// Returns all device memory (MMIO) regions on the platform.
    fn mmio_regions() -> &'static [PhysMemRegion] {
        todo!()
    }
}
