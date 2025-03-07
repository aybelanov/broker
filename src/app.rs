use actix_web::{web, App, HttpServer};
use crate::config;
use crate::data::db;
use crate::defaults::{CFG_FILE_PATH, DB_FILE_PATH};
use crate::endpoints::receive_data;

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
            .service(receive_data)
        //.route("/settings", web::get().to(get_settings))
    })
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}