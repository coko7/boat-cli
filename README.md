# ⛵ boat

`boat` - A **B**asic **O**pinionated **A**ctivity **T**racker, inspired by [bartib](https://github.com/nikolassv/bartib).

This is only the code for the command line application. It relies on [`boat-lib`](https://github.com/coko7/boat-lib) for core functions.

[![Crates info](https://img.shields.io/crates/v/boat-cli.svg)](https://crates.io/crates/boat-cli)
[![License: GPL-3.0](https://img.shields.io/github/license/coko7/boat-cli?color=blue)](LICENSE)
![Rust](https://img.shields.io/github/languages/top/coko7/boat-cli?color=orange)
[![Tests](https://github.com/coko7/boat-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/coko7/boat-cli/actions/workflows/rust.yml)

> [!WARNING]  
> 🚧 Work in Progress
>
> This cli is actively being developed. Since it's in its early stages, things will likely break often.
> Don't use it for now.

## Installation

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

## Commands

```help
Basic Opinionated Activity Tracker

Usage: boat [OPTIONS] <COMMAND>

Commands:
  new     Create a new activity
  start   Start/resume an activity
  pause   Manage configuration Pause/stop the current activity
  modify  Modify an activity
  delete  Delete an activity
  get     Get the current activity
  list    List activities and tags
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```
