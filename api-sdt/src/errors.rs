use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ApiSdtError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl std::error::Error for ApiSdtError {}

impl ApiSdtError {
    fn error_response(&self) -> String {
        match self {
            ApiSdtError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            ApiSdtError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            ApiSdtError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            ApiSdtError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for ApiSdtError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiSdtError::DBError(_msg) | ApiSdtError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiSdtError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            ApiSdtError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for ApiSdtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for ApiSdtError {
    fn from(err: actix_web::error::Error) -> Self {
        ApiSdtError::ActixError(err.to_string())
    }
}

impl From<mongodb::error::Error> for ApiSdtError {
    fn from(err: mongodb::error::Error) -> Self {
        ApiSdtError::DBError(err.to_string())
    }
}