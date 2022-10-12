use crate::cal::{calendar::Calendar, calmar_error::CalmarError};
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Display, io::Write};
use struct_field_names_as_array::FieldNamesAsArray;

/// Holds a "pointer" to a file containing a `Calendar` struct.
/// # Fields
/// `name`: name of the calendar in file under `path`
/// `path`: path to the file containing a `Calendar` struct
/// `active`: determines if the `Calendar` under `path` is currently selected.
#[derive(Clone, Debug, Serialize, Deserialize, FieldNamesAsArray)]
pub struct CalendarReference {
    name: String,
    path: String,
    active: bool,
}

impl Display for CalendarReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reference name: {} | Path: {} | Active: {}",
            self.name, self.path, self.active
        )
    }
}

impl CalendarReference {
    pub fn new(name: String, path: String, active: bool) -> Self {
        CalendarReference { name, path, active }
    }

    // Getters
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn active(&self) -> bool {
        self.active
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path
    }

    pub fn set_active(&mut self) {
        self.active = true
    }

    pub fn set_inactive(&mut self) {
        self.active = false
    }

    //Other
    pub fn create_file(&self) -> Result<(), CalmarError> {
        let mut calendar_file = match std::fs::File::create(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(CalmarError::CreateFile { e }),
        };

        let calendar_json: String =
            match serde_json::to_string_pretty(&Calendar::new(self.name.as_str())) {
                Ok(result) => result,
                Err(e) => return Err(CalmarError::ToJSON { e }),
            };

        if let Err(e) = write!(calendar_file, "{}", calendar_json) {
            return Err(CalmarError::WriteFile { e });
        }
        Ok(())
    }
}
