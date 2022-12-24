#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::unused_self,
    clippy::needless_pass_by_value // For all the usage of ToString
)]

#[cfg(feature = "axum")]
mod axum;
mod hal;
mod response;
mod utils;
pub mod values;

pub use hal::*;
pub use response::*;
