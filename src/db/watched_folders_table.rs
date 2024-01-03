use std::path::Path;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Error};
use strum_macros::Display;

use super::tables::{Table, WatchedFoldersRows};

pub struct WatchedFoldersDb {
    conn: r2d2::PooledConnection<SqliteConnectionManager>,
}

pub struct WatchedFolder {
    pub path: String,
}

impl WatchedFoldersDb {
    pub fn new(conn: r2d2::PooledConnection<SqliteConnectionManager>) -> WatchedFoldersDb {
        WatchedFoldersDb { conn }
    }

    pub fn create(&self, path: &String) -> Result<WatchedFolder, Error> {
        match self
            .conn
            .execute("INSERT INTO WatchedFolders (Path) VALUES (?1)", &[path])
        {
            Ok(_) => self.get(path),
            Err(error) => Err(error),
        }
    }

    pub fn get(&self, path: &String) -> Result<WatchedFolder, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT path FROM WatchedFolders WHERE path = ?1")?;

        stmt.query_row([path], |row| Ok(WatchedFolder { path: row.get(0)? }))
    }

    pub fn list(&self) -> Result<Vec<WatchedFolder>, Error> {
        let mut stmt = self.conn.prepare("SELECT path FROM WatchedFolders")?;

        let x = stmt
            .query_map([], |row| Ok(WatchedFolder { path: row.get(0)? }))?
            .collect();
        x
    }

    pub fn delete(&self, path: &String) -> Result<usize, Error> {
        self.conn
            .execute("DELETE FROM WatchedFolders WHERE path = ?1", &[path])
    }
}
