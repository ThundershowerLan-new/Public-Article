use serde::Deserialize;
use crate::database::{Article, User};

pub(crate) mod get;
pub(crate) mod post;
pub(crate) mod delete;
pub(crate) mod put;

#[derive(Deserialize)]
struct Request {
    pub(crate) user: User,
    pub(crate) article: Article,
}