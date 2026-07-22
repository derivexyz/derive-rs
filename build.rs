#[path = "build/openapi.rs"]
mod openapi;

#[path = "build/asyncapi_rpc.rs"]
mod asyncapi_rpc;

#[path = "build/asyncapi_subs.rs"]
mod asyncapi_subs;

fn main() {
    openapi::generate();
    asyncapi_rpc::generate();
    asyncapi_subs::generate();
}
