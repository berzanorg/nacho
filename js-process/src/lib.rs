mod error;
mod js_process;

pub use error::JsProcessError;
pub use js_process::{interact, spawn, wait};
