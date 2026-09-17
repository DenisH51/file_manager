
use axum_extra::extract::{CookieJar};
use axum::{extract::FromRequestParts};
use axum::http::request::Parts;
use std::convert::Infallible;
use crate::responses::{flash_message::{FlashCode, FlashKind, FlashMessage}};



pub fn get_flash_message(
    jar: &CookieJar,
) -> Option<FlashMessage> {

    for (key, kind) in [
        ("flash_error", FlashKind::Error),
        ("flash_success", FlashKind::Success),
    ] {
        if let Some(cookie) = jar.get(key) {

            let code = FlashCode::from_str(cookie.value());

            if let Some(code) = code {
                let message = match &code {
                    FlashCode::SessionErrors(error) => error.flash_message(),
                    FlashCode::Success(success) => success.flash_message(),
                };

                return Some(FlashMessage {
                    code,
                    kind,
                    message,
                });
            }
        }
    }

    None
}




pub struct Flash(pub Option<FlashMessage>);


impl<S> FromRequestParts<S> for Flash 
where 
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let jar = CookieJar::from_request_parts(parts, state).await?;

        let flash = get_flash_message(&jar);

        Ok(Flash(flash))
    }
}