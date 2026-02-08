pub(crate) mod user;
pub(crate) mod article;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) password: String,
    #[sqlx(skip)]
    pub(crate) articles: Vec<Article>
}

#[derive(Clone, Debug, Deserialize, FromRow, PartialEq, Serialize)]
pub(crate) struct Article {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) creator: u32
}

impl User {
    pub(crate) fn new(id: u32, name: &String, password: &String) -> User {
        User{
            id,
            name: name.clone(),
            password: password.clone(),
            articles: Vec::<Article>::new()
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User::new(self.id, &self.name, &self.password)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.name == other.name &&
            self.password == other.password
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id ||
            self.name != other.name ||
            self.password != other.password
    }
}

impl Article {
    pub(crate) fn new(id: u32, title: &String, body: &String, creator: u32) -> Article {
        Article{
            id,
            title: title.clone(),
            body: body.clone(),
            creator
        }
    }
}

pub(crate) async fn initialize_database() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./database.db")
        .await
        .expect("Failed to initialize database")
}