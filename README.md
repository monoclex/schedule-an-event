# schedule-an-event
[![Actions Status](https://github.com/SirJosh3917/schedule-an-event/workflows/Docker%20Image%20CI/badge.svg)](https://github.com/SirJosh3917/schedule-an-event/actions)

`schedule-an-event` is a simple tool designed to allow creators to schedule events for their platforms, wwithout *any* friction. Popular tools, such as [time and date](https://www.timeanddate.com/) require you to do things such as putting in your location just for a timezone. This is a minor inconvenience. Thus, I made schedule an event just for this purpose!

## Goals
Scheduling an event should not be a complicated thing. Neither should the process or tooling for it be complicated. The goals of this project is to have the service be minimally in the way of the user, and that it should be extremely simple. Child-like-ishly simple, even. Here are some requirements:
- The site should put scheduling an event, and viewing when an event will happen at the forefront of everything. Nothing takes higher priority than what the visitor is using the site for.
- Keep the page for scheduled events **light**. Under 5KiB, before gzipping. There's no good reason it should be large.
- The page for scheduling events shouldn't be too large either. No set requirements, but just keep it down.

## Contributing
Contributions are welcome! If you don't know what to work on, maybe [try your hand at the things in the milestones?](https://github.com/SirJosh3917/schedule-an-event/milestones)

```shell
git clone "https://github.com/SirJosh3917/schedule-an-event.git"
cd schedule-an-event

# if you have Docker
docker build .

# if you have rust installed:
# for debugging:
cargo run

# for release:
cargo build --release
```

[If you don't have rust installed, get it here.](https://rustup.rs/)

## Donations
Donations happily accepted! You can give me your money on [Patreon](https://www.patreon.com/sirjosh3917) if you'd like to.