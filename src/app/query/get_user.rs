use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUser {
    pub id: i64,
    pub username: String,
}

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: i64) -> Result<GetUser, AppError>;
}
pub struct GetUserQuery<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> GetUserQuery<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub async fn execute(&self, id: i64) -> Result<GetUser, AppError> {
        // many business logic could be here
        let user = self.repository.get(id).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::adapters::postgres::PostgresRepository;

    use super::*;

    #[sqlx::test]
    async fn test_get_user_query(pool: PgPool) {
        // Given
        let repository = PostgresRepository::new(pool.clone());
        let query = GetUserQuery::new(repository);

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
        )
        .bind("testuser")
        .bind("password")
        .fetch_one(&pool)
        .await
        .unwrap();

        // When
        let result = query.execute(user.id).await.unwrap();

        // Then
        assert_eq!(result.id, user.id);
        assert_eq!(result.username, "testuser");
    }
}
