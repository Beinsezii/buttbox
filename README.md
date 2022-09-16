# ButtBox 0.1.0
Easily create a box of buttons to do things!

## Features
  * Make buttons that do things
    * Configure with [TOML](./demo.toml)
  * Pure Rust, very portable.

## WIP/Why this isn't 1.0 yet
  * Themes / UI overhaul
  * Read from STDIN
  * Investigte CLI as opposed to reading TOML
  * ???
  
## Usage
See the [example TOML](./demo.toml)

<img src="./demo.png"/>

### Compiling
Have Rust 2021 installed, clone repo and just run `cargo build`.
`build_bin.sh` will build in binaries in release mode for linux/windows, moving the binaries to ./bin/

## F.A.Q.
Question|Answer
---|---
Why?|I already have a custom power menu, and when I decided to add a new button for rebooting straight into Windows using systemd-boot, I decided I should just make a dynamic button program instead of recompiling my other one every time I change something.
Someone's already thought of this!|Probably.
