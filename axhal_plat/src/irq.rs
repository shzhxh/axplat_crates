//! Interrupt request (IRQ) handling.

pub use handler_table::HandlerTable;

/// The type if an IRQ handler.
pub type IrqHandler = handler_table::Handler;

/// IRQ management interface.
#[def_plat_interface]
pub trait IrqIf {
    /// Enables or disables the given IRQ.
    fn set_enable(vector: usize, enabled: bool);

    /// Registers an IRQ handler for the given IRQ.
    ///
    /// Returns `true` if the handler is successfully registered.
    fn register(vector: usize, handler: IrqHandler) -> bool;

    /// Unregisters the IRQ handler for the given IRQ.
    ///
    /// Returns the existing handler if it is registered, `None` otherwise.
    fn unregister(vector: usize) -> Option<IrqHandler>;

    /// Handles the IRQ.
    ///
    /// This function is called by the common interrupt handler. It should look
    /// up in the IRQ handler table and calls the corresponding handler. If
    /// necessary, it also acknowledges the interrupt controller after handling.
    fn handle(vector: usize);
}
