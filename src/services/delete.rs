use actix_web::{delete, web, HttpRequest, HttpResponse};
use libsql::Connection;
use crate::database;

#[delete("/{id}")]
pub(crate) async fn users(
    connection: web::Data<Connection>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();
    let password = request
            .cookie("password")
            .map_or("".to_string(), |cookie| cookie.value().to_string());

    match database::user::verifier_by_id(&connection, id, password).await {
        Ok(_) => database::user::deleter(&connection, id)
            .await
            .map_or(HttpResponse::InternalServerError(), |_| HttpResponse::NoContent()),
        Err(_) => HttpResponse::NotFound()
    }.finish()
}

#[delete("/{id}")]
pub(crate) async fn articles(
    connection: web::Data<Connection>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();
    let creator = request
        .cookie("id")
        .map_or(0, |cookie| cookie.value().parse().unwrap_or_default());
    let password = request
        .cookie("password")
        .map_or("".to_string(), |cookie| cookie.value().to_string());

    match database::user::verifier_by_id(&connection, creator, password).await {
        Ok(_) => database::article::deleter(&connection, id)
            .await
            .map_or(HttpResponse::InternalServerError(), |_| HttpResponse::NoContent()),
        Err(_) => HttpResponse::NotFound()
    }.finish()
}