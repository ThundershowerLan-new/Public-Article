use actix_web::{put, web, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use sqlx::{Error, SqlitePool};
use crate::database;
use crate::database::User;
use crate::services::Request;

#[put("/{id}")]
pub(crate) async fn users(
    pool: web::Data<SqlitePool>,
    data: web::Json<Request>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();

    match database::user::getter(pool.get_ref(), id).await {
        Ok(current_user) => {
            if current_user.password != request.cookie("password")
                .map(|password| password.value().to_string())
                .unwrap_or_else(|| "".to_string()) {
                return HttpResponse::Unauthorized().finish();
            }

            database::user::updater(pool.get_ref(), id, &data.user.name, &data.user.password)
                .await
                .map(|updated_user| {
                    HttpResponse::Ok()
                        .cookie(Cookie::build("password", &updated_user.password)
                            .path("/")
                            .http_only(true)
                            .finish())
                        .json(updated_user)
                })
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        },
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[put("/{id}")]
pub(crate) async fn articles(
    pool: web::Data<SqlitePool>,
    data: web::Json<Request>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();

    match database::article::getter(pool.get_ref(), id).await {
        Ok(current_article) => {
            if database::user::getter(pool.get_ref(), current_article.creator)
                .await
                .unwrap_or_else(|_| User::new(0, &"".to_string(), &"".to_string()))
                .password !=
                request.cookie("password")
                    .map(|password| password.value().to_string())
                    .unwrap_or_else(|| "".to_string()) {
                return HttpResponse::Unauthorized().finish();
            }

            database::article::updater(pool.get_ref(), id, &data.article.title, &data.article.body)
                .await
                .map(|updated_article| HttpResponse::Ok().json(updated_article))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        },
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}