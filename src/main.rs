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
    collector::{
        CommandOutput, CompileTimeInformation, EnvironmentVariables, OperatingSystem,
        SoftwareVersion,
    },
    format::Markdown,
};
use clap::{App, AppSettings, Arg};
use log::error;
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use std::path::PathBuf;

fn public_clap_app<'a>() -> App<'a> {
    App::new("emplace")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .after_help("https://github.com/tversteeg/emplace")
        // Use the order specified for the subcommands, not alphabetically
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // Print the help in colors if the terminal allows it
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new("install")
                .about("Install the packages that have been mirrored from other machines")
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .about("Don't prompt the user and try to install everything"),
                ),
        )
        .subcommand(App::new("clean").about("Remove package synching"))
        .arg(
            Arg::new("config-path")
                .short('c')
                .about("The location of the configuration file")
                .required(false)
                .global(true)
                .env("EMPLACE_CONFIG"),
        )
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
			App::new("init")
				.about("Prints the shell function used to execute emplace")
				.arg(
					Arg::new("shell")
						.value_name("SHELL")
						.about(
							"The name of the currently running shell\nCurrently supported options: bash, nu, fish & zsh",
						)
						.required(true)
				)
		)
		.subcommand(
			App::new("catch")
				.about("Capture a command entered in a terminal")
				.arg(
					Arg::new("line")
						.value_name("LINE")
						.about("The command as entired in the terminal")
						.required(true),
				),
		)
		.subcommand(
			App::new("history")
				.about("Parses your history file and retrieves installations")
				.arg(
					Arg::new("history_file")
                        .value_name("PATH")
						.about("Path to shell history file")
						.required(true)
				)
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .about("Don't prompt the user and select everything"),
                ),
		)
        .subcommand(
            App::new("config")
            .about("Provides options for managing configuration")
            .arg(
                Arg::new("new")
                .short('n')
                .long("new")
                .about("Create a new config")
                .takes_value(false)
            )
            .arg(
                Arg::new("path")
                .short('p')
                .long("path")
                .about("Print out path to config")
                .takes_value(false)
            ),
        )
        .subcommand(
            App::new("bugreport")
            .about("Collect and print information that can be send along with a bug report")
        )
		.get_matches();

    let config_path = matches
        .value_of_t("config-path")
        .unwrap_or_else(|_| Config::default_path());

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let shell_name = sub_m.value_of("shell").context("shell name is missing")?;

            init::init_main(config_path, shell_name)
        }
        Some(("catch", sub_m)) => {
            let line = sub_m.value_of("line").context("line is missing")?;

            catch::catch(config_path, line).context("catching a command")
        }
        Some(("install", sub_m)) => {
            install::install(config_path, sub_m.is_present("yes")).context("installing packages")
        }
        Some(("clean", _)) => clean::clean(config_path).context("cleaning packages"),
        Some(("history", sub_m)) => {
            let hist_path = PathBuf::from(
                &sub_m
                    .value_of("history_file")
                    .context("path to history file is not provided")?,
            );

            history::history(config_path, &hist_path, sub_m.is_present("yes"))
                .context("capturing history")
        }
        // Config subcommand, if path is present and new is not
        // it will just print the default path for the config file,
        // otherwise it will create a new config and ask what to do about the repository
        Some(("config", subm)) => {
            if subm.is_present("path") && !subm.is_present("new") {
                println!("Your config path is {}", config_path.to_str().unwrap());

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
                .info(OperatingSystem::default())
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
