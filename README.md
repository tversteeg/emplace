<h1 align="center">emplace</h1>
<p align="center">
   Command-line tool to mirror installed software on multiple machines.
</p>

<p align="center">
   <a href="https://actions-badge.atrox.dev/tversteeg/emplace/goto"><img src="https://github.com/tversteeg/sprite-gen/workflows/CI/badge.svg" alt="Build Status"/></a>
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
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | RUA |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | Yay |
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
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/gnubash.svg" width="18" height="18" /> | [Bash](#bash) |
    | <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/dollar-sign.svg" width="18" height="18" /> | [Zsh](#zsh) |
    | <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/terminal.svg" width="18" height="18" /> | [Fish](#fish) |

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
pacman -S zsh
# -- or --
rua install peek
# -- or --
yay -S thunar
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

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/lberrymage"><img src="https://avatars0.githubusercontent.com/u/46338569?v=4" width="100px;" alt=""/><br /><sub><b>Logan Magee</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/issues?q=author%3Alberrymage" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/0xd34b33f"><img src="https://avatars2.githubusercontent.com/u/32823131?v=4" width="100px;" alt=""/><br /><sub><b>Vladimir</b></sub></a><br /><a href="#platform-0xd34b33f" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/tversteeg/emplace/commits?author=0xd34b33f" title="Code">💻</a> <a href="https://github.com/tversteeg/emplace/issues?q=author%3A0xd34b33f" title="Bug reports">🐛</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
