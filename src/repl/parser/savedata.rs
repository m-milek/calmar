use crate::event::Event;
use std::fs::{File, OpenOptions};
use std::io::Write;

/*
Takes an Event argument, converts it to a EventJSON helper struct, serializes it and saves to the currently selected calendar
 */
pub fn save_event(event: Event, calendar: String) {
    let mut file = match OpenOptions::new().write(true).append(true).open(calendar){
	Ok(file) => file,
	Err(err) => {
	    println!("{}", err);
	    todo!();
	}
    };
    if let Err(e) = writeln!(file, "A new line") {
	println!("Couldn't write to file: {}", e);
    }
}
