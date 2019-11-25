# emplace
Command-line tool to mirror installed software on multiple machines.

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ftversteeg%2Femplace%2Fbadge&style=flat)](https://actions-badge.atrox.dev/tversteeg/emplace/goto)
[![](https://img.shields.io/crates/d/emplace.svg)](#downloads)
[![](https://img.shields.io/crates/v/emplace.svg)](https://crates.io/crates/emplace)

![Example](./docs/capture.svg)

## Features

- [x] Outputs a human-readable (RON) file to sync between machines: `.emplace`
- [x] Version control with git to automatically push & pull updates
- [x] Automatic prompt after installing a package
- [x] Integrates well with dotfiles repositories
- Package managers supported:
   - [x] Apt (_Linux_)
   - [x] Pacman (_Linux_)
   - [x] Snap (_Linux_)
   - [x] Chocolatey (_Windows_)
   - [x] Cargo (_*_)
   - [x] Rustup Component (_*_)
   - [x] Python Pip (_*_)
   - [x] Python Pip 3 (_*_)
   - [x] Node Package Manager (_*_)
   - [ ] Homebrew (_Mac_)
   - [ ] Nix (_Linux_)
   - [ ] Yum (_Linux_)
   - [ ] DNF (_Linux_)
   - [ ] Portage (_Linux_)
- Shells supported:
   - [x] Bash
   - [x] Zsh
   - [ ] Fish
   - [ ] nu

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
   
   source <(emplace init bash)
   ```

   #### Zsh
   
   Add the following to the end of `~/.zshrc`:
   
   ```sh
   # ~/.zshrc
   
   eval "$(emplace init zsh)"
   ```
   
## Usage

### Mirroring Packages

Invoke the supported package manager of choice using the command line in a normal way and you will automatically be prompted to mirror the package, for example:

```sh
cargo install topgrade
# -- or --
rustup component add rustfmt
# -- or --
sudo apt install meld
# -- or --
pacman -Sy zsh
# -- or --
sudo snap install scrcpy
# -- or --
npm install -g release
# -- or --
pip3 install --user termtosvg
# -- or --
choco install neovim
```

### Synchronizing On Another Machine

Just invoke `emplace install` and you will be prompted with the packages to install:

```sh
emplace install
```
