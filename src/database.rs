pub(crate) mod user;
pub(crate) mod article;

use std::env;
use actix_web::web::Data;
use libsql::{Builder, Connection, Error, Row};

pub(crate) trait FromRow {
    fn from_row(row: Row) -> Result<Self, Error> where Self: Sized;
    fn from_option_row(option: Option<Row>) -> Result<Self, Error> where Self: Sized;
}


pub(crate) async fn initialize_database() -> Data<Connection> {
    Data::new(
        Builder::new_remote(
            env::var("URL")
                .expect("Failed to get URL"),
            env::var("TOKEN")
                .expect("Failed to get TOKEN")
        )
        .build()
        .await
        .expect("Failed to initialize database")
        .connect()
        .expect("Failed to connect to database")
    )
}