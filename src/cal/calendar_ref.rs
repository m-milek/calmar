#![allow(dead_code)]

use crate::cal::calendar::Calendar;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use super::calmar_error::CalmarError;

/// Holds a "pointer" to a file containing a `Calendar` struct.
/// # Fields
/// `name`: name of the calendar in file under `path`
/// `path`: path to the file containing a `Calendar` struct
/// `active`: determines if the `Calendar` under `path` is currently selected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalendarReference {
    name: String,
    path: String,
    active: bool,
}

impl CalendarReference {
    
    pub fn new(name: String, path: String, active: bool) -> Self {
        CalendarReference { name, path, active }
    }

    // Getters
    pub fn name(&self) -> &String {
	&self.name
    }
    pub fn path(&self) -> &String {
	&self.path
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
            Err(e) => {
		return Err(CalmarError::CreateFile { e })
            }
        };

        let calendar_json: String =
            match serde_json::to_string_pretty(&Calendar::new(self.name.as_str())) {
                Ok(result) => result,
                Err(e) => return Err(CalmarError::ToJSON { e })
            };

        if let Err(e) = write!(calendar_file, "{}", calendar_json) {
           return Err(CalmarError::WriteFile { e })
        }
	Ok(())
    }
}

