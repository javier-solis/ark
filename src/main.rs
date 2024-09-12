mod app;
mod data_ops;
mod types;
mod utils;
use anyhow::{Context, Result};
use data_ops::get_data;
use utils::print_courses;

const CATALOG_PATH: &str = "catalog.csv";
const ROADMAP_PATH: &str = "roadmap.json";
const SCHEMA_PATH: &str = "schema.json";

fn main() -> Result<()> {
  // todo: simplify reference to same parent folder of "data"

  // todo: move this validation outside main?
  match utils::validate_json(ROADMAP_PATH, SCHEMA_PATH) {
    Ok(json_instance) => {
      // print_result(&year_0_entry, &json_instance);

      let gpa = app::scan(&json_instance)?;
      println!("GPA: {:.2}", gpa);
    }

    Err(error) => {
      eprintln!("{}", error);
    }
  }

  match get_data::courses(CATALOG_PATH) {
    Ok(courses) => {
      // courses.sort_by_key(|r| Reverse(r.units.clone())); // todo: add optional user-toggle

      print_courses(&courses, true); // todo: remember to make the "minimal print" toggle a user-input

      let class_id = "6.100A";
      if let Ok(units) = get_data::units_by_id(class_id, &courses) {
        println!("Units for course {}: {}", class_id, units);
      } else {
        println!("Course {} not found", class_id);
      }
    }

    Err(error) => {
      eprintln!("{}", error);
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_units_by_id() {
    match get_data::courses(CATALOG_PATH) {
      Ok(courses) => {
        let class_id = "6.08";
        match get_data::units_by_id(class_id, &courses) {
          Ok(units) => {
            assert_eq!(units, 12);
          }
          Err(_) => {
            panic!("Class {} not found", class_id);
          }
        }
      }
      Err(error) => {
        panic!("Error loading courses: {}", error);
      }
    }
  }
}
