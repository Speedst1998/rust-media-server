use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::thread;
use webrtc::util::Conn;
use super::tables::TABLES;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub trait ConnectionProvider {
    fn get_connection(&self) -> PooledConnection<SqliteConnectionManager>;
}

pub struct DatabaseHandler {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

impl ConnectionProvider for DatabaseHandler {
    fn get_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool.get().unwrap()
    }
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
}

