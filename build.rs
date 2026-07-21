#[path = "build/openapi.rs"]
mod openapi;

#[path = "build/asyncapi_rpc.rs"]
mod asyncapi_rpc;

fn main() {
    openapi::generate();
    asyncapi_rpc::generate();
}
