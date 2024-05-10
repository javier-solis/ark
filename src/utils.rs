use anyhow::{Context, Result};
use jsonschema::JSONSchema;
use std::{fs::File, io::Read};

pub fn read_file(file_path: &str) -> Result<String> {
    let mut file =
        File::open(file_path).with_context(|| format!("Failed to open file '{}'", file_path))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read file '{}'", file_path))?;

    Ok(contents)
}

// todo: use anyhow for error management
/// Validate a JSON file against a [JSON Schema](https://json-schema.org/).
/// If successful: returns the JSON as a serde object. Else: Returns an error message.
pub fn validate_json(file_path: &str, schema_path: &str) -> Result<serde_json::Value, String> {
    let json_data = match read_file(file_path) {
        Ok(data) => data,
        Err(error) => return Err(format!("Error reading JSON file: {}", error)),
    };

    let schema_data = match read_file(schema_path) {
        Ok(data) => data,
        Err(error) => return Err(format!("Error reading JSON schema file: {}", error)),
    };

    let json_schema: serde_json::Value = serde_json::from_str(&schema_data)
        .map_err(|e| format!("Failed to parse JSON schema: {}", e))?;
    let compiled_schema = JSONSchema::compile(&json_schema)
        .map_err(|e| format!("Failed to compile JSON schema: {}", e))?;

    let json_array: Vec<serde_json::Value> = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    if json_array.is_empty() {
        return Err("JSON array is empty".to_string());
    }

    let json_instance = serde_json::Value::Array(json_array);

    let result = compiled_schema.validate(&json_instance);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("- {}\n  Instance path: {}", e, e.instance_path))
            .collect();
        return Err(format!("Validation errors:\n{}", error_messages.join("\n")));
    }

    Ok(json_instance.clone()) // todo: does this have to be a .clone?
}
