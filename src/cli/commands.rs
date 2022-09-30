use crate::{
    active_calendar, active_calendar_reference,
    cal::{calendar_index::CalendarIndex, calmar_error::CalmarError, event::Event},
    calendar_index,
    cli::{
        config::Config,
        functions::{
            add_entry, closest_occurence_start, delete_entry, edit_event, generate_until,
            get_new_calendar_reference, get_new_event,
        },
        getdata::{get_valid_calendar_name, get_valid_event_name, parse_into_duration},
        messages::{error, print_err_msg, success, warning},
        repl::get_input,
        util::{duration_fmt, get_now_even, round_to_full_day},
        validator::{get_home_dir, validate_duration},
    },
    CONFIG,
};
use chrono::{Duration, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    ops::Neg,
    path::{Path, PathBuf},
    str::FromStr,
};

/*
Given 'name' of a new calendar, the function gets the home directory,
verifies the existence of a $HOME/.calmar directory,
creates a JSON file with the given 'name' under $HOME/.calmar.
If file named 'name' already exists, it asks the user for confirmation.
 */
/// Create a new calendar and save it to the calendar index.
pub fn cal(split_input: &Vec<&str>) {
    let mut index = calendar_index!();

    if split_input.len() == 1 {
        let mut new_ref = get_new_calendar_reference(None);
        add_entry(&mut index, &new_ref);
        if index.calendars().is_empty() {
            new_ref.set_active()
        }
        if let Err(e) = new_ref.create_file() {
            print_err_msg(e, new_ref.path())
        }
    } else {
        split_input[1..].iter().for_each(|n| {
            let mut new_ref = get_new_calendar_reference(Some(n.to_string()));
            if index.calendars().is_empty() {
                new_ref.set_active()
            }
            add_entry(&mut index, &new_ref);
            if let Err(e) = new_ref.create_file() {
                print_err_msg(e, new_ref.path())
            }
        })
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
        return;
    };
}

/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = calendar_index!();

    if split_input.len() == 1 {
        delete_entry(&mut index, get_valid_calendar_name());
    } else {
        split_input[1..]
            .iter()
            .for_each(|n| delete_entry(&mut index, n.to_string()));
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
    }
}

/// Delete events from the active calendar
pub fn remove(split_input: &Vec<&str>) {
    let mut active_calendar = active_calendar!();
    let path = active_calendar_reference!().path();

    if split_input.len() == 1 {
        active_calendar
            .events_mut()
            .retain(|e| e.name() != get_valid_event_name());
    } else {
        active_calendar
            .events_mut()
            .retain(|e| !split_input[1..].contains(&e.name().as_str()));
    }
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let mut index = calendar_index!();
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!(
                "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    match index.num_named(&name) {
        0 => {
            warning(format!("No calendars named {name}"));
            return;
        }
        1 => {}
        x => {
            warning(format!("{x} calendars named {name}. There must be only one"));
            return;
        }
    }

    match index.number_of_active_calendars() {
        0 | 1 => {
            index.set_active(name);
            if let Err(e) = index.save() {
                print_err_msg(e, &CONFIG.index_path);
            };
        }
        _ => {
            warning(
                "More than one calendar is set as active. Please correct this and retry."
                    .to_string(),
            );
        }
    }
}

/*
Call event creation with name given optionally
 */
/// Create a new event and save it to the active calednar.
pub fn add(split_input: &Vec<&str>) {
    let mut active_calendar = active_calendar!();
    if split_input.len() == 1 {
        active_calendar.add_event(get_new_event(None));
    } else {
        split_input[1..].iter().for_each(|n| {
            if CONFIG.print_success_messages {
                success(format!("Adding {n}"))
            }
            active_calendar.add_event(get_new_event(Some(n.to_string())));
        })
    }

    let path = active_calendar_reference!().path().clone();
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &[&str]) {
    split_input[1..].iter().for_each(|e| edit_event(e))
}

/// Display events in the active calendar
pub fn raw(split_input: &[&str]) {
    active_calendar!()
        .events()
        .iter()
        .filter(|e| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&e.name().as_str())
            } else {
                true
            }
        })
        .for_each(|e| println!("{e}"))
}

/// Clear the screen
pub fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            warning(format!(
                "clear: Invalid number of arguments. Expected: 0. Got: {}",
                split_input.len() - 1
            ));
        }
    }
}

