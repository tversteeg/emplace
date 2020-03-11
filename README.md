<h1 align="center">emplace</h1>
<p align="center">
   Command-line tool to mirror installed software on multiple machines.
</p>

<p align="center">
   <a href="https://actions-badge.atrox.dev/tversteeg/emplace/goto"><img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ftversteeg%2Femplace%2Fbadge&style=flat" alt="Build Status"/></a>
   <a href="https://github.com/tversteeg/emplace/releases"><img src="https://img.shields.io/crates/d/emplace.svg" alt="Downloads"/></a>
   <a href="https://crates.io/crates/emplace"><img src="https://img.shields.io/crates/v/emplace.svg" alt="Version"/></a>
   <br/><br/>
   
   <img src="docs/capture.svg"/>
   <br/>
</p>

## Features

- [x] Outputs a human-readable (RON) file to sync between machines: `.emplace`
- [x] Version control with git to automatically push & pull updates
- [x] Automatic prompt after installing a package
- [x] Integrates well with dotfiles repositories

- Package managers supported:

| OS | Package Manager |
| :---: | --- |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/debian.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/ubuntu.svg" width="18" height="18" /> | Apt |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | Pacman |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /> | Snap |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Chocolatey |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Scoop |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Cargo |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Rustup Component |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Python Pip |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Python Pip 3 |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Node Package Manager |

- Shells supported:

| | Shell |
| :---: | --- |
| <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/gnubash.svg" width="18" height="18" /> | Bash |
| <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/dollar-sign.svg" width="18" height="18" /> | Zsh |
| <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/terminal.svg" width="18" height="18" /> | Fish |

## Installation

### Getting Started

1. Install the *emplace* binary:

   ### Windows
   
   #### Scoop
   
   ```sh
   scoop install emplace
   ```
   
   ### Source

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

   #### Fish

   Add the following to the end of `~/.config/fish/config.fish`:

	```sh
	# ~/.config/fish/config.fish

	emplace init fish | source
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
# -- or --
scoop install curl
```

### Synchronizing On Another Machine

Just invoke `emplace install` and you will be prompted with the packages to install:

```sh
emplace install
```

### Removing Synchronized Packages

Run `emplace clean` and select the packages you want to be cleaned, they won't be removed from your system.
