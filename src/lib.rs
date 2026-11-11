#![allow(clippy::module_inception)]

pub use common::*;
pub use expression::*;
pub use statement::*;

mod common;
mod expression;
mod statement;

#[cfg(feature = "c")]
pub mod c;
#[cfg(feature = "cpp")]
pub mod cpp;
#[cfg(feature = "csharp")]
pub mod csharp;
#[cfg(feature = "java")]
pub mod java;
#[cfg(feature = "kotlin")]
pub mod kotlin;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "rust")]
pub mod rust;
#[cfg(feature = "typescript")]
pub mod typescript;
