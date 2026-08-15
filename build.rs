#[path = "build/utils.rs"]
mod utils;

#[path = "build/models.rs"]
mod models;

#[path = "build/subscriptions.rs"]
mod subscriptions;

#[path = "build/rpc.rs"]
mod rpc;

fn main() {
    println!("cargo::rerun-if-changed=schemas/openapi.json");
    println!("cargo::rerun-if-changed=schemas/ws_asyncapi_rpc.json");
    println!("cargo::rerun-if-changed=schemas/ws_asyncapi_subscriptions.json");
    models::generate();
    subscriptions::generate_subscriptions();
    rpc::generate_rpc();
}
