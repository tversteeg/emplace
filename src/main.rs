#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate failure;

mod init;
mod config;
mod catch;
mod authentication;
mod repo;

use clap::{App, AppSettings, Arg, SubCommand};

use config::Config;
use repo::Repo;

fn main() {
    let matches = App::new("emplace")
        .version(crate_version!())
        .author(crate_authors!())
        .after_help("https://github.com/tversteeg/emplace")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
            .about("Prints the shell function used to execute emplace")
            .arg(
                Arg::with_name("shell")
                .value_name("SHELL")
                .help(
                    "The name of the currently running shell\nCurrently supported options: bash",
                )
                .required(true)
            )
        )
        .subcommand(
            SubCommand::with_name("catch")
            .about("Capture a command entired in a terminal")
            .arg(
                Arg::with_name("line")
                .value_name("LINE")
                .help(
                    "The command as entired in the terminal",
                )
                .required(true)
            )
        )
        .subcommand(
            SubCommand::with_name("install")
            .about("Install the packages that have been mirrored from other machines")
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("Shell name is missing.");
            init::init_main(shell_name).expect("Could not initialize terminal script");
        },
        ("catch", Some(sub_m)) => {
            let line = sub_m.value_of("line").expect("Line is missing");
            let catches = catch::catch(line).expect("Could not parse line");

            let len = catches.0.len();

            // Nothing found, just return
            if len == 0 {
                return;
            }

            // Print the info
            match len {
                1 => println!("Mirror this command?"),
                n => println!("Mirror these {} commands?", n)
            }
            for catch in catches.0.iter() {
                println!("- {}", catch);
            }

            // Ask if it needs to be mirrored
            if !dialoguer::Confirmation::new()
                .interact()
                .expect("Could not create dialogue") {
                // Exit, we don't need to do anything
                return
            }

            // Get the config
            let config = match Config::from_default_file().expect("Retrieving config went wrong") {
                Some(config) => config,
                None => Config::new().expect("Initializing new config failed")
            };

            let repo = Repo::new(config).expect("Could not initialize git repository");

            repo.mirror(catches).expect("Could not mirror commands");
        },
        ("install", Some(_)) => {

        },
        (&_, _) => { }
    }
}
