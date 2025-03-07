use broker::*;

#[actix_web::main]
async fn main()->std::io::Result<()>  {
    
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("Starting broker application.");

    let app = app::start_app().await;
    
    app
}