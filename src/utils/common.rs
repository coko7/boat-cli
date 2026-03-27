use anyhow::Result;
use serde::Serialize;

use crate::models::{RowPrintable, TablePrintable};

pub fn list_printable_items<T: RowPrintable + Serialize>(
    items: Vec<T>,
    show_as_json: bool,
) -> Result<()> {
    if show_as_json {
        let json = serde_json::to_string(&items)?;
        println!("{json}");
        return Ok(());
    }

    let table = items.to_printable_table();
    println!("{table}");
    Ok(())
}
