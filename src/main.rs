#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate askama;

use askama::Template;
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

fn main() {
    rocket::ignite()
        .mount("/", routes![schedule, event, event_without_name])
        .launch();
}