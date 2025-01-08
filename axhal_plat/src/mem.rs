//! Physical memory information.

use core::fmt;

use memory_addr::{PhysAddr, PhysAddrRange};

bitflags::bitflags! {
    /// The flags of a physical memory region.
    #[derive(Clone, Copy)]
    pub struct MemRegionFlags: usize {
        /// Readable.
        const READ          = 1 << 0;
        /// Writable.
        const WRITE         = 1 << 1;
        /// Executable.
        const EXECUTE       = 1 << 2;
        /// Device memory. (e.g., MMIO regions)
        const DEVICE        = 1 << 4;
        /// Uncachable memory. (e.g., framebuffer)
        const UNCACHED      = 1 << 5;
        /// Reserved memory, do not use for allocation.
        const RESERVED      = 1 << 6;
        /// Free memory for allocation.
        const FREE          = 1 << 7;
    }
}

impl fmt::Debug for MemRegionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// A physical memory region.
#[derive(Debug, Clone, Copy)]
pub struct PhysMemRegion {
    /// The start physical address of the region.
    pub paddr: PhysAddr,
    /// The size in bytes of the region.
    pub size: usize,
    /// The region flags, see [`MemRegionFlags`].
    pub flags: MemRegionFlags,
    /// The region name, used for identification.
    pub name: &'static str,
}

impl PhysMemRegion {
    /// Returns a [`PhysAddrRange`] that represents its physical address range.
    pub fn pa_range(&self) -> PhysAddrRange {
        PhysAddrRange::from_start_size(self.paddr, self.size)
    }
}

/// Fills the `.bss` section with zeros.
///
/// It requires the symbols `_sbss` and `_ebss` to be defined in the linker script.
///
/// # Safety
///
/// This function is unsafe because it writes `.bss` section directly.
pub unsafe fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(_sbss as usize as *mut u8, _ebss as usize - _sbss as usize)
            .fill(0);
    }
}

unsafe extern "C" {
    fn _sbss();
    fn _ebss();
}

/// Physical memory interface.
#[def_plat_interface]
pub trait MemIf {
    /// Returns all normal memory (RAM) regions on the platform.
    fn ram_regions() -> &'static [PhysMemRegion];

    /// Returns all device memory (MMIO) regions on the platform.
    fn mmio_regions() -> &'static [PhysMemRegion];
}
