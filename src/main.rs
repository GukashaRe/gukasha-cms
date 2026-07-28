pub mod api_endpoints;
mod config;
mod db_connctor;
mod dntype;
pub mod mailer;
mod middleware;
pub mod resp;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::{Result, anyhow};
use config::create_or_load_config;
use db_connctor::connect_db;
use env_logger::Builder;
use log::{LevelFilter, error, info, warn};

use crate::mailer::create_mailer;
use api_endpoints::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::new();
    let config = match create_or_load_config().await {
        Ok(config) => config,
        Err(e) => return Err(e),
    };
    let config_clone = config.clone();
    if config_clone.debug_log {
        builder.filter_level(LevelFilter::Debug);
        builder.init();
    } else {
        builder.filter_level(LevelFilter::Info);
        builder.init();
    }
    info!("Server running at port {}", config_clone.serve_port);
    let mut listening_host = "127.0.0.1";
    if config_clone.unsafe_deploy {
        warn!(
            "WARNING: Server running in UNSAFE mode (unsafe_deploy=true) - DO NOT use in \
        production"
        );
        listening_host = "0.0.0.0";
    }
    let pool = connect_db(&config_clone.database_url, &mut config_clone.clone()).await;
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            error!("dbms connection fail: {}", e);
            return Err(anyhow!("server startup failed!"));
        }
    };
    let _mailer = create_mailer(&config_clone);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(middleware::SayHi)
            .wrap(cors)
            .wrap(
                actix_web::middleware::DefaultHeaders::new().add(("Powered-by", "GukashaProject")),
            )
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(pool.clone()))
            .service(health)
            .service(list_enterprises)
            .service(get_enterprise)
            .service(get_enterprise_credit)
    })
    .bind((listening_host, config_clone.serve_port))?
    .run()
    .await?;

    Ok(())
}
