use tabular::{Row, Table};
use yansi::Paint;

pub mod activity;
pub mod activity_log;
pub mod boat_data;
pub mod log;
pub mod tag;

pub trait TablePrintable {
    fn to_printable_table(&self) -> Table;
}

pub trait RowPrintable {
    fn row_spec() -> String;
    fn header_names() -> Vec<String>;
    fn row_values(&self) -> Vec<String>;
    fn style_cell(&self, value: String) -> String {
        value
    }
}

impl<T> TablePrintable for Vec<T>
where
    T: RowPrintable,
{
    fn to_printable_table(&self) -> Table {
        let mut table = Table::new(&T::row_spec());

        let mut header = Row::new();
        for h in T::header_names() {
            header.add_ansi_cell(Paint::new(h).underline().to_string());
        }
        table.add_row(header);

        for item in self.iter() {
            let mut row = Row::new();
            for value in item.row_values() {
                let styled = item.style_cell(value.to_string());
                row.add_ansi_cell(styled);
            }
            table.add_row(row);
        }

        table
    }
}
