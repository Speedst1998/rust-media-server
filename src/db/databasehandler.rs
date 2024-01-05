use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::thread;

use super::tables::TABLES;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub struct DatabaseHandler {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

impl DatabaseHandler {
    pub fn new(manager: SqliteConnectionManager) -> Self {
        let pool: r2d2::Pool<SqliteConnectionManager> = r2d2::Pool::new(manager).unwrap();

        TABLES.iter().for_each(|(table_name, table)| {
            pool.get()
                .unwrap()
                .execute(&table.creation_string, params![])
                .unwrap();
        });

        DatabaseHandler { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool.get().unwrap()
    }
}
