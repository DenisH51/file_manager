use axum_extra::extract::{cookie::Cookie};
use crate::responses::flash_message::FlashCode;


pub fn create_flash_cookie(message: FlashCode) -> Cookie<'static> {
    let (message_type, code) = match message {
        FlashCode::SessionErrors(error) => {
            ("flash_error", error.flash_code())
        }


        FlashCode::Success(success) => {
            ("flash_success", success.flash_code())
        }
    };


    Cookie::build((message_type, code.to_owned()))
        .http_only(true)
        .path("/")
        .build()
}

