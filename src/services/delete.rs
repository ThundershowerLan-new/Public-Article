use actix_web::{delete, web, HttpRequest, HttpResponse};
use sqlx::{Error, SqlitePool};
use crate::database;
use crate::database::User;

#[delete("/{id}")]
pub(crate) async fn users(
    pool: web::Data<SqlitePool>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    match database::user::getter(pool.get_ref(), path.into_inner()).await {
        Ok(current_user) => {
            if current_user.password != if let Some(password) = request.cookie("password") { password.value().to_string() } else { "".to_string() } {
                return HttpResponse::Unauthorized().finish();
            }

            database::user::deleter(pool.get_ref(), current_user.id)
                .await
                .map(|current_user| HttpResponse::Ok().json(current_user))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        },
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
pub(crate) async fn articles(
    pool: web::Data<SqlitePool>,
    request: HttpRequest,
    path: web::Path<u32>
) -> HttpResponse {
    match database::article::getter(pool.get_ref(), path.into_inner()).await {
        Ok(current_article) => {
            if database::user::getter(pool.get_ref(), current_article.creator)
                    .await
                    .unwrap_or_else(|_| User::new(0, &"".to_string(), &"".to_string()))
                    .password != 
                if let Some(password) = request.cookie("password") { password.value().to_string() } else { "".to_string() } {
                return HttpResponse::Unauthorized().finish();
            }

            database::article::deleter(pool.get_ref(), current_article.id)
                .await
                .map(|current_article| HttpResponse::Ok().json(current_article))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        },
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}