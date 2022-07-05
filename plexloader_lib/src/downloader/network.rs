use ureq::{Agent, Request, MiddlewareNext, Response, Error};
use serde::de::DeserializeOwned;
use crate::constants::*;
use crate::NetworkResponseError;

fn plex_default_header_middleware(req: Request, next: MiddlewareNext) -> Result<Response, Error> {
    next.handle(req.set(PLEX_CLIENT_IDENTIFIER_HEADER_NAME, PLEX_CLIENT_IDENTIFIER_HEADER_VALUE)
        .set(PLEX_PRODUCT_HEADER_NAME, PLEX_PRODUCT_HEADER_VALUE)
        .set(ACCEPT_JSON_HEADER_NAME, ACCEPT_JSON_HEADER_VALUE))
}

fn plex_agent() -> Agent {
    ureq::builder()
        .middleware(plex_default_header_middleware)
        .build()
}

fn plex_login_params<'a>(username: &'a str, password: &'a str) -> [(&'a str, &'a str); 2] {
    [(PLEX_LOGIN_USERNAME_FORM_NAME, username), (PLEX_LOGIN_PASSWORD_FORM_NAME, password)]
}

pub fn plex_servers(auth_token: &str) -> Result<Response, Error> {
    plex_agent()
        .get(PLEX_SERVER_ENDPOINT)
        .set(PLEX_TOKEN_HEADER_NAME, auth_token)
        .call()
}

pub fn plex_login(username: &str, password: &str) -> Result<Response, Error> {
    plex_agent()
        .post(PLEX_USER_SIGN_IN_ENDPOINT)
        .send_form(&plex_login_params(username, password))
}

pub fn get_json_from_response<T: DeserializeOwned>(req: Result<Response, Error>) -> Result<T,NetworkResponseError> {
    match req {
        Ok(response) => {
           match serde_json::from_reader(response.into_reader()) {
                Ok(response_json) => Ok(response_json),
                Err(e) => Err(NetworkResponseError::SerializationError(e))
          }
        },
        Err(Error::Status(401, _r)) => Err(NetworkResponseError::UnAuthorized),
        Err(e) => Err(NetworkResponseError::UnknownError(e))
    }
}
