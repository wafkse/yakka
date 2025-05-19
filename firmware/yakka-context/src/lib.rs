#![no_std]
#![forbid(missing_docs, missing_debug_implementations)]
//! Global context for the flight control system.
//!
//! This crate provides a global context structure holding several components required for the system.

/// The context associated to the firmware.
#[derive(Debug, Clone, Copy)]
pub struct Context {}
