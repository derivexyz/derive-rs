#[path = "build/utils.rs"]
mod utils;

#[path = "build/models.rs"]
mod models;

// #[path = "build/openapi.rs"]
// mod openapi;

// #[path = "build/asyncapi_rpc.rs"]
// mod asyncapi_rpc;

// #[path = "build/asyncapi_subs.rs"]
// mod asyncapi_subs;

#[path = "build/subscriptions.rs"]
mod subscriptions;

#[path = "build/rpc.rs"]
mod rpc;

fn main() {
    // Generate unified models first - this deduplicates all model definitions
    models::generate();

    // These can be removed later if they only generated models
    // For now, keeping them in case they have other generation logic
    // openapi::generate();
    // asyncapi_rpc::generate();
    // asyncapi_subs::generate();

    subscriptions::generate_subscriptions();
    rpc::generate_rpc();
}
