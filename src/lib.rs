//! # cid
//!
//! Implementation of [cid](https://github.com/ipld/cid) in Rust.

#![deny(missing_docs)]

mod cid;
mod codec;
mod error;
mod version;

#[cfg(any(test, feature = "test"))]
mod arb;

pub use self::cid::{Cid, CidGeneric};
pub use self::codec::Codec;
pub use self::error::{Error, Result};
pub use self::version::Version;

#[cfg(feature = "impl-serde")]
mod serde;

#[cfg(feature = "impl-serde")]
pub use self::serde::{
    ipld_dag_cbor::{self, IPLD_DAG_CBOR_TAG_CID, RAW_BINARY_MULTIBASE_IDENTITY},
    ipld_dag_json,
};
