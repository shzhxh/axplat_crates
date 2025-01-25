//! Physical memory information.

use axhal_plat::mem::{MemIf, RawRange};
use memory_addr::{PhysAddr, VirtAddr};

use crate::config::devices::MMIO_RANGES;
use crate::config::plat::{PHYS_MEMORY_BASE, PHYS_MEMORY_SIZE, PHYS_VIRT_OFFSET};

pub const fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    va!(paddr.as_usize() + PHYS_VIRT_OFFSET)
}

struct MemIfImpl;

#[impl_plat_interface]
impl MemIf for MemIfImpl {
    /// Returns all physical memory (RAM) ranges on the platform.
    fn phys_ram_ranges() -> &'static [RawRange] {
        &[(PHYS_MEMORY_BASE, PHYS_MEMORY_SIZE)]
    }

    /// Returns all reserved physical memory ranges on the platform.
    ///
    /// Lower 1MiB memory is reserved and not allocatable.
    fn reserved_phys_ram_ranges() -> &'static [RawRange] {
        &[(0, 0x100000)]
    }

    /// Returns all device memory (MMIO) ranges on the platform.
    fn mmio_ranges() -> &'static [RawRange] {
        &MMIO_RANGES
    }
}
