use crate::responses::errors::auth_err::{BusinessError, InfrastructureError, ValidationError};
use crate::models::api_error::ApiErrorResponse;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};


pub fn validation_response(error: ValidationError) -> Response{
    let response = ApiErrorResponse{
        field: error.field(),
        message: error.message(),
    };

    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(response),
    )
    .into_response()
}



pub fn business_response(error: BusinessError) -> Response{
    let response = ApiErrorResponse{
        field: error.field(),
        message: error.message(),
    };

    (
        StatusCode::UNAUTHORIZED,
        Json(response),
    ).into_response()
}


pub fn infrastructure_response(error: InfrastructureError) -> Response{
    let response = ApiErrorResponse{
        field: error.field(),
        message: error.message(),
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(response),
    ).into_response()
}
