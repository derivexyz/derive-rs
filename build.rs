#[path = "build/openapi.rs"]
mod openapi;

#[path = "build/asyncapi_rpc.rs"]
mod asyncapi_rpc;

#[path = "build/asyncapi_subs.rs"]
mod asyncapi_subs;

#[path = "build/subscriptions.rs"]
mod subscriptions;

#[path = "build/rpc.rs"]
mod rpc;

fn main() {
    openapi::generate();
    asyncapi_rpc::generate();
    asyncapi_subs::generate();

    subscriptions::generate_subscriptions();
    rpc::generate_rpc();
}
