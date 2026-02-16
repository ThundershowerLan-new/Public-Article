use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse};
use libsql::{Connection, Error, Row};
use serde::Serialize;
use crate::database;
use crate::database::FromRow;

#[derive(Serialize)]
struct User {
    name: String,
    articles: Vec<AttachedArticle>
}

#[derive(Serialize)]
struct Article {
    title: String,
    body: String,
    creator: u32
}

#[derive(Serialize)]
struct AttachedArticle {
    id: u32,
    title: String
}

impl FromRow for User {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            name: row.get(0)?,
            articles: Vec::new()
        })
    }

    fn from_option_row(option: Option<Row>) -> Result<Self, Error> {
        match option {
            Some(row) => Self::from_row(row),
            None => Err(Error::QueryReturnedNoRows)
        }
    }
}

impl FromRow for Article {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            title: row.get(0)?,
            body: row.get(1)?,
            creator: row.get(2)?
        })
    }

    fn from_option_row(option: Option<Row>) -> Result<Self, Error> {
        match option {
            Some(row) => Self::from_row(row),
            None => Err(Error::QueryReturnedNoRows)
        }
    }
}

impl FromRow for AttachedArticle {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(AttachedArticle {
            id: row.get(0)?,
            title: row.get(1)?,
        })
    }

    fn from_option_row(option: Option<Row>) -> Result<Self, Error> {
        match option {
            Some(row) => Self::from_row(row),
            None => Err(Error::QueryReturnedNoRows)
        }
    }
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
    connection: web::Data<Connection>,
    path: web::Path<u32>
) -> HttpResponse {
    let id = path.into_inner();

    match database::user::getter::<User>(&connection, id).await {
        Ok(mut current_user) => {
            current_user.articles = database::article::finder::<AttachedArticle>(&connection, id)
                .await
                .unwrap_or_default();
            HttpResponse::Ok().json(current_user)
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[get("/{id}")]
pub(crate) async fn articles(
    connection: web::Data<Connection>,
    path: web::Path<u32>
) -> HttpResponse {
    match database::article::getter::<Article>(&connection, path.into_inner()).await {
        Ok(current_article) => HttpResponse::Ok().json(current_article),
        Err(_) => HttpResponse::NotFound().finish()
    }
}