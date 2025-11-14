use async_trait::async_trait;

use crate::{app::query::get_user::User, error::AppError};

#[async_trait]
pub trait UserWriteRepository {
    async fn create(&self, username: String, password: String) -> Result<User, AppError>;
}

pub struct CreateUserCommand<R>
where
    R: UserWriteRepository,
{
    repository: R,
}

impl<R> CreateUserCommand<R>
where
    R: UserWriteRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, username: String, password: String) -> Result<User, AppError> {
        // many business logic could be here
        let user = self.repository.create(username, password).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::postgres::PostgresRepository;

    #[sqlx::test]
    async fn test_create_user_command(pool: sqlx::PgPool) {
        // Given
        let repository = PostgresRepository::new(pool);
        let command = CreateUserCommand::new(repository);

        // When
        let result = command
            .execute("testuser".to_owned(), "password".to_owned())
            .await
            .unwrap();

        // Then
        assert_eq!(result.username, "testuser");
    }

    #[sqlx::test]
    async fn test_create_user_command_duplicate_username(pool: sqlx::PgPool) {
        // Given
        let repository = PostgresRepository::new(pool.clone());
        let command = CreateUserCommand::new(repository.clone());

        // When
        let _ = command
            .execute("testuser".to_owned(), "password".to_owned())
            .await
            .unwrap();
        let result = command
            .execute("testuser".to_owned(), "newpassword".to_owned())
            .await;

        // Then
        assert!(matches!(result, Err(AppError::InternalError)));
    }
}
