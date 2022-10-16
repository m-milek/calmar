use crate::{
    active_calendar, active_calendar_reference,
    cal::{
        calendar_index::CalendarIndex, calmar_error::CalmarError, calmar_trait::CalendarDataType,
        deadline::Deadline, event::Event,
    },
    calendar_index,
    cli::{
        config::Config,
        display::display_events,
        functions::{
            add_entry, closest_occurence_start, delete_entry, edit_calendar, edit_event,
            generate_until, get_new_calendar_reference, get_new_event,
        },
        getdata::{get_valid_calendar_name, get_valid_event_name, parse_into_duration},
        messages::print_err_msg,
        repl::get_input,
        util::{duration_fmt, get_now_even, round_to_full_day},
        validator::{get_home_dir, validate_duration},
    },
    error, success, warning, CONFIG,
};
use chrono::{Duration, Local};
use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::Write,
    ops::Neg,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::{
    display::colorize_deadline,
    functions::choose_struct_idx,
    getdata::{get_date, get_priority, get_time},
    util::select_in_range,
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
        if index.calendars().is_empty() {
            new_ref.set_active()
        }
        if let Err(e) = new_ref.create_file() {
            print_err_msg(e, new_ref.path());
        }
        add_entry(&mut index, &new_ref);
        success!("Added {}", new_ref.name());
    } else {
        for n in &split_input[1..] {
            if n.trim().is_empty() {
                warning!("Calendar name cannot be empty.");
                continue;
            }
            let mut new_ref = get_new_calendar_reference(Some(n.to_string()));
            if index.calendars().is_empty() {
                new_ref.set_active()
            }
            if let Err(e) = new_ref.create_file() {
                print_err_msg(e, new_ref.path());
                continue;
            }
            add_entry(&mut index, &new_ref);
            success!("Added {}", new_ref.name());
        }
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
    };
}

/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = calendar_index!();
    let names = index
        .calendars()
        .iter()
        .map(|r| r.name())
        .collect::<Vec<String>>();

    if split_input.len() == 1 {
        let name = get_valid_calendar_name();
        delete_entry(&mut index, name.clone());
        success!("Removed {name}");
    } else {
        split_input[1..].iter().for_each(|n| {
            if names.contains(&n.to_string()) {
                delete_entry(&mut index, n.to_string());
                success!("Removed {n}");
            } else {
                warning!("No calendard named {n} found");
            }
        })
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
    }
}

/// Delete events from the active calendar
pub fn remove(split_input: &Vec<&str>) {
    let mut active_calendar = active_calendar!();
    let path = active_calendar_reference!().path();
    let names = active_calendar
        .events()
        .iter()
        .map(|e| e.name())
        .collect::<Vec<String>>();

    if split_input.len() == 1 {
        let name = get_valid_event_name();
        active_calendar.events_mut().retain(|e| e.name() != name);
        success!("Removed {name}");
    } else {
        split_input[1..].iter().for_each(|n| {
            if !names.contains(&n.to_string()) {
                warning!("No event named {n}")
            } else {
                success!("Removed {n}");
            }
        });
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
            warning!(
                "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            );
            return;
        }
    };

    match index.num_named(&name) {
        0 => {
            warning!("No calendars named {name}");
            return;
        }
        1 => {}
        x => {
            warning!("{x} calendars named {name}. There must be only one");
            return;
        }
    }

    match index.number_of_active_calendars() {
        0 | 1 => {
            index.set_active(name.clone());
            success!("Set {name} as active");
            if let Err(e) = index.save() {
                print_err_msg(e, &CONFIG.index_path);
            };
        }
        _ => {
            warning!("More than one calendar is set as active. Please correct this and retry.");
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
        let new_event = get_new_event(None);
        active_calendar.add_event(new_event.clone());
        success!("Added {}", new_event.name());
    } else {
        split_input[1..].iter().for_each(|n| {
            // inform about what is being currently added when there are at least 2 event names passed
            if split_input.len() > 2 {
                success!("Adding {n}");
            }
            let new_event = get_new_event(Some(n.to_string()));
            active_calendar.add_event(new_event.clone());
            success!("Added {}", new_event.name());
        })
    }

    let path = active_calendar_reference!().path();
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &[&str]) {
    split_input[1..].iter().for_each(|e| {
        success!("Editing {e}");
        edit_event(e)
    })
}

/// Display events in the active calendar
pub fn raw(split_input: &[&str]) {
    let active_calendar = active_calendar!();
    let names: Vec<String> = active_calendar.events().iter().map(|e| e.name()).collect();
    split_input[1..].iter().for_each(|a| {
        if !names.contains(&a.to_string()) {
            warning!("No event named {a}")
        }
    });
    active_calendar
        .events()
        .iter()
        .filter(|e| {
            if split_input.len() != 1 {
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
            println!("\x1b[H\x1b[J");
        }
        _ => {
            warning!(
                "clear: Invalid number of arguments. Expected: 0. Got: {}",
                split_input.len() - 1
            );
        }
    }
}

// List calendars and their properties
pub fn listcal(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let names: Vec<String> = index.calendars().iter().map(|r| r.name()).collect();
    split_input[1..].iter().for_each(|a| {
        if !names.contains(&a.to_string()) {
            warning!("No calendar named {a}");
        }
    });

    index
        .calendars()
        .iter()
        .filter(|r| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&r.name().as_str())
            } else {
                true
            }
        })
        .for_each(|r| println!("{r}"));
}

