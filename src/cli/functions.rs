use crate::{
    cal::{
        calendar::Calendar, calendar_index::CalendarIndex, calendar_ref::CalendarReference,
        event::Event,
    },
    cli::{
        commands::default_or_custom_save_path,
        getdata::{
            get_difficulty, get_dir_path, get_duration, get_end_date, get_end_time, get_priority,
            get_repeat, get_start_date, get_start_time, get_valid_event_name,
        },
        messages::{error, print_err_msg, warning},
        repl::get_input,
        util::{select_in_range, uppercase_first_letter},
    },
    CONFIG,
};
use chrono::{DateTime, Duration, Local};
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

/// Create a new event and return it.
pub fn get_new_event(name: Option<String>) -> Event {
    let name = match name {
        Some(name) => name,
        None => get_valid_event_name(),
    };

    let start_date = get_start_date();

    let start_time = get_start_time();

    let duration = get_duration();

    let end_date;
    let end_time;
    if duration.is_zero() {
        end_date = get_end_date(&start_date);

        end_time = get_end_time(&start_date, &start_time, &end_date);
    } else {
        let end_timedate = start_date.and_time(start_time).unwrap() + duration;
        end_date = end_timedate.date();
        end_time = end_timedate.time();
    }

    let repeat = get_repeat();

    let difficulty = get_difficulty();

    let priority = get_priority();

    Event::new(
        name,
        start_date.and_time(start_time).unwrap(),
        end_date.and_time(end_time).unwrap(),
        repeat,
        priority,
        difficulty,
    )
}

pub fn edit_event(event_name: &str) {
    let index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    let active_ref = match index.active_calendar_reference() {
        Ok(r) => r,
        Err(e) => {
            print_err_msg(e, &String::new());
            return;
        }
    };
    let path = active_ref.path();

    let mut active_calendar = match index.active_calendar() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, path);
            return;
        }
    };

    let mut index_map = HashMap::<usize, usize>::with_capacity(active_calendar.events().len());

    let mut i = 0;
    for (num, event) in active_calendar.events().iter().enumerate() {
        if event.name() == event_name {
            index_map.insert(i, num);
            i += 1;
        }
    }

    // Choose an event to be edited
    let events_named_like_arg: Vec<Event> = active_calendar
        .events()
        .clone()
        .into_iter()
        .filter(|event| event.name() == event_name)
        .collect();

    if events_named_like_arg.is_empty() {
        warning(format!("No event named {} found.", event_name));
        return;
    }
    println!("{:#?}", events_named_like_arg);
    let index_to_select = match events_named_like_arg.len() {
        1 => 0,
        _ => select_in_range("Select an event to edit", events_named_like_arg.len()) - 1,
    };

    // Choose a property to be edited
    let fields = Event::FIELD_NAMES_AS_ARRAY.to_vec();
    let mut fields_list: Vec<String> = fields.into_iter().map(uppercase_first_letter).collect();
    // Duration is not a struct property, but it still should be easily editable
    fields_list.insert(2, "Duration".to_string());

    fields_list
        .iter()
        .enumerate()
        .for_each(|(i, field)| println!("{}. {field}", i + 1));

    let edited_event = &mut active_calendar.events_mut()[index_map[&index_to_select]];
    let num: usize = select_in_range("Select what to edit", fields_list.len());

    match num {
        // Edit name
        1 => {
            print!("Name: ");
            edited_event.set_name(&get_valid_event_name());
        }
        // Edit start timedate
        2 => {
            println!("1. Start date\n2. Start time\n3. Start datetime");
            let num = select_in_range("Select what to edit", 3);
            let current_end = edited_event.end();
            let current_start = edited_event.start();

            if num == 1 || num == 3 {
                print!("Start date: ");
                let mut new_start_date = get_start_date();
                while new_start_date.and_time(current_start.time()).unwrap() > current_end {
                    println!("Start timedate cannot be after end timedate");
                    print!("Start date: ");
                    new_start_date = get_start_date();
                }
                edited_event.set_start(&new_start_date.and_time(current_start.time()).unwrap())
            }
            if num == 2 || num == 3 {
                print!("Start time: ");
                let mut new_start_time = get_start_time();
                while current_start.date().and_time(new_start_time).unwrap() > current_end {
                    println!("Start timedate cannot be after end timedate");
                    print!("Start date: ");
                    new_start_time = get_start_time();
                }
                edited_event.set_start(&current_start.date().and_time(new_start_time).unwrap())
            }
        }
        // Edit duration
        3 => {
            print!("Duration: ");
            let new_duration = get_duration();
            let start = edited_event.start();
            edited_event.set_end(&(start + new_duration));
        }
        // Edit end datetime
        4 => {
            println!("1. End date\n2. End time\n3. End datetime");
            let num: usize = select_in_range("Select what to edit", 3);
            let mut current_end = edited_event.end();
            let current_start = edited_event.start();

            if num == 1 || num == 3 {
                print!("End date: ");
                let mut new_end_date = get_end_date(&current_start.date());
                while new_end_date.and_time(current_end.time()).unwrap() < current_start {
                    println!("End timedate cannot be before start timedate");
                    print!("End date: ");
                    new_end_date = get_end_date(&current_start.date());
                }
                edited_event.set_end(&new_end_date.and_time(current_end.time()).unwrap());
            }
            if num == 2 || num == 3 {
                current_end = edited_event.end();
                print!("End time: ");
                let mut new_end_time = get_end_time(
                    &current_start.date(),
                    &current_start.time(),
                    &current_end.date(),
                );
                while current_end.date().and_time(new_end_time).unwrap() < current_start {
                    println!("End timedate cannot be before start timedate");
                    print!("End date: ");
                    new_end_time = get_end_time(
                        &current_start.date(),
                        &current_start.time(),
                        &edited_event.end().date(),
                    );
                }
                edited_event.set_end(&current_end.date().and_time(new_end_time).unwrap());
            }
        }
        // Edit repeat
        5 => {
            print!("Repeat: ");
            edited_event.set_repeat(&get_repeat())
        }
        // Edit priority
        6 => {
            print!("Priority: ");
            edited_event.set_priority(get_priority())
        }
        // Edit difficulty
        7 => {
            print!("Difficulty: ");
            edited_event.set_difficulty(get_difficulty())
        }
        _ => panic!("Impossible"),
    }

    if let Err(e) = active_calendar.save(path) {
        print_err_msg(e, path)
    }
}

