use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub field: &'static str,
    pub message: &'static str,
}