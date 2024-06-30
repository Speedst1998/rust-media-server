use std::path::Path;
use std::sync::Arc;
use iced::keyboard::KeyCode::T;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Error, Connection};
use strum_macros::Display;
use crate::db::databasehandler::ConnectionProvider;
use super::tables::{Table, WatchedFoldersRows};

pub struct WatchedFoldersDb {
    pool: Arc<dyn ConnectionProvider>,
}

pub struct WatchedFolder {
    pub path: String,
}

impl WatchedFoldersDb {
    pub fn new(connection_provider: Arc<dyn ConnectionProvider>) -> WatchedFoldersDb {
        WatchedFoldersDb { pool: connection_provider }
    }

    pub fn create(&self, path: &String) -> Result<WatchedFolder, Error> {
        match self
            .pool
            .get_connection()
            .execute("INSERT INTO WatchedFolders (Path) VALUES (?1)", &[path])
        {
            Ok(_) => self.get(path),
            Err(error) => Err(error),
        }
    }

    pub fn get(&self, path: &String) -> Result<WatchedFolder, Error> {
        let connection = self.pool.get_connection();
        let mut stmt = connection
            .prepare("SELECT path FROM WatchedFolders WHERE path = ?1")?;

        stmt.query_row([path], |row| Ok(WatchedFolder { path: row.get(0)? }))
    }

    pub fn list(&self) -> Result<Vec<WatchedFolder>, Error> {
        let connection = self.pool.get_connection();
        let mut stmt = connection.prepare("SELECT path FROM WatchedFolders")?;

        let x = stmt
            .query_map([], |row| Ok(WatchedFolder { path: row.get(0)? }))?
            .collect();
        x
    }

    pub fn delete(&self, path: &String) -> Result<usize, Error> {
        self.pool
            .get_connection()
            .execute("DELETE FROM WatchedFolders WHERE path = ?1", &[path])
    }
}
