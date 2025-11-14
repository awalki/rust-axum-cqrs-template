use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    app::{
        command::create_user::UserWriteRepository,
        query::get_user::{GetUser, User, UserRepository},
    },
    error::AppError,
};

#[derive(Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn get(&self, id: i64) -> Result<GetUser, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .ok();

        match user {
            Some(u) => Ok(GetUser {
                id: u.id,
                username: u.username,
            }),
            None => Err(AppError::NotFound),
        }
    }
}

#[async_trait]
impl UserWriteRepository for PostgresRepository {
    async fn create(&self, username: String, password: String) -> Result<User, AppError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::InternalError)
    }
}
