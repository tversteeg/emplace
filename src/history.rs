use crate::{
    config::Config,
    package::{Package, Packages},
    repo::Repo,
};
use anyhow::Result;
use colored::Colorize;
use dialoguer::MultiSelect;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

/// Capture a history file.
pub fn history<P1, P2>(config_path: P1, path: P2, select_all: bool) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let hist_file = File::open(path)?;
    let reader = BufReader::new(hist_file);

    // Read all the lines from the file
    let lines = reader.lines();

    // Extract the package information from the lines
    let catches = filter_lines(lines);
    if catches.is_empty() {
        return Ok(());
    };

    // Get the config
    let config = Config::from_path_or_new(&config_path)?;

    // Get the repository from the config
    let repo = Repo::new(config, true)?;

    let mut catches = Packages::from(catches);
    catches.filter_saved_packages(&repo.read()?);
    catches.dedup();

    if catches.is_empty() {
        // Nothing found after filtering
        return Ok(());
    }

    let selections = if select_all {
        // Select everything from the history
        (0..catches.len()).collect()
    } else {
        // Prompt the user for the packages to sync
        let colored_selection: Vec<String> = catches.iter().map(|x| x.color_full_name()).collect();

        MultiSelect::new()
            .items(&colored_selection)
            .with_prompt("Select packages to sync")
            .interact()?
    };

    let mut checked = vec![];
    selections
        .iter()
        .for_each(|x| checked.push(catches[*x].clone()));

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

    if !select_all && !dialoguer::Confirm::new().interact()? {
        // Exit, we don't need to do anything
        return Ok(());
    }

    // Save the packages
    repo.mirror(Packages::from(checked))
        .expect("Could not mirror commands");

    Ok(())
}

/// Extract the packages from the history lines.
pub fn filter_lines<R>(lines: Lines<R>) -> Vec<Package>
where
    R: BufRead,
{
    // Filter actual lines
    lines
        .map_while(Result::ok)
        .map(|x| x.split_whitespace().join(" "))
        // Remove duplicates
        .sorted()
        .dedup()
        // Get an iterator over all the matches in the line
        .map(|x| Packages::from_line(&x))
        // Make it a big list
        .flat_map(|packages| packages.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_bash_history() {
        let bash_history = Cursor::new(indoc::indoc! {"
            sudo apt install linux-perf-5.6
            cargo flamegraph --example basic
            sudo nvim /etc/sysctl.conf
            fg
        "});

        let packages = super::filter_lines(bash_history.lines());
        assert_eq!(packages.len(), 1);
    }

    #[test]
    fn test_fish_history() {
        let fish_history = Cursor::new(indoc::indoc! {r#"
            - cmd: sudo apt install fzf
              when: 1575643236
            - cmd: git commit -am "Commit message"
              when: 1575643236
            - cmd: sudo apt -qq install meld
              when: 1575643236
        "#});

        let packages = super::filter_lines(fish_history.lines());
        assert_eq!(packages.len(), 2);
    }

    #[test]
    fn test_zsh_history() {
        let zsh_history = Cursor::new(indoc::indoc! {r#"
            : 1610989542:0;ls
            : 1610989544:0;echo $HISTFILE
            : 1610989549:0;cat  $HISTFILE
            : 1610989572:0;sudo apt install test
            : 1610989577:0;cargo install test
            : 1610989583:0;cat  $HISTFILE
            : 1610989600:0;nvim .zsh_history
        "#});

        let packages = super::filter_lines(zsh_history.lines());
        assert_eq!(packages.len(), 2);
    }
}
