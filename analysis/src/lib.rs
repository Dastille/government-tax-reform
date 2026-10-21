//! Reproducible revenue model for the consumption-tax whitepaper.
//!
//! - [`data`] holds every sourced input.
//! - [`model`] holds the arithmetic.
//! - [`ledger`] states the status and evidence of every input.
//! - [`report`] renders the tables; [`docs`] renders the paper, README and claims ledger.
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

pub mod data;
pub mod docs;
pub mod error;
pub mod ledger;
pub mod model;
pub mod report;
pub mod transition;

pub use error::{Error, Result};
