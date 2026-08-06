use std::{
    collections::{HashMap, hash_map::Entry as HashMapEntry},
    fs,
    path::Path,
};

use serde_json::{Map, Value, json};

/// Write file only if contents have changed
pub fn write_if_changed(path: &Path, contents: &str) {
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

/// Merge schema definitions from a source into the main definitions map
/// Panics if there are conflicting definitions for the same name
pub fn merge_definitions(
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

/// Normalize schema references and handle nullable types
pub fn normalize_schema(value: &mut Value) {
    match value {
        Value::Object(object) => {
            // Normalize numeric values FIRST (before processing nested schemas)
            normalize_numeric_values(object);

            // Rewrite component refs to definition refs
            if let Some(Value::String(reference)) = object.get_mut("$ref")
                && let Some(schema_name) = reference.strip_prefix("#/components/schemas/")
            {
                *reference = format!("#/definitions/{schema_name}");
            }

            name_union_variants(object);

            // Normalize nested schemas
            for nested in object.values_mut() {
                normalize_schema(nested);
            }

            // `nullable` is an OpenAPI keyword, not a Draft-07 keyword
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

/// Normalize numeric values to use integers where possible
/// This handles cases where specs have 0.0 vs 0, which are semantically identical
fn normalize_numeric_values(object: &mut Map<String, Value>) {
    let numeric_keys = [
        "minimum",
        "maximum",
        "multipleOf",
        "minLength",
        "maxLength",
        "minItems",
        "maxItems",
    ];

    for key in numeric_keys {
        if let Some(value) = object.get_mut(key)
            && let Some(num) = value.as_f64()
        {
            // If the number is a whole number, convert to integer
            if num.fract() == 0.0 && num.is_finite() {
                *value = json!(num as i64);
            }
        }
    }
}

/// Add title to union variants for better type generation
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

/// Convert a schema to be nullable
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
            // Defaults and documentation belong to the outer nullable schema
            let default = schema.remove("default");
            let title = schema.remove("title");
            let description = schema.remove("description");

            let non_null_schema = Value::Object(std::mem::take(schema));

            schema.insert(
                "anyOf".to_owned(),
                Value::Array(vec![
                    non_null_schema,
                    json!({
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
            // Invalid or unsupported `type` representation
        }
    }
}

/// Annotate decimal types for BigDecimal conversion
pub fn annotate_decimal_types(value: &mut Value) {
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

fn bigdecimal_extension() -> Value {
    json!({
        "crate": "bigdecimal",
        "version": ">=0.4.0, <0.5.0",
        "path": "bigdecimal::BigDecimal"
    })
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
