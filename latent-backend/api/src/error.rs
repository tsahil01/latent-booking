use poem_openapi::{payload::Json, ApiResponse};
use poem_openapi::Object;

#[derive(Debug, Object)]
pub struct ErrorBody {
    pub message: String,
}

#[derive(ApiResponse, Debug)]
pub enum AppError {
    /// Something went wrong internally (500)
    #[oai(status = 500)]
    InternalServerError(Json<ErrorBody>),
    
    // If you have other error scenarios, you can add more variants:
    // #[oai(status = 400)]
    // BadRequest(Json<ErrorBody>),
    // etc.
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        println!("{}", err);
        AppError::InternalServerError(Json(ErrorBody {
            message: "Error while updating DB".to_string(),
        }))
    }
}
