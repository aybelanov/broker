use std::{collections::HashMap, error::Error};
use crate::models::{Source, Record};
use sqlx::{Pool, Row, Sqlite};

 /// Gets a setting value by a key
pub async fn get_setting_by_key(pool:&Pool<Sqlite>, key:&str)
     -> Result<Option<String>, Box<dyn Error>> {
    
     let query = sqlx::query(
         "SELECT value FROM settings WHERE key = ?"
     ).bind(key);
     
     let row = query.fetch_one(pool).await?;
     let val = row.get("value");

     Ok(val)
}

/// Gets all setting dictionary
pub async fn get_all_setting(pool:&Pool<Sqlite>)
    -> Result<HashMap<String, String>, Box<dyn Error>> {
   
    let query = sqlx::query(
        "SELECT key, value FROM settings"
    );
    
    let rows = query
        .fetch_all(pool)
        .await?;

    let settings: HashMap<String, String> = rows
        .into_iter()
        .map(|row| {
            let key: String = row.get("key");
            let value: String = row.get("value");
            (key, value)
        })
        .collect();

    Ok(settings)
}

/// Gets all data sources
pub async fn get_all_sources(pool:&Pool<Sqlite>)
    ->Result<Vec<Source>, Box<dyn Error>> {

    let query = sqlx::query(
        r#"SELECT src_id, cfg, "on" FROM sources"#
    );

    let rows = query
        .fetch_all(pool)
        .await?;
    
    let sources: Vec<Source> = rows
        .into_iter()
        .map(|row| Source {
            src_id: row.get("src_id"),
            cfg: row.get("cfg"),       
            on: row.get("on"), 
        })
        .collect();
    
    Ok(sources)
}

/// Gets a data source by its name (as src_id)
pub async fn get_source_by_name(pool:&Pool<Sqlite>, name: &str)-> Result<Source, Box<dyn Error>>{
    
    let query = sqlx::query(
        r#"SELECT src_id, cfg, "on" FROM source WHERE name = ?"#
    ).bind(name);

    let row = query.fetch_one(pool).await?;
    let source = Source {
        src_id: row.get("src_id"),
        cfg: row.get("cfg"),
        on: row.get("on"),
    };
    
    Ok(source)
}

/// Adds the data source to database
pub async fn add_source(pool:&Pool<Sqlite>, source: &Source)->Result<(), Box<dyn Error>>{

    let mut tx = pool.begin().await?;
    
    let query = sqlx::query(
        r#"INSERT INTO source (src_id, cfg, "on") VALUES (?, ?, ?)"#
    )
        .bind(&source.src_id)
        .bind(&source.cfg)
        .bind(source.on);

    query.execute(&mut *tx).await?;
    tx.commit().await?;
    
    Ok(())
}

/// Updates the data source
pub async  fn update_source(pool:&Pool<Sqlite>, source: Source)->Result<(), Box<dyn Error>>{
    
    let query = sqlx::query(
        r#"UPDATE source SET cfg = ?, "on" = ? WHERE src_id = ?"#
    )
        .bind(&source.cfg)
        .bind(source.on)
        .bind(&source.src_id);
    
    query.execute(pool).await?;

    Ok(())
}

/// Deletes a data source by the src_id
pub async fn delete_source(pool: &Pool<Sqlite>, src_id: &str)-> Result<(), Box<dyn Error>>{
    
    let query = sqlx::query(
        r#"DELETE FROM source WHERE src_id = ?"#
    ).bind(src_id);
    
    query.execute(pool).await?;

    Ok(())
}

/// Gets last data records
pub async fn get_last_data(pool: &Pool<Sqlite>, count: &u32)
    -> Result<Vec<Record>, Box<dyn Error>> {

    let query = sqlx::query_as::<_, Record>(
        r#"
            SELECT * FROM records 
            WHERE sent = 0
            ORDER BY id DESC
            LIMIT ? OFFSET 0
        "#
    ).bind(count);

    let records = query.fetch_all(pool).await?;

    Ok(records)
}

/// Gets last data records of the data source
pub async fn get_data_by_src_id(pool: &Pool<Sqlite>, src_id: &str, count: &u32) 
    -> Result<Vec<Record>, Box<dyn Error>> {
    let query = sqlx::query_as::<_, Record>(
        r#"
            SELECT * FROM records 
            WHERE src_id = ? AND sent = 0
            ORDER BY id DESC
            LIMIT ? OFFSET 0
        "#
    )
        .bind(src_id)
        .bind(count);

    let records = query.fetch_all(pool).await?;

    Ok(records)
}

