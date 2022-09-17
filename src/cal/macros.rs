/// Returns the calendar index. On error, prints an error message from cli::messages::print_err_msg and returns the function
#[macro_export]
macro_rules! calendar_index {
    () => {
        match $crate::cal::calendar_index::CalendarIndex::get() {
            Ok(i) => i,
            Err(e) => {
                $crate::cli::messages::print_err_msg(e, &CONFIG.index_path);
                return;
            }
        }
    }
}

/// Returns the active calendar reference. On error, prints an error message from cli::messages::print_err_msg and returns the function
#[macro_export]
macro_rules! active_calendar_reference {
    () => {
        match calendar_index!().active_calendar_reference() {
            Ok(r) => r,
            Err(e) => {
                $crate::cli::messages::print_err_msg(e, &String::new());
                return;
            }
        }
    }
}

/// Returns the active calendar. On error, prints an error message from cli::messages::print_err_msg and returns the function
#[macro_export]
macro_rules! active_calendar {
    () => {
        match calendar_index!().active_calendar() {
            Ok(c) => c,
            Err(e) => {
                $crate::cli::messages::print_err_msg(e, active_calendar_reference!().path());
                return;
            }
        }
    }
}

