use crate::{
    cal::{calendar_ref::CalendarReference, event::Event, deadline::Deadline},
    cli::util::duration_fmt
};
use tabled::{Disable, Style, Table, Tabled};

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

impl From<&Event> for DetailedEvent {
    fn from(event: &Event) -> DetailedEvent {
        DetailedEvent {
            name: event.name(),
            start_time: format!("{} {}", event.start().date_naive(),
				event.start().time().format("%H:%M")),
            end_time: format!("{} {}", event.end().date_naive(),
			      event.end().time().format("%H:%M")),
            repeat: match event.repeat().is_zero() {
		true => "None".to_string(),
		false => duration_fmt(event.repeat())
	    },
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

impl From<&Event> for SimpleEvent {
    fn from(event: &Event) -> SimpleEvent {
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

impl From<&CalendarReference> for DisplayedCalendarReference {
    fn from(r: &CalendarReference) -> DisplayedCalendarReference {
        DisplayedCalendarReference {
            name: r.name(),
            path: r.path(),
            active: r.active(),
        }
    }
}

pub fn display_simple_events(events: Vec<Event>) {
    let displayed_events: Vec<SimpleEvent> =
	events.iter().map(|e| SimpleEvent::from(e)).collect();
    let table = Table::new(displayed_events).with(Style::modern());
	//.with(Disable::Column(1..1));
    println!("{table}");
}

pub fn display_detailed_events(events: Vec<Event>) {
    let displayed_events: Vec<DetailedEvent> =
	events.iter().map(|e| DetailedEvent::from(e)).collect();

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
    let date = match events.get(0) {
	Some(e) => e,
	None => return
    }.start();
    if events.iter().any(|e| e.start() != date) {
	display_detailed_events(events);
	return;
    }
    display_simple_events(events);
}

pub fn colorize_deadline(d: &Deadline) {
    let s = d.to_string();
    let split = s.split('\t').collect::<Vec<&str>>();
    println!("{split:?}");
}
