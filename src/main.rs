mod catch;
mod clean;
mod config;
mod git;
mod history;
mod init;
mod install;
mod link;
mod migrate;
mod package;
mod package_manager;
mod package_manager_impl;
mod repo;

use anyhow::{anyhow, Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use log::error;
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use std::path::{Path, PathBuf};

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
        .subcommand(
            SubCommand::with_name("config")
            .about("Provides options for managing configuration")
            .arg(
                Arg::with_name("new")
                .short("n")
                .long("new")
                .help("Create a new config")
                .required_unless("path")
                .takes_value(false)
            )
            .arg(
                Arg::with_name("path")
                .short("p")
                .long("path")
                .help("Print out path to config")
                .required_unless("new")
                .takes_value(false)
            ),
        )
		.subcommand(
			SubCommand::with_name("link")
				.about("Automatically replace the file with a symbolic link in the repository, the link will be stored in the emplace configuration file")
				.arg(
					Arg::with_name("target_path")
						.help("Path of the file to replace with a symlink in the repository")
                        .required(true)
						.takes_value(true)
				)
                .arg(
                    Arg::with_name("repository_path")
                        .help("Relative path of the file in the repository")
                        .short("r")
                        .long("repository-path")
                        .takes_value(true)
                )
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
        ("link", Some(sub_m)) => {
            let target_path = Path::new(
                sub_m
                    .value_of("target_path")
                    .context("target path is missing")?,
            );
            // Get the path of the target file in the repository or use the target path filename as a default
            let repository_path = sub_m
                .value_of("repository_path")
                .map(|path| Path::new(path))
                .unwrap_or({
                    Path::new(target_path.file_name().ok_or_else(|| {
                        anyhow!(
                            "target path {:?} doesn't contain a proper file",
                            target_path
                        )
                    })?)
                });

            link::link(target_path, repository_path).context("linking file")
        }
        // Config subcommand, if path is present and new is not
        // it will just print the default path for the config file,
        // otherwise it will create a new config and ask what to do about the repository
        ("config", Some(subm)) => {
            if subm.is_present("path") && !subm.is_present("new") {
                println!(
                    "Your config path is {}",
                    config::Config::default_path().to_str().unwrap()
                );

                Ok(())
            } else {
                let mut config = config::Config::new()?;
                config.clone_repo_ask()?;

                Ok(())
            }
        }
        (&_, _) => Ok(()),
    }
}

fn main() {
    if let Err(err) = safe_main() {
        error!("Critical Emplace error while {:?}", err);
    }
}
