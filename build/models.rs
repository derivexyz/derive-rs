use std::{collections::HashMap, fs, path::Path};

use serde_json::{Value, json};
use typify::{CrateVers, TypeSpace, TypeSpaceImpl, TypeSpaceSettings};

use crate::utils::{annotate_decimal_types, merge_definitions, normalize_schema, write_if_changed};

struct FieldDecimalPatch {
    model_name: &'static str,
    fields: &'static [&'static str],
}

pub fn generate() {
    println!("cargo:warning=Generating unified models from all specs");

    let output_path = "src/models/models.rs";

    // Models that should be skipped during extraction (e.g., custom implementations)
    // Note: We don't skip TickerSlimSnapshot here because typify needs the schema
    // to exist for references to work. Instead, we use .with_replacement() below.
    let models_to_skip: [&str; 0] = [];

    // Models that should be renamed for consistency
    let models_to_rename = [("ERC20Details", "Erc20CompleteDetails")];
    // let models_to_patch_to_decimal = [
    //     ("Instrument", ["tick_size"])
    // ];

    let models_to_patch_to_decimal: Vec<FieldDecimalPatch> = vec![FieldDecimalPatch {
        model_name: "Instrument",
        fields: &["tick_size", "minimum_amount", "maximum_amount"],
    }];

    let mut all_definitions: HashMap<String, Value> = HashMap::new();

    // 1. Extract definitions from OpenAPI spec
    extract_openapi_definitions(&mut all_definitions, &models_to_skip);

    patch_openapi_definitions(&mut all_definitions);

    // 2. Extract definitions from AsyncAPI RPC spec
    extract_asyncapi_definitions(
        &mut all_definitions,
        "schemas/ws_asyncapi_rpc.json",
        "AsyncAPI RPC",
        &models_to_skip,
    );

    // 3. Extract definitions from AsyncAPI Subscriptions spec
    extract_asyncapi_definitions(
        &mut all_definitions,
        "schemas/ws_asyncapi_subscriptions.json",
        "AsyncAPI Subscriptions",
        &models_to_skip,
    );

    println!(
        "cargo:warning=Total unique models: {}",
        all_definitions.len()
    );

    // Convert to Value for transformations
    let mut definitions = Value::Object(all_definitions.into_iter().collect());

    // Apply model renames
    rename_model_definitions(&mut definitions, &models_to_rename);

    // Rewrite all refs to point to renamed models
    rewrite_refs(&mut definitions, &models_to_rename);

    // Patch specific models to add big decimal format annotations for fields that are expected to be BigDecimal in Rust
    patch_specific_models(&mut definitions, &models_to_patch_to_decimal);

    // Annotate decimal types for BigDecimal
    annotate_decimal_types(&mut definitions);

    // Create root schema
    let root_schema = serde_json::from_value(json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": definitions,
    }))
    .expect("failed to convert definitions into a root schema");

    // Configure TypeSpace with all settings
    let mut settings = TypeSpaceSettings::default();

    settings
        .with_struct_builder(true)
        .with_crate(
            "bigdecimal",
            CrateVers::parse("0.4.10").expect("invalid BigDecimal crate version"),
            None,
        )
        .with_replacement(
            "TickerSlimSnapshot",
            "crate::models::ticker_slim_schema::TickerSlimSchema",
            std::iter::empty::<TypeSpaceImpl>(),
        );

    let mut type_space = TypeSpace::new(&settings);

    type_space
        .add_root_schema(root_schema)
        .expect("Typify failed to process the root schema");

    let syntax = syn::parse2::<syn::File>(type_space.to_stream())
        .expect("Typify produced invalid Rust syntax");

    let contents = prettyplease::unparse(&syntax);

    let headers = [
        "// This file is generated from multiple API specs.",
        "// Do not edit manually.",
        "",
        "#![allow(clippy::derivable_impls)]",
        "#![allow(clippy::type_complexity)]",
        "#![allow(clippy::should_implement_trait)]",
    ];

    let header = headers.join("\n");
    let output = format!("{header}\n\n{contents}");

    write_if_changed(Path::new(output_path), &output);

    println!("cargo:warning=Successfully generated unified models at {output_path}");
}

fn extract_openapi_definitions(
    all_definitions: &mut HashMap<String, Value>,
    models_to_skip: &[&str],
) {
    let path = "schemas/openapi.json";
    println!("cargo:rerun-if-changed={path}");

    let content =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));

    let mut openapi: Value = serde_json::from_str(&content)
        .unwrap_or_else(|error| panic!("failed to parse {path}: {error}"));

    let schemas = openapi
        .pointer_mut("/components/schemas")
        .unwrap_or_else(|| panic!("{path} does not contain components.schemas"))
        .take();

    // Normalize all schemas before merging
    let mut schemas_obj = schemas
        .as_object()
        .expect("schemas must be an object")
        .clone();
    for schema in schemas_obj.values_mut() {
        normalize_schema(schema);
    }

    merge_definitions(
        all_definitions,
        Value::Object(schemas_obj),
        "OpenAPI",
        models_to_skip,
    );

    println!(
        "cargo:warning=Extracted {} definitions from OpenAPI",
        all_definitions.len()
    );
}

