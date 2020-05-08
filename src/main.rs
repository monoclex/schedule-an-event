#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate askama;

use rocket::uri;
use askama::Template;

#[derive(Template)]
#[template(path = "schedule.html")]
struct ScheduleTemplate {
    event_url: String
}

#[get("/")]
fn schedule() -> ScheduleTemplate {
    ScheduleTemplate {
        event_url: uri!(event: "Feed the Chickens", 0).to_string()
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
fn event(name: String, time: u64) -> EventTemplate {
    let friendly_time_string = time.to_string();

    let context = EventTemplate {
        event_name: name,
        friendly_time_string: friendly_time_string,
        unix_epoch_time_offset: time
    };

    context
}

fn main() {
    rocket::ignite()
        .mount("/", routes![schedule, event])
        .launch();
}