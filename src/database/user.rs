use libsql::{params, Connection, Error};
use crate::database::FromRow;

pub(crate) async fn creator<U: FromRow> (
    connection: &Connection,
    name: String,
    password: String
) -> Result<U, Error> {
    U::from_option_row(
        connection.query(
            "INSERT INTO users (name, password) VALUES (?, ?) RETURNING id",
            params!(name, password),
        ).await?.next().await?
    )
}

pub(crate) async fn deleter(
    connection: &Connection,
    id: u32
) -> Result<(), Error> {
    connection.execute(
        "DELETE FROM users WHERE id = ?",
        params!(id)
    ).await?;
    Ok(())
}

pub(crate) async fn updater(
    connection: &Connection,
    id: u32, 
    name: String,
    password: String
) -> Result<(), Error> {
    match connection.query(
            "UPDATE users SET name = ?, password = ? WHERE id = ?",
            params!(name, password, id),
    ).await?.next().await? {
        Some(_) => Ok(()),
        None => Err(Error::QueryReturnedNoRows)
    }
}

pub(crate) async fn getter<U: FromRow>(
    connection: &Connection,
    id: u32
) -> Result<U, Error> {
    U::from_option_row(
        connection.query(
            "SELECT name FROM users WHERE id = ?",
            params!(id)
        ).await?.next().await?
    )
}

pub(crate) async fn verifier_by_id(
    connection: &Connection,
    id: u32,
    password: String
) -> Result<u64, Error> {
    connection.execute(
        "SELECT 1 FROM users WHERE id = ? AND password = ?",
        params!(id, password)
    ).await
}

pub(crate) async fn verifier_by_name<U: FromRow>(
    connection: &Connection,
    name: String,
    password: String
) -> Result<U, Error> {
    U::from_option_row(
        connection.query(
            "SELECT id FROM users WHERE name = ? AND password = ?",
            params!(name, password)
        ).await?.next().await?
    )
}