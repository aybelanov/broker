use actix_web::{web, App, HttpServer};
use actix_web::middleware::from_fn;
use crate::config;
use crate::api::endpoints;
use crate::data::db;
use crate::common::defaults::{CFG_FILE_PATH, DB_FILE_PATH};
use crate::api::filters::only_private_ip;

pub async fn start_app() -> std::io::Result<()>  {
    // 1. initializes app configuration
    let cfg = config::get_config(CFG_FILE_PATH);
    log::info!("App configuration has been read successfully.");

    if !cfg.enabled {
        log::info!("Application is disabled, exiting.");
        return Ok(());
    }

    // 2. initializes SQLite file data base 
    let pool = db::init_db(DB_FILE_PATH)
        .await.unwrap();

    // 3. starts receiving data from the data sources
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(from_fn(only_private_ip))
            .service(endpoints::receive_data)
        //.route("/settings", web::get().to(get_settings))
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}