use serde::Deserialize;

/// # GPA Module
/// Enums for different GPA scales.
pub mod gpa {
  use strum::EnumString;

  #[derive(EnumString)]
  pub enum Five {
    A = 5,
    B = 4,
    C = 3,
    D = 2,
    F = 0,
  }

  #[derive(EnumString)]
  pub enum Four {
    A = 4,
    B = 3,
    C = 2,
    D = 1,
    F = 0,
  }

  // todo: add `impl`s for these enums, for their various use cases?
}

/// # Course Struct
/// Maintains the data structure of a course. Should match what can be found in `roadmap.json`.
// todo: instead of makign the fields public, create getters and setters as needed
#[derive(Deserialize, Clone)]
pub struct Course {
  pub primary_id: String,
  pub legacy_id: String,
  pub dept: String,
  pub transcript_title: String,
  pub alternative_title: String,
  pub units: u32,
  pub technical: bool,
  pub fulfills: String,
}

// todo: update to use references?
impl IntoIterator for Course {
  type Item = String;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    vec![
      self.primary_id.to_owned(),
      self.legacy_id.to_owned(),
      self.dept.to_owned(),
      self.transcript_title.to_owned(),
      self.alternative_title.to_owned(),
      self.units.to_string(),
      self.technical.to_string(),
      self.fulfills.to_owned(),
    ]
    .into_iter()
  }
}
