use crate::config::settings;
use  super::DB_POOL;
use anyhow::Result;
use rusqlite::{params, TransactionBehavior};

/*
  Если сервер будет записывать в SQLite, убедитесь, что используется режим WAL (PRAGMA journal_mode=WAL),
  чтобы избежать блокировок при одновременных операциях.
*/
const INIT_DB_SCRIPT: &str = r#"
    PRAGMA journal_mode=WAL

    CREATE TABLE IF NOT EXISTS settings(
        key TEXT NOT NULL UNIQUE CONSTRAINT PK_settings PRIMARY KEY,
        value TEXT NOT NULL
    );
                    
    CREATE UNIQUE INDEX IF NOT EXISTS IX_settings_key ON settings (key);

    CREATE TABLE IF NOT EXISTS sources(
        src TEXT NOT NULL UNIQUE CONSTRAINT PK_sources PRIMARY KEY,
        cfg TEXT NULL,
        on BOOLEAN NOT NULL CHECK(sources.on IN (0,1))
    );

    CREATE TABLE IF NOT EXISTS records(
        id INTEGER NOT NULL CONSTRAINT PK_records PRIMARY KEY AUTOINCREMENT,
        src TEXT NOT NULL,
        data BLOB NOT NULL,
        sent BOOLEAN NOT NULL CHECK(records.sent IN (0,1)),
        FOREIGN KEY(src) REFERENCES sources(src) ON DELETE CASCADE
    );
"#;

pub fn init_db() -> Result<()> {
    let mut conn = DB_POOL.get()?;
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute(INIT_DB_SCRIPT, [])?;

    let settings_values = [
        (settings::BROKER_CONFIGURATION_KEY, "{}"),
        (settings::CLEAR_DATA_DELAY_KEY, "3600"),
        (settings::DATA_FLOW_RECONNECT_DELAY_KEY, "10000"),
        (settings::DATA_SENDING_DELAY_KEY, "1000"),
        (settings::DESCRIPTION_KEY, "Embedded broker"),
        (settings::MAX_COUNT_DATA_ROWS_KEY, "1000000"),
        (settings::MODIFIED_TICKS_KEY, "0"),
        (settings::PACKET_SIZE_KEY, "1000"),
        (settings::VIDEO_SEGMENTS_EXPIRATION_KEY, "72"),
    ];

    for (key, value) in settings_values {
        tx.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
    }

    tx.commit()?;
    
    Ok(())
}
