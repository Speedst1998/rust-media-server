use once_cell::sync::Lazy;

pub struct Table {
    pub name: String,
    pub column_names: Vec<String>,
    pub creation_string: String,
}

impl Table {
    pub fn new(name: String, column_names: Vec<String>) -> Table {
        Table {
            name: name.clone(),
            column_names: column_names.clone(),
            creation_string: format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                name.clone(),
                column_names.clone().join(" varchar(255),")
            ),
        }
    }
}

// const Folders_To_Watch: &str = "Folders_To_Watch";

pub const TABLES: Lazy<Vec<Table>> = Lazy::new(|| {
    vec![Table::new(
        "Folders_To_Watch".to_string(),
        vec!["folder".to_string()],
    )]
});
