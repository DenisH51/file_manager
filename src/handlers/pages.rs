use::axum::response::Html;

pub async fn home(){
    println!("home page handler");
}

pub async fn login_page(){
    println!("login page handler");
}

pub async fn register_page(){
    println!("register page handler");
}

pub async fn dashboard(){
    println!("dashboard handler");
}