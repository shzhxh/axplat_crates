use axhal_plat::irq::{IrqHandler, IrqIf};

struct IrqIfImpl;

#[impl_plat_interface]
impl IrqIf for IrqIfImpl {
    /// Enables or disables the given IRQ.
    fn set_enable(vector: usize, enabled: bool) {
        todo!()
    }

    /// Registers an IRQ handler for the given IRQ.
    ///
    /// Returns `true` if the handler is successfully registered.
    fn register(vector: usize, handler: IrqHandler) -> bool {
        todo!()
    }

    /// Unregisters the IRQ handler for the given IRQ.
    ///
    /// Returns the existing handler if it is registered, `None` otherwise.
    fn unregister(vector: usize) -> Option<IrqHandler> {
        todo!()
    }

    /// Handles the IRQ.
    ///
    /// This function is called by the common interrupt handler. It should look
    /// up in the IRQ handler table and calls the corresponding handler. If
    /// necessary, it also acknowledges the interrupt controller after handling.
    fn handle(vector: usize) {
        todo!()
    }
}
