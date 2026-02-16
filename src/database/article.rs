use libsql::{params, Connection, Error};
use crate::database::FromRow;

pub(crate) async fn creator<A: FromRow> (
    connection: &Connection,
    title: String,
    body: String,
    creator: u32
) -> Result<A, Error> {
    A::from_option_row(
        connection.query(
            "INSERT INTO articles (title, body, creator) VALUES (?, ?, ?) RETURNING id",
            params!(title, body, creator)
        ).await?.next().await?
    )
}

pub(crate) async fn deleter(
    connection: &Connection,
    id: u32
) -> Result<(), Error> {
    connection.execute(
        "DELETE FROM articles WHERE id = ?",
        params!(id)
    ).await?;
    Ok(())
}

pub(crate) async fn updater(
    connection: &Connection,
    id: u32, 
    title: String,
    body: String
) -> Result<u64, Error> {
    connection.execute(
            "UPDATE articles SET title = ?, body = ? WHERE id = ?",
            params!(title, body, id)
    ).await
}

pub(crate) async fn getter<A: FromRow>(
    connection: &Connection,
    id: u32
) -> Result<A, Error> {
    A::from_option_row(
        connection.query(
            "SELECT title, body, creator FROM articles WHERE id = ?",
            params!(id)
        ).await?.next().await?
    )
}

pub(crate) async fn finder<A: FromRow>(
    connection: &Connection,
    creator: u32
) -> Result<Vec<A>, Error> {
    let mut articles = Vec::<A>::new();

    let mut rows = connection.query(
        "SELECT id, title FROM articles WHERE creator = ?",
        params!(creator)
    ).await?;

    while let Some(row) = rows.next().await? {
        articles.push(A::from_row(row)?)
    }

    Ok(articles)
}