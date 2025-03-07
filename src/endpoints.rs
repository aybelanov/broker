use actix_web::{post, HttpResponse, Responder};

/// Receives data from a data source
#[post("/add")]
pub async fn receive_data()-> impl Responder {
    HttpResponse::Ok()
}