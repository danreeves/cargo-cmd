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
    let (command, mut rest) = match cli {
        Cli::Cmd { command, rest } => (command, rest),
    };
    let cmds = exit_if_error(get_cmds(&command));
    let shell_command = cmds.get(&command);

    if let Some(command) = shell_command {
        println!("> {}", command);
        println!(
            "{:?}",
            Exec::shell(command).args(rest.iter_mut().into_slice())
        );
        let exit = Exec::shell(command)
            .args(rest.iter_mut().into_slice())
            .join()
            .unwrap();

        if exit.success() {
            process::exit(0);
        } else {
            match exit {
                ExitStatus::Exited(exit_code) => process::exit(exit_code as i32),
                _ => process::exit(1),
            }
        }
    } else {
        println!("Command \"{}\" not found in Cargo.toml", &command);
        process::exit(1);
    }
}

fn exit_if_error<T>(result: Result<T, String>) -> T {
    match result {
        Err(error_msg) => {
            clap::Error::with_description(&error_msg[..], clap::ErrorKind::HelpDisplayed).exit();
        }
        Ok(thing) => thing,
    }
}

fn get_cmds(command: &str) -> Result<HashMap<String, String>, String> {
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

    {
        let command_to_run = &commands.get(command);
        if command_to_run.is_none() {
            return Err(format!("Command \"{}\" not found in Cargo.toml", &command));
        }
    }

    Ok(commands)
}
