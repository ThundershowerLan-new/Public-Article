use crate::database;
use crate::database::{Article, User};
use crate::services::Request;
use actix_web::{post, web, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use sqlx::{Error, SqlitePool};

#[post("/coffee")]
pub(crate) async fn error() -> HttpResponse {
    HttpResponse::ImATeapot().body("Sorry, I'm a teapot!\n")
}

#[post("/")]
pub(crate) async fn index(
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let result =
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id > 0")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::InternalServerError().json({})
    }
}

#[post("")]
pub(crate) async fn user(
    pool: web::Data<SqlitePool>,
    data: web::Json<Request>
) -> HttpResponse {
    match database::user::finder(pool.get_ref(), &data.user.name).await {
        Ok(current_user) => {
            if current_user.password != data.user.password {
                return HttpResponse::Unauthorized().finish();
            }

            HttpResponse::Ok()
                .cookie(Cookie::build("password", &current_user.password)
                    .path("/")
                    .http_only(true)
                    .finish())
                .json(current_user)
        },
        Err(Error::RowNotFound) => {
            database::user::creator(pool.get_ref(), &data.user.name, &data.user.password)
                .await
                .map(|new_user| HttpResponse::Created()
                    .cookie(Cookie::build("password", &new_user.password)
                        .http_only(true)
                        .finish())
                    .json(new_user))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[post("")]
pub(crate) async fn article(
    pool: web::Data<SqlitePool>,
    data: web::Json<Request>,
    request: HttpRequest
) -> HttpResponse {
    match database::user::getter(pool.get_ref(), data.user.id).await {
        Ok(current_user) => {
            if current_user != (User{
                password: if let Some(password) = request.cookie("password") { password.value().to_string() } else { "".to_string() },
                ..data.user.clone()
            }) {
                return HttpResponse::Unauthorized().finish();
            }

            database::article::creator(pool.get_ref(), &data.article.title, &data.article.body, current_user.id)
                .await
                .map(|result| HttpResponse::Created().json(result))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
        },
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}