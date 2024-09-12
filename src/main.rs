mod app;
mod data_ops;
mod types;
mod utils;
use anyhow::{Context, Result};
use data_ops::get_data;
use utils::print_courses;

fn main() -> Result<()> {
  let roadmap_path = "roadmap.json";
  let schema_path = "schema.json";
  let catalog_path = "catalog.csv";

  // todo: move this validation outside main?
  match utils::validate_json(roadmap_path, schema_path) {
    Ok(json_instance) => {
      // print_result(&year_0_entry, &json_instance);

      let gpa = app::scan(&json_instance)?;
      println!("GPA: {:.2}", gpa);
    }

    Err(error) => {
      eprintln!("{}", error);
    }
  }

  match get_data::courses(catalog_path) {
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