/// Adds data records to the database
pub async fn add_data(pool: &Pool<Sqlite>, records: &Vec<Record>)
    -> Result<Vec<u32>, Box<dyn Error>> {

    if records.is_empty() {
        return Ok(Vec::new());
    }

    let mut tx = pool.begin().await?;
    let mut ids = Vec::new();

    for record in records {
        let id = sqlx::query_scalar::<_, u32>(
            r#"INSERT INTO records (src_id, data, sent) VALUES (?, ?, ?) RETURNING id"#
        )
            .bind(&record.src_id)
            .bind(&record.data)
            .bind(record.sent)
            .fetch_one(&mut *tx)
            .await?;

        ids.push(id);
    }

    tx.commit().await?;

    Ok(ids)
}

/// Adds data records to the database
pub async fn bulk_add_data(pool: &Pool<Sqlite>, records: &Vec<Record>)
    -> Result<Vec<u32>, Box<dyn Error>> {

    if records.is_empty() {
        return Ok(Vec::new());
    }
    
    let placeholders: String = records
        .iter()
        .map(|_| "(?, ?, ?)") 
        .collect::<Vec<_>>()
        .join(", ");
    let query_str = format!(
        r#"INSERT INTO records (src_id, data, sent) VALUES {} RETURNING id"#,
        placeholders
    );
    
    let mut query = sqlx::query_scalar::<_, u32>(&query_str);
    
    for record in records {
        query = query
            .bind(&record.src_id)
            .bind(&record.data)
            .bind(record.sent);
    }
    
    let mut tx = pool.begin().await?;
    
    let ids = query.fetch_all(&mut *tx).await?;
    
    tx.commit().await?;

    Ok(ids)
}

/// Updates data records in the database
pub async fn update_data(pool: &Pool<Sqlite>, records: &Vec<Record>)
    -> Result<(), Box<dyn Error>> {

    if records.is_empty() {
        return Ok(());
    }
    
    let mut tx = pool.begin().await?;
    
    for record in records {
        let query = sqlx::query(
            r#"UPDATE records SET src_id = ?, data = ?, sent = ? WHERE id = ?"#
        )
            .bind(&record.src_id)
            .bind(&record.data)
            .bind(record.sent)
            .bind(record.id);

        query.execute(&mut *tx).await?;
    }
    
    tx.commit().await?;
    
    Ok(())
}

/// Deletes data by identifiers collection
pub async fn delete_data(pool: &Pool<Sqlite>, ids: Vec<u32>)
    -> Result<(), Box<dyn Error>>{

    if ids.is_empty() {
        return Ok(());
    }
    
    let placeholders: String = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query_str = format!(
        r#"DELETE FROM records WHERE id IN ({})"#,
        placeholders
    );
    
    let mut query = sqlx::query(&query_str);
    
    for id in &ids {
        query = query.bind(id);
    }
    
    let mut tx = pool.begin().await?;
    query.execute(&mut *tx).await?;
    tx.commit().await?;

    Ok(())
}

/// Deletes sent data records by identifiers collection
pub async fn delete_sent_data(pool: &Pool<Sqlite>)-> Result<(), Box<dyn Error>> {

    sqlx::query("DELETE FROM records WHERE sent = 1")
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::db::app_db::init_db;

    async fn setup_pool() -> Result<Pool<Sqlite>, Box<dyn Error>> {
        let pool = init_db("sqlite::memory:").await?;
        Ok(pool)
    }

    // Юнит-тесты для базовой функциональности
    #[tokio::test]
    async fn test_add_source() -> Result<(), Box<dyn Error>> {
        let pool = setup_pool().await?;
        let source = Source {
            src_id: "src1".to_string(),
            cfg: Some("cfg1".to_string()),
            on: true,
        };
        add_source(&pool, &source).await?;
        let result = get_source_by_name(&pool, "src1").await?;
        assert_eq!(result, source);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_last_data() -> Result<(), Box<dyn Error>> {
        let pool = setup_pool().await?;
        sqlx::query(r#"INSERT INTO records (src_id, data, sent) VALUES ('src1', X'0102', 0)"#)
            .execute(&pool)
            .await?;
        let result = get_last_data(&pool, &1).await?;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].src_id, "src1");
        Ok(())
    }
}