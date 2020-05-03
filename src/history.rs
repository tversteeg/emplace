use crate::{
    config::Config,
    package::{Package, Packages},
    repo::Repo,
};
use anyhow::Result;
use colored::Colorize;
use console::Style;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Capture a history file.
pub fn history(path: &Path) -> Result<()> {
    let hist_file = File::open(path)?;
    let reader = BufReader::new(hist_file);

    let catches: Vec<Package> = reader
        // Get all the lines of the reader
        .lines()
        // Filter actual lines
        .filter_map(|x| x.ok())
        .map(|x| x.split_whitespace().join(" "))
        // Remove duplicates
        .sorted()
        .dedup()
        // Get an iterator over all the matches in the line
        .map(|x| Packages::from_line(&x))
        .map(|packages| packages.into_iter())
        // Make it a big list
        .flatten()
        .collect();

    if catches.is_empty() {
        return Ok(());
    };

    // Filter out the packages that are already in the repository

    // Get the config
    let config = Config::from_default_file_or_new()?;

    // Get the repository from the config
    let repo = Repo::new(config)?;

    let mut catches = Packages::from(catches);
    catches.filter_saved_packages(&repo.read()?);

    if catches.is_empty() {
        // Nothing found after filtering
        return Ok(());
    }

    let colored_selection: Vec<String> = catches.iter().map(|x| x.color_full_name()).collect();

    let mut select_style = ColorfulTheme::default();
    select_style.active_item_style = Style::underlined(select_style.active_item_style);
    let ms = MultiSelect::with_theme(&select_style)
        .items(&colored_selection)
        .with_prompt("Select packages to sync")
        .paged(true)
        .interact()?;

    let mut checked = vec![];

    ms.iter().for_each(|x| checked.push(catches[*x].clone()));

    let len = checked.len();
    if len == 0 {
        // Nothing found after filtering
        println!("Nothing is checked");

        return Ok(());
    }

    // Print the info
    match len {
        1 => println!("{}", "Mirror this command?".green().bold()),
        n => println!("{}", format!("Mirror these {} commands?", n).green().bold()),
    }
    for catch in checked.iter() {
        println!("- {}", catch.color_full_name());
    }

    if !dialoguer::Confirm::new().interact()? {
        // Exit, we don't need to do anything
        return Ok(());
    }

    // Save the packages
    repo.mirror(Packages::from(checked))
        .expect("Could not mirror commands");

    Ok(())
}
