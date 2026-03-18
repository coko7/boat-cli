# ⛵ boat

![build](https://github.com/coko7/boat/actions/workflows/rust.yml/badge.svg)

`boat` - A **B**asic **O**pinionated **A**ctivity **T**racker, inspired by [bartib](https://github.com/nikolassv/bartib).

This is only the code for the command line application. It relies on [`boat-lib`](https://github.com/coko7/boat-lib) for core functions.

## 🚧 Work in Progress

> [!WARNING]  
> This cli is actively being developed. Since it's in its early stages, things will likely break often.
> Don't use it for now.

## Commands

```help
boat                            (no arg), launch in interactive mode (Ratatui TUI)

boat s,st,start,sail <NAME>     start a new activity
    -c, --category <NAME>
    -q, --quiet                 do not output the new task id
    -v, --verbose               will output the activity ID to stdout
    (no arg) => interactive input

boat r,res,resume <ID>          resume a previous activity
    (no arg) => interactive fzf with list of activities

list c,can,cancel <ID>          cancel a given activity
    (no arg) => interactive fzf with list of activities

boat f,fin,finish <ID>          finish an activity
    (no arg) => interactive fzf with list of activities

boat e,ed,edit <ID>             alias to modify
boat m,mod,modify <ID>          modify an activity
    -n, --name <NAME>           change name of an activity
    -p, --project <PROJECT>     change the project of an activity
    -r, --raw                   manually edit the JSON into your `$EDITOR`
    (no arg) => interactive fzf with list of activities

boat x,exp,export:              export boat activity to a given format
    -j, --json                  JSON format
    -t, --toml                  TOML format
    -c, --csv                   CSV format
    -p, --plain                 plain text format
    -m, --markdown              Markdown report
    -j, --jira                  Jira integration?
    -b, --bartib                bartib cli format

boat l,ls,list                  list activities
    -a, --all                   show all 
    -c, --current               show active only
    -p, --projects              show all projects
    -j, --json                  output in json format

boat h,help:                    show help message

boat g,graph                    get graph information
```

## Data tables

File format (same as `bartib`, simple, efficient):
```csv
START-TIME - END-TIME | category | name
```

Simple first:
```rust
struct Activity {
    id: u64,
    name: String,
    category: String,
    tracking: HashSet<(DateTime, Option<DateTime>)>
}
```

Then we go mad:
```rust
struct Category {
    id: u64,
    name: String,
    notes_path: Option<PathBuf>,
}

struct Activity {
    id: u64,
    name: String,
    categoryId: u64,
    note_file_path: Option<PathBuf>,
    tracking: HashSet<(DateTime, Option<DateTime>)>
}
```

## JSON

```jsonc
{
    "activities": [
        { 
            "id": 0,
            "name": "work on project 1",
            "project": "Main",
            "tracking": [
                "08:05:32-09:10:24", "09:40-12:00"
            ]
        },
        { 
            "id": 1,
            "name": "fix critical bug",
            "project": "Main",
            "tracking": [
                "09:10:24-09:40:01"
            ]
        },
        { 
            "id": 2,
            "name": "big project",
            "project": "Main",
            "tracking": [
                "13:00:48-"
            ]
        }
    ],
}
```
