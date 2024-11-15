# Rust Text Editor

## Introduction

A crossplatform, nano-like, terminal-based text editor written in Rust. Tested and working on both `Windows` and `Linux` (`MacOS` should work but is untested) as well as both `x86_64` and `aarch64` systems. This is still in very early alpha so there are a lot of features missing, and there is a lot of room for optimization.

## Installation

### Download

Compiled binaries available in the `Releases` tab.

### Building

**NOTE: `rustc v1.74.0` or newer required to build. Some linux distros may provide older versions.**

1. [Install latest Rust version](https://www.rust-lang.org/tools/install) (Some linux distros will provide a `rustup` or `cargo` package _e.g. [Arch](https://archlinux.org/packages/?name=rustup)_)
2. Either clone the repo (`git clone https://github.com/bons0002/rust-text-editor.git`) or download the zip and extract it.
3. Step into the directory
4. Run `cargo build --release`
5. The compiled binary will be at `./target/release/app`

## Usage

`/path/to/app filename` (Recommended to add an alias to `.bashrc`/`.zshrc` for the app)

Currently, the only way to open a file is by passing the file path as a runtime argument, and there can only be one file open at a time. In the future, this will be changed.

## Terminal Emulators

Full funtionality is dependent on the terminal emulator:

* `Alacritty`, `Kitty`, and `Konsole` have full functionality.
    * Note: `Kitty` by default uses the `Shift + Ctrl` for its modifier key, so they can't be used together in this app unless rebound.
    * `Kitty Keyboard Protocol` must be implemented by the terminal emulator in order to use multiple modifiers at once.
* `Terminator` works but keybinds requiring multiple modifier keys don't work (i.e. highlighting while jumping using `Shift + Ctrl`).
* `Powershell` on Windows has odd interactions with unicode characters but works for the most part.
    * Specifically, pasting text with unicode characters will omit the unicode characters.
* `xterm` works for the most part. `Shift + PgUp/PgDn` doesn't work though.
* `tty`'s work with limited functionality.

Terminal multiplexers such as `tmux` and `screen` can cause weird interactions.
