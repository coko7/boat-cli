use boat_lib::models::tag::Tag as DatabaseTag;
use boat_lib::repository::Id;
use serde::{Deserialize, Serialize};

use crate::models::RowPrintable;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintableTag {
    pub id: Id,
    pub name: String,
}

impl PrintableTag {
    pub fn from_tag(tag: &DatabaseTag) -> Self {
        Self {
            id: tag.id,
            name: tag.name.clone(),
        }
    }
}

impl RowPrintable for PrintableTag {
    fn row_spec() -> String {
        "{:>}  {:<}".to_string()
    }

    fn header_names() -> Vec<String> {
        ["ID", "Name"].iter().map(|s| s.to_string()).collect()
    }

    fn row_values(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_from_tag_works() {
        let db_tag = DatabaseTag {
            id: 1,
            name: "foo".to_string(),
        };
        let tag = PrintableTag::from_tag(&db_tag);
        assert_eq!(tag.id, 1);
        assert_eq!(tag.name, "foo");
    }
}
