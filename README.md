# ⛵ boat

`boat` - A **B**asic **O**pinionated **A**ctivity **T**racker, inspired by [bartib](https://github.com/nikolassv/bartib).

<img width="1280" height="640" alt="boat-cli" src="https://github.com/user-attachments/assets/0cb3114c-7128-470b-8f6c-516056cedafa" />

Like its name implies, `boat` allows you to track the time you spend on everyday tasks.

It has mainly been designed to be easy to embed in custom bash scripts so that you can augment it with fuzzy-finding.
That said, if you plan to use the CLI directly (without external scripts), it also benefits from a [variety of handy aliases](#-usage).

`boat` stores its data in a SQLite database file which is kept in the config directory by default (`.config/boat/boat.db`).

This repository contains only the code for the command line application.
It relies on [`boat-lib`](https://github.com/coko7/boat-lib) for core functions.

[![Crates info](https://img.shields.io/crates/v/boat-cli.svg)](https://crates.io/crates/boat-cli)
[![License: GPL-3.0](https://img.shields.io/github/license/coko7/boat-cli?color=blue)](LICENSE)
![Rust](https://img.shields.io/github/languages/top/coko7/boat-cli?color=orange)
[![Tests](https://github.com/coko7/boat-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/coko7/boat-cli/actions/workflows/rust.yml)

## Contents

- [🚀 Demo](#-demo)
- [🤔 Why was this tool created?](#-why-was-this-tool-created)
- [🛠️ Installation](#%EF%B8%8F-installation)
    - [Install with a bundled version of SQLite](#install-with-a-bundled-version-of-sqlite)
- [⚙️ Configuration](#%EF%B8%8F-configuration)
- [✨ Usage](#-usage)

## 🚀 Demo

![demo](https://github.com/user-attachments/assets/007809ee-fda9-4848-bc1c-ccd0131a3616)

> [!TIP]
> The `mp4` version of the demo is available [here](https://github.com/user-attachments/assets/919cb800-1d65-4c3f-bbae-a3195db3b3f6)

## 🤔 Why was this tool created?

The [`bartib`](https://github.com/nikolassv/bartib) cli is what inspired me to create `boat`.
It's a feature-full tool that I used for a while, but I found it quite limiting for my usage due to its [lack of support for machine-readable output](https://github.com/nikolassv/bartib/pull/26).
And that's it. All I wanted was an activity tracker that I could combine easily with [`jq`](https://github.com/jqlang/jq) and so I decided to make my own tool.

## 🛠️ Installation

The easiest way to install is through [crates.io](https://crates.io/crates/boat-cli):
```sh
cargo install boat-cli
```

The other option is to build from source:
```sh
git clone https://github.com/coko7/boat-cli.git
cd boat-cli
cargo build --release
```

### Install with a bundled version of SQLite

If you are on Windows or you do not have SQLite on your system, you can install `boat` with the `bundled-sqlite` feature.
This will increase the compilation time but guarantees a working out-of-the-box exprience.

When using cargo:
```sh
cargo install boat-cli --features bundled-sqlite
```
When building from source:
```sh
git clone https://github.com/coko7/boat-cli.git
cd boat-cli
cargo build --release --features bundled-sqlite
```

## ⚙️ Configuration

By default, `boat` will create a configuration file in one of the following dirs:
- 🐧 **Linux:** `/home/<user>/.config/boat/config.toml`
- 🪟 **Windows:** `C:\Users\<user>\AppData\Roaming\boat\config.toml`
- 🍎 **macOS:** `/Users/<user>/Library/Application Support/boat/config.toml`

It will also store the SQLite database file `boat.db` in the same directory (unless specified otherwise in config):
```toml
database_path = "/home/<user>/.config/boat/boat.db"
```
You can override the default configuration file path by setting the `BOAT_CONFIG` environment variable.

## ✨ Usage

If you have ever used [`bartib`](https://github.com/nikolassv/bartib), then `boat` is going to feel very familiar.
Try `boat help` for a quick list of commands:
```help
boat 0.6.0

Basic Opinionated Activity Tracker

Usage:
boat [OPTIONS] <COMMAND>

Commands:
  new     Create a new activity
  start   Start/resume an activity
  cancel  Cancel the current activity
  pause   Pause/stop the current activity
  modify  Modify an activity
  delete  Delete an activity
  get     Get the current activity
  list    List activities
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

> [!TIP]
> `boat` comes bundled with many command aliases:
> - new: `n`, `new`, `create`
> - start: `s`, `st`, `start`, `sail`, `continue`, `resume`
> - cancel: `c`, `can`, `cancel`
> - pause: `p`, `pause`, `stop`
> - modify: `m`, `mod`, `modify`
> - delete: `d`, `del`, `delete`, `rm`, `rem`, `remove`
> - get: `g`, `get`
> - list: `l`, `ls`, `list`
> - help: `h`, `help`, `-h`, `--help`
>
> Prefer using the full length command names in scripts as they are more explicit and unlikely to be changed (unlike shorter aliases).

I really wanted to have each command start with a different character so that I could assign a single-char alias to all of them.
That explains why some of the commands do not use a more fitting keyword.

Like `stop` would have been a better command than `pause` but since it shares the same starting charcter as the `start` command, I could not use it.
Maybe I will drop this in the future, let's see.

*I have included some fallback in case you type `stop`/`remove` instead of `pause`/`delete` 👀*
