#![forbid(unsafe_code)]

//! Framework-independent domain types and application ports.
//!
//! This crate intentionally has no infrastructure, transport, or provider dependencies.

mod domain;
mod error;
mod ids;
mod ports;

pub use domain::*;
pub use error::{CoreResult, DomainError};
pub use ids::*;
pub use ports::*;
