//! Description tables (per-CPU GDT, per-CPU ISS, IDT)

use axhal_cpu::{GdtStruct, IdtStruct, TaskStateSegment};
use lazyinit::LazyInit;

static IDT: LazyInit<IdtStruct> = LazyInit::new();

#[percpu::def_percpu]
static TSS: LazyInit<TaskStateSegment> = LazyInit::new();

#[percpu::def_percpu]
static GDT: LazyInit<GdtStruct> = LazyInit::new();

fn init_percpu() {
    percpu::init_percpu_reg(super::current_cpu_id());
    unsafe {
        IDT.load();
        let tss = TSS.current_ref_mut_raw();
        let gdt = GDT.current_ref_mut_raw();
        tss.init_once(TaskStateSegment::new());
        gdt.init_once(GdtStruct::new(tss));
        gdt.load();
        gdt.load_tss();
    }
}

/// Initializes IDT, GDT on the primary CPU.
pub fn init_primary() {
    axplat::console_println!("\nInitialize IDT & GDT...");
    percpu::init();
    IDT.init_once(IdtStruct::new());
    init_percpu();
}

/// Initializes IDT, GDT on secondary CPUs.
#[cfg(feature = "smp")]
pub fn init_secondary() {
    init_percpu();
}
