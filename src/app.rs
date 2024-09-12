use crate::types::gpa;
use crate::{data_ops::get_data, CATALOG_PATH};
use anyhow::{Context, Result};
use colored::Colorize;
use std::str::FromStr;

// todo: update scan arg + result with these:
// struct ScanOptions {
//   total_classes: Option<bool>,
// }
// struct ScanResults {
//   total_classes: Option<u32>,
// }

// todo: option to generate output according to how a transcript would look (use legacy id if applicable, grade, PNR, etc.)
// todo: highlight rows depending on dept
pub fn scan(json_instance: &serde_json::Value) -> Result<f64> {
  let mut total_units = 0;

  let mut total_units_gpa = 0;
  let mut total_units_gpa_p = 0;

  let mut total_classes = 0;

  // weighted grade points: 5.0 and 4.0 scale
  let mut wgp5: u32 = 0;
  let mut wgp4: u32 = 0;

  let catalog = get_data::courses(CATALOG_PATH)?;

  let entries = match json_instance.as_array() {
    Some(array) => array,
    None => return Ok(0.0), // todo: return error?
  };

  for entry in entries {
    if let Some(0) = entry.get("year").and_then(|year| year.as_i64()) {
      continue; // Skip the entry if the year is 0 (special case)
    }

    let terms_array = match entry.get("terms").and_then(|terms| terms.as_array()) {
      Some(array) => array,
      None => continue,
    };

    for term in terms_array {
      let classes_array = match term.get("classes").and_then(|classes| classes.as_array()) {
        Some(array) => array,
        None => continue,
      };

      for class in classes_array {
        if let Some(grade_str) = class.get("grade").and_then(|grade| grade.as_str()) {
          // todo: multiply grade by # of units, add up units.
          // that will become the "weighted gpa / real_gpa"
          // and another for "transcript_gpa" where we exclude P's

          // todo: there's way too much nesting going on with the Some(). try not to have so many, perhaps using unwrap?
          // no need to throw errors if it can be handled gracefully (so use unwrap_or, I think)
          match grade_str {
            "A" | "B" | "C" | "D" | "F" | "P" => {
              if let Some(id_str) = class.get("id").and_then(|id| id.as_str()) {
                let class_units = get_data::units_by_id(id_str, &catalog)?;
                total_units += class_units;
                if grade_str == "P" {
                  total_units_gpa_p += class_units;

                  let output = format!(
                    "[Warning | GPA] Ignoring class \"{}\" in GPA calculations",
                    id_str
                  )
                  .yellow();
                  println!("{}", output);
                } else {
                  total_units_gpa += class_units;

                  wgp5 += (gpa::Five::from_str(grade_str).unwrap() as u32) * class_units;
                  wgp4 += (gpa::Four::from_str(grade_str).unwrap() as u32) * class_units;
                }
              }
            }
            _ => {
              // todo: something special for 'S' classes
              let output = format!(
                "[Warning | Scan] Ignoring class \"{}\" in scan",
                class.get("id").and_then(|id| id.as_str()).unwrap_or("")
              )
              .yellow();

              println!("{}", output);
            }
          }
          total_classes += 1;
        }
      }
    }
  }

  if total_classes > 0 {
    println!("Total number of classes: {}", total_classes);
    Ok(wgp5 as f64 / total_units_gpa as f64)
  } else {
    eprint!("error, should have counted classes");
    Ok(0.0)
  }
}
