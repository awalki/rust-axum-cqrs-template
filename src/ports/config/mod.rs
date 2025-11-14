use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().expect("failed to load env")
});