pub fn sort(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let mut active_calendar = active_calendar!(index);
    let active_calendar_reference = active_calendar_reference!(index);

    if !(1..=3).contains(&split_input.len()) {
        warning!(
            "sort: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        );
        return;
    }

    let mut events_std: Vec<Event> = active_calendar.events().to_vec();

    match split_input.len() {
        1 => {
            events_std.sort();
            success!("Sorted by standard key");
        }
        _ => {
            match split_input[1].trim() {
                "name" => events_std.sort_by_key(|e| e.name()),
                "start" => events_std.sort_by_key(|e| e.start()),
                "end" => events_std.sort_by_key(|e| e.end()),
                "priority" => events_std.sort_by_key(|e| e.priority()),
                "difficulty" => events_std.sort_by_key(|e| e.difficulty()),
                _ => {
                    warning!("sort: {} is not a valid key.", { split_input[1].trim() });
                    return;
                }
            }
            success!("Sorted by non-standard key");
        }
    }

    match split_input.get(2) {
        Some(arg) => match arg.trim() {
            "ascending" | "asc" | "a" => {}
            "descending" | "desc" | "d" | "rev" | "reverse" => events_std.reverse(),
            _ => {
                warning!("sort: {} is not a valid ordering argument", split_input[2]);
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
    let names: Vec<String> = active_calendar.events().iter().map(|e| e.name()).collect();
    split_input[1..].iter().for_each(|a| {
        if !names.contains(&a.to_string()) {
            warning!("No event named {a}")
        }
    });
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
    let names: Vec<String> = active_calendar.events().iter().map(|e| e.name()).collect();
    split_input[1..].iter().for_each(|a| {
        if !names.contains(&a.to_string()) {
            warning!("No event named {a}")
        }
    });

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
            warning!("No event named {}", name)
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
                warning!("{} is not a valid duration input.", split_input[1]);
                return;
            }
        }
        _ => warning!(
            "list: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        ),
    }

    let re_days = regex::Regex::new("^[0-9]+(d| +d|days| +days)$").unwrap();

    // if the user typed something like '3d', round the duration
    // to full days for convenience
    let mut end_date = get_now_even() + span;
    if split_input.len() == 2 && re_days.is_match(split_input[1]) {
        end_date = round_to_full_day(end_date);
    }

    let events = generate_until(&active_calendar, end_date);
    display_events(events);
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
            warning!(
                "write: Invalid number of arguments. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            );
            return;
        }
    }

    let current_dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to get current directory.\n{e}");
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
            error!("Failed to create file {}.\n{e}", current_dir.join(filename).display());
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

    let gen_events = generate_until(&calendar, end_date);

    gen_events.iter().for_each(|e| {
        if let Err(e) = writeln!(&mut file, "{e}") {
            error!("Failed to write to file {filename}.\n{e}");
        }
    });
    success!("Wrote calendar until {end_date} to {filename}");
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
    let path = active_calendar_reference!(index).path();
    let now = get_now_even();

    let before = active_calendar.events().len();

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

    let after = active_calendar.events().len();
    println!("{after} {before}");
    success!("Removed {} old event/s", before - after);
    success!("Brought nearest event occurences up to date");
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path);
    }
}

