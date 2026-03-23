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

## Why was this tool created?

The [`bartib`](https://github.com/nikolassv/bartib) cli is what inspired me to create `boat`.
It's a feature-full tool that I used for a while, but I found it quite limiting for my usage due to its [lack of support for machine-readable output](https://github.com/nikolassv/bartib/pull/26).
That's it, I wanted an activity tracker that I could combine easily with [`jq`](https://github.com/jqlang/jq) and so I decided to make my own tool.

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

## Usage

To get a feel of how `boat` can be used, you can try `boat help` to get the list of commands:
```help
Basic Opinionated Activity Tracker

Usage: boat <COMMAND>

Commands:
  new     Create a new activity
  start   Start/resume an activity
  config  Manage configuration
  pause   Pause/stop the current activity
  modify  Modify an activity
  delete  Delete an activity
  get     Get the current activity
  list    List activities and tags
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

If you want to invoke `boat` from your command-line directly, you can make use of a variety of shorter aliases:
```help
Commands:
  new     n
  start   s, st, sail
  config  c, cfg, conf
  pause   p
  modify  m, mod
  delete  d, del
  get     g
  list    l, ls
```
I really wanted to have each command start with a different character so that I could assign a single-char alias to all of them.
That explains why some of the commands do not use a more fitting keyword.
Like `stop` would have been a better command than `pause` but since it shares the same starting charcter as the `start` command, I could not use it.
Maybe I will drop this in the future, let's see.
