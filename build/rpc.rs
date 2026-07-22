use core::panic;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use heck::ToSnakeCase;
use serde_json::Value;

#[derive(Debug, Clone)]
struct RpcEndpoint {
    pub operation_name: String,
    pub path: String,
    pub namespace: String,
    pub _is_private: bool,
    pub request_schema_name: String,
    pub response_schema_name: String,
    pub is_array_response: bool,
}

pub fn generate_rpc() {
    let rpc_output_path = "src/rpc/mod.rs";
    let api_spec_path = "schemas/openapi.json";

    let api_spec: Value = fs::read_to_string(api_spec_path)
        .map(|content| serde_json::from_str(&content).expect("Invalid JSON"))
        .unwrap_or_else(|_| panic!("Failed to read API spec from {api_spec_path}"));

    let endpoints = extract_endpoints_from_api_spec(&api_spec);

    let name_space_to_endpoints = endpoints.clone().into_iter().fold(
        std::collections::HashMap::<String, Vec<RpcEndpoint>>::new(),
        |mut acc, endpoint| {
            acc.entry(endpoint.namespace.clone())
                .or_insert_with(Vec::new)
                .push(endpoint);
            acc
        },
    );

    for (namespace, endpoints) in name_space_to_endpoints {
        // let generated_code = generate_rpc_client(&endpoints, &namespace);
        // let output_path = format!("src/rpc/{}.rs", namespace.to_snake_case());
        // fs::write(&output_path, generated_code)
        //     .unwrap_or_else(|_| panic!("Failed to write RPC client to {output_path}"));

        let generated_code = generate_rpc_namespace(&namespace, &endpoints);

        let syntax_tree = syn::parse_file(&generated_code.to_string())
            .expect("generated RPC namespace contained invalid Rust");
        let formatted_code = prettyplease::unparse(&syntax_tree);
        let output_path = format!("src/rpc/{}.rs", namespace.to_snake_case());
        fs::write(&output_path, &formatted_code)
            .unwrap_or_else(|_| panic!("Failed to write RPC namespace to {output_path}"));
    }

    let mod_file = generate_rpc_root(&endpoints);
    let syntax_tree =
        syn::parse_file(&mod_file.to_string()).expect("generated RPC root contained invalid Rust");
    let formatted_code = prettyplease::unparse(&syntax_tree);
    fs::write(rpc_output_path, &formatted_code)
        .unwrap_or_else(|_| panic!("Failed to write RPC root to {rpc_output_path}"));
}

fn extract_endpoints_from_api_spec(api_spec: &Value) -> Vec<RpcEndpoint> {
    let mut endpoints = Vec::new();

    let mut models_to_rename = HashMap::<&str, &str>::new();

    models_to_rename.insert("RFQPollResponse", "RfqPollResponse");
    models_to_rename.insert("RFQGetResponse", "RfqGetResponse");
    models_to_rename.insert("TickerSlimSnapshot", "TickerSlimSchema");

    let mut public_methods_to_rename = HashMap::<&str, &str>::new();

    public_methods_to_rename.insert("order_quote", "public_order_quote");

    let paths_to_skip = ["/public/withdraw_debug"];

    for (path, methods) in api_spec["paths"].as_object().unwrap() {
        if paths_to_skip.contains(&path.as_str()) {
            continue;
        }

        // we expect each endpoint to have a single method (POST), we ensure that here
        if methods.as_object().unwrap().len() != 1 {
            panic!("Expected a single method for path: {}", path);
        }

        // we extract the post method and its details
        let details = methods["post"].as_object().unwrap();
        // we assert there is only a single tag for the endpoint, which is the namespace
        let tags = details["tags"].as_array().unwrap();
        if tags.len() != 1 {
            panic!("Expected a single tag for path: {}", path);
        }
        let namespace = tags[0].as_str().unwrap().to_string();

        let path = details["summary"].as_str().unwrap().to_string();
        let mut operation_name = path.split("/").last().unwrap().to_string();
        let is_private = path.contains("private");

        let request_schema_name =
            details["requestBody"]["content"]["application/json"]["schema"]["$ref"]
                .as_str()
                .map(|s| s.split("/").last().unwrap());

        // let response_schema_name = details["responses"]["200"]["content"]["application/json"]["schema"]["$ref"].as_str().map(|s| {
        //     s.split("/").last().unwrap()
        // });

        // we check if it is an array type response, if so we extract the items schema name

        let (mut response_schema_name, is_array_response) = extract_response_schema(details)
            .unwrap_or_else(|| {
                panic!("Failed to extract response schema for path: {}", path);
            });

        if let Some(rename) = models_to_rename.get(response_schema_name.as_str()) {
            response_schema_name = rename.to_string();
        }

        // check if its both public and in the public_methods_to_rename map, if so we rename it
        if !is_private && let Some(rename) = public_methods_to_rename.get(operation_name.as_str()) {
            operation_name = rename.to_string();
        }

        let endpoint = RpcEndpoint {
            operation_name,
            path,
            namespace,
            _is_private: is_private,
            request_schema_name: request_schema_name.unwrap().to_string(),
            response_schema_name,
            is_array_response,
        };
        endpoints.push(endpoint);
    }

    endpoints
}

use serde_json::Map;

