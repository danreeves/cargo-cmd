#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate structopt;
extern crate subprocess;
extern crate toml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process;
use structopt::StructOpt;
use subprocess::{Exec, ExitStatus};

#[derive(StructOpt, Debug)]
#[structopt(name = "cargo-cmd", bin_name = "cargo")]
enum Cli {
    #[structopt(name = "cmd")]
    Cmd {
        #[structopt(name = "command", index = 1)]
        command: String,
        #[structopt(multiple = true)]
        rest: Vec<String>,
    },
}

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
    let cli = Cli::from_args();
    let (command, rest) = match cli {
        Cli::Cmd { command, rest } => (command, rest),
    };
    let commands = unwrap_or_exit(get_commands(&command));
    let is_multiple_commands = commands.len() > 1;

    for (index, command) in commands.iter().enumerate() {
        if is_multiple_commands {
            println!("\n[{}]", &command.0);
        }
        let command = &command.1;
        let exit = execute_command(command, &rest);

        if exit.success() {
            if index == commands.len() {
                process::exit(0);
            }
        } else {
            match exit {
                ExitStatus::Exited(exit_code) => process::exit(exit_code as i32),
                _ => process::exit(1),
            }
        }
    }
}

fn execute_command(command: &str, rest: &Vec<String>) -> ExitStatus {
    // This is naughty but Exec::shell doesn't let us do it with .args because
    // it ends up as an argument to sh/cmd.exe instead of our user command
    // or escaping things weirdly.
    let command = format!("{} {}", command, rest.join(" "));
    println!("> {}", command);
    let sh = Exec::shell(command);
    sh.join().unwrap_or(ExitStatus::Exited(0))
}

fn unwrap_or_exit<T>(result: Result<T, String>) -> T {
    match result {
        Err(error_msg) => {
            clap::Error::with_description(&error_msg[..], clap::ErrorKind::InvalidValue).exit();
        }
        Ok(thing) => thing,
    }
}

fn get_commands(command: &str) -> Result<Vec<(String, String)>, String> {
    let mut cargo_toml = File::open("Cargo.toml").or(Err(
        "Could not find or open Cargo.toml in the current directory",
    ))?;
    let mut cargo_str = String::new();
    let mut commands = vec![];
    let names = vec![
        format!("pre{}", command),
        command.to_string(),
        format!("post{}", command),
    ];

    cargo_toml
        .read_to_string(&mut cargo_str)
        .or(Err("Could not read the contents of Cargo.toml"))?;

    let cargo_toml: Cargotoml =
        toml::from_str(&cargo_str[..]).or(Err("Could not find commands in Cargo.toml"))?;

    let cargo_commands = cargo_toml.package.metadata.commands;

    for name in names {
        let command_to_run = &cargo_commands.get(&name);

        if name == command && command_to_run.is_none() {
            return Err(format!("Command \"{}\" not found in Cargo.toml", &command));
        }

        if command_to_run.is_some() {
            commands.push((name, command_to_run.unwrap().to_string()));
        }
    }

    Ok(commands)
}
