use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    err: anyhow::Error,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl actix_web::error::ResponseError for CustomError {}

impl From<anyhow::Error> for CustomError {
    fn from(err: anyhow::Error) -> CustomError {
        CustomError { err }
    }
}

#[derive(Debug)]
pub enum AuthError {
    NoToken,
    TokenValidation,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NoToken => f.write_str("No token"),
            AuthError::TokenValidation => f.write_str("Incorrect token"),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::NoToken | AuthError::TokenValidation => StatusCode::UNAUTHORIZED,
        }
    }
}
