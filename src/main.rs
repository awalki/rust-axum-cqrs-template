use std::sync::Arc;

use crate::{adapters::postgres::PostgresRepository, ports::config::CONFIG};

pub mod adapters;
pub mod app;
pub mod di;
pub mod error;
pub mod ports;

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect(&CONFIG.database_url).await.unwrap();
    let repo = PostgresRepository::new(pool.clone());

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let container = Arc::new(di::Container::new(repo.clone(), repo));
    let server = ports::httpapi::Server::new(3001, container);
    server.run().await;
}
