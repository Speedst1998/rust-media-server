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
        pool.get()
            .unwrap()
            .execute("CREATE TABLE IF NOT EXISTS foo (bar INTEGER)", params![])
            .unwrap();

        let _ = (0..10)
            .map(|i| {
                let pool = pool.clone();
                thread::spawn(move || {
                    let conn = pool.get().unwrap();
                    conn.execute("INSERT INTO foo (bar) VALUES (?)", &[&i])
                        .unwrap();
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(thread::JoinHandle::join);
        // .collect::<Result<_, _>>()
        // .unwrap();

        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO WatchedFolders (path) VALUES (?)",
            &["/somepath"],
        );

        DatabaseHandler { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool.get().unwrap()
    }
}
