use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use actix_http::body::MessageBody;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, web, App, Error};
use actix_web::middleware::from_fn;
use bytes::Bytes;
use broker::api::endpoints;
use broker::api::filters::only_private_ip;
use broker::data::db::init_db_in_memory;
use broker::data::rep;
use broker::models::Source;

async fn init_app() -> impl Service<
    Request,
    Response = ServiceResponse<impl MessageBody>,
    Error = Error,
> {
    let pool = init_db_in_memory().await.unwrap();

    rep::add_source(&pool,&Source{
        src_id: "src1".to_string(),
        cfg: None,
        active: true
    }).await.unwrap();

    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(from_fn(only_private_ip))
            .service(endpoints::receive_data),
    ).await
}

#[actix_web::test]
async fn test_save_data_success() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)); 
    let socket_addr = SocketAddr::new(ip, 12345); 

    let mut req = test::TestRequest::post().uri("/add");
    req = req.peer_addr(socket_addr);
    req = req.insert_header(("X-Source-Id", "src1"));
    req = req.set_payload(Bytes::from("a")); // Используем Bytes

    let app = init_app().await;
    let http_req = req.to_request();
    let resp = test::call_service(&app, http_req).await;

    assert_eq!(resp.status(), 200); // Проверяем статус ответа

    let body = test::read_body(resp).await;
    assert_eq!(body, "1");
}

// empty payload is not allowed and must call error 500
#[actix_web::test]
async fn test_save_data_error() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let socket_addr = SocketAddr::new(ip, 12345);

    let mut req = test::TestRequest::post().uri("/add");
    req = req.peer_addr(socket_addr);
    req = req.insert_header(("X-Source-Id", "src1"));
    
    let app = init_app().await;
    let http_req = req.to_request();
    let resp = test::call_service(&app, http_req).await;

    assert_eq!(resp.status(), 500);
}