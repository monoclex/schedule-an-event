#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate askama;

use askama::Template;
use std::time::{Duration, SystemTime};
use std::option::Option;

/// Defines a template with no values. This allows us to make use of the existing "template" infrastructure when
/// serving static files.
#[derive(Template)]
#[template(path = "schedule.html")]
struct ScheduleTemplate;

/// On the main path, we serve schedule.html
#[get("/")]
fn schedule() -> ScheduleTemplate {
    ScheduleTemplate {
    }
}

/// Defines a template for a given event.
#[derive(Template)]
#[template(path = "event.html")]
struct EventTemplate {
    event_name: String,

    /// This is the time of the event, exactly as the JS may format it. This is so that the user can go to the page,
    /// and immediately see the time the event is taking place without having to wait for the JS to update it.
    friendly_time_string: String,

    /// This is the time from the unix epoch that the event will take place. This is injected into the JS so that the
    /// JS will update it every second.
    unix_epoch_time_offset: u64,
}

// note: the `event` route, even with `Option<String>`, doesn't work for an optional route without the name, so i made
// another endpoint which will pass `Option::None` to the endpoint.
// see: https://rocket.rs/v0.4/guide/requests/#optional-parameters
#[get("/event?<time>")]
fn event_without_name(time: u64) -> EventTemplate {
    event(Option::None, time)
}

#[get("/event/<name>?<time>")]
fn event(name: Option<String>, time: u64) -> EventTemplate {
    let name = match name {
        Some(mut query_input) => {
            mutate_url_input(&mut query_input);
            query_input
        },
        None => "Unnamed Event".to_string(),
    };

    EventTemplate {
        event_name: name,
        friendly_time_string: get_time(time),
        unix_epoch_time_offset: time
    }
}

/// This mutates some query input into an event name.
/// For SEO purposes, this turns all spaces into hyphens and vice versa.
#[inline]
fn mutate_url_input(query_input: &mut String) {
    unsafe {
        for character in query_input.as_mut_vec() {
            // this is perfectly legal and safe
            // UTF-8 guarantees that no mutli character sequences have ASCII in them, so this is completely safe.
            *character = match *character {
                b' ' => b'-',
                b'-' => b' ',
                _ => *character,
            };
        }
    }
}

// TODO: rename/clean up these functions

#[inline]
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

    // TODO: this is ***nasty***
    if start_time < 0 {
        if days > 0 {
            return format!("-{} days, -{:0>2}:{:0>2}:{:0>2} left.", days, hours, minutes, seconds);
        } else {
            // -0 days doesn't make sense
            return format!("{} days, -{:0>2}:{:0>2}:{:0>2} left.", days, hours, minutes, seconds);
        }
    } else {
        return format!("{} days, {:0>2}:{:0>2}:{:0>2} left.", days, hours, minutes, seconds);
    }
}

#[inline]
fn get_event_secs(event_time_ms: u64) -> i128 {
    println!("running get_event_secs for: {}", event_time_ms);
    let event_time = event_time_since_epoch(event_time_ms);
    let now_time = SystemTime::now();

    // the event is in the past
    if now_time > event_time {
        let past = now_time.duration_since(event_time).unwrap();
        -(past.as_secs() as i128)
    }
    else if now_time < event_time {
        // event is in the future
        let future = event_time.duration_since(now_time).unwrap();
        future.as_secs() as i128
    }
    else {
        // event and now are exactly the same. return 0
        0
    }
}

/// This attemps to add the given time to the unix epoch to get a resulting time.
#[inline]
fn event_time_since_epoch(event_time_ms: u64) -> SystemTime {
    let duration = Duration::from_millis(event_time_ms);
    let event_time = SystemTime::UNIX_EPOCH.checked_add(duration);

    match event_time {
        // if we could add the time, we can return the time
        Some(time) => time,

        // if we couldn't add the time (unlikely to ever happen given that we're passing in a u64), return unix epoch
        None => SystemTime::UNIX_EPOCH,
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![schedule, event, event_without_name])
        .launch();
}