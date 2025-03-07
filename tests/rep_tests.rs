use broker::*;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::error::Error;
use anyhow::Result;
use broker::data::db::init_db_in_memory;
use broker::models::{Record, Source};

async fn setup_pool() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let pool = init_db_in_memory().await?;
    Ok(pool)
}

#[tokio::test]
async fn test_get_setting_by_key() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    
    sqlx::query("INSERT INTO settings (key, value) VALUES ('test_key', 'test_value')")
        .execute(&pool)
        .await?;
    
    let result = get_setting_by_key(&pool, "test_key").await?;
    assert_eq!(result, Some("test_value".to_string()));
    
    let result = get_setting_by_key(&pool, "non_existent").await?;
    assert_eq!(result, None);
    
    Ok(())
}

#[tokio::test]
async fn test_get_all_setting() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    sqlx::query("INSERT INTO settings (key, value) VALUES ('key1', 'val1'), ('key2', 'val2')")
        .execute(&pool)
        .await?;
    let result = get_all_setting(&pool).await?;
    let expected: HashMap<String, String> = HashMap::from([
        ("key1".to_string(), "val1".to_string()),
        ("key2".to_string(), "val2".to_string()),
    ]);
    assert_eq!(result.len(), expected.len() + 9);
    Ok(())
}

#[tokio::test]
async fn test_get_all_setting2() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    let result = get_all_setting(&pool).await?;
    assert_eq!(result[defaults::DESCRIPTION_KEY], "Embedded broker");
    Ok(())
}

#[tokio::test]
async fn test_get_all_sources() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    sqlx::query(
        r#"
                INSERT INTO sources (src_id, cfg, active)
                VALUES ('src1', 'cfg1', 1), ('src2', NULL, 0)
             "#
    )
        .execute(&pool)
        .await?;
    let result = get_all_sources(&pool).await?;
    let expected = vec![
        Source { src_id: "src1".to_string(), cfg: Some("cfg1".to_string()), active: true },
        Source { src_id: "src2".to_string(), cfg: None, active: false },
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[tokio::test]
async fn test_get_source_by_name() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    sqlx::query(r#"INSERT INTO sources (src_id, cfg, active) VALUES ('src1', 'cfg1', 1)"#)
        .execute(&pool)
        .await?;
    let result = get_source_by_name(&pool, "src1").await?;
    let expected = Source { src_id: "src1".to_string(), cfg: Some("cfg1".to_string()), active: true };
    assert_eq!(result, expected);
    Ok(())
}

#[tokio::test]
async fn test_update_source() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    sqlx::query(r#"INSERT INTO sources (src_id, cfg, active) VALUES ('src1', 'old_cfg', 0)"#)
        .execute(&pool)
        .await?;
    let updated_source = Source {
        src_id: "src1".to_string(),
        cfg: Some("new_cfg".to_string()),
        active: true,
    };
    update_source(&pool, updated_source.clone()).await?;
    let result = get_source_by_name(&pool, "src1").await?;
    assert_eq!(result, updated_source);
    Ok(())
}

#[tokio::test]
async fn test_delete_source() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;
    sqlx::query(r#"INSERT INTO sources (src_id, cfg, active) VALUES ('src1', 'cfg1', 1)"#)
        .execute(&pool)
        .await?;
    delete_source(&pool, "src1").await?;
    let result = get_source_by_name(&pool, "src1").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_get_data_by_src_id() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(r#"INSERT INTO sources (src_id, cfg, active) VALUES ('src1', 'cfg1', 1)"#)
        .execute(&pool)
        .await?;
    
    sqlx::query(
        r#"
                INSERT INTO records (src_id, data, sent)
                VALUES ('src1', X'0102', 0), ('src1', X'0304', 0)
             "#
    )
        .execute(&pool)
        .await?;
    let result = get_data_by_src_id(&pool, "src1", &2).await?;
    let expected = vec![
        Record { id: 2, src_id: "src1".to_string(), data: vec![3, 4], sent: false },
        Record { id: 1, src_id: "src1".to_string(), data: vec![1, 2], sent: false },
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[tokio::test]
async fn test_add_data() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(r#"INSERT INTO sources (src_id, cfg, active) VALUES ('src1', 'cfg1', 1)"#)
        .execute(&pool)
        .await?;
    
    let records = vec![
        Record { id: 0, src_id: "src1".to_string(), data: vec![1, 2], sent: false },
    ];
    let ids = add_data(&pool, &records).await?;
    assert_eq!(ids, vec![1]);
    let result = get_last_data(&pool, &1).await?;
    assert_eq!(result[0].src_id, "src1");
    Ok(())
}

#[tokio::test]
async fn test_bulk_add_data() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(
        r#"
            INSERT INTO sources (src_id, cfg, active) 
            VALUES ('src1', 'cfg1', 1),
                   ('src2', 'cfg2', 0)
        "#
    )
        .execute(&pool)
        .await?;
    
    let records = vec![
        Record { id: 0, src_id: "src1".to_string(), data: vec![1, 2], sent: false },
        Record { id: 0, src_id: "src2".to_string(), data: vec![3, 4], sent: false },
    ];
    let ids = bulk_add_data(&pool, &records).await?;
    assert_eq!(ids.len(), 2);
    let result = get_last_data(&pool, &2).await?;
    assert_eq!(result.len(), 2);
    Ok(())
}

#[tokio::test]
async fn test_update_data() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(
        r#"
            INSERT INTO sources (src_id, cfg, active) 
            VALUES ('src1', 'cfg1', 1),
                   ('src2', 'cfg2', 0)
        "#
    )
        .execute(&pool)
        .await?;
    
    sqlx::query(r#"INSERT INTO records (src_id, data, sent) VALUES ('src1', X'0102', 0)"#)
        .execute(&pool)
        .await?;
    
    let records = vec![
        Record { id: 1, src_id: "src2".to_string(), data: vec![3, 4], sent: true },
    ];
    
    update_data(&pool, &records).await?;
    
    let updated = sqlx::query_as::<_, Record>("SELECT * FROM records WHERE id = 1")
        .fetch_one(&pool)
        .await?;
    
    assert_eq!(updated.src_id, "src2");
    Ok(())
}

#[tokio::test]
async fn test_delete_data() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(
        r#"
            INSERT INTO sources (src_id, cfg, active) 
            VALUES ('src1', 'cfg1', 1)
        "#
    )
        .execute(&pool)
        .await?;
    
    sqlx::query(r#"INSERT INTO records (src_id, data, sent) VALUES ('src1', X'0102', 0)"#)
        .execute(&pool)
        .await?;
    delete_data(&pool, vec![1]).await?;
    let result = get_last_data(&pool, &1).await?;
    assert_eq!(result.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_delete_sent_data() -> Result<(), Box<dyn Error>> {
    let pool = setup_pool().await?;

    sqlx::query(
        r#"
            INSERT INTO sources (src_id, cfg, active) 
            VALUES ('src1', 'cfg1', 1),
                   ('src2', 'cfg2', 0)
        "#
    )
        .execute(&pool)
        .await?;
    
    sqlx::query(
        r#"
                INSERT INTO records (src_id, data, sent)
                VALUES ('src1', X'0102', 1), ('src2', X'0304', 0)
             "#
   )
        .execute(&pool)
        .await?;
    
    delete_sent_data(&pool).await?;
    let result = get_last_data(&pool, &2).await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].src_id, "src2");
    Ok(())
}