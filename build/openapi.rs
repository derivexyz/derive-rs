use std::{
    collections::{HashMap, hash_map::Entry as HashMapEntry},
    fs,
    path::Path,
};

use serde_json::{Map, Value, json, map::Entry as JsonMapEntry};
use typify::{CrateVers, TypeSpace, TypeSpaceSettings};

pub fn generate() {
    let paths = ["schemas/openapi.json"];

    let output_path = "src/models/openapi.rs";
    let models_to_skip = ["TickerSlimSnapshot", "PublicVaultActionResponse"];
    let models_to_rename = [("ERC20Details", "Erc20CompleteDetails")];

    let mut definitions: HashMap<String, Value> = HashMap::new();

    for input_path in paths {
        println!("cargo:rerun-if-changed={input_path}");
        let content = fs::read_to_string(input_path)
            .unwrap_or_else(|error| panic!("failed to read {input_path}: {error}"));
        let mut openapi: Value = serde_json::from_str(&content)
            .unwrap_or_else(|error| panic!("failed to parse {input_path}: {error}"));
        let schemas = openapi
            .pointer_mut("/components/schemas")
            .unwrap_or_else(|| panic!("{input_path} does not contain components.schemas"))
            .take();
        merge_definitions(&mut definitions, schemas, input_path, &models_to_skip);
    }

    let mut definitions = Value::Object(definitions.into_iter().collect());

    rename_model_definitions(&mut definitions, &models_to_rename);
    rewrite_openapi_refs(&mut definitions, &models_to_rename);
    annotate_decimal_types(&mut definitions);

    let root_schema = serde_json::from_value(json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": definitions,
    }))
    .expect("failed to convert definitions into a root schema");

    let mut settings = TypeSpaceSettings::default();

    settings
        .with_struct_builder(false)
        // .with_derive("Default".to_owned())
        .with_crate(
            "bigdecimal",
            CrateVers::parse("0.4.10").expect("invalid BigDecimal crate version"),
            None,
        );

    let mut type_space = TypeSpace::new(&settings);

    type_space
        .add_root_schema(root_schema)
        .expect("Typify failed to process the root schema");

    let syntax = syn::parse2::<syn::File>(type_space.to_stream())
        .expect("Typify produced invalid Rust syntax");

    let contents = prettyplease::unparse(&syntax);

    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent).unwrap_or_else(|error| {
            panic!(
                "failed to create output directory {}: {error}",
                parent.display(),
            )
        });
    }

    let header = "#![allow(clippy::derivable_impls)]";

    let contents = format!("{header}\n\n{contents}");

    println!("Output path: {output_path}");

    fs::write(output_path, contents)
        .unwrap_or_else(|error| panic!("failed to write {output_path}: {error}"));
}

fn merge_definitions(
    definitions: &mut HashMap<String, Value>,
    schemas: Value,
    source: &str,
    models_to_skip: &[&str],
) {
    let schemas = schemas
        .as_object()
        .expect("components.schemas must be an object");

    for (name, schema) in schemas {
        if models_to_skip.contains(&name.as_str()) {
            println!("Skipping model: {name}");
            continue;
        }

        match definitions.entry(name.clone()) {
            HashMapEntry::Vacant(entry) => {
                entry.insert(schema.clone());
            }

            HashMapEntry::Occupied(entry) => {
                if entry.get() != schema {
                    panic!(
                        "conflicting schema definition for {name:?} in \
                         {source}:\n\
                         existing definition:\n{}\n\
                         conflicting definition:\n{}",
                        serde_json::to_string_pretty(entry.get()).unwrap(),
                        serde_json::to_string_pretty(schema).unwrap(),
                    );
                }

                // The same name has already been inserted with an identical
                // definition, so there is nothing to do.
            }
        }
    }
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
            JsonMapEntry::Vacant(entry) => {
                println!("Renaming model: {old_name} -> {new_name}");

                entry.insert(schema);
            }

            JsonMapEntry::Occupied(entry) if entry.get() == &schema => {
                // The target name already exists with the same definition.
                // The old definition has already been removed.
            }

            JsonMapEntry::Occupied(entry) => {
                panic!(
                    "cannot rename schema {old_name:?} to \
                     {new_name:?}: target name already contains a \
                     different definition\n\
                     existing definition:\n{}\n\
                     incoming definition:\n{}",
                    serde_json::to_string_pretty(entry.get()).unwrap(),
                    serde_json::to_string_pretty(&schema).unwrap(),
                );
            }
        }
    }
}

fn rewrite_openapi_refs(value: &mut Value, models_to_rename: &[(&str, &str)]) {
    match value {
        Value::Object(object) => {
            if let Some(Value::String(reference)) = object.get_mut("$ref")
                && let Some(name) = reference.strip_prefix("#/components/schemas/")
            {
                let rewritten_name = models_to_rename
                    .iter()
                    .find_map(|(old_name, new_name)| (*old_name == name).then_some(*new_name))
                    .unwrap_or(name);

                *reference = format!("#/definitions/{rewritten_name}");
            }

            for child in object.values_mut() {
                rewrite_openapi_refs(child, models_to_rename);
            }
        }

        Value::Array(values) => {
            for child in values {
                rewrite_openapi_refs(child, models_to_rename);
            }
        }

        _ => {}
    }
}

fn bigdecimal_extension() -> Value {
    json!({
        "crate": "bigdecimal",
        "version": ">=0.4.0, <0.5.0",
        "path": "bigdecimal::BigDecimal"
    })
}

fn annotate_decimal_types(value: &mut Value) {
    match value {
        Value::Object(object) => {
            let is_decimal = object.get("format").and_then(Value::as_str) == Some("decimal");

            let is_string = object.get("type").and_then(Value::as_str) == Some("string");

            let is_nullable_string =
                object
                    .get("type")
                    .and_then(Value::as_array)
                    .is_some_and(|types| {
                        types.len() == 2
                            && types.iter().any(|value| value.as_str() == Some("string"))
                            && types.iter().any(|value| value.as_str() == Some("null"))
                    });

            if is_decimal && is_nullable_string {
                replace_nullable_decimal_schema(value);
                return;
            }

            if is_decimal && is_string {
                object.insert("x-rust-type".to_owned(), bigdecimal_extension());
            }

            for child in object.values_mut() {
                annotate_decimal_types(child);
            }
        }

        Value::Array(values) => {
            for child in values {
                annotate_decimal_types(child);
            }
        }

        _ => {}
    }
}

fn replace_nullable_decimal_schema(value: &mut Value) {
    let original = value
        .as_object()
        .expect("nullable decimal schema must be an object");

    let mut decimal_schema = original.clone();

    decimal_schema.insert("type".to_owned(), Value::String("string".to_owned()));

    decimal_schema.insert("x-rust-type".to_owned(), bigdecimal_extension());

    let mut replacement = Map::new();

    for metadata_key in [
        "title",
        "description",
        "default",
        "deprecated",
        "readOnly",
        "writeOnly",
        "examples",
    ] {
        if let Some(metadata) = original.get(metadata_key).cloned() {
            replacement.insert(metadata_key.to_owned(), metadata);
        }
    }

    replacement.insert(
        "anyOf".to_owned(),
        Value::Array(vec![
            Value::Object(decimal_schema),
            json!({
                "type": "null"
            }),
        ]),
    );

    *value = Value::Object(replacement);
}
