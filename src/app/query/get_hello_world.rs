use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn get_hello_world(&self) -> &'static str;
}
pub struct GetHelloWorldQuery<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> GetHelloWorldQuery<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub async fn execute(&self) -> &'static str {
        // many business logic could be here
        self.repository.get_hello_world().await
    }
}

#[derive(Default)]
pub struct InMemoryRepository;

#[async_trait]
impl Repository for InMemoryRepository {
    async fn get_hello_world(&self) -> &'static str {
        "Hello, World!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hello_world_query() {
        // Given
        let repository = InMemoryRepository;
        let query = GetHelloWorldQuery::new(repository);

        // When
        let result = query.execute().await;

        // Then
        assert_eq!(result, "Hello, World!");
    }
}
