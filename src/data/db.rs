use std::error::Error;
use sqlx::{Pool, Sqlite};
use crate::defaults;

const INIT_DB_SCRIPT: &str = r#"
    CREATE TABLE IF NOT EXISTS settings(
        key TEXT NOT NULL UNIQUE CONSTRAINT PK_settings PRIMARY KEY,
        value TEXT NOT NULL
    );
                    
    CREATE UNIQUE INDEX IF NOT EXISTS IX_settings_key ON settings (key);

    CREATE TABLE IF NOT EXISTS sources(
        src_id TEXT NOT NULL UNIQUE CONSTRAINT PK_sources PRIMARY KEY,
        cfg TEXT NULL,
        on BOOLEAN NOT NULL CHECK(sources.on IN (0,1))
    );

    CREATE TABLE IF NOT EXISTS records(
        id INTEGER NOT NULL CONSTRAINT PK_records PRIMARY KEY AUTOINCREMENT,
        src_id TEXT NOT NULL,
        data BLOB NOT NULL,
        sent BOOLEAN NOT NULL CHECK(records.sent IN (0,1)),
        FOREIGN KEY(src) REFERENCES sources(src) ON DELETE CASCADE
    );
"#;

pub async fn init_db(db_file_path:&str) -> Result<Pool<Sqlite>, Box<dyn Error>> {

    let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(db_file_path)
        .create_if_missing(true);
    
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;
    
    let mut tx = pool.begin().await?;
    
    sqlx::query("PRAGMA journal_mode=WAL;").execute(&mut *tx).await?;
    sqlx::query("BEGIN IMMEDIATE;").execute(&mut *tx).await?;
    sqlx::query(INIT_DB_SCRIPT).execute(&mut *tx).await?;
    
    let settings_values = [
        (defaults::BROKER_CONFIGURATION_KEY, "{}"),
        (defaults::CLEAR_DATA_DELAY_KEY, "3600"),
        (defaults::DATA_FLOW_RECONNECT_DELAY_KEY, "10000"),
        (defaults::DATA_SENDING_DELAY_KEY, "1000"),
        (defaults::DESCRIPTION_KEY, "Embedded broker"),
        (defaults::MAX_COUNT_DATA_ROWS_KEY, "1000000"),
        (defaults::MODIFIED_TICKS_KEY, "0"),
        (defaults::PACKET_SIZE_KEY, "1000"),
        (defaults::VIDEO_SEGMENTS_EXPIRATION_KEY, "72"),
    ];
    
    let placeholders = settings_values.iter().map(|_| "(?, ?)").collect::<Vec<_>>().join(", ");
    let query = format!("INSERT OR IGNORE INTO settings (key, value) VALUES {}", placeholders);
    let mut sql = sqlx::query(&query);
    
    for setting in settings_values {
        sql = sql
            .bind(setting.0.to_string())
            .bind(setting.1.to_string());
    }
    
    sql.execute(&mut *tx).await?;
    
    tx.commit().await?;
    
    Ok(pool)
}