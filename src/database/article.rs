use crate::database::Article;
use sqlx::SqlitePool;

pub(crate) async fn creator(
    pool: &SqlitePool, 
    title: &String, 
    body: &String, 
    creator: u32
) -> Result<Article, sqlx::Error> {
    sqlx::query_as::<_, Article>("INSERT INTO articles (title, body, creator) VALUES (?, ?, ?) RETURNING *")
        .bind(title)
        .bind(body)
        .bind(creator)
        .fetch_one(pool)
        .await
}

pub(crate) async fn deleter(
    pool: &SqlitePool, 
    id: u32
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn updater(
    pool: &SqlitePool, 
    id: u32, 
    title: &String, 
    body: &String
) -> Result<Article, sqlx::Error> {
    sqlx::query_as::<_, Article>("UPDATE articles SET title = ?, body = ? WHERE id = ? RETURNING *")
        .bind(title)
        .bind(body)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn getter(
    pool: &SqlitePool, 
    id: u32
) -> Result<Article, sqlx::Error> {
    sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn finder(
    pool: &SqlitePool,
    creator: u32
) -> Result<Vec<Article>, sqlx::Error> {
    sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE creator = ?")
        .bind(creator)
        .fetch_all(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialize_database;

    #[actix_web::test]
    async fn lifetime() {
        let pool = initialize_database().await;

        if let Err(error) = creator(&pool, &"TestTitle".to_string(), &"TestBody".to_string(), 0).await {
            panic!("{}", error);
        }

        match getter(&pool, 1).await {
            Ok(article) => {
                assert_eq!(article.id, 1);
                assert_eq!(article.title, "TestTitle".to_string());
                assert_eq!(article.body, "TestBody".to_string());
                assert_eq!(article.creator, 0);
            },
            Err(error) => panic!("{}", error),
        }

        if let Err(error) = updater(&pool, 1, &"UpdatedTitle".to_string(), &"UpdatedBody".to_string()).await {
            panic!("{}", error);
        }

        match getter(&pool, 1).await {
            Ok(article) => {
                assert_eq!(article.id, 1);
                assert_eq!(article.title, "UpdatedTitle".to_string());
                assert_eq!(article.body, "UpdatedBody".to_string());
                assert_eq!(article.creator, 0);
            },
            Err(error) => panic!("{}", error),
        }

        if let Err(error) = deleter(&pool, 1).await {
            panic!("{}", error);
        }

        if let Ok(article) = getter(&pool, 1).await {
            panic!("{:?}", article);
        }
    }
}