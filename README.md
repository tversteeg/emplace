<h1 align="center">emplace</h1>
<p align="center">
   Command-line tool to mirror installed software on multiple machines.
</p>

<p align="center">
   <a href="https://actions-badge.atrox.dev/tversteeg/emplace/goto"><img src="https://github.com/tversteeg/emplace/workflows/CI/badge.svg" alt="Build Status"/></a>
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
- [x] Parses history file
- Package managers supported:

    | OS | Package Manager |
    | :---: | --- |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/debian.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/ubuntu.svg" width="18" height="18" /> | Apt |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /> | GNU Guix |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/nixos.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /> | Nix |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | Pacman |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | RUA |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/archlinux.svg" width="18" height="18" /> | Yay |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/suse.svg" width="18" height="18" /> | Zypper |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /> | Snap |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/fedora.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/redhat.svg" width="18" height="18" /> | DNF/YUM |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /> | Homebrew |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/freebsd.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/android.svg" width="18" height="18" /> | Pkg |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Chocolatey |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Scoop |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Cargo |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Rustup Component |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Python Pip |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Python Pip 3 |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Node Package Manager |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Ruby Gem |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" /><img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" /> | Go |


- Shells supported:

    | | Shell |
    | :---: | --- |
    | <img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/gnubash.svg" width="18" height="18" /> | [Bash](#bash) |
    | <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/dollar-sign.svg" width="18" height="18" /> | [Zsh](#zsh) |
    | <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/terminal.svg" width="18" height="18" /> | [Fish](#fish) |
    | <img src="https://cdn.rawgit.com/feathericons/feather/master/icons/chevron-right.svg" width="18" height="18" /> | [Nu](#nu) |

## Installation <a href="https://repology.org/project/emplace/versions"><img src="https://repology.org/badge/vertical-allrepos/emplace.svg" alt="Packaging status" align="right"></a>

### Getting Started

1. Install the *emplace* binary:

   ### Windows
   
   #### Scoop
   
   ```sh
   scoop install emplace
   ```
   
   ### NixOS
   
   ```sh
   nix-env -iA nixos.emplace
   ```
   
   ### Linux, MacOS, Windows

   #### Rust
   
   ```sh
   cargo install emplace
   ```
   
   #### Binary
   
   Download the latest binary from [releases](https://github.com/tversteeg/emplace/releases) for your OS and put it somewhere in your PATH.

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

   #### Nu

   Run the following in your shell:

	```sh
	emplace init nu | config set_into prompt
	```

3. Change the configuration file (optional)

   When you want to use a different configuration file from the default one you can change `emplace init $SHELL` with `emplace init $SHELL -c path/to/my/config.toml`.
   
   This will set the `EMPLACE_CONFIG="path/to/my/config.toml"` environment variable. This can be overwritten with `export EMPLACE_CONFIG="path/to/other/config.toml"` _after_ the init function, except in the Nu shell.
   
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
# -- or --
nix-env -iA nixpkgs.nixfmt
# -- or --
brew install wget
# -- or --
sudo dnf install htop
# -- or --
guix install 0ad
# -- or --
gem install aasm-diagram
# -- or --
go get colorgo
# -- or --
pkg install nginx
```

### Synchronizing On Another Machine

Just invoke `emplace install`, and you will be prompted with the packages to install:

```sh
emplace install
```

### History Analysis

You can also parse history files, this will try to extract installation commands from each line.

#### Bash & Zsh

```sh
emplace history $HISTFILE
```

#### Fish

```sh
emplace history ~/.local/share/fish/fish_history
```

### Removing Synchronized Packages

Run `emplace clean` and select the packages you want to be cleaned, they won't be removed from your system.

### Creating and modifying config files

Run `emplace config --new` to create a new config and configure the repository or `emplace config --path` to retrieve the path to the configuration file.

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/lberrymage"><img src="https://avatars0.githubusercontent.com/u/46338569?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Logan Magee</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/commits?author=lberrymage" title="Code">💻</a> <a href="https://github.com/tversteeg/emplace/issues?q=author%3Alberrymage" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/0xd34b33f"><img src="https://avatars2.githubusercontent.com/u/32823131?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Vladimir</b></sub></a><br /><a href="#platform-0xd34b33f" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/tversteeg/emplace/commits?author=0xd34b33f" title="Code">💻</a> <a href="https://github.com/tversteeg/emplace/issues?q=author%3A0xd34b33f" title="Bug reports">🐛</a> <a href="#maintenance-0xd34b33f" title="Maintenance">🚧</a> <a href="https://github.com/tversteeg/emplace/pulls?q=is%3Apr+reviewed-by%3A0xd34b33f" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://stackoverflow.com/users/story/4363604"><img src="https://avatars3.githubusercontent.com/u/1708197?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Jozef Hollý</b></sub></a><br /><a href="#ideas-j2ghz" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="https://blog.brightone.space"><img src="https://avatars1.githubusercontent.com/u/12615679?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Oleksii Filonenko</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/commits?author=filalex77" title="Code">💻</a> <a href="#platform-filalex77" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/tversteeg/emplace/pulls?q=is%3Apr+reviewed-by%3Afilalex77" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/x0f5c3"><img src="https://avatars0.githubusercontent.com/u/62678514?v=4?s=100" width="100px;" alt=""/><br /><sub><b>x0f5c3</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/commits?author=x0f5c3" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/bensleveritt"><img src="https://avatars0.githubusercontent.com/u/390779?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Benjamin S. Leveritt</b></sub></a><br /><a href="#ideas-bensleveritt" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-bensleveritt" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/mborejdo"><img src="https://avatars3.githubusercontent.com/u/52340?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Michael Borejdo</b></sub></a><br /><a href="#ideas-mborejdo" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/l1a"><img src="https://avatars3.githubusercontent.com/u/634380?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ken Tobias</b></sub></a><br /><a href="#platform-l1a" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/adamcstephens"><img src="https://avatars2.githubusercontent.com/u/2071575?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Adam Stephens</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/issues?q=author%3Aadamcstephens" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/ram02z"><img src="https://avatars2.githubusercontent.com/u/59267627?v=4?s=100" width="100px;" alt=""/><br /><sub><b>omar</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/issues?q=author%3Aram02z" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/5c0tt-b0t"><img src="https://avatars.githubusercontent.com/u/63571864?v=4?s=100" width="100px;" alt=""/><br /><sub><b>5c0tt-b0t</b></sub></a><br /><a href="#ideas-5c0tt-b0t" title="Ideas, Planning, & Feedback">🤔</a> <a href="https://github.com/tversteeg/emplace/issues?q=author%3A5c0tt-b0t" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/ModdedGamers"><img src="https://avatars.githubusercontent.com/u/35778371?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Modded Gamers</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/issues?q=author%3AModdedGamers" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://www.openhub.net/accounts/jayvdb"><img src="https://avatars.githubusercontent.com/u/15092?v=4?s=100" width="100px;" alt=""/><br /><sub><b>John Vandenberg</b></sub></a><br /><a href="https://github.com/tversteeg/emplace/issues?q=author%3Ajayvdb" title="Bug reports">🐛</a> <a href="#ideas-jayvdb" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
