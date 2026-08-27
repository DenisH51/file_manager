use axum::response::Html;

use crate::templates;

pub async fn auth_home() -> Html<String> {
    Html(
        templates::AUTH_HOME.to_string()
    )
}