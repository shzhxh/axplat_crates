//! GICv3 中断控制器

use arm_gic_driver::v3::Gic;
use kspin::SpinNoIrq;
use alloc::boxed::Box;
use core::ptr::NonNull;
use aarch64_cpu::registers::*;
use axplat::irq::{IrqIf, HandlerTable, IrqHandler};


const MAX_IRQ_COUNT: usize = 1024;
static IRQ_HANDLER_TABLE: HandlerTable<MAX_IRQ_COUNT> = HandlerTable::new();

static GICD: SpinNoIrq<Option<Gic>> = SpinNoIrq::new(None);
static GICR: SpinNoIrq<Option<Box<dyn arm_gic_driver::local::Interface>>> = SpinNoIrq::new(None);

pub struct IrqIfImpl;
impl IrqIf for IrqIfImpl {
    fn set_enable(irq: usize, enabled: bool) {
        warn!("call GIC set enable: {} {}, but it is not implemented", irq, enabled);
    }

    fn register(irq: usize, handler: IrqHandler) -> bool {
        if IRQ_HANDLER_TABLE.register_handler(irq, handler) {
            Self::set_enable(irq, true);
            true
        } else {
            false
        }
    }

    fn unregister(irq: usize) -> Option<IrqHandler> {
        Self::set_enable(irq, false);
        IRQ_HANDLER_TABLE.unregister_handler(irq)
    }

    fn handle(_unused: usize) {
        let Some(irq) =  GICR.lock().as_mut().unwrap().ack() else {
            return;
        };
        let irq_num: usize = irq.into();
        // trace!("IRQ {}", irq_num);
        if !IRQ_HANDLER_TABLE.handle(irq_num as _) {
            warn!("Unhandled IRQ {irq_num}");
        }

        GICR.lock().as_mut().unwrap().eoi(irq);
        if GICR.lock().as_mut().unwrap().get_eoi_mode() {
            GICR.lock().as_mut().unwrap().dir(irq);
        }
    }

    fn send_ipi(irq_num: usize, target: axplat::irq::IpiTarget) {
        todo!("send ipi");
    }
}

pub(crate) fn init(gicd_base: axplat::mem::VirtAddr, gicr_base: axplat::mem::VirtAddr) {
 let mut gicd = arm_gic_driver::v3::Gic::new(
        NonNull::new(gicd_base.as_mut_ptr()).unwrap(),
        NonNull::new(gicr_base.as_mut_ptr()).unwrap(),
    );

     debug!("Initializing GICD at {:#x}", gicd_base);
    gicd.open().unwrap();

    info!(
        "Initializing GICR for BSP. Global GICR base at {:#x}",
        gicr_base
    );
    let mut interface = gicd.cpu_local().unwrap();
    interface.open().unwrap();

    GICD.lock().replace(gicd);
    GICR.lock().replace(interface);
    info!("GIC initialized {}",current_cpu());
}

#[allow(dead_code)]
pub(crate) fn init_current_cpu() {
    debug!("Initializing GICR for current CPU {}",current_cpu());
    let mut interface = GICD.lock().as_mut().unwrap().cpu_local().unwrap();
    interface.open().unwrap();
    GICR.lock().replace(interface);
    debug!(  "Initialized GICR for current CPU {}",current_cpu());
}

fn current_cpu() -> usize {
    MPIDR_EL1.get() as usize & 0xffffff
}

pub(crate) fn set_enable(irq_num: usize, enabled: bool) {
    use arm_gic_driver::local::cap::ConfigLocalIrq;

    let mut gicd = GICD.lock();
    let d = gicd.as_mut().unwrap();

    if irq_num < 32 {
        trace!("GICR set enable: {} {}", irq_num, enabled);

        if enabled {
            d.get_gicr().irq_enable(irq_num.into()).unwrap();
        } else {
            d.get_gicr().irq_disable(irq_num.into()).unwrap();
        }
    } else {
        trace!("GICD set enable: {} {}", irq_num, enabled);

        if enabled {
            d.irq_enable(irq_num.into()).unwrap();
        } else {
            d.irq_disable(irq_num.into()).unwrap();
        }
    }
}