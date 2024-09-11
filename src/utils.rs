use colored::Colorize;

use anyhow::{Context, Result};
use colored_json::to_colored_json_auto;
use jsonschema::JSONSchema;
use std::{fs::File, io::Read};

use crate::types::Course;

pub fn read_file(file_path: &str) -> Result<String> {
  let mut file =
    File::open(file_path).with_context(|| format!("Failed to open file '{}'", file_path))?;

  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .with_context(|| format!("Failed to read file '{}'", file_path))?;

  Ok(contents)
}

// todo: use `anyhow` for error management
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

  let json_array: Vec<serde_json::Value> =
    serde_json::from_str(&json_data).map_err(|e| format!("Failed to parse JSON data: {}", e))?;

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

// todo: repurpose or remove
fn print_result(year_0_entry: &serde_json::Value, json_instance: &serde_json::Value) {
  println!("Year 0 entry (excluded from validation):");
  println!("{}", to_colored_json_auto(year_0_entry).unwrap());

  println!("JSON data (excluding year 0) is valid against the schema");
  println!("Pretty-printed JSON:");
  println!("{}", to_colored_json_auto(json_instance).unwrap());
}

/// just prints course stats. minimal keeps print small, to a reasonable minimum
pub fn print_courses(courses: &[Course], minimal: bool) -> () {
  // todo: I think minimal=false doesn't work

  // todo: figure out sorting by ID, Units, title
  if minimal {
    println!("== Minimal Print ==");
  } else {
    println!("== Standard Print ==");
  }

  // todo: simplify implementation of formating (spacing)
  println!("{:<8} {:<10} {}", "ID", "Units", "Title");

  for course in courses {
    // todo: print empty fields as "na", and warn user about that

    let primary_id = &course.primary_id;
    let transcript_title = &course.transcript_title;
    let alternative_title = &course.alternative_title;
    let units = course.units;

    let formatted_line = if minimal {
      let title = if !alternative_title.is_empty() && alternative_title != "na" {
        alternative_title
      } else {
        transcript_title
      };

      format!("{:<8} {:<10} {}", primary_id, units, title)
    } else {
      let legacy_id = &course.legacy_id;
      let dept = &course.dept;
      let technical = course.technical;
      let fulfills = &course.fulfills;

      format!("") // todo
    };

    let has_todo = course.clone().into_iter().any(|field| field == "todo");

    if has_todo {
      println!("{}", formatted_line.red());
    } else {
      println!("{}", formatted_line);
    }
  }
}
