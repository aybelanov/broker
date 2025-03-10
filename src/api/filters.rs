use actix_web::{dev::{ServiceRequest, ServiceResponse}, web, Error, HttpRequest, HttpResponse};
use actix_web::body::MessageBody;
use actix_web::error::ErrorForbidden;
use actix_web::middleware::Next;
use sqlx::SqlitePool;
use crate::common::helpers;
use crate::data::rep;

/// Private IP address verification middleware (IPv4 и IPv6)
pub async fn only_private_ip (
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    
    // pre-processing
    let peer_addr = req.peer_addr()
        .ok_or_else(|| ErrorForbidden("Unable to determine client IP address"))?;
    
    let ip = peer_addr.ip();
    if !helpers::is_private_ip(&ip) {
        // return  Ok()
        return Err(ErrorForbidden("Access is allowed only from private IP addresses"));
    }
    
    next.call(req).await
    // post-processing
}

/// Checks X-Source-Id header
pub async fn validate_source_id(req: &HttpRequest, pool: &web::Data<SqlitePool>)
    -> Result<String, HttpResponse> {
  
    let source_id = req
        .headers()
        .get("X-Source-Id")
        .ok_or_else(|| HttpResponse::BadRequest().body("Missing X-Source-Id header"))?
        .to_str()
        .map_err(|_| HttpResponse::BadRequest().body("Invalid X-Source-Id header value"))?;

    if source_id.is_empty() {
        return Err(HttpResponse::BadRequest().body("X-Source-Id header cannot be empty"));
    }

    let source_entity = match  rep::get_source_by_id(&pool, &source_id).await {
        Ok(source) => source,
        Err(e) => return Err(HttpResponse::InternalServerError().body(e.to_string()))
    };
    
    let source = match source_entity {
        Some(source) => source,
        None => return Err(HttpResponse::Forbidden().body(
            format!("Source with ID {} does not registered. Access denied.", source_id))
        ),
    };
    
    if !source.active {
        return Err(HttpResponse::Forbidden().body(
            format!("Source with ID {} is disabled. Access denied.", source_id))
        );
    }
    
    Ok(source_id.to_string())
}