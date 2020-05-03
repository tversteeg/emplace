mod config;
mod git;
mod init;
//mod install;
mod catch;
mod package;
mod package_manager;
mod package_manager_impl;
mod repo;

use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use simplelog::{LevelFilter, TermLogger, TerminalMode};

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

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .context("No interactive terminal")?;

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
            let shell_name = sub_m.value_of("shell").context("Shell name is missing.")?;

            init::init_main(shell_name)
        }
        ("catch", Some(sub_m)) => {
            let line = sub_m.value_of("line").context("Line is missing")?;

            catch::catch(line)
        }
        /*
        ("install", Some(_)) => {
            // Get the config
            let config = match Config::from_default_file().expect("Retrieving config went wrong") {
                Some(config) => config,
                None => Config::new().expect("Initializing new config failed"),
            };

            let repo = Repo::new(config).expect("Could not initialize git repository");

            match repo.read() {
                Ok(mut packages) => {
                    // Set all the package names, this is not done in catch for performance reasons
                    packages
                        .iter_mut()
                        .for_each(|package| package.set_package_name());

                    if let Err(err) = crate::install::install(packages) {
                        error!("Could not install new changes: {}.", err);
                    }
                }
                Err(err) => error!("{}", err),
            };
        }
        ("clean", Some(_)) => {
            // Get the config
            let config = match Config::from_default_file().expect("Retrieving config went wrong") {
                Some(config) => config,
                None => Config::new().expect("Initializing new config failed"),
            };

            let repo = Repo::new(config).expect("Could not initialize git repository");

            match repo.read() {
                Ok(mut packages) => {
                    // Set all the package names, this is not done in catch for performance reasons
                    packages
                        .iter_mut()
                        .for_each(|package| package.set_package_name());
                    match crate::install::clean(packages) {
                        Ok(packages) => repo.clean(packages).expect("Could not clean repo."),
                        Err(err) => error!("Could not remove from repository: {}.", err),
                    }
                }
                Err(err) => error!("{}", err),
            };
        }
        ("history", Some(path)) => history_processing(path)?,
        */
        (&_, _) => Ok(()),
    }
}

/*
fn history_processing(matches: &ArgMatches) -> Result<()> {
    let histpath = PathBuf::from(
        &matches
            .value_of("history_parse")
            .expect("Path to history file is not provided"),
    );
    let hist_file = File::open(histpath)?;
    let reader = BufReader::new(hist_file);
    let catches: Vec<Package> = reader
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.split_whitespace().join(" "))
        .sorted()
        .dedup()
        .map(|x| catch::catch(&x))
        .filter_map(|x| x.ok())
        .map(|x| x.0)
        .flatten()
        .sorted()
        .dedup()
        .collect();
    if catches.is_empty() {
        return Ok(());
    };
    // Filter out the packages that are already in the repository
    // Get the config
    let config = match Config::from_default_file().expect("Retrieving config went wrong") {
        Some(config) => config,
        None => Config::new().expect("Initializing new config failed"),
    };
    let repo = Repo::new(config).expect("Could not initialize git repository");
    let mut catches = Packages::from(catches);
    catches.filter_saved_packages(
        &repo
            .read()
            .expect("Could not read packages file from repository"),
    );

    let len = catches.0.len();
    if len == 0 {
        // Nothing found after filtering
        return Ok(());
    }
    let colored_selection: Vec<String> = catches.0.iter().map(|x| x.colour_full_name()).collect();
    let mut select_style = dialoguer::theme::ColorfulTheme::default();
    select_style.active_item_style = console::Style::underlined(select_style.active_item_style);
    let ms = dialoguer::MultiSelect::with_theme(&select_style)
        .items(&colored_selection)
        .with_prompt("Select packages to sync")
        .paged(true)
        .interact()
        .expect("Failed creating dialog");
    let mut checked = vec![];
    ms.iter().for_each(|x| checked.push(catches.0[*x].clone()));
    let len = checked.len();
    if len == 0 {
        // Nothing found after filtering
        info!("Nothing is checked");
        return Ok(());
    }
    // Print the info
    match len {
        1 => info!("{}", "Mirror this command?".green().bold()),
        n => info!("{}", format!("Mirror these {} commands?", n).green().bold()),
    }
    for catch in checked.iter() {
        info!("- {}", catch.colour_full_name());
    }

    if !dialoguer::Confirm::new()
        .interact()
        .expect("Could not create dialogue")
    {
        // Exit, we don't need to do anything
        return Ok(());
    }

    repo.mirror(Packages::from(checked))
        .expect("Could not mirror commands");
    Ok(())
}

fn catch_processing(mut catches: Packages) -> Result<()> {
    if catches.0.is_empty() {
        // Nothing found, just return
        return Ok(());
    }

    // Filter out the packages that are already in the repository
    // Get the config
    let config = match Config::from_default_file().expect("Retrieving config went wrong") {
        Some(config) => config,
        None => Config::new().expect("Initializing new config failed"),
    };
    let repo = Repo::new(config).expect("Could not initialize git repository");
    catches.filter_saved_packages(
        &repo
            .read()
            .expect("Could not read packages file from repository"),
    );

    let len = catches.0.len();
    if len == 0 {
        // Nothing found after filtering
        return Ok(());
    }

    // Print the info
    match len {
        1 => info!("{}", "Mirror this command?".green().bold()),
        n => info!("{}", format!("Mirror these {} commands?", n).green().bold()),
    }
    for catch in catches.0.iter() {
        info!("- {}", catch.colour_full_name());
    }

    // Ask if it needs to be mirrored
    if !dialoguer::Confirm::new()
        .interact()
        .expect("Could not create dialogue")
    {
        // Exit, we don't need to do anything
        return Ok(());
    }

    repo.mirror(catches).expect("Could not mirror commands");
    Ok(())
}
*/
