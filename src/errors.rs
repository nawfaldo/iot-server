use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, From};
use sea_orm::DbErr;

#[derive(Debug, Display, From)]
pub enum CustomError {
    #[display("Database error: {}", _0)]
    DbError(DbErr),

    #[display("Invalid credentials")]
    InvalidCredentials,

    #[display("Unauthorized")]
    Unauthorized,

    #[display("{}", _0)]
    Other(String),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::DbError(_) => HttpResponse::InternalServerError().json("Database Error"),
            CustomError::InvalidCredentials => {
                HttpResponse::Unauthorized().json("Invalid Credentials")
            }
            CustomError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            CustomError::Other(msg) => HttpResponse::BadRequest().json(msg),
        }
    }
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::DbError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            CustomError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            CustomError::Other(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
