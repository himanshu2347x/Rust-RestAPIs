use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;



pub enum AppError{
  DatabaseError(mongodb::error::Error),
  InvalidObjectId(String),
  TodoNotFound(String),
  ValidationError(String)  
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {  
            AppError::DatabaseError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err))
            }
            AppError::InvalidObjectId(id) => {
                (StatusCode::BAD_REQUEST, format!("Invalid ObjectId: {}", id))
            }
            AppError::TodoNotFound(id) => {
                (StatusCode::NOT_FOUND, format!("Todo not found with id: {}", id))
            }
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, format!("Validation error: {}", msg))
            }
        };
        let body = Json(json!({
          "error":error_message
        }));
    return (status,body).into_response();
  }

}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
