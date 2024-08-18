use crate::{pkg::repository::error::DatabaseError, preludes::log};
use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use derive_more::Display;
use serde::Serialize;

pub type ApiResult = Result<Response, ApiError>;

#[derive(Debug, thiserror::Error, Display)]
#[display("{self:?}")]
pub enum ApiError {
    DatabaseError(#[from] DatabaseError),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: Option<String>,
}

impl ApiError {
    fn log_error(&self) {
        match self {
            ApiError::DatabaseError(e) => match e {
                DatabaseError::Binding(e) => {
                    log!(error, "Database error: {:?}", e);
                }
                DatabaseError::Sql(e) => {
                    log!(error, "Database error: {:?}", e);
                }
                _ => {}
            },
        }
    }

    fn error_response(&self) -> ErrorResponse {
        match self {
            ApiError::DatabaseError(e) => match e {
                DatabaseError::Duplicate { message } => {
                    let duplicate = message.split(": ").last().map(|v| {
                        v.split(", ")
                            .flat_map(|x| x.split(".").last())
                            .collect::<Vec<_>>()
                    });

                    let formatted_message = match duplicate {
                        Some(values) if values.len() > 1 => {
                            format!("The following values already exist: {:?}", values)
                        }
                        Some(values) => format!("The following value already exists: {:?}", values),
                        None => "Duplicate entry".to_string(),
                    };
                    ErrorResponse {
                        code: StatusCode::CONFLICT.as_u16(),
                        error: "DuplicateError".to_string(),
                        message: Some(formatted_message),
                    }
                }
                DatabaseError::NotFound { message } => ErrorResponse {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    error: "NotFoundError".to_string(),
                    message: Some(message.clone()),
                },
                DatabaseError::Binding(_) | DatabaseError::Sql(_) => ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    error: "InternalServerError".to_string(),
                    message: Some("An internal error occurred".to_string()),
                },
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.log_error();
        let error_response = self.error_response();
        (
            StatusCode::from_u16(error_response.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            [(CONTENT_TYPE, "application/json")],
            Json(error_response),
        )
            .into_response()
    }
}
