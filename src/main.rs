mod catch;
mod clean;
mod config;
mod git;
mod history;
mod init;
mod install;
mod migrate;
mod package;
mod package_manager;
mod package_manager_impl;
mod repo;

use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use log::error;
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use std::path::PathBuf;

fn public_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("emplace")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .after_help("https://github.com/tversteeg/emplace")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("install")
                .about("Install the packages that have been mirrored from other machines"),
        )
        .subcommand(SubCommand::with_name("clean").about("Remove package synching"))
}

fn safe_main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .context("no interactive terminal")?;

    let matches = public_clap_app()
		.subcommand(
			SubCommand::with_name("init")
				.about("Prints the shell function used to execute emplace")
				.arg(
					Arg::with_name("shell")
						.value_name("SHELL")
						.help(
							"The name of the currently running shell\nCurrently supported options: bash & zsh",
						)
						.required(true)
				)
		)
		.subcommand(
			SubCommand::with_name("catch")
				.about("Capture a command entered in a terminal")
				.arg(
					Arg::with_name("line")
						.value_name("LINE")
						.help("The command as entired in the terminal")
						.required(true),
				),
		)
		.subcommand(
			SubCommand::with_name("history")
				.about("Parses your history file and retrieves installations")
				.arg(
					Arg::with_name("history_parse")
						.help("Parses history. Just place `$HISTFILE` as input, and it will do all work;")
						.required(false)
						.takes_value(true)
				),
		)
		.get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").context("shell name is missing")?;

            init::init_main(shell_name)
        }
        ("catch", Some(sub_m)) => {
            let line = sub_m.value_of("line").context("line is missing")?;

            catch::catch(line).context("catching a command")
        }
        ("install", Some(_)) => install::install().context("installing packages"),
        ("clean", Some(_)) => clean::clean().context("cleaning packages"),
        ("history", Some(_)) => {
            let hist_path = PathBuf::from(
                &matches
                    .value_of("history_parse")
                    .context("path to history file is not provided")?,
            );

            history::history(&hist_path).context("capturing history")
        }
        (&_, _) => Ok(()),
    }
}

fn main() {
    if let Err(err) = safe_main() {
        error!("Critical Emplace error while {:?}", err);
    }
}
