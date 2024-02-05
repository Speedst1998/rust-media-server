use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use strum_macros::Display;
use webrtc::util::conn;

use super::{
    errors::DbError,
    tables::{Table, WatchedFoldersRows},
};

pub struct WatchedFoldersDb<'a> {
    connection_pool: &'a r2d2::PooledConnection<SqliteConnectionManager>,
}

pub struct WatchedFolder {
    pub path: String,
}

impl<'a> WatchedFoldersDb<'a> {
    pub fn new(connection_pool: &r2d2::PooledConnection<SqliteConnectionManager>) -> WatchedFoldersDb {
        WatchedFoldersDb { connection_pool }
    }

    fn get_connection(&self) -> Result<PooledConnection<SqliteConnectionManager>, r2d2::Error> {
        let connection = self.connection_pool.get()?;
        Ok(connection)
    }

    pub fn create(&self, path: &String) -> Result<WatchedFolder, DbError> {
        self.get_connection()
            .map_err(|_e| DbError::CreateError)
            .and_then(|connection| {
                connection
                    .execute("INSERT INTO WatchedFolders (Path) VALUES (?1)", &[path])
                    .map_err(|_e| DbError::CreateError)
            })
            .and_then(|_rows| self.get(path))
        // match self.get_connection() {
        //     Ok(connection) => {
        //         match connection.execute("INSERT INTO WatchedFolders (Path) VALUES (?1)", &[path]) {
        //             Ok(rows) => match self.get(path) {
        //                 Ok(result) => Ok(result),
        //                 Err(e) => Err(DbError::CreateError),
        //             },
        //             Err(e) => Err(DbError::CreateError),
        //         }
        //     }
        //     Err(e) => Err(DbError::CreateError),
        // }
    }

    pub fn get(&self, path: &String) -> Result<WatchedFolder, DbError> {
        self.get_connection()
            .map_err(|_e| DbError::ReadError)
            .and_then(|connection| {
                connection
                    .prepare("SELECT path FROM WatchedFolders WHERE path = ?1")
                    .map_err(|_e| DbError::ReadError)
            })
            .and_then(|mut stmt| {
                stmt.query_row([path], |row| Ok(WatchedFolder { path: row.get(0)? }))
                    .map_err(|_e| DbError::ReadError)
            })
    }

    pub fn list(&self) -> Result<Vec<WatchedFolder>, DbError> {
        self.get_connection()
            .map_err(|_e| DbError::CreateError)
            .and_then(|connection| {
                connection
                    .prepare("SELECT path FROM WatchedFolders")
                    .map_err(|_e| DbError::ReadError)
            })
            .and_then(|mut stmt| {
                stmt.query_map([], |row| Ok(WatchedFolder { path: row.get(0)? }))
                    .map_err(|_e| DbError::ReadError)?
                    .map(|row| row.map_err(|_e| DbError::ReadError))
                    .collect()
            })
    }

    pub fn delete(&self, path: &String) -> Result<usize, DbError> {
        self.get_connection().map_err(|_e| DbError::ReadError).and_then(|connection| {
            connection
                .execute("DELETE FROM WatchedFolders WHERE path = ?1", &[path])
                .map_err(|_e| DbError::ReadError)
        })
    }
}
