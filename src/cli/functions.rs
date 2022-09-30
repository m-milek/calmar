use crate::{
    active_calendar, active_calendar_reference,
    cal::{
        calendar::Calendar, calendar_index::CalendarIndex, calendar_ref::CalendarReference,
        calmar_error::CalmarError, event::Event,
    },
    calendar_index,
    cli::{
        getdata::{
            get_difficulty, get_dir_path, get_duration, get_end_date, get_end_time, get_priority,
            get_repeat, get_start_date, get_start_time, get_valid_event_name,
        },
        messages::{error, print_err_msg, warning},
        repl::get_input,
        util::{
            default_or_custom_save_path, levenshtein_distance, select_in_range,
            uppercase_first_letter,
        },
        validator::{get_home_dir, validate_duration},
    },
    CONFIG,
};
use chrono::{DateTime, Local};
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
    let path = active_calendar_reference!().path().clone();
    let mut active_calendar = active_calendar!();
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
                let mut new_end_time =
                    get_end_time(&current_start.date(), &current_start.time(), &current_end.date());
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

    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/// Create a calendar reference and return it.
pub fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
    let name = match name {
        Some(name) => name,
        None => get_input("Calendar Name: "),
    };

    //    print!("Path: ");
    let path = default_or_custom_save_path(get_dir_path());
    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
        Some(string) => string,
        None => {
            error(format!("Failed to convert {} to string.", path_to_calendar.display()));
            std::process::exit(1);
        }
    };
    CalendarReference::new(name, path_to_calendar_string.to_owned(), false)
}

