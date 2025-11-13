use std::sync::Arc;

pub mod app;
pub mod di;
pub mod ports;

#[tokio::main]
async fn main() {
    let container = Arc::new(di::Container::new(
        app::query::get_hello_world::InMemoryRepository,
    ));
    let server = ports::httpapi::Server::new(3001, container);
    server.run().await;
}
