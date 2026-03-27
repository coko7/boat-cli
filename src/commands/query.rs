use std::cmp::Reverse;

use anyhow::Result;
use boat_lib::repository::tags_repository as tags;
use rusqlite::Connection;

use crate::{cli, models::tag::PrintableTag, utils};

pub fn query_subcommand(conn: &mut Connection, command: &cli::QuerySubcommand) -> Result<()> {
    todo!()
}

fn list_tags(conn: &mut Connection, args: &cli::ListArgs) -> Result<()> {
    let mut all_tags: Vec<_> = tags::get_all(conn)?
        .iter()
        .map(PrintableTag::from_tag)
        .collect();
    all_tags.sort_by_key(|t| Reverse(t.id));

    utils::common::list_printable_items(all_tags, args.use_json_format)
}