pub fn generate_until(calendar: Calendar, end: DateTime<Local>) -> Vec<Event> {
    let event_vec = Arc::new(Mutex::new(vec![]));
    let mut threads = vec![];
    let events = calendar.events().to_vec();

    for event in events {
        threads.push(thread::spawn({
            let clone = Arc::clone(&event_vec);
            move || {
                // If the event is not recurring, just push its only occurrence and return
                if event.repeat().is_zero() {
                    let mut v = clone.lock().unwrap();
                    v.push(event);
                    return;
                }
                let mut temp_vec = vec![];
                let now = Local::now();
                let mut e_to_push = event.to_owned();
                let mut new_start = e_to_push.start();
                let mut new_end = new_start + e_to_push.duration();
                while new_start < end {
                    let mut e = e_to_push.clone();
                    e.set_end(&new_end);
                    if e.start() >= now || e.is_happening_on(now) {
                        temp_vec.push(e);
                    }
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

pub fn handle_unknown_command(s: &str) {
    // command shortcuts such as `ls` might be added here later
    let command_list = [
        "add",
        "cal",
        "clear",
        "duration",
        "edit",
        "help",
        "list",
        "listcal",
        "raw",
        "remove",
        "removecal",
        "set",
        "sort",
        "until",
        "quit",
        "write",
    ];

    let mut best_match: &str = "not found"; // this never gets printed
    let mut min_distance: usize = s.len();

    // Find the best match among commands based on edit distance
    for command in command_list {
        let distance = levenshtein_distance(s, command);
        if distance < min_distance {
            best_match = command;
            min_distance = distance;
        }
    }

    // If the match would be somewhat helpful
    // (distance has to be small, hence 0.8 multiplier) print the suggestion
    if (min_distance as f32) < (s.len() as f32) {
        warning(format!("Unknown command: {}. Did you mean '{}'?", s.trim(), best_match));
        return;
    }
    warning(format!("Unknown command: {}", s.trim()))
}

pub fn closest_occurence_start(event: &Event) -> DateTime<Local> {
    // Searches for the closest occurence of an event.
    // if the event is currently happening, return start datetime of the current occurence.
    // if it's not, return start datetime of the next occurence

    let mut start = event.start();
    let now = Local::now();

    let is_happenning =
        |event: &Event| event.start() < now && event.start() + event.duration() > now;

    while start < now {
        start += event.repeat();
    }
    if is_happenning(event) {
        start -= event.repeat()
    }
    start
}

pub fn check_calmar_dir() {
    let path = get_home_dir().join(".config/calmar");
    if path.exists() {
        return;
    }
    error(format!("{} doesn't exist. Do you want to create it?", path.display()));
    match get_input("[Y/n]: ").to_lowercase().trim() {
        "yes" | "y" => warning(
            "Use the \"mkindex\" command to generate an empty index.json in the created directory."
                .to_string(),
        ),
        _ => return,
    }
    if let Err(e) = std::fs::create_dir(&path) {
        print_err_msg(CalmarError::CreateDir { e }, path.display());
    }
}

// Verify config values
pub fn check_config() {
    let permitted_date_formats = ["DD/MM/YYYY"];
    let permitted_time_formats = ["HH:MM"];
    let mut permitted_colors = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ]
    .map(|s| s.to_string())
    .to_vec();
    let mut brights: Vec<String> = permitted_colors
        .iter()
        .map(|s| "bright_".to_owned() + &s)
        .collect();
    permitted_colors.append(&mut brights);
    let warning = "Invalid config.json values.\n";

    if !permitted_date_formats.contains(&CONFIG.date_format.as_str()) {
        error(format!("{warning}{} is not a valid date format.\nSupported formats: {permitted_date_formats:?}", &CONFIG.date_format));
        std::process::exit(1);
    }
    if !permitted_time_formats.contains(&CONFIG.time_format.as_str()) {
        error(format!("{warning}{} is not a valid time format.\nSupported formats: {permitted_time_formats:?}", &CONFIG.time_format));
        std::process::exit(1);
    }
    if !validate_duration(&CONFIG.default_calendar_span) {
        error(format!(
            "{warning}{} is not a valid duration.\nExamples of valid durations: '7d', '10h', '15m'",
            CONFIG.default_calendar_span
        ));
        std::process::exit(1);
    }
    if !permitted_colors.contains(&CONFIG.prompt_color) {
        error(format!(
            "{warning}{} is not a valid color.\nValid colors: {permitted_colors:?}",
            CONFIG.prompt_color
        ));
        std::process::exit(1);
    }
}

/// Adds a new `CalendarReference` to `self.calendars`.
///
/// # Executed steps
/// * Check for `CalendarReference`s with calendars named like the new one.
/// Remove those entries and associated files if the user agrees.
///
/// * Check for `CalendarReference`s with a path like the new one.
/// Remove those entries and associated files if the user agrees.
///
/// * Push the new `CalendarReference` to the `self.calendars`.
pub fn add_entry(i: &mut CalendarIndex, new_calendar: &CalendarReference) {
    let saved_names: Vec<String> = i.calendars().iter().map(|r| r.name()).collect();

    if saved_names.contains(&new_calendar.name()) {
        match get_input(
            format!(
                "Calendar named {} already exists. Do you want to overwrite it? [y/N]: ",
                new_calendar.name()
            )
            .as_str(),
        )
        .to_lowercase()
        .as_str()
        {
            "y" | "yes" => {}
            _ => return,
        }

        // Remove all calendar files with the same name
        for reference in i.calendars() {
            if reference.name() == new_calendar.name() {
                if let Err(e) = std::fs::remove_file(&reference.path()) {
                    error(format!("Failed to delete file {}.\n{}", reference.path(), e));
                    std::process::exit(1);
                }
            }
        }
        // Remove all references with the same name
        i.calendars_mut()
            .retain(|calendar| calendar.name() != new_calendar.name());
    }

    let saved_paths: Vec<String> = i.calendars().iter().map(|r| r.path()).collect();

    if saved_paths.contains(&new_calendar.path()) {
        match get_input(
            format!(
                "Calendar with path {} already exists. Do you want to overwrite it?",
                new_calendar.path()
            )
            .as_str(),
        )
        .as_str()
        {
            "y" | "yes" => {}
            _ => return,
        }
        // Remove all calendar files with the same path
        for reference in i.calendars() {
            if reference.path() == new_calendar.path() {
                if let Err(e) = std::fs::remove_file(&reference.path()) {
                    error(format!("Failed to delete file {}.\n{}", reference.path(), e));
                    std::process::exit(1);
                }
            }
        }
        // Remove all references with the same path
        i.calendars_mut()
            .retain(|calendar| calendar.path() != new_calendar.path());
    }
    // Now the index is cleaned of any calendars named like the new one and the files are deleted.
    i.calendars_mut().push(new_calendar.clone());
}

/// Deletes an entry from index by name.
/// Disallows unambigous situations where the number of `CalendarReference`s
/// named `name` is not equal to one
pub fn delete_entry(i: &mut CalendarIndex, name: String) {
    let mut tmp_reference_vec = i.calendars().clone();
    tmp_reference_vec.retain(|r| r.name() == name);

    match tmp_reference_vec.len() {
        0 => {
            warning(format!("No calendar named {} found.", name));
            return;
        }
        1 => match std::fs::remove_file(&tmp_reference_vec[0].path()) {
            Ok(_) => (),
            Err(e) => {
                error(format!("Failed to remove file {}.\n{}", tmp_reference_vec[0].path(), e));
                return;
            }
        },
        _ => {
            error(format!("Multiple calendars named {} found. Please fix index.json before proceeding. Calendars must have unique names.", name));
            return;
        }
    }
    i.calendars_mut().retain(|r| r.name() != name);
}
