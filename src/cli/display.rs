use crate::{
    cal::{
        calendar_ref::CalendarReference, calmar_trait::CalendarDataType, deadline::Deadline,
        event::Event,
    },
    cli::util::duration_fmt,
    error, CONFIG,
};
use chrono::{Datelike, Duration};
use colored::Colorize;
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
            start_time: format!(
                "{} {}",
                event.start(),
                event.start().time().format("%H:%M")
            ),
            end_time: format!(
                "{} {}",
                event.end(),
                event.end().time().format("%H:%M")
            ),
            repeat: match event.repeat().is_zero() {
                true => "None".to_string(),
                false => duration_fmt(event.repeat()),
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
            time: format!(
                "{} - {}",
                event.start().format("%H:%M"),
                event.end().time().format("%H:%M")
            ),
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
    let displayed_events: Vec<SimpleEvent> = events.iter().map(|e| SimpleEvent::from(e)).collect();
    let table = Table::new(displayed_events).with(Style::modern());
    println!("{table}");
}

pub fn display_detailed_events(events: Vec<Event>) {
    // at this point, the events vector is guaranteed to not be empty.
    let date_range = events[0].start().date()..events.iter().last().unwrap().start().date();
    let mut current_date = date_range.start;
    loop {
        let displayed_events = events
            .iter()
            .filter(|e| e.start().date() == current_date)
            .map(DetailedEvent::from)
            .collect::<Vec<DetailedEvent>>();
        println!(
            "{}, {}",
            current_date.to_string().bold(),
            current_date.weekday().to_string().bold()
        );
        let mut table = Table::new(&displayed_events).with(Style::modern());
        if displayed_events.iter().all(|e| e.repeat == "None") {
            table = table.with(Disable::Column(3..4));
        }
        println!("{table}");
        current_date += Duration::days(1);
        if current_date == date_range.end {
            break;
        }
    }
}

pub fn display_events(events: Vec<Event>) {
    let date = match events.get(0) {
        Some(e) => e,
        None => return,
    }
    .start();
    if events.iter().any(|e| e.start() != date) {
        display_detailed_events(events);
        return;
    }
    display_simple_events(events);
}

pub fn colorize_deadline(d: &Deadline) -> String {
    let s = d.to_string();
    let split = s.split('\t').collect::<Vec<&str>>();
    // bold name, colorized everything
    let out = vec![
        split[0].bold(),
        split[1].clear(),
        split[2].clear(),
        split[3].clear(),
        split[4].clear(),
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
    let x = out.join("\t");
    match d.priority() {
        0..=5 => x.green(),
        6..=8 => x.yellow(),
        9..=10 => x.red(),
        _ => {
            error!("Invalid priority in deadline {}", d.name());
            std::process::exit(1)
        }
    }
    .to_string()
}
