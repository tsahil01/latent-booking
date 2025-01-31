use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(Debug, Object)]
pub struct ErrorBody {
    pub message: String,
}

#[derive(ApiResponse, Debug)]
pub enum AppError {
    /// Database error (500)
    #[oai(status = 500)]
    Database(Json<ErrorBody>),

    /// Not found (404)
    #[oai(status = 404)]
    NotFound(Json<ErrorBody>),

    /// Invalid credentials (401)
    #[oai(status = 401)]
    InvalidCredentials(Json<ErrorBody>),

    /// Unauthorized (401)
    #[oai(status = 401)]
    Unauthorized(Json<ErrorBody>),

    /// Internal server error (500)
    #[oai(status = 500)]
    InternalServerError(Json<ErrorBody>),

    /// Bad request (400)
    #[oai(status = 400)]
    BadRequest(Json<ErrorBody>),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound(Json(ErrorBody {
                message: "Resource not found".to_string(),
            })),
            _ => AppError::Database(Json(ErrorBody {
                message: "Database error occurred".to_string(),
            })),
        }
    }
}
