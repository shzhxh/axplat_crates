#![no_std]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate log;
extern crate alloc;

pub mod generic_timer;
pub mod gic;
pub mod pl011;
pub mod pl031;
pub mod psci;
