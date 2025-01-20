use serde::{Deserialize, Serialize};
use poem::{web::{Data, Json}, Error};
use poem_openapi::OpenApi;
use crate::{error::AppError, Api, AppState};
use poem_openapi::Object;

#[derive(Serialize, Deserialize)]
struct CreateUser {
    number: String,
}

#[derive(Deserialize, Serialize, Object)]
struct CreateUserResponse {
    message: String
}

#[derive(Serialize, Deserialize)]
struct CreateUserVerify {
    number: String,
    totp: String
}

#[derive(Deserialize, Serialize, Object)]
struct VerifyUserResponse {
    token: String,
    message: String
}

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "post")]
    async fn create_user(&self, body: Json<CreateUser>, state: Data<&AppState>) -> poem::Result<poem_openapi::payload::Json<CreateUserResponse>, AppError> {
        let number = body.0.number;
        state.db.create_user(number).await?;

        Ok(poem_openapi::payload::Json(CreateUserResponse {
            message: "User created".to_string()
        }))
    }

    #[oai(path = "/verify", method = "post")]
    async fn create_user_verify(&self, body: Json<<CreateUserVerify>) -> poem::Result<poem_openapi::payload::Json<<VerifyUserResponse>>{
        let number = body.0.number;
        state.db.verify_user(number)
    }

    async fn login_using_otp(&self, body: Json<LoginUsingOtp>)
}