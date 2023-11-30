#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::unused_self,
    clippy::needless_pass_by_value, // For all the usage of ToString
    clippy::ignored_unit_patterns
)]

#[cfg(feature = "axum")]
mod axum;
mod hal;
mod response;
mod utils;
pub mod values;

pub use hal::*;
pub use response::*;
