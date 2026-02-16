mod services;
mod database;

use std::env;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use crate::database::initialize_database;
use crate::services::{delete, get, post, put};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = initialize_database().await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(Logger::default())
            .service(get::favicon)
            .service(Files::new("/static", "./static"))
            .service(get::index)
            .service(post::index)
            .service(get::login)
            .service(get::create)
            .service(web::scope("/user")
                .service(get::user)
                .service(post::user)
                .service(get::users)
                .service(delete::users)
                .service(put::users))
            .service(web::scope("/article")
                .service(get::article)
                .service(post::article)
                .service(get::articles)
                .service(delete::articles)
                .service(put::articles))
    })
        .bind(("0.0.0.0", env::var("PORT")
            .expect("Failed to get PORT")
            .parse()
            .expect("Failed to parse PORT")))?
        .run()
        .await
}
