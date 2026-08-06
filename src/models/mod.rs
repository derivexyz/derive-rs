#[allow(clippy::module_inception)]
mod models;
pub use models::*;

// Keep the custom ticker schema implementation
pub mod ticker_slim_schema;

// Legacy modules - can be removed once all references are updated
// pub mod openapi;
// pub mod asyncapi_rpc;
// pub mod asyncapi_subs;
