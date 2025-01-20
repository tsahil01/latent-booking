use crate::Db;
use sqlx::Error;
use sqlx::{postgres::PgRow, FromRow};

#[derive(FromRow)]
struct User {
    id: String,
    number: String
}

impl Db {
    pub async fn create_user(&self, phone_number: String) -> Result<PgRow, Error> {
        sqlx::query("INSERT INTO users (number) VALUES ($1)")
            .bind(phone_number)
            .fetch_one(&self.client).await
    }

    pub async fn verify_user(&self, phone_number: String) -> Result<PgRow, Error> {
        sqlx::query("UPDATE TABLE users SET verified=true WHERE number=$1")
        .bind(phone_number)
        .fetch_one(&self.client).await
    }

}
