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

use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use bugreport::{
    bugreport,
    collector::{CommandOutput, CompileTimeInformation, EnvironmentVariables, SoftwareVersion},
    format::Markdown,
};
use camino::Utf8PathBuf;
use clap::{Arg, ArgAction, ColorChoice, Command};
use log::error;
use simplelog::{ColorChoice as LogColorChoice, LevelFilter, TermLogger, TerminalMode};
use std::path::PathBuf;

fn public_clap_app() -> Command {
    clap::command!()
        .arg_required_else_help(true)
        // Print the help in colors if the terminal allows it
        .color(ColorChoice::Auto)
        .subcommand(
            Command::new("install")
                .about("Install the packages that have been mirrored from other machines")
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Don't prompt the user and try to install everything")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("clean").about("Remove package synching"))
        .arg(
            Arg::new("config-path")
                .short('c')
                .help("The location of the configuration file")
                .required(false)
                .global(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(Utf8PathBuf))
                .env("EMPLACE_CONFIG"),
        )
}

fn safe_main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        LogColorChoice::Auto,
    )
    .context("no interactive terminal")?;

    let matches = public_clap_app()
		.subcommand(
			Command::new("init")
				.about("Prints the shell function used to execute emplace")
				.arg(
					Arg::new("shell")
						.value_name("SHELL")
						.help(
							"The name of the currently running shell\nCurrently supported options: bash, nu, fish & zsh",
						)
						.required(true)
                        .action(ArgAction::Set)
				)
		)
		.subcommand(
			Command::new("catch")
				.about("Capture a command entered in a terminal")
				.arg(
					Arg::new("line")
						.value_name("LINE")
						.help("The command as entired in the terminal")
						.required(true)
                        .action(ArgAction::Set)
				),
		)
		.subcommand(
			Command::new("history")
				.about("Parses your history file and retrieves installations")
				.arg(
					Arg::new("history_file")
                        .value_name("PATH")
						.help("Path to shell history file")
						.required(true)
						.value_parser(clap::value_parser!(Utf8PathBuf))
                        .action(ArgAction::Set)
				)
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Don't prompt the user and select everything")
                        .action(ArgAction::SetTrue)
                ),
		)
        .subcommand(
            Command::new("config")
            .about("Provides options for managing configuration")
            .arg(
                Arg::new("new")
                .short('n')
                .long("new")
                .help("Create a new config")
                .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("path")
                .short('p')
                .long("path")
                .help("Print out path to config")
                .action(ArgAction::SetTrue)
            ),
        )
        .subcommand(
            Command::new("bugreport")
            .about("Collect and print information that can be send along with a bug report")
        )
		.get_matches();

    let config_path: Utf8PathBuf = matches
        .get_one("config-path")
        .cloned()
        .unwrap_or_else(Config::default_path);

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let shell_name: &String = sub_m.get_one("shell").context("shell name is missing")?;

            init::init_main(&config_path, shell_name)
        }
        Some(("catch", sub_m)) => {
            let line: &String = sub_m.get_one("line").context("line is missing")?;

            catch::catch(config_path, line).context("catching a command")
        }
        Some(("install", sub_m)) => {
            install::install(config_path, sub_m.get_flag("yes")).context("installing packages")
        }
        Some(("clean", _)) => clean::clean(config_path).context("cleaning packages"),
        Some(("history", sub_m)) => {
            let hist_path: &Utf8PathBuf = sub_m
                .get_one("history_file")
                .context("path to history file is not provided")?;

            history::history(config_path, hist_path, sub_m.get_flag("yes"))
                .context("capturing history")
        }
        // Config subcommand, if path is present and new is not
        // it will just print the default path for the config file,
        // otherwise it will create a new config and ask what to do about the repository
        Some(("config", subm)) => {
            if subm.get_flag("path") && !subm.get_flag("new") {
                println!("Your config path is {config_path}");

                Ok(())
            } else {
                let mut config = config::Config::new(config_path)?;
                config.clone_repo_ask()?;

                Ok(())
            }
        }
        // Print information that can be used in bug report tickets
        Some(("bugreport", _)) => {
            bugreport!()
                .info(SoftwareVersion::default())
                .info(EnvironmentVariables::list(&["SHELL", "EMPLACE_CONFIG"]))
                .info(CommandOutput::new("Git version", "git", &["--version"]))
                .info(CompileTimeInformation::default())
                .print::<Markdown>();

            Ok(())
        }
        Some((other, _)) => Err(anyhow!("Unrecognized subcommand \"{}\"", other)),
        None => Ok(()),
    }
}

fn main() {
    if let Err(err) = safe_main() {
        error!("Critical Emplace error while {:?}", err);
    }
}
