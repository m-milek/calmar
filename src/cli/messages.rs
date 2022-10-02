use crate::{cal::calmar_error::CalmarError, CONFIG, error};

pub fn print_err_msg<T: std::fmt::Display>(err: CalmarError, info: T) {
    match err {
        CalmarError::ReadFile { e } => error!("Failed to read {} \n{}", info, e),
        CalmarError::ParseJSON { e } => error!("Failed to parse {} as JSON.\n{}", info, e),
        CalmarError::WriteFile { e } => error!("Failed to write to {}.\n{}", info, e),
        CalmarError::CreateFile { e } => {
            error!("Failed to create file at {}.\n{}", info, e)
        }
        CalmarError::ToJSON { e } => error!("Failed to serialize struct to JSON.\n{}", e),
        CalmarError::ActiveCalendarCount { e } => error!(
            "There are {} calendars set as 'active'. There should be exactly one.",
            e
        ),
        CalmarError::CreateDir { e } => {
            error!("Failed to create directory at {info}.\n{e}")
        }
    }
}
