/// # Data Getters
/// Functions that get data from `catalog.csv`
// todo: generalize this module for all CRUD ops that interact with the csv file
// todo: add these as impls for `Course`?
pub mod get_data {

  use crate::*; // todo: be more specific
  use types::Course;

  // todo: check that catalog is actually a csv file. as well as the appropriate headers

  /// Turn courses from CSV catalog of classes into a vector of Classes.
  pub fn courses(catalog_path: &str) -> Result<Vec<Course>> {
    let csv_file = utils::read_file(catalog_path)?;

    let mut reader = csv::Reader::from_reader(csv_file.as_bytes());

    let mut courses: Vec<Course> = Vec::new();

    for record in reader.deserialize() {
      let record: Course = record.context("record not in desired format")?;
      courses.push(record);
    }

    Ok(courses)
  }

  pub fn units_by_id(id: &str, courses: &[Course]) -> Result<u32> {
    for course in courses {
      if course.primary_id == id {
        return Ok(course.units); // todo: check that it's an integer >0
      }
    }
    anyhow::bail!("Units not found for \"{}\"", id);
  }
}
