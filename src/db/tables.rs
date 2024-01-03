use std::collections::HashMap;

use once_cell::sync::Lazy;

pub struct Table {
    pub name: String,
    pub column_names: HashMap<String, String>,
    pub creation_string: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TableNames {
    WatchedFolders,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WatchedFoldersRows {
    Path,
}

impl Table {
    pub fn new(name: String, column_names: HashMap<String, String>) -> Table {
        Table {
            name: name.clone(),
            column_names: column_names.clone(),
            creation_string: format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                name.clone(),
                column_names
                    .clone()
                    .iter()
                    .map(|(column_name, column_type)| format!("{} {}", column_name, column_type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

fn combine_column_options(options: Vec<String>) -> String {
    options.join(" ")
}

// Enter table name and rows name here to make new table
pub static TABLES: Lazy<HashMap<TableNames, Table>> = Lazy::new(|| {
    HashMap::from([(
        TableNames::WatchedFolders,
        Table::new(
            "WatchedFolders".to_string(),
            HashMap::from([(
                "path".to_string(),
                combine_column_options(Vec::from([
                    "varchar(255)".to_string(),
                    "not null".to_string(),
                    "unique".to_string(),
                ])),
            )]),
        ),
    )])
});
