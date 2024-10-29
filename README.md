# Rust Text Editor

## Introduction

A crossplatorm, nano-like, terminal-based text editor written in Rust. Tested and working on both Windows and Linux (MacOS should work but is untested). This is still in very early alpha, so there are a lot of features missing and there is a lot of room for optimization still.

## Terminal Emulators

Full funtionality is dependent on the terminal emulator:

    - `Alacritty` has full functionality
    - `Terminator` works, but doesn't implement the `Kitty Keyboard Protocal` so keybinds requiring multiple modifier keys don't work (i.e. highlighting while jumping)
    - `Powershell` on Windows has odd interactions with unicode characters, specifically when pasting them, but works for the most part
    - `Konsole` has weird kerning problems
    - `tty` work with limited functionality

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) (Some linux distros will provide a `rustup` package (i.e. [Arch](https://archlinux.org/packages/?name=rustup)))
2. Either clone the repo (`git clone https://github.com/bons0002/rust-text-editor.git`) or download the zip and extract it.
3. Step into the directory
4. Run `cargo build --release`
5. The compiled binary will be at `./target/release/app`

## Usage

Currently, the only way to open a file is by passing the file path as a runtime argument, and there can only be one file open at a time. In the future, this will be changed.
