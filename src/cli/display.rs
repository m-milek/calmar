use super::functions::duration_fmt;
use crate::{
    active_calendar, active_calendar_reference, cal::event::Event, calendar_index, CONFIG,
};
use tabled::{Disable, Style, Table, Tabled};

#[derive(Tabled, Debug)]
pub struct DisplayedEvent {
    name: String,
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
    repeat: String,
    priority: u8,
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
