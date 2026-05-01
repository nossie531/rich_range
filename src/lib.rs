//! Range calculation helper.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

#![cfg_attr(feature = "doc_on", doc = doc::sub::examples::all!())]
#![cfg_attr(feature = "doc_on", doc = doc::sub::core_items::all!())]
#![cfg_attr(feature = "doc_on", doc = doc::sub::range_operations::all!())]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

doc_file!(doc, "README.md#");

pub mod conv;
pub mod norm;
pub mod parts;
pub mod prelude;
pub mod shorthands;
pub mod util;
pub mod values;
pub use inst::*;
pub use ranges::*;
pub use values::*;

mod calc;
mod inst;
mod msg;
mod ranges;
use rustdoc_copy::prelude::*;

#[cfg(doc)]
use core::ops::*;
