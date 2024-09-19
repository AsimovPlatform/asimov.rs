// This is free and unencumbered software released into the public domain.

#![no_std]
#![deny(unsafe_code)]

extern crate alloc;
extern crate core;

#[cfg(feature = "std")]
extern crate std;

pub mod block {
    mod definition;
    pub use definition::*;
}

pub mod error;
pub use error::*;

pub mod flow {
    pub use ::protoflow_core::*;

    mod execution_state;
    pub use execution_state::*;
}

pub mod model {
    mod manifest;
    pub use manifest::*;
}

pub mod module {
    mod registration;
    pub use registration::*;
}

pub mod system {}

pub use ::dogma::traits::{Labeled, Named};
pub use ::dogma::traits::{MaybeLabeled, MaybeNamed};