fn extract_response_schema(details: &Map<String, Value>) -> Option<(String, bool)> {
    let schema = details
        .get("responses")
        .and_then(|responses| responses.get("200"))
        .and_then(|response| response.get("content"))
        .and_then(|content| content.get("application/json"))
        .and_then(|content| content.get("schema"))?;

    let is_vec = schema.get("type").and_then(Value::as_str) == Some("array");

    let target_schema = if is_vec {
        schema
            .get("items")
            .unwrap_or_else(|| panic!("array response schema has no `items` "))
    } else {
        schema
    };

    let response_schema_name =
        if let Some(schema_ref) = target_schema.get("$ref").and_then(Value::as_str) {
            schema_ref
                .rsplit('/')
                .next()
                .expect("invalid schema $ref")
                .to_owned()
        } else if let Some(schema_type) = target_schema.get("type").and_then(Value::as_str) {
            schema_type.to_owned()
        } else {
            panic!("expected either `$ref` or `type` in response schema")
        };

    Some((response_schema_name, is_vec))
}

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn schema_type_tokens(schema_name: &str) -> TokenStream {
    match schema_name {
        "string" => quote!(String),
        "integer" => quote!(i64),
        "number" => quote!(f64),
        "boolean" => quote!(bool),
        "null" => quote!(()),

        model_name => {
            let model_ident = format_ident!("{}", model_name);
            quote!(#model_ident)
        }
    }
}

fn generate_rpc_method(endpoint: &RpcEndpoint) -> TokenStream {
    let method_ident = format_ident!("{}", endpoint.operation_name.to_snake_case());

    let request_type = schema_type_tokens(&endpoint.request_schema_name);
    let response_item_type = schema_type_tokens(&endpoint.response_schema_name);

    let response_type = if endpoint.is_array_response {
        quote!(Vec<#response_item_type>)
    } else {
        response_item_type
    };

    let path = &endpoint.path;

    quote! {
        pub async fn #method_ident(
            &self,
            params: #request_type,
        ) -> Result<#response_type, ClientError> {
            let params_json = serde_json::to_value(&params)?;

            self.ws_client
                .send_rpc(#path, params_json)
                .await
        }
    }
}

fn generate_rpc_namespace(namespace: &str, endpoints: &[RpcEndpoint]) -> TokenStream {
    println!("Generating RPC namespace for: {}", namespace);
    let namespace_ident = format_ident!("{}Namespace", namespace.to_upper_camel_case());

    let methods = endpoints.iter().map(generate_rpc_method);

    let extra_import = if namespace == "Market Data" {
        quote! {
            use crate::models::ticker_slim_schema::TickerSlimSchema;
        }
    } else {
        quote! {}
    };

    quote! {
        use crate::{
            models::openapi::*,
            types::ClientError,
            ws_client::WsClient,
        };
        #extra_import

        pub struct #namespace_ident<'a> {
            pub ws_client: &'a WsClient,
        }

        impl<'a> #namespace_ident<'a> {
            pub fn new(ws_client: &'a WsClient) -> Self {
                Self { ws_client }
            }

            #(#methods)*
        }
    }
}

fn namespace_module_name(namespace: &str) -> String {
    sanitise_namespace(namespace).to_snake_case()
}

fn namespace_type_name(namespace: &str) -> String {
    sanitise_namespace(namespace).to_upper_camel_case()
}

fn sanitise_namespace(namespace: &str) -> String {
    namespace
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                ' '
            }
        })
        .collect()
}

fn generate_rpc_root(endpoints: &[RpcEndpoint]) -> TokenStream {
    let mut namespaces = BTreeMap::<String, String>::new();

    for endpoint in endpoints {
        let module_name = namespace_module_name(&endpoint.namespace);
        let type_prefix = namespace_type_name(&endpoint.namespace);

        match namespaces.get(&module_name) {
            Some(existing_type) if existing_type != &type_prefix => {
                panic!(
                    "namespace collision: `{}` maps to an existing module `{module_name}`",
                    endpoint.namespace,
                );
            }

            Some(_) => {
                // This namespace was already found on another endpoint.
            }

            None => {
                namespaces.insert(module_name, type_prefix);
            }
        }
    }

    let modules = namespaces.keys().map(|module_name| {
        let module_ident = format_ident!("{module_name}");

        quote! {
            pub mod #module_ident;
        }
    });

    let imports = namespaces.iter().map(|(module_name, type_prefix)| {
        let module_ident = format_ident!("{module_name}");
        let type_ident = format_ident!("{type_prefix}Namespace");

        quote! {
            #module_ident::#type_ident
        }
    });

    let accessors = namespaces.iter().map(|(module_name, type_prefix)| {
        let method_ident = format_ident!("{module_name}");
        let type_ident = format_ident!("{type_prefix}Namespace");

        quote! {
            pub fn #method_ident(&self) -> #type_ident<'a> {
                #type_ident::new(self.client)
            }
        }
    });

    quote! {
        use crate::{
            rpc::{
                #(#imports),*
            },
            ws_client::WsClient,
        };

        #(#modules)*

        pub struct Rpc<'a> {
            pub client: &'a WsClient,
        }

        impl<'a> Rpc<'a> {
            pub fn new(client: &'a WsClient) -> Self {
                Self { client }
            }

            #(#accessors)*
        }
    }
}
