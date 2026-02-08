use crate::database::User;
use sqlx::SqlitePool;

pub(crate) async fn creator(
    pool: &SqlitePool, 
    name: &String, 
    password: &String
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("INSERT INTO users (name, password) VALUES (?, ?) RETURNING *")
        .bind(name)
        .bind(password)
        .fetch_one(pool)
        .await
}

pub(crate) async fn deleter(
    pool: &SqlitePool, 
    id: u32
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn updater(
    pool: &SqlitePool,
    id: u32, 
    name: &String, 
    password: &String
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("UPDATE users SET name = ?, password = ? WHERE id = ? RETURNING *")
        .bind(name)
        .bind(password)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn getter(
    pool: &SqlitePool, 
    id: u32
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn finder(
    pool: &SqlitePool, 
    name: &String
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await
}

#[cfg(test)]
mod tests {
    use crate::database::initialize_database;
    use super::*;

    #[actix_web::test]
    async fn lifetime() {
        let pool = initialize_database().await;

        if let Err(error) = creator(&pool, &"TestName".to_string(), &"TestPassword".to_string()).await {
            panic!("{}", error);
        }

        is_exact_test(getter(&pool, 1).await);
        is_exact_test(finder(&pool, &"TestName".to_string()).await);

        if let Err(error) = updater(&pool, 1, &"UpdatedName".to_string(), &"UpdatedPassword".to_string()).await {
            panic!("{}", error);
        }

        is_exact_updated(getter(&pool, 1).await);
        is_exact_updated(finder(&pool, &"UpdatedName".to_string()).await);

        if let Err(error) = deleter(&pool, 1).await {
            panic!("{}", error);
        }

        if let Ok(user) = getter(&pool, 1).await {
            panic!("{:?}", user)
        }
    }

    fn is_exact_test(from: Result<User, sqlx::Error>) {
        match from {
            Ok(user) => {
                assert_eq!(user.id, 1);
                assert_eq!(user.name, "TestName".to_string());
                assert_eq!(user.password, "TestPassword".to_string());
            },
            Err(error) => panic!("{}", error),
        }
    }
    
    fn is_exact_updated(from: Result<User, sqlx::Error>) {
        match from {
            Ok(user) => {
                assert_eq!(user.id, 1);
                assert_eq!(user.name, "UpdatedName".to_string());
                assert_eq!(user.password, "UpdatedPassword".to_string());
            },
            Err(error) => panic!("{}", error),
        }
    }
}