use std::error::Error;
use actix_web::body::to_bytes;
use actix_web::http::header;
use actix_web::test::TestRequest;
use actix_web::web::Data;
use sqlx::SqlitePool;
use broker::api::filters::validate_source_id;
use broker::data::db::init_db_in_memory;
use broker::data::rep;
use broker::models::Source;

async fn setup_pool() -> Result<Data<SqlitePool>, Box<dyn Error>> {
    let pool = init_db_in_memory().await?;
    let res = Data::new(pool);
    Ok(res)
}

#[tokio::test]
async fn test_validate_source_id_missing_header() -> Result<(), Box<dyn Error>> {
    let req = TestRequest::default().to_http_request();
    let pool = Data::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());

    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    if let Err(response) = result {
        assert_eq!(response.status(), 400);
        // Extracting the response body as Bytes
        let body_bytes = to_bytes(response.into_body()).await?;
        // Converting Bytes to a string for comparison
        assert_eq!(body_bytes, "Missing X-Source-Id header");
    }
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_invalid_header()-> Result<(), Box<dyn Error>> {
    // Создаём запрос с невалидным заголовком (не UTF-8)
    let req = TestRequest::default()
        .insert_header((header::HeaderName::from_static("x-source-id"), vec![0xFF]))
        .to_http_request();
    
    let pool = Data::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());

    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    if let Err(response) = result {
        assert_eq!(response.status(), 400);
        let body_bytes = to_bytes(response.into_body()).await?;
        assert_eq!(
            body_bytes,
            "Invalid X-Source-Id header value"
        );
    }
    
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_empty_header() -> Result<(), Box<dyn Error>> {
    let req = TestRequest::default()
        .insert_header(("X-Source-Id", ""))
        .to_http_request();
    let pool = Data::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());

    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    if let Err(response) = result {
        assert_eq!(response.status(), 400);
        let body_bytes = to_bytes(response.into_body()).await?;
        assert_eq!(
            body_bytes,
            "X-Source-Id header cannot be empty"
        );
    }
    
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_source_not_found() -> Result<(), Box<dyn Error>> {
    
    let req = TestRequest::default()
        .insert_header(("X-Source-Id", "src1"))
        .to_http_request();
   
    let pool = setup_pool().await?;

    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    if let Err(response) = result {
        assert_eq!(response.status(), 403);
        let body_bytes = to_bytes(response.into_body()).await?;
        assert_eq!(
           body_bytes,
           format!("Source with ID {} does not registered. Access denied.","src1"),
        );
    }
    
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_source_disabled() -> Result<(), Box<dyn Error>> {
    
    let pool = setup_pool().await?;
  
    let source = Source{
        src_id: "src1".to_string(),
        cfg: None,
        active: false,
    };
    
    rep::add_source(&pool, &source).await?;
    
    let req = TestRequest::default()
        .insert_header(("X-Source-Id", "src1"))
        .to_http_request();
    
    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    
    if let Err(response) = result {
        assert_eq!(response.status(), 403);
        let body_bytes = to_bytes(response.into_body()).await?;
        assert_eq!(
            body_bytes,
            format!("Source with ID {} is disabled. Access denied.","src1"),
        );
    }
    
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_success() -> Result<(), Box<dyn Error>>{
  
    let pool = setup_pool().await?;

    let source = Source {
        src_id: "src1".to_string(),
        cfg: None,
        active: true,
    };

    rep::add_source(&pool, &source).await?;
  
    let req = TestRequest::default()
        .insert_header(("X-Source-Id", "src1"))
        .to_http_request();
    
    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "src1");
    
    Ok(())
}

#[tokio::test]
async fn test_validate_source_id_database_error() -> Result<(), Box<dyn Error>> {
    
    let req = TestRequest::default()
        .insert_header(("X-Source-Id", "src1"))
        .to_http_request();
   
    let pool = Data::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());
    
    let result = validate_source_id(&req, &pool).await;

    assert!(result.is_err());
    if let Err(response) = result {
        assert_eq!(response.status(), 500);
        let body_bytes = to_bytes(response.into_body()).await?;
        assert_eq!(
            body_bytes,
            "error returned from database: (code: 1) no such table: sources"
        );
    }

    Ok(())
}