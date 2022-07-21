pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// re-exports
pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use domain::time::Time;
pub use data::DataError;
pub use service::ServiceError;