# emplace
Command-line tool to mirror installed software on multiple machines.

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ftversteeg%2Femplace%2Fbadge&style=flat)](https://actions-badge.atrox.dev/tversteeg/emplace/goto)
[![Build status](https://ci.appveyor.com/api/projects/status/1aykje7hu7b88k8e?svg=true)](https://ci.appveyor.com/project/tversteeg/emplace)
[![](https://img.shields.io/crates/d/emplace.svg)](#downloads)
[![](https://img.shields.io/crates/v/emplace.svg)](https://crates.io/crates/emplace)

![Example](./docs/capture.svg)

## Features

- [x] Outputs a human-readable (JSON) file to sync between machines: `.emplace`
- [x] Version control with git to automatically push & pull updates
- [x] Automatic prompt after installing a package
- [x] Integrates well with dotfiles repositories
- [x] Package managers supported:
   - [x] Cargo (_*_)
   - [x] Apt (_Linux_)
   - [x] Chocolatey (_Windows_)
   - [x] Python Pip (_*_)
   - [x] Python Pip 3 (_*_)
   - [ ] Homebrew (_Mac_)
   - [ ] Snap (_Linux_)
   - [ ] Yum (_Linux_)
   - [ ] DNF (_Linux_)
   - [ ] Pacman (_Linux_)
   - [ ] Portage (_Linux_)
   - [ ] NPM (_*_)

## Installation

### Getting Started

1. Install the *emplace* binary:

   #### Rust
   
   ```sh
   cargo install emplace
   ```

2. Add the init script to your shell's config file:

   #### Bash
   
   Add the following to the end of `~/.bashrc`:
   
   ```sh
   # ~/.bashrc
   
   eval "$(emplace init bash)"
   ```

[![Stargazers over time](https://starchart.cc/tversteeg/emplace.svg)](https://starchart.cc/tversteeg/emplace)
