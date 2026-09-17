
use axum::response::Html;
use axum_extra::extract::CookieJar;

use crate::extractor::flash::Flash;
use crate::templates;


pub async fn home(Flash(flash): Flash, jar: CookieJar,) -> (CookieJar, Html<String>){
    
    let (flash_message, flash_class) = match flash {
        Some(flash) => {
            (flash.message, flash.css_class())
        }
        None => ("", ""),
    };

    let html = templates::HOME
        .replace("{{FLASH_MESSAGE}}", flash_message)
        .replace("{{FLASH_CLASS}}", flash_class);






    let jar = jar
        .remove("flash_error")
        .remove("flash_success");


    (jar, Html(html))

}


pub async fn login_page(Flash(flash): Flash, jar: CookieJar,) -> (CookieJar, Html<String>){
    
    
    let (flash_message, flash_class) = match flash {
        Some(flash) => {
            (flash.message, flash.css_class())
        }
        None => ("", ""),
    };

    let html = templates::LOGIN
        .replace("{{FLASH_MESSAGE}}", flash_message)
        .replace("{{FLASH_CLASS}}", flash_class);






    let jar = jar
        .remove("flash_error")
        .remove("flash_success");


    (jar, Html(html))
}

pub async fn register_page(Flash(flash): Flash, jar: CookieJar,) -> (CookieJar, Html<String>){
    let (flash_message, flash_class) = match flash {
        Some(flash) => {
            (flash.message, flash.css_class())
        }
        None => ("", ""),
    };

    let html = templates::REGISTER
        .replace("{{FLASH_MESSAGE}}", flash_message)
        .replace("{{FLASH_CLASS}}", flash_class);






    let jar = jar
        .remove("flash_error")
        .remove("flash_success");


    (jar, Html(html))
}









