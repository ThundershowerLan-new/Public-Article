use actix_web::{put, web, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use libsql::Connection;
use serde::Deserialize;
use crate::database;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String
}

#[derive(Deserialize)]
struct Article {
    title: String,
    body: String,
    creator: u32
}

#[put("/{id}")]
pub(crate) async fn users(
    connection: web::Data<Connection>,
    data: web::Json<User>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();
    let password = request
        .cookie("password")
        .map_or("".to_string(), |cookie| cookie.value().to_string());

    match database::user::verifier_by_id(&connection, id, password).await {
        Ok(_) => database::user::updater(&connection, id, data.name.clone(), data.password.clone())
                .await
                .map_or(HttpResponse::InternalServerError().finish(),
                        |_| {
                            HttpResponse::Ok()
                                .cookie(Cookie::build("password", data.password.clone())
                                    .path("/")
                                    .http_only(true)
                                    .finish())
                                .finish()
                        }),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[put("/{id}")]
pub(crate) async fn articles(
    connection: web::Data<Connection>,
    data: web::Json<Article>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();
    let password = request
        .cookie("password")
        .map_or("".to_string(), |cookie| cookie.value().to_string());

    match database::user::verifier_by_id(&connection, data.creator.clone(), password).await {
        Ok(_) => database::article::updater(&connection, id, data.title.clone(), data.body.clone())
            .await
            .map_or(HttpResponse::InternalServerError().finish(),
                    |_| HttpResponse::Ok().finish()),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}