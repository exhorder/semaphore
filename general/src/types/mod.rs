//! Basic types for the meta protocol.
//!
//! This only provides the types (or aliases) that are used for the meta
//! part of the protocol.  This is the core annotation system as well as
//! the dynamic value parts and the metadata that goes with it.

mod annotated;
mod impls;
mod meta;
mod traits;
mod value;

pub use self::annotated::{Annotated, MetaMap, MetaTree, ValueAction};
pub use self::impls::SerializePayload;
pub use self::meta::{Error, ErrorKind, Meta, Range, Remark, RemarkType};
pub use self::traits::{Empty, FromValue, SkipSerialization, ToValue};
pub use self::value::{Array, Map, Object, Timestamp, Value, ValueDescription};
