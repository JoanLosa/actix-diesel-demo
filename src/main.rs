#[macro_use]
extern crate derive_builder;

mod controller;
mod model;
mod service;
mod schema;
mod config;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port  = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let pool = config::db::init_db_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::app::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}
