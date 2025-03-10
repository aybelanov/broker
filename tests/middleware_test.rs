use actix_web::{dev::{ServiceResponse}, test, web, App, Error, HttpResponse};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use actix_web::body::MessageBody;
use actix_web::dev::Service;
use actix_http::Request;
use actix_web::middleware::from_fn;
use broker::api::filters::only_private_ip;

// Function for creating a test application
async fn init_app() -> impl Service<
    Request,
    Response = ServiceResponse<impl MessageBody>,
    Error = Error,
> {
    test::init_service(
        App::new()
            .wrap(from_fn(only_private_ip))
            .route("/", web::get().to(|| async { 
                HttpResponse::Ok().body("Hello from private network!")
            })),
    ).await
}


// Auxiliary function for creating a Service Request with a specified IP address
fn create_request_with_ip(ip: Option<IpAddr>) -> Request {
    let mut req = test::TestRequest::get().uri("/");
    if let Some(ip) = ip {
        let socket_addr = SocketAddr::new(ip, 12345); // Порт не важен для теста
        req = req.peer_addr(socket_addr);
    }
    req.to_request()
}

// Tests
#[actix_web::test]
async fn test_private_ipv4_allowed() {
    let app = init_app().await;
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)); // Приватный IPv4
    let req = create_request_with_ip(Some(ip));
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Private IPv4 should be allowed");
}

#[actix_web::test]
#[should_panic(expected = "Access is allowed only from private IP addresses")]
async fn test_public_ipv4_forbidden() {
    let app = init_app().await;
    let ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)); // Публичный IPv4
    let req = create_request_with_ip(Some(ip));

    test::call_service(&app, req).await;
}

#[actix_web::test]
async fn test_private_ipv6_allowed() {
    let app = init_app().await;
    let ip = IpAddr::V6(Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1)); // Приватный IPv6 (ULA)
    let req = create_request_with_ip(Some(ip));
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Private IPv6 should be allowed");
}

#[actix_web::test]
#[should_panic(expected = "Access is allowed only from private IP addresses")]
async fn test_public_ipv6_forbidden() {
    let app = init_app().await;
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)); // Публичный IPv6
    let req = create_request_with_ip(Some(ip));

    test::call_service(&app, req).await;
}

#[actix_web::test]
#[should_panic(expected = "Unable to determine client IP address")]
async fn test_no_ip_forbidden() {
    let app = init_app().await;
    let req = create_request_with_ip(None); // Запрос без IP-адреса

    test::call_service(&app, req).await;
}

