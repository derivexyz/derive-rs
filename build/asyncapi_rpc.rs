use std::{fs, path::Path, println};

use schemars::schema::RootSchema;
use serde_json::{Map, Value};
use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate() {
    let source_path = "schemas/ws_asyncapi_rpc.json";
    let output_path = "src/models/asyncapi_rpc.rs";

    println!("cargo:rerun-if-changed={source_path}");

    let source = fs::read_to_string(source_path).expect("Unable to read path.");

    let document: Value = serde_json::from_str(&source).expect("Unable to parse src.");

    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .expect("Unable to extract schemas.");

    let messages = document
        .pointer("/components/messages")
        .and_then(Value::as_object)
        .expect("Unable to extract messages.");

    let mut definitions: Map<String, Value> = schemas.clone();

    // for schema in definitions.values_mut() {
    //     rewrite_component_refs(schema);
    // }

    for (message_name, message) in messages {
        let payload = message
            .get("payload")
            .cloned()
            .unwrap_or_else(|| panic!("message `{message_name}` has no payload"));

        definitions.insert(message_name.clone(), payload);
    }

    for schema in definitions.values_mut() {
        normalize_schema(schema);
    }
    let root_schema_value = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": definitions,
    });

    let root_schema: RootSchema = serde_json::from_value(root_schema_value)
        .expect("failed to convert AsyncAPI schemas into a JSON Schema root");

    let settings = TypeSpaceSettings::default();
    let mut type_space = TypeSpace::new(&settings);

    // we print out the schemas.

    println!("definitions: {:#?}", root_schema.definitions);

    type_space
        .add_root_schema(root_schema)
        .expect("Typify failed to generate types from the AsyncAPI schemas");

    let syntax_tree: syn::File =
        syn::parse2(type_space.to_stream()).expect("Typify generated invalid Rust syntax");

    let generated = prettyplease::unparse(&syntax_tree);

    let output = format!(
        "// This file is generated from ws_asyncapi_rpc.json.\n\
         // Do not edit manually.\n\n\
         #![allow(clippy::derivable_impls)]
         {generated}"
    );

    write_if_changed(Path::new(&output_path), &output);
}

fn write_if_changed(path: &Path, contents: &str) {
    if fs::read_to_string(path).ok().as_deref() == Some(contents) {
        return;
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|error| {
            panic!(
                "failed to create output directory {}: {error}",
                parent.display()
            )
        });
    }

    fs::write(path, contents).unwrap_or_else(|error| {
        panic!(
            "failed to write generated Rust types to {}: {error}",
            path.display()
        )
    });
}

fn normalize_schema(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if let Some(Value::String(reference)) = object.get_mut("$ref")
                && let Some(schema_name) = reference.strip_prefix("#/components/schemas/")
            {
                *reference = format!("#/definitions/{schema_name}");
            }

            name_union_variants(object);

            // Normalise nested schemas first.
            for nested in object.values_mut() {
                normalize_schema(nested);
            }

            // `nullable` is an OpenAPI keyword, not a Draft-07 keyword.
            let nullable = matches!(object.remove("nullable"), Some(Value::Bool(true)));

            if nullable {
                make_nullable(object);
            }
        }

        Value::Array(values) => {
            for nested in values {
                normalize_schema(nested);
            }
        }

        _ => {}
    }
}

fn name_union_variants(object: &mut Map<String, Value>) {
    let union_key = if object.contains_key("oneOf") {
        "oneOf"
    } else if object.contains_key("anyOf") {
        "anyOf"
    } else {
        return;
    };

    let Some(variants) = object.get_mut(union_key).and_then(Value::as_array_mut) else {
        return;
    };

    for variant in variants {
        let Some(variant_object) = variant.as_object_mut() else {
            continue;
        };

        if variant_object.contains_key("title") {
            continue;
        }

        let Some(properties) = variant_object.get("properties").and_then(Value::as_object) else {
            continue;
        };

        let title = if properties.contains_key("result") {
            Some("Success")
        } else if properties.contains_key("error") {
            Some("Error")
        } else {
            None
        };

        if let Some(title) = title {
            variant_object.insert("title".to_owned(), Value::String(title.to_owned()));
        }
    }
}

fn make_nullable(schema: &mut Map<String, Value>) {
    match schema.get_mut("type") {
        Some(Value::String(schema_type)) => {
            if schema_type != "null" {
                let schema_type = std::mem::take(schema_type);

                schema.insert(
                    "type".to_owned(),
                    Value::Array(vec![
                        Value::String(schema_type),
                        Value::String("null".to_owned()),
                    ]),
                );
            }
        }

        Some(Value::Array(types)) => {
            let contains_null = types.iter().any(|value| value.as_str() == Some("null"));

            if !contains_null {
                types.push(Value::String("null".to_owned()));
            }
        }

        // Nullable `$ref`, `oneOf`, `allOf`, etc.
        None => {
            // Defaults and documentation belong to the outer nullable schema.
            let default = schema.remove("default");
            let title = schema.remove("title");
            let description = schema.remove("description");

            let non_null_schema = Value::Object(std::mem::take(schema));

            schema.insert(
                "anyOf".to_owned(),
                Value::Array(vec![
                    non_null_schema,
                    serde_json::json!({
                        "type": "null"
                    }),
                ]),
            );

            if let Some(default) = default {
                schema.insert("default".to_owned(), default);
            }

            if let Some(title) = title {
                schema.insert("title".to_owned(), title);
            }

            if let Some(description) = description {
                schema.insert("description".to_owned(), description);
            }
        }

        Some(_) => {
            // Invalid or unsupported `type` representation.
        }
    }
}
