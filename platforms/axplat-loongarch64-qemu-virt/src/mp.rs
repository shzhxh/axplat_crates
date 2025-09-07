use axplat::mem::{PhysAddr, phys_to_virt, virt_to_phys};
use loongArch64::ipi::{csr_mail_send, send_ipi_single};

use crate::config::plat::BOOT_VIRT_OFFSET;

const ACTION_BOOT_CPU: u32 = 1;

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(cpu_id: usize, stack_top: PhysAddr) {
    let entry =
        virt_to_phys((crate::boot::_start_secondary as usize).into()).as_usize() | BOOT_VIRT_OFFSET;
    csr_mail_send(entry as _, cpu_id, 0);

    let stack_top = stack_top.as_usize() | BOOT_VIRT_OFFSET;
    csr_mail_send(stack_top as _, cpu_id, 1);

    send_ipi_single(cpu_id, ACTION_BOOT_CPU);
}
