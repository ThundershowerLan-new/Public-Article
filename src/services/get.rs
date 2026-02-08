use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse};
use sqlx::{Error, SqlitePool};
use crate::database;
use crate::database::User;

#[get("/coffee")]
pub(crate) async fn error() -> HttpResponse {
    HttpResponse::ImATeapot().body("Sorry, I'm a teapot!\n")
}

#[get("/favicon.ico")]
pub(crate) async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

#[get("/")]
pub(crate) async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./html/index.html")?)
}

#[get("/login")]
pub(crate) async fn login() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./html/login.html")?)
}

#[get("/create")]
pub(crate) async fn create() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./html/create.html")?)
}

#[get("")]
pub(crate) async fn user() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./html/user.html")?)
}

#[get("")]
pub(crate) async fn article() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./html/article.html")?)
}

#[get("/{id}")]
pub(crate) async fn users(
    pool: web::Data<SqlitePool>,
    path: web::Path<u32>
) -> HttpResponse {
    match database::user::getter(pool.get_ref(), path.into_inner()).await {
        Ok(current_user) => HttpResponse::Ok().json(User{
            password: "".to_string(),
            articles: match database::article::finder(pool.get_ref(), current_user.id).await {
                Ok(current_articles) => current_articles,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            },
            ..current_user
        }),
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}")]
pub(crate) async fn articles(
    pool: web::Data<SqlitePool>,
    path: web::Path<u32>
) -> HttpResponse {
    match database::article::getter(pool.get_ref(), path.into_inner()).await {
        Ok(current_article) => HttpResponse::Ok().json(current_article),
        Err(Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}