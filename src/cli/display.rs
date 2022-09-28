use super::functions::duration_fmt;
use crate::{
    active_calendar, active_calendar_reference, cal::{event::Event, calendar_ref::CalendarReference}, calendar_index, CONFIG,
};
use tabled::{Disable, Style, Table, Tabled};

#[derive(Tabled, Debug)]
pub struct DisplayedEvent {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Start Date")]
    start_date: String,
    #[tabled(rename = "Start Time")]
    start_time: String,
    #[tabled(rename = "End Date")]
    end_date: String,
    #[tabled(rename = "End Time")]
    end_time: String,
    #[tabled(rename = "Repeat Every")]
    repeat: String,
    #[tabled(rename = "Priority")]
    priority: u8,
    #[tabled(rename = "Difficulty")]
    difficulty: u8,
}

impl DisplayedEvent {
    pub fn from_event(event: &Event) -> DisplayedEvent {
        DisplayedEvent {
            name: event.name(),
            start_date: format!("{}", event.start().date_naive()),
            start_time: format!("{}", event.start().time()),
            end_date: format!("{}", event.end().date_naive()),
            end_time: format!("{}", event.end().time()),
            repeat: duration_fmt(event.repeat()),
            priority: event.priority(),
            difficulty: event.difficulty(),
        }
    }
}

#[derive(Tabled, Debug)]
pub struct DisplayedCalendarReference {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Path")]
    path: String,
    #[tabled(rename = "Is Active?")]
    active: bool,
}

impl From<CalendarReference> for DisplayedCalendarReference {
    fn from(r: CalendarReference) -> DisplayedCalendarReference {
	DisplayedCalendarReference {
	    name: r.name().to_owned(),
	    path: r.path().to_owned(),
	    active: r.active(),
	}
    }
}

pub fn print_stuff() {
    let events = active_calendar!().events().clone();
    let displayed_events: Vec<DisplayedEvent> = events
        .iter()
        .map(|e| DisplayedEvent::from_event(e))
        .collect();
    // skip kolumn
    let t = Table::new(displayed_events)
        .with(Disable::Column(0..2))
        .with(Style::modern());
    println!("{t}");
}
