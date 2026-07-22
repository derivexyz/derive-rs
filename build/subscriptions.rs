use core::panic;
use std::{
    collections::{BTreeMap, HashMap},
    fs, println,
};

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;

#[derive(Debug)]
struct ChannelDefinition {
    name: String,
    namespace: String,
    channel: String,
    is_private: bool,
    params: Vec<String>,
    notification_model: String,
}

fn channel_to_ns(channel: &str) -> String {
    match channel {
        "auctionsWatch" => "liquidations",
        "marginWatch" => "liquidations",
        "orderbook" => "market_data",
        "spotFeed" => "market_data",
        "subaccountBalances" => "accounting",
        "subaccountBestQuotes" => "rfqs",
        "subaccountOrders" => "trading",
        "subaccountQuotes" => "rfqs",
        "subaccountTrades" => "trading",
        "subaccountTradesTxStatus" => "trading",
        "tickerSlim" => "market_data",
        "tradesByInstrument" => "market_data",
        "tradesByInstrumentTypeCurrency" => "market_data",
        "tradesByInstrumentTypeCurrencyTxStatus" => "market_data",
        "walletRfqs" => "rfqs",
        _ => panic!("Unknown channel: {channel}"),
    }
    .to_string()
}

fn extract_channel_string_from_channel_spec(channel_spec: &Value) -> String {
    let channel = channel_spec
        .get("description")
        .expect("channel_spec must have a channel field")
        .as_str()
        .expect("channel field must be a string");
    let re = regex::Regex::new(r"`([^`]*)`").unwrap();
    let captures = re
        .captures(channel)
        .expect("channel field must have a string between ` and `");
    let channel = captures
        .get(1)
        .expect("channel field must have a string between ` and `")
        .as_str();
    channel.to_string()
}

fn is_private_from_channel_spec(channel_spec: &Value) -> bool {
    channel_spec
        .get("description")
        .expect("channel_spec must have a description field")
        .as_str()
        .expect("description field must be a string")
        .contains("(requires authentication)")
}

fn extract_params_from_channel_string(channel_string: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{([^}]*)\}").unwrap();
    let captures = re.captures_iter(channel_string);
    let mut params = Vec::new();
    for capture in captures {
        let param = capture
            .get(1)
            .expect("channel string must have a string between { and }")
            .as_str();
        params.push(param.to_string());
    }
    params
}

fn extract_notification_model_name_from_channel_spec(channel_spec: &Value) -> String {
    let messages = channel_spec
        .get("messages")
        .expect("channel_spec must have a messages field");
    // we verify there are only 2 messages, one for the request and one for the notification
    if messages.as_object().unwrap().len() != 3 {
        panic!(
            "channel_spec must have exactly 3 messages, one for the request and one for the notification"
        );
    }
    let notification_message = messages
        .as_object()
        .unwrap()
        .iter()
        .find(|(key, _)| key.contains("Notification"))
        .expect("channel_spec must have a notification message");
    let notification_message_name = notification_message.0;
    notification_message_name.to_string()
}

pub fn generate_subscriptions() {
    let path = "schemas/ws_asyncapi_subscriptions.json";

    // we read the asyncapi_subs.json file and parse it into a serde_json::Value
    println!("cargo:rerun-if-changed={path}");

    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    let asyncapi_subs: Value = serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("failed to parse {path}: {error}"));

    let mut ns_to_channel_definitions: HashMap<String, Vec<ChannelDefinition>> = HashMap::new();

    for key in asyncapi_subs
        .pointer("/channels")
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
    {
        let ns = channel_to_ns(key);
        let channel = asyncapi_subs.pointer(&format!("/channels/{key}")).unwrap();
        let channel_str = extract_channel_string_from_channel_spec(channel);
        let is_private = is_private_from_channel_spec(channel);
        let params = extract_params_from_channel_string(&channel_str);
        let notification_model_name = extract_notification_model_name_from_channel_spec(channel);
        let channel_definition = ChannelDefinition {
            name: key.to_string(),
            namespace: ns.clone(),
            channel: channel_str.clone(),
            is_private,
            params,
            notification_model: notification_model_name.clone(),
        };
        ns_to_channel_definitions
            .entry(ns.clone())
            .or_default()
            .push(channel_definition);
    }

    // collect all channel definitions into a single vector
    let mut all_channel_definitions: Vec<ChannelDefinition> = Vec::new();
    for (namespace, channel_definitions) in ns_to_channel_definitions {
        let generated = generate_subscription_client(&channel_definitions, &namespace);
        let subscriptions_output_path =
            format!("src/subscriptions/{}.rs", namespace.to_snake_case());

        println!("{}", &generated.to_string());
        let syntax_tree = syn::parse_file(&generated.to_string())
            .expect("generated subscription clients contained invalid Rust");

        let formatted = prettyplease::unparse(&syntax_tree);

        fs::write(&subscriptions_output_path, formatted)
            .unwrap_or_else(|error| panic!("failed to write {subscriptions_output_path}: {error}"));

        all_channel_definitions.extend(channel_definitions);
        // println!("Generated subscription clients for namespace: {namespace}");
    }

    let generated = generate_channel_specs(&all_channel_definitions);
    let channel_spec_output_path = "src/subscriptions/channel_specs.rs";
    let syntax_tree = syn::parse_file(&generated.to_string())
        .expect("generated channel specs contained invalid Rust");
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(channel_spec_output_path, formatted)
        .expect("failed to write generated channel specs");
}

