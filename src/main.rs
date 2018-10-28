#![forbid(unsafe_code, bad_style, future_incompatible)] 
#![forbid(rust_2018_idioms, rust_2018_compatibility)] 
#![forbid(missing_debug_implementations)] 
#![forbid(missing_docs)]

#[macro_use]
extern crate serde_derive;
extern crate subprocess;
extern crate toml;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use subprocess::{Exec, ExitStatus};

#[derive(Deserialize, Debug)]
struct Cargotoml {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    commands: HashMap<String, String>,
}

fn main() {
    let cmds = exit_if_error(get_cmds());
    let cmd = exit_if_error(get_cmd(env::args()));

    let command_to_run = cmds.get(&cmd.0);

    if let Some(command) = command_to_run {
        println!("> {}", command);
        let exit = Exec::shell(command).join().unwrap();

        if exit.success() {
            process::exit(0);
        } else {
            match exit {
                ExitStatus::Exited(exit_code) => process::exit(exit_code as i32),
                _ => process::exit(1),
            }
        }
    } else {
        println!("Command \"{}\" not found in Cargo.toml", &cmd.0);
        process::exit(1);
    }
}

fn exit_if_error<T>(result: Result<T, String>) -> T {
    match result {
        Err(error_msg) => {
            println!("{}", error_msg);
            process::exit(1);
        }
        Ok(thing) => thing,
    }
}

fn get_cmds() -> Result<HashMap<String, String>, String> {
    let mut cargo_toml = File::open("Cargo.toml").or(Err(
        "Could not find or open Cargo.toml in the current directory",
    ))?;
    let mut cargo_str = String::new();

    cargo_toml
        .read_to_string(&mut cargo_str)
        .or(Err("Could not read the contents of Cargo.toml"))?;

    let cargo_toml: Cargotoml =
        toml::from_str(&cargo_str[..]).or(Err("Could not find commands in Cargo.toml"))?;

    let commands = cargo_toml.package.metadata.commands;

    Ok(commands)
}

fn get_cmd(mut args: env::Args) -> Result<(String, Option<Vec<String>>), String> {
    // Pop off the binary
    args.next();
    // Prop off the "cmd"
    args.next();

    let command = match args.next() {
        Some(string) => string,
        None => return Err("You must provide a command: cargo cmd [command]".to_string()),
    };

    // This is eventually let you pass in arguments to the command
    Ok((command, None))
}
