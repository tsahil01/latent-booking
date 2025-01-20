use config::Config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub mod user;
pub mod config;

#[derive(Clone)]
pub struct Db {
    client: Pool<Postgres>,
}

impl Db {
    pub async fn new() -> Self {
        let config = Config::default();
        let client = PgPoolOptions::new().max_connections(5).connect(&config.db_url).await.unwrap();

        let db = Db { 
            client
        };
        return db;
    }
}

