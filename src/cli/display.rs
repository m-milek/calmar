use crate::{
    active_calendar, active_calendar_reference,
    cal::{calendar_ref::CalendarReference, event::Event},
    calendar_index,
    cli::util::duration_fmt,
    CONFIG,
    
};
use tabled::{Disable, Style, Table, Tabled};
use chrono::{NaiveTime, Timelike, Datelike};

#[derive(Tabled, Debug, Clone)]
pub struct DetailedEvent {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Start Time")]
    start_time: String,
    #[tabled(rename = "End Time")]
    end_time: String,
    #[tabled(rename = "Repeat Every")]
    repeat: String,
    #[tabled(rename = "Priority")]
    priority: u8,
    #[tabled(rename = "Difficulty")]
    difficulty: u8,
}

impl DetailedEvent {
    pub fn from(event: &Event) -> DetailedEvent {
	
        DetailedEvent {
            name: event.name(),
            start_time: format!("{} {}", event.start().date_naive(),
				event.start().time().format("%H:%M")),
            end_time: format!("{} {}", event.end().date_naive(),
			      event.end().time().format("%H:%M")),
            repeat: duration_fmt(event.repeat()),
            priority: event.priority(),
            difficulty: event.difficulty(),
        }
    }
}

#[derive(Tabled, Debug, Clone)]
pub struct SimpleEvent {
    #[tabled(rename = "Time")]
    time: String,
    #[tabled(rename = "Event")]
    name: String,
    #[tabled(rename = "Priority")]
    priority: u8,
    #[tabled(rename = "Difficulty")]
    difficulty: u8,
}

impl SimpleEvent {
    pub fn from(event: &Event) -> SimpleEvent {
        SimpleEvent {
            name: event.name(),
            time: format!("{} - {}", event.start().format("%H:%M"),
			  event.end().time().format("%H:%M")),
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
            name: r.name(),
            path: r.path(),
            active: r.active(),
        }
    }
}

pub fn print_stuff() {
    let events = active_calendar!().events().clone();
    let displayed_events: Vec<DetailedEvent> = events
        .iter()
        .map(DetailedEvent::from)
        .collect();
    // skip kolumn
    let t = Table::new(displayed_events)
        .with(Disable::Column(0..2))
        .with(Style::modern());
    println!("{t}");
}

pub fn display_simple_events(events: Vec<Event>) {
    let displayed_events: Vec<SimpleEvent> =
	events.iter().map(SimpleEvent::from).collect();
    let table = Table::new(displayed_events).with(Style::modern());
	//.with(Disable::Column(1..1));
    println!("{table}");
}

pub fn display_detailed_events(events: Vec<Event>) {
    let displayed_events: Vec<DetailedEvent> =
	events.iter().map(DetailedEvent::from).collect();
    

    let mut hide_repeat = true;
    let repeat = "0s";
    displayed_events.iter().for_each(|event| {
	if event.repeat != repeat {
	    hide_repeat = false;
	}
    });

    let mut table = Table::new(displayed_events).with(Style::modern());
    if hide_repeat {
	table = table.with(Disable::Column(3..4));
    }
    println!("{table}");
}

pub fn display_events(events: Vec<Event>) {
    let mut there_are_multiple_dates = false;
    let date = events[0].start();
    events.iter().for_each(|event| {
	if event.start() != date {
	    there_are_multiple_dates = true;
	}
    });
    if there_are_multiple_dates {
	display_detailed_events(events);
	return;
    }
    display_simple_events(events);
}