// List calendars and their properties
pub fn listcal(split_input: &Vec<&str>) {
    calendar_index!()
        .calendars()
        .iter()
        .filter(|r| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&r.name().as_str())
            } else {
                true
            }
        })
        .for_each(|r| println!("{r}"))
}

pub fn sort(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let mut active_calendar = active_calendar!(index);
    let active_calendar_reference = active_calendar_reference!(index);

    if !(1..=3).contains(&split_input.len()) {
        warning(format!(
            "sort: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        ));
        return;
    }

    let mut events_std: Vec<Event> = active_calendar.events().to_vec();

    match split_input.len() {
        1 => {
            events_std.sort();
            println!("Sorted normally");
        }
        _ => match split_input[1].trim() {
            "name" => events_std.sort_by_key(|e| e.name()),
            "start" => events_std.sort_by_key(|e| e.start()),
            "end" => events_std.sort_by_key(|e| e.end()),
            "priority" => events_std.sort_by_key(|e| e.priority()),
            "difficulty" => events_std.sort_by_key(|e| e.difficulty()),
            _ => {
                warning(format!("sort: {} is not a valid key.", { split_input[1].trim() }));
                return;
            }
        },
    }

    match split_input.get(2) {
        Some(arg) => match arg.trim() {
            "ascending" | "asc" | "a" => {}
            "descending" | "desc" | "d" | "rev" | "reverse" => events_std.reverse(),
            _ => {
                warning(format!("sort: {} is not a valid ordering argument", split_input[2]));
                return;
            }
        },
        None => {}
    }

    active_calendar.set_events(events_std);
    if let Err(e) = active_calendar.save(&active_calendar_reference.path()) {
        print_err_msg(e, active_calendar_reference.path());
    }
}

pub fn duration(split_input: &Vec<&str>) {
    let active_calendar = active_calendar!();

    let name_arr = match split_input.len() {
        1 => {
            vec![get_valid_event_name()]
        }
        _ => split_input[1..].iter().map(|a| a.to_string()).collect(),
    };

    active_calendar.events().iter().for_each(|e| {
        if name_arr.contains(&e.name()) {
            println!("Duration of {}: {}", e.name(), duration_fmt(e.duration()))
        }
    })
}

pub fn until(split_input: &Vec<&str>) {
    let active_calendar = active_calendar!();

    let name_arr = match split_input.len() {
        1 => {
            vec![get_valid_event_name()]
        }
        _ => split_input[1..].iter().map(|a| a.to_string()).collect(),
    };

    active_calendar.events().iter().for_each(|e| {
        if name_arr.contains(&e.name()) {
            let now = Local::now();
            if now < e.start() {
                println!("Until {}: {}", e.name(), duration_fmt(e.start() - now))
            } else {
                println!("{} started {} ago", e.name(), duration_fmt((e.start() - now).neg()))
            }
        }
    });

    for name in &name_arr {
        if !active_calendar
            .events()
            .iter()
            .map(|e| e.name())
            .any(|x| x == *name)
        {
            warning(format!("No event named {}", name))
        }
    }
}

/// Generate and view
pub fn list(split_input: &Vec<&str>) {
    let mut span = parse_into_duration(&CONFIG.default_calendar_span);

    let active_calendar = active_calendar!();

    match split_input.len() {
        1 => {}
        2 => {
            if validate_duration(split_input[1]) {
                span = parse_into_duration(split_input[1]);
            } else {
                warning(format!("{} is not a valid duration input.", split_input[1]));
                return;
            }
        }
        _ => warning(format!(
            "list: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        )),
    }

    let re_days = regex::Regex::new("^[0-9]+(d| +d|days| +days)$").unwrap();

    // if the user typed something like '3d', round the duration
    // to full days for convenience
    let mut end_date = get_now_even() + span;
    if split_input.len() == 2 && re_days.is_match(split_input[1]) {
        end_date = round_to_full_day(end_date);
    }

    generate_until(active_calendar, end_date)
        .iter()
        .for_each(|e| println!("{e}"))
}

/// Generate, output to a file
pub fn write(split_input: &Vec<&str>) {
    // write filename - default span
    // write 10h filename

    let filename: String;
    let span: Duration;

    match split_input.len() {
        2 => {
            span = parse_into_duration(&CONFIG.default_calendar_span);
            filename = split_input[1].to_string();
        }
        3 => {
            span = parse_into_duration(split_input[1]);
            filename = split_input[2].to_string();
        }
        _ => {
            warning(format!(
                "write: Invalid number of arguments. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    }

    let current_dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            error(format!("Failed to get current directory.\n{e}"));
            return;
        }
    };

    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(current_dir.join(&filename))
    {
        Ok(f) => f,
        Err(e) => {
            error(format!("Failed to create file {}.\n{e}", current_dir.join(filename).display()));
            return;
        }
    };

    let re_days = regex::Regex::new("^[0-9]+(d| +d|days| +days)$").unwrap();

    // if the user typed something like '3d', round the duration
    // to full days for convenience
    let mut end_date = get_now_even() + span;
    if re_days.is_match(split_input[1]) {
        end_date = round_to_full_day(end_date);
    }

    let calendar = active_calendar!();

    let gen_events = generate_until(calendar, end_date);

    gen_events.iter().for_each(|e| {
        if let Err(e) = writeln!(&mut file, "{e}") {
            error(format!("Failed to write to file {filename}.\n{e}"));
        }
    })
}

pub fn date() {
    println!("{}", Local::now().date_naive())
}

pub fn time() {
    println!("{}", Local::now().time().format("%H:%M:%S"))
}

pub fn update() {
    let index = calendar_index!();
    let mut active_calendar = active_calendar!(index);
    let path = active_calendar_reference!(index).path().clone();
    let now = get_now_even();

    // Set time of recurring events to their nearest occurence
    for event in active_calendar.events_mut() {
        if !event.repeat().is_zero() {
            let new_start = closest_occurence_start(event);
            let duration = event.duration();
            event.set_start(&new_start);
            event.set_end(&(new_start + duration));
        }
    }

    // retain only events that are recurring or they will end in the future.
    // this retains events currently happening
    active_calendar
        .events_mut()
        .retain(|e| !e.repeat().is_zero() || (e.end() > now && e.repeat().is_zero()));

    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path);
    }
}

pub fn mkindex() {
    if PathBuf::from_str(&&CONFIG.index_path).unwrap().exists() {
        warning("This will revert your index.json to its default contents. Proceed?".to_string());
        match get_input("[y/N]: ").to_lowercase().trim() {
            "yes" | "y" => {}
            _ => return,
        }
    }

    let new_index_json = match serde_json::to_string_pretty(&CalendarIndex::new()) {
        Ok(s) => s,
        Err(e) => {
            print_err_msg(CalmarError::ToJSON { e }, &"".to_string());
            return;
        }
    };
    let path = PathBuf::from_str(&CONFIG.index_path).unwrap();
    let path_str = path.to_str().unwrap();
    let mut file = match OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&path_str)
    {
        Ok(f) => f,
        Err(e) => {
            print_err_msg(CalmarError::CreateFile { e }, path_str);
            return;
        }
    };

    if let Err(e) = file.write(new_index_json.as_bytes()) {
        print_err_msg(CalmarError::WriteFile { e }, path_str);
    }
}

pub fn mkconfig() {
    if get_home_dir().join(".config/calmar/config.json").exists() {
        warning("This will revert your config.json to its default contents. Proceed?".to_string());
        match get_input("[y/N]: ").to_lowercase().trim() {
            "yes" | "y" => {}
            _ => return,
        }
    }

    let new_config = Config::default();
    let path = get_home_dir().join(".config/calmar/config.json");

    let mut file = match OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(e) => {
            print_err_msg(CalmarError::CreateFile { e }, &path.to_str().unwrap().to_string());
            return;
        }
    };

    let new_config_json = match serde_json::to_string_pretty(&new_config) {
        Ok(s) => s,
        Err(e) => {
            print_err_msg(CalmarError::ToJSON { e }, &"".to_string());
            return;
        }
    };
    if let Err(e) = file.write(new_config_json.as_bytes()) {
        print_err_msg(CalmarError::WriteFile { e }, &path.to_str().unwrap().to_string())
    }
}

pub fn update_index() {
    let mut index = calendar_index!();
    index
        .calendars_mut()
        .retain(|r| Path::new(&r.path()).exists());
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
    }
}
