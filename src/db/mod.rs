use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub mod sqlite;
pub mod rep;
pub mod rep_async;

const DB_FILE_PATH: &str = "broker.db";

lazy_static! {
    pub static ref DB_POOL: Pool<SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file(DB_FILE_PATH);
        Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create SQLite pool")
    };
}