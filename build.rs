// https://github.com/rust-lang/cargo/issues/6833#issue-430806770
extern crate failure;
extern crate which;

use failure::Fallible;

use std::env;
use std::fs::copy;
use std::path::Path;
use std::process::Command;

use which::which;

fn run<F>(name: &str, mut configure: F) -> Fallible<()>
where
    F: FnMut(&mut Command) -> &mut Command,
{
    let mut command = Command::new(name);
    let configured = configure(&mut command);
    if !configured.spawn()?.wait()?.success() {
        panic!("failed to execute {:?}", configured);
    }
    Ok(())
}

fn build_client() -> Fallible<()> {
    let client = Path::new("./website-src");
    let current_dir = env::current_dir()?;
    env::set_current_dir(&client)?;

    let npm_path = which("npm")?;
    run(npm_path.to_str().unwrap(), |command| command.arg("run").arg("build"))?;

    env::set_current_dir(current_dir)?;
    Ok(())
}

fn copy_to_templates_directory() -> Fallible<()> {
    copy(Path::new("./website-src/dist/event.html"), Path::new("./templates/event.html"))?;
    copy(Path::new("./website-src/dist/schedule.html"), Path::new("./templates/schedule.html"))?;

    Ok(())
}

fn main() -> Fallible<()> {
    println!("cargo:rerun-if-changed=website-src/src/");

    build_client()?;
    copy_to_templates_directory()?;

    Ok(())
}