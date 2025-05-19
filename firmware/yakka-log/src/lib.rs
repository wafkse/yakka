#![no_std]
#![forbid(missing_docs, missing_debug_implementations)]
//! Yakka Log
//!
//! This is the logging facade for all embedded systems, based on the `defmt` deffered logging framework.
//!
//! To allow for great developer experience, this library is designed to be used both as a over-the-air and over-USB logging facade, forwarding all logs to a remote server.
//!
//! This crate also provides a panic handler that will forward the panic message to the logger.

pub mod panic;

pub mod usb;

pub mod facade;

pub mod embed;