fn generate_channel_specs(definitions: &[ChannelDefinition]) -> TokenStream {
    let mut namespaces: BTreeMap<&str, Vec<&ChannelDefinition>> = BTreeMap::new();

    for definition in definitions {
        namespaces
            .entry(&definition.namespace)
            .or_default()
            .push(definition);
    }

    let namespace_modules = namespaces.into_iter().map(|(namespace, definitions)| {
        let namespace_ident = format_ident!("{namespace}");

        let channel_specs = definitions.into_iter().map(generate_channel_spec);

        quote! {
            pub mod #namespace_ident {
                use super::*;

                #(#channel_specs)*
            }
        }
    });

    quote! {
        // Adjust these imports to match your crate.
        use crate::models::asyncapi_subs::*;
        use crate::types::{ChannelSpec, RequestScope};

        #(#namespace_modules)*
    }
}

fn generate_channel_spec(definition: &ChannelDefinition) -> TokenStream {
    let struct_name = format!("{}ChannelSpec", upper_first(&definition.name),);

    let output_name = upper_first(&definition.notification_model);

    let struct_ident = format_ident!("{struct_name}");
    let output_ident = format_ident!("{output_name}");

    let field_idents: Vec<_> = definition
        .params
        .iter()
        .map(|param| format_ident!("{param}"))
        .collect();

    let channel_literal = Literal::string(&definition.channel);

    let scope = if definition.is_private {
        quote!(RequestScope::Private)
    } else {
        quote!(RequestScope::Public)
    };

    /*
     * Generates named format arguments such as:
     *
     * instrument_name = self.instrument_name.as_str()
     * interval = self.interval.as_str()
     *
     * These correspond directly to placeholders such as:
     *
     * "ticker_slim.{instrument_name}.{interval}"
     */
    let format_arguments = field_idents.iter().map(|field| {
        quote! {
            #field = self.#field.as_str()
        }
    });

    let constructor_arguments = field_idents.iter().map(|field| {
        quote! {
            #field: impl Into<String>
        }
    });

    let constructor_fields = field_idents.iter().map(|field| {
        quote! {
            #field: #field.into()
        }
    });

    quote! {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        pub struct #struct_ident {
            #(pub #field_idents: String,)*
        }

        impl #struct_ident {
            pub fn new(#(#constructor_arguments),*) -> Self {
                Self {
                    #(#constructor_fields,)*
                }
            }
        }

        impl ChannelSpec for #struct_ident {
            type Output = #output_ident;

            fn scope(&self) -> RequestScope {
                #scope
            }

            #[allow(clippy::needless_return, clippy::useless_format)]
            fn channel(&self) -> String {
                return format!(
                    #channel_literal,
                    #(#format_arguments),*
                );
            }
        }
    }
}

fn upper_first(value: &str) -> String {
    let mut chars = value.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn generate_subscription_client(definitions: &[ChannelDefinition], namespace: &str) -> TokenStream {
    let namespace_subscriptions = generate_namespace_subscriptions(namespace, definitions);
    let namespace_ident = format_ident!("{}", namespace.to_snake_case());

    quote! {
        use crate::{
            models::asyncapi_subs::*,
            subscriptions::channel_specs::#namespace_ident::*,
            types::{ClientError, EventStream},
            ws_client::WsClient,
        };

        #namespace_subscriptions
    }
}

fn generate_namespace_subscriptions(
    namespace: &str,
    definitions: &[ChannelDefinition],
) -> TokenStream {
    let subscriptions_ident = format_ident!("{}Subscriptions", namespace.to_upper_camel_case());

    let methods = definitions.iter().map(generate_subscription_method);

    quote! {
        pub struct #subscriptions_ident<'a> {
            client: &'a WsClient,
        }

        impl<'a> #subscriptions_ident<'a> {
            pub fn new(client: &'a WsClient) -> Self {
                Self { client }
            }

            #(#methods)*
        }
    }
}

fn generate_subscription_method(definition: &ChannelDefinition) -> TokenStream {
    let method_ident = format_ident!("{}", definition.name.to_snake_case());

    let spec_ident = format_ident!("{}ChannelSpec", definition.name.to_upper_camel_case());

    let output_ident = format_ident!("{}", definition.notification_model.to_upper_camel_case());

    let parameter_idents: Vec<_> = definition
        .params
        .iter()
        .map(|param| format_ident!("{}", param.to_snake_case()))
        .collect();

    quote! {
        pub async fn #method_ident(
            &self,
            #(#parameter_idents: &str),*
        ) -> Result<EventStream<#output_ident>, ClientError> {
            self.client
                .subscribe(#spec_ident {
                    #(
                        #parameter_idents:
                            #parameter_idents.to_owned()
                    ),*
                })
                .await
        }
    }
}