/// Create a calendar reference and return it.
pub fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
    let name = match name {
        Some(name) => name,
        None => get_input("Calendar Name: "),
    };

    print!("Path: ");
    let path = default_or_custom_save_path(get_dir_path());
    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
        Some(string) => string,
        None => {
            error(format!(
                "Failed to convert {} to string.",
                path_to_calendar.display()
            ));
            std::process::exit(1);
        }
    };
    CalendarReference::new(name, path_to_calendar_string.to_owned(), false)
}

pub fn duration_fmt(duration: Duration) -> String {
    if duration.num_seconds() < 60 {
        format!("{}s", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!(
            "{}m {}s",
            duration.num_minutes(),
            duration.num_seconds() - duration.num_minutes() * 60
        )
    } else if duration.num_hours() < 24 {
        let num_h = duration.num_hours();
        // remaining minutes after accounting for the whole hours (occurs further into the function as well)
        let num_m = duration.num_minutes() - num_h * 60;
        format!("{}h {}m", num_h, num_m)
    } else {
        let num_d = duration.num_days();
        let num_h = duration.num_hours() - num_d * 24;
        let num_m = duration.num_minutes() - num_h * 60 - num_d * 24 * 60;
        format!("{}d {}h {}m", num_d, num_h, num_m)
    }
}

pub fn generate_until(calendar: Calendar, end: DateTime<Local>) -> Vec<Event> {
    let event_vec = Arc::new(Mutex::new(vec![]));
    let mut threads = vec![];
    let events = calendar.events().to_vec();

    for event in events {
        threads.push(thread::spawn({
            let clone = Arc::clone(&event_vec);
            move || {
                let mut temp_vec = vec![];
                let mut e_to_push = event.to_owned();
                let mut new_start = e_to_push.start();
                let mut new_end = new_start + e_to_push.duration();
                while new_start + e_to_push.repeat() < end {
                    let mut e = e_to_push.clone();
                    e.set_end(&new_end);
                    temp_vec.push(e);
                    new_start += e_to_push.repeat();
                    new_end = new_start + e_to_push.duration();
                    e_to_push.set_start(&new_start);
                    e_to_push.set_end(&new_end);
                }
                let mut v = clone.lock().unwrap();
                v.append(&mut temp_vec);
            }
        }))
    }
    for t in threads {
        t.join().unwrap()
    }

    // get the Vec<Event> out of Arc and Mutex
    let getter = event_vec.lock().unwrap();
    let mut out = vec![];
    for item in &*getter {
        out.push(item.clone());
    }
    out.sort();
    out
}