pub fn mkindex() {
    if PathBuf::from_str(&CONFIG.index_path).unwrap().exists() {
        warning!("This will revert your index.json to its default contents. Proceed?");
        match get_input("[y/N]: ", None).to_lowercase().trim() {
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

    success!("Wrote new index.json to {}", CONFIG.index_path);

    if let Err(e) = file.write(new_index_json.as_bytes()) {
        print_err_msg(CalmarError::WriteFile { e }, path_str);
    }
}

pub fn mkconfig() {
    if get_home_dir().join(".config/calmar/config.json").exists() {
        warning!("This will revert your config.json to its default contents. Proceed?");
        match get_input("[y/N]: ", None).to_lowercase().trim() {
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

    success!("Wrote default config to {}", path.display());

    if let Err(e) = file.write(new_config_json.as_bytes()) {
        print_err_msg(CalmarError::WriteFile { e }, &path.to_str().unwrap().to_string())
    }
}

pub fn update_index() {
    let mut index = calendar_index!();
    let before = index.calendars().len();
    index
        .calendars_mut()
        .retain(|r| Path::new(&r.path()).exists());
    let after = index.calendars().len();
    success!("Removed {} where the file didn't exist", after - before);
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
    }
}

pub fn backup(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let mut i = 0;
    for reference in index.calendars() {
        if split_input.len() == 1 || (split_input[1..].contains(&reference.name().as_str()) && split_input.len() > 1)
        {
            if Path::new(&reference.path()).exists() {
                let backup_path = reference.path() + ".bak";
                match OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&backup_path)
                {
                    Ok(_) => {
                        if let Err(e) = std::fs::copy(reference.path(), &backup_path) {
                            error!(
                                "Failed to copy from {} to {}.\n{e}",
                                reference.path(),
                                backup_path
                            );
                        } else {
                            i += 1
                        }
                    }
                    Err(e) => {
                        error!(
                            "Cannot backup {}. Failed to open/create {}.\n{e}",
                            reference.name(),
                            backup_path
                        );
                    }
                }
            } else {
                error!(
                    "Cannot backup {}. File {} does not exist.",
                    reference.name(),
                    reference.path()
                );
            }
        }
    }
    success!("Backed up {i} calendar(s)");
}

pub fn edit_cal(split_input: &[&str]) {
    split_input[1..].iter().for_each(|e| {
        success!("Editing {e}");
        edit_calendar(e)
    });
}

/// Add an exception to a recurring event, for example when it is cancelled on a given day
pub fn except(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let mut active_calendar = active_calendar!(index);
    let path = active_calendar_reference!().path();
    let options = ["Add exception", "Remove exception"];

    split_input[1..].iter().for_each(|n| {
        if active_calendar.events().iter().all(|e| e.name() != **n) {
            warning!("No event named {n}");
        } else {
            let idx = match choose_struct_idx(
                active_calendar.events().to_vec(),
                "Select an event to except",
                n,
            ) {
                Some(i) => i,
                None => return,
            };
            let edited_event = &mut active_calendar.events_mut()[idx];
            options
                .iter()
                .enumerate()
                .for_each(|(i, o)| println!("{}. {o}", i + 1));
            let num = select_in_range("Select an option", options.len());
            match num {
                // add an exception
                1 => edited_event
                    .exceptions_mut()
                    .push(get_date("Date: ").and_time(get_time("Time: ")).unwrap()),
                // remove an exception
                2 => {
                    if edited_event.exceptions().is_empty() {
                        warning!("No exceptions");
                        return;
                    }
                    edited_event
                        .exceptions()
                        .iter()
                        .enumerate()
                        .for_each(|(i, e)| println!("{}. {e}", i + 1));
                    let len = edited_event.exceptions().len();
                    edited_event
                        .exceptions_mut()
                        .remove(select_in_range("Select an exception: ", len) - 1);
                }
                _ => panic!("Impossible, this should be checked in select_in_range"),
            }
        }
    });
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, path)
    }
}

pub fn deadline(split_input: &Vec<&str>) {
    let path = active_calendar_reference!().path();
    let mut active_calendar = active_calendar!();
    split_input[1..].iter().for_each(|n| {
        success!("Adding {n} deadline");
        active_calendar.add_deadline(Deadline::new(
            n.to_string(),
            get_date("Deadline date: ")
                .and_time(get_time("Deadline time: "))
                .unwrap(),
            get_priority(),
        ))
    });
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, path);
    }
}

// temporary solution probably, unitl display improves
pub fn ls_deadlines(split_input: &Vec<&str>) {
    let active_calendar = active_calendar!();
    let len = split_input.len();
    let mut x = active_calendar
        .deadlines()
        .iter()
        .filter(|a| {
            if len == 1 {
                true
            } else {
                split_input[1..].contains(&a.name().as_str())
            }
        })
        .collect::<Vec<&Deadline>>();
    x.sort_by(|d, o| d.date().cmp(&o.date()));
    x.iter().for_each(|d| println!("{}", colorize_deadline(d)))
}

pub fn remove_deadline(split_input: &Vec<&str>) {
    let mut active_calendar = active_calendar!();
    let path = &active_calendar_reference!().path();
    for a in split_input[1..].iter() {
        let idx = match choose_struct_idx(
            active_calendar.deadlines().to_vec(),
            "Select a deadline to remove",
            a,
        ) {
            Some(i) => i,
            None => {
                warning!("No deadline named {a}");
                continue;
            }
        };
        active_calendar.deadlines_mut().remove(idx);
    }
    if let Err(e) = active_calendar.save(path) {
        print_err_msg(e, path)
    }
}

pub fn briefing() {
    let cal = active_calendar!();
    let gen = generate_until(&cal, round_to_full_day(Local::now()+Duration::days(7)));
    println!("{}",
	     format!("{} and {}. {} in the next 7 days and {} in the next 14.",
		     format!("{:?} event(s) left today",
			     gen.iter().filter(|e| e.is_happening_on(Local::now()) || e.will_happen_today()).count()
		     ).bold(),
		     format!("{} tomorrow",
			     gen.iter().filter(|e| e.start().date() == (Local::now() + Duration::days(1)).date()).count()
		     ).bold(),
		     format!("{} deadline(s)",
			     cal.deadlines().iter().filter(|d| d.date().date() < round_to_full_day(Local::now()+Duration::days(7)).date()).count()
		     ).bold(),
		     format!("{}",
			     cal.deadlines().iter().filter(|d| d.date().date() < round_to_full_day(Local::now()+Duration::days(14)).date()).count()
		     ).bold()
	     )
    )
}
