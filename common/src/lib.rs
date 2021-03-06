//! Common functionality for the sentry relay.
#![warn(missing_docs)]

#[macro_use]
mod macros;

#[macro_use]
pub mod metrics;

mod auth;
mod config;
mod retry;
mod types;
mod upstream;
mod utils;

pub use crate::auth::*;
pub use crate::config::*;
pub use crate::retry::*;
pub use crate::types::*;
pub use crate::upstream::*;
pub use crate::utils::*;

pub use sentry_types::{
    Auth, AuthParseError, Dsn, DsnParseError, ProjectId, ProjectIdParseError, Scheme, Uuid,
};
