use log::info;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use std::thread;
use webrtc::util::Conn;
use refinery::Migration;

refinery::embed_migrations!("./migrations");

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

        let mut conn = Connection::open("./file.db").unwrap();

        let use_iteration = std::env::args().any(|a| a.to_lowercase().eq("--iterate"));
        
        if use_iteration {
            // create an iterator over migrations as they run
            for migration in migrations::runner().run_iter(&mut conn) {
                process_migration(migration.expect("Migration failed!"));
            }
        } else {
            // or run all migrations in one go
            migrations::runner().run(&mut conn).unwrap();
        }

        DatabaseHandler { pool }
    }
}

fn process_migration(migration: Migration) {
    #[cfg(not(feature = "enums"))]
    {
        // run something after each migration
        info!("Post-processing a migration: {}", migration)
    }
}

