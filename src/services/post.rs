use crate::database;
use actix_web::{post, web, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use libsql::{params, Connection, Error, Row};
use serde::{Deserialize, Serialize};
use crate::database::FromRow;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String
}

#[derive(Deserialize)]
struct Article {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct Return {
    id: u32
}

#[derive(Serialize)]
struct ShowableArticle {
    id: u32,
    title: String
}

impl FromRow for Return {
    fn from_row(row: Row) -> Result<Self, Error> {
        Ok(Self {
            id: row.get(0)?
        })
    }

    fn from_option_row(option: Option<Row>) -> Result<Self, Error> {
        match option {
            Some(row) => Self::from_row(row),
            None => Err(Error::QueryReturnedNoRows)
        }
    }
}

#[post("/")]
pub(crate) async fn index(
    connection: web::Data<Connection>
) -> HttpResponse {
    if let Ok(mut rows) = connection.query(
        "SELECT id, title FROM articles",
        params!()
    ).await {
        let mut articles = Vec::<ShowableArticle>::new();

        let _ = rows.next().await;
        while let Ok(option) = rows.next().await {
            match option {
                Some(row) => {
                    articles.push(ShowableArticle {
                        id: row.get(0).unwrap_or_default(),
                        title: row.get(1).unwrap_or_default()
                    })
                },
                None => break
            }
        }

        HttpResponse::Ok().json(articles)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("")]
pub(crate) async fn user(
    connection: web::Data<Connection>,
    data: web::Json<User>
) -> HttpResponse {
    match database::user::verifier_by_name::<Return>(&connection, data.name.clone(), data.password.clone()).await {
        Ok(id) => {
            HttpResponse::Ok()
                .cookie(Cookie::build("password", &data.password)
                    .path("/")
                    .http_only(true)
                    .finish())
                .json(id)
        },
        Err(_) => {
            database::user::creator::<Return>(&connection, data.name.clone(), data.password.clone())
                .await
                .map_or(HttpResponse::InternalServerError().finish(),
                        |id| HttpResponse::Created()
                    .cookie(Cookie::build("password", &data.password)
                        .http_only(true)
                        .finish())
                    .json(id))
        }
    }
}

#[post("")]
pub(crate) async fn article(
    connection: web::Data<Connection>,
    data: web::Json<Article>,
    request: HttpRequest
) -> HttpResponse {
    let creator = request
        .cookie("id")
        .map_or(0, |cookie| cookie.value().parse().unwrap_or_default());
    let password = request
        .cookie("password")
        .map_or("".to_string(), |cookie| cookie.value().to_string());

    match database::user::verifier_by_id(&connection, creator, password).await {
        Ok(_) => {
            database::article::creator::<Return>(&connection, data.title.clone(), data.body.clone(), creator)
                .await
                .map_or(HttpResponse::InternalServerError().finish(),
                        |id| HttpResponse::Created().json(id))
        },
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}