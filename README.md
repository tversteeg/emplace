# emplace
Command-line tool to mirror installed software on multiple machines.

## Installation

### Getting Started

1. Install the *emplace* binary:

   #### Rust
   
   ```sh
   cargo install emplace
   ```

2. Add the init script to your shell's config file:

   #### Bash
   
   Add the following tot he end of `~/.bashrc`:
   
   ```sh
   # ~/.bashrc
   
   eval "$(emplace init bash)"
   ```