fn extract_asyncapi_definitions(
    all_definitions: &mut HashMap<String, Value>,
    source_path: &str,
    source_name: &str,
    models_to_skip: &[&str],
) {
    println!("cargo:rerun-if-changed={source_path}");

    let source = fs::read_to_string(source_path)
        .unwrap_or_else(|error| panic!("Unable to read {source_path}: {error}"));

    let document: Value = serde_json::from_str(&source)
        .unwrap_or_else(|error| panic!("Unable to parse {source_path}: {error}"));

    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .expect("Unable to extract schemas from AsyncAPI document");

    let messages = document
        .pointer("/components/messages")
        .and_then(Value::as_object)
        .expect("Unable to extract messages from AsyncAPI document");

    // First normalize and merge schemas
    let mut schemas_obj = schemas.clone();
    for schema in schemas_obj.values_mut() {
        normalize_schema(schema);
    }

    let schemas_value = Value::Object(schemas_obj);
    let count_before = all_definitions.len();
    merge_definitions(all_definitions, schemas_value, source_name, models_to_skip);

    // Then extract and merge message payloads
    for (message_name, message) in messages {
        if models_to_skip.contains(&message_name.as_str()) {
            println!("Skipping message: {message_name}");
            continue;
        }

        let mut payload = message
            .get("payload")
            .cloned()
            .unwrap_or_else(|| panic!("message `{message_name}` in {source_name} has no payload"));

        // Normalize the payload schema
        normalize_schema(&mut payload);

        // Check for conflicts
        if let Some(existing) = all_definitions.get(message_name) {
            if existing != &payload {
                panic!(
                    "conflicting schema definition for message {message_name:?} in {source_name}:\n\
                     existing definition:\n{}\n\
                     conflicting definition:\n{}",
                    serde_json::to_string_pretty(existing).unwrap(),
                    serde_json::to_string_pretty(&payload).unwrap(),
                );
            }
        } else {
            all_definitions.insert(message_name.clone(), payload);
        }
    }

    let count_after = all_definitions.len();
    println!(
        "cargo:warning=Extracted {} new definitions from {source_name} ({} schemas, {} messages)",
        count_after - count_before,
        schemas.len(),
        messages.len()
    );
}

fn rename_model_definitions(schemas: &mut Value, models_to_rename: &[(&str, &str)]) {
    let definitions = schemas
        .as_object_mut()
        .expect("definitions must be an object");

    for &(old_name, new_name) in models_to_rename {
        if old_name == new_name {
            continue;
        }

        let Some(schema) = definitions.remove(old_name) else {
            continue;
        };

        match definitions.entry(new_name.to_owned()) {
            serde_json::map::Entry::Vacant(entry) => {
                println!("cargo:warning=Renaming model: {old_name} -> {new_name}");
                entry.insert(schema);
            }

            serde_json::map::Entry::Occupied(entry) if entry.get() == &schema => {
                // The target name already exists with the same definition.
                // The old definition has already been removed.
            }

            serde_json::map::Entry::Occupied(entry) => {
                panic!(
                    "cannot rename schema {old_name:?} to {new_name:?}: \
                     target name already contains a different definition\n\
                     existing definition:\n{}\n\
                     incoming definition:\n{}",
                    serde_json::to_string_pretty(entry.get()).unwrap(),
                    serde_json::to_string_pretty(&schema).unwrap(),
                );
            }
        }
    }
}

fn rewrite_refs(value: &mut Value, models_to_rename: &[(&str, &str)]) {
    match value {
        Value::Object(object) => {
            // Handle both OpenAPI and JSON Schema refs
            if let Some(Value::String(reference)) = object.get_mut("$ref") {
                // Handle OpenAPI component refs
                if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                    let rewritten_name = models_to_rename
                        .iter()
                        .find_map(|(old_name, new_name)| (*old_name == name).then_some(*new_name))
                        .unwrap_or(name);

                    *reference = format!("#/definitions/{rewritten_name}");
                }
                // Handle already-converted definition refs (update renames)
                else if let Some(name) = reference.strip_prefix("#/definitions/") {
                    let rewritten_name = models_to_rename
                        .iter()
                        .find_map(|(old_name, new_name)| (*old_name == name).then_some(*new_name))
                        .unwrap_or(name);

                    *reference = format!("#/definitions/{rewritten_name}");
                }
            }

            for child in object.values_mut() {
                rewrite_refs(child, models_to_rename);
            }
        }

        Value::Array(values) => {
            for child in values {
                rewrite_refs(child, models_to_rename);
            }
        }

        _ => {}
    }
}

fn patch_openapi_definitions(all_definitions: &mut HashMap<String, Value>) {
    // Patch the OpenAPI definitions to remove the "mm_credits" property from the Subaccount schema
    if let Some(subaccount_schema) = all_definitions.get_mut("Subaccount")
        && let Some(required) = subaccount_schema.get_mut("required")
        && let Some(required_array) = required.as_array_mut()
    {
        required_array.retain(|item| item != "mm_credits");
    }
}

// add format: decimal to specific fields in specific models
fn patch_specific_models(
    all_definitions: &mut serde_json::Value,
    models_to_patch: &[FieldDecimalPatch],
) {
    for patch in models_to_patch.iter() {
        if let Some(model_schema) = all_definitions.get_mut(patch.model_name)
            && let Some(properties) = model_schema.get_mut("properties")
            && let Some(properties_obj) = properties.as_object_mut()
        {
            for field in patch.fields {
                if let Some(field_schema) = properties_obj.get_mut(*field)
                    && let Some(field_obj) = field_schema.as_object_mut()
                {
                    field_obj.insert("format".to_string(), Value::String("decimal".to_string()));
                }
            }
        }
    }
}
