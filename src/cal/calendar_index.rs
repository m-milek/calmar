use crate::{
    cal::{
        calendar::Calendar,
        calendar_ref::CalendarReference,
        calmar_error::CalmarError,
    },
    cli::validator::get_home_dir
};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::{fmt::Display, fs::read_to_string};

/// Holds a vector of `CalendarReference` structs.
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    calendars: Vec<CalendarReference>,
}

impl Display for CalendarIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Calendars: {:#?}",
            self.calendars
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
        )
    }
}

impl CalendarIndex {
    /// Returns `CalendarIndex` struct from `$HOME/.config/calmar/index.json`.
    pub fn get() -> Result<Self, CalmarError> {
        let index_file_path = get_home_dir().join(".config/calmar/index.json");

        let content = match read_to_string(&index_file_path) {
            Ok(result) => result,
            Err(e) => return Err(CalmarError::ReadFile { e }),
        };

        match serde_json::from_str(&content) {
            Ok(result) => Ok(result),
            Err(e) => Err(CalmarError::ParseJSON { e }),
        }
    }

    pub fn new() -> Self {
        CalendarIndex { calendars: vec![] }
    }

    // Getters
    pub fn calendars(&self) -> &Vec<CalendarReference> {
        &self.calendars
    }
    #[allow(dead_code)]
    pub fn calendars_mut(&mut self) -> &mut Vec<CalendarReference> {
        &mut self.calendars
    }

    pub fn num_named(&self, name: &String) -> usize {
	self.calendars.iter().filter(|r| r.name() == *name).count()
    }

    /// Returns `Calendar` struct parsed from the file pointed at by a `CalendarReference`
    /// currently set as active in `$HOME/.config/calmar/index.json`.
    pub fn active_calendar(&self) -> Result<Calendar, CalmarError> {
        let num = self.calendars.iter().filter(|r| r.active()).count();

        let current_calendar = match num {
            1 => &self.calendars[self.calendars.iter().position(|r| r.active()).unwrap()],
            _ => return Err(CalmarError::ActiveCalendarCount { e: num }),
        };

        let current_calendar_content = match read_to_string(&current_calendar.path()) {
            Ok(content) => content,
            Err(e) => return Err(CalmarError::ReadFile { e }),
        };

        match serde_json::from_str(&current_calendar_content) {
            Ok(result) => Ok(result),
            Err(e) => Err(CalmarError::ParseJSON { e }),
        }
    }

    /// Returns a `CalendarReference` currently set as active in `$HOME/.config/calmar/index.json`.
    pub fn active_calendar_reference(&self) -> Result<CalendarReference, CalmarError> {
        let mut refs = self.calendars.clone();
        let num = refs.iter().filter(|r| r.active()).count();
        match num {
            1 => {
                refs.retain(|r| r.active());
                Ok(refs[0].clone())
            }
            _ => Err(CalmarError::ActiveCalendarCount { e: num }),
        }
    }

    pub fn number_of_active_calendars(&self) -> usize {
        self.calendars.iter().filter(|c| c.active()).count()
    }

    pub fn save(&self) -> Result<(), CalmarError> {
        let index_file_path = get_home_dir().join(".config/calmar/index.json");

        let mut index_file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(index_file_path)
        {
            Ok(file) => file,
            Err(e) => return Err(CalmarError::ReadFile { e }),
        };
        let calendar_index_json: String = match serde_json::ser::to_string_pretty(&self) {
            Ok(result) => result,
            Err(e) => return Err(CalmarError::ToJSON { e }),
        };

        match write!(index_file, "{}", calendar_index_json) {
            Ok(_) => Ok(()),
            Err(e) => Err(CalmarError::WriteFile { e }),
        }
    }

    pub fn set_active(&mut self, name: String) {
        // Set the currently active calendar as not active
        // Set the desired calendar as active

        for r in &mut self.calendars {
            if r.active() {
                r.set_inactive()
            }
            if r.name() == name {
                r.set_active()
            }
        }
    }
}
