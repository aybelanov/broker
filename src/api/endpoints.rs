use actix_web::{HttpResponse, Responder, HttpRequest, web, post};
use bytes::Bytes;
use sqlx::SqlitePool;
use crate::data::rep;
use crate::models::Record;

#[post("/add")]
pub async fn receive_data (
    req: HttpRequest,
    body: Bytes,
    pool: web::Data<SqlitePool>,
) -> impl Responder {

    if body.len() == 0 {
        return  HttpResponse::InternalServerError().body("Empty data is not allowed.");
    }
    
    let source_id = match super::filters::validate_source_id(&req, &pool).await {
        Ok(source_id) => source_id,
        Err(response) => return response,
    };
    
    let record = Record {
        id: 0_u32,
        src_id: source_id,
        data: body.to_vec(),
        sent: false,
    };
    
    let arr = vec![record];
    let res = rep::add_data(&pool, &arr).await;
    
    match res {
        Ok(_) => HttpResponse::Ok().body(body.len().to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}