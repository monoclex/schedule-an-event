#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate askama;

use rocket::uri;
use askama::Template;
use std::time::{Duration, SystemTime};

#[derive(Template)]
#[template(path = "schedule.html")]
struct ScheduleTemplate {
}

#[get("/")]
fn schedule() -> ScheduleTemplate {
    ScheduleTemplate {
    }
}

#[derive(Template)]
#[template(path = "event.html")]
struct EventTemplate {
    event_name: String,
    friendly_time_string: String,
    unix_epoch_time_offset: u64,
}

#[get("/event/<name>?<time>")]
fn event(mut name: String, time: u64) -> EventTemplate {
    // rewrite 'name' so that all - become spaces (and vice versa)
    unsafe {
        // UTF-8 guarantees that all valid ASCII (spaces, dashes) are NOT part of multi byte sequences
        // this is entirely safe and fast and allocation free.
        for mut char in name.as_mut_vec() {
            // dash
            if *char == b'-' {
                *char = b' ';
            }
            
            // space
            else if *char == b' ' {
                *char = b'-';
            }
        }
    }

    let friendly_time_string = get_time(time);

    let context = EventTemplate {
        event_name: name,
        friendly_time_string: friendly_time_string,
        unix_epoch_time_offset: time
    };

    context
}

// TODO: rename/clean up these functions

fn get_time(event_time_ms: u64) -> String {
    let start_time = get_event_secs(event_time_ms);
    let mut time = if start_time < 0 { -start_time } else { start_time };

    let seconds = time % 60;
    time -= seconds;
    time /= 60;

    let minutes = time % 60;
    time -= minutes;
    time /= 60;

    let hours = time % 24;
    time -= hours;
    time /= 24;

    let days = time;

    if start_time < 0 {
        return format!("-{} days, -{:0>2}:{:0>2}:{:0>2} left.", days, hours, minutes, seconds);
    } else {
        return format!("{} days, {:0>2}:{:0>2}:{:0>2} left.", days, hours, minutes, seconds);
    }
}

fn get_event_secs(event_time_ms: u64) -> i128 {
    let event_time = event_time_since_epoch(event_time_ms);

    let time_until_event = event_time.duration_since(SystemTime::now());

    if let Ok(duration) = time_until_event {
        // event *is* in the future
        return duration.as_secs() as i128;
    } else {
        // event was in the past
        let time_since_event = SystemTime::now().duration_since(event_time);

        match time_since_event {
            Ok(duration) => {
                return -(duration.as_secs() as i128);
            },
            _ => panic!("wtf")
        };
    }
}

fn event_time_since_epoch(event_time_ms: u64) -> SystemTime {
    let event_time = SystemTime::UNIX_EPOCH.checked_add(Duration::from_millis(event_time_ms));

    if let None = event_time {
        SystemTime::UNIX_EPOCH
    } else {
        event_time.unwrap()
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![schedule, event])
        .launch();
}