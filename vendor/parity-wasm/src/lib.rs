//! WebAssembly format library
//#![warn(missing_docs)]
#![cfg_attr(not(feature = "mesalock_sgx"), warn(missing_docs))]

#![cfg_attr(any(not(feature = "std"),
                all(feature = "mesalock_sgx", not(target_env = "sgx"))), no_std)]

// warning: the feature `alloc` has been stable since 1.36.0 and no longer
// requires an attribute to enable
#![cfg_attr(any(not(feature = "std"),
                all(feature = "mesalock_sgx", target_env = "sgx")),
            feature(alloc))]

#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate byteorder;

#[cfg(not(feature = "std"))]
#[macro_use]

#[cfg(not(feature="std"))] #[macro_use]
extern crate alloc;

pub mod elements;
pub mod builder;
#[cfg(not(feature = "mesalock_sgx"))]
mod io;
#[cfg(feature = "mesalock_sgx")]
pub mod io;

pub use elements::{
	Error as SerializationError,
	deserialize_buffer,
	serialize,
	peek_size,
};

#[cfg(feature = "std")]
pub use elements::{
	deserialize_file,
	serialize_to_file,
};

#[cfg(not(feature = "std"))]
pub (crate) mod rust {
	pub use core::*;
	pub use ::alloc::format;
	pub use ::alloc::vec;
	pub use ::alloc::string;
	pub use ::alloc::boxed;
	pub use ::alloc::borrow;
}

#[cfg(feature="std")]
pub (crate) mod rust {
	pub use std::*;
}
