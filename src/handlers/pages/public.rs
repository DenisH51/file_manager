use::axum::response::Html;
use crate::templates;

pub async fn home() -> Html<String>{
    
    Html(
        templates::HOME.to_string()
    )

}


pub async fn login_page() -> Html<String>{
    Html(
        templates::LOGIN.to_string()
    )
}

pub async fn registrate_page() -> Html<String>{
    Html(
        templates::REGISTRATE.to_string()
    )
}