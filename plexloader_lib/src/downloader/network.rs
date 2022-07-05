use ureq::{Agent, Request, MiddlewareNext, Response, Error};
use crate::constants::{PLEX_CLIENT_IDENTIFIER_HEADER_NAME, PLEX_CLIENT_IDENTIFIER_HEADER_VALUE, PLEX_PRODUCT_HEADER_NAME, PLEX_PRODUCT_HEADER_VALUE, ACCEPT_JSON_HEADER_VALUE, PLEX_USER_SIGN_IN_ENDPOINT, PLEX_LOGIN_USERNAME_FORM_NAME, PLEX_LOGIN_PASSWORD_FORM_NAME, PLEX_SERVER_ENDPOINT, PLEX_TOKEN_HEADER_NAME, ACCEPT_JSON_HEADER_NAME };
use crate::{PlexUser, PlexUserResponse, PlexServer};

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

pub fn plex_servers(auth_token: &str) -> Vec<PlexServer> {
    let response = plex_agent()
        .get(PLEX_SERVER_ENDPOINT)
        .set(PLEX_TOKEN_HEADER_NAME, auth_token)
        .call()
        .expect("Unable to get plex servers");
    response.into_json::<Vec<PlexServer>>().expect("Unable to deserialize to json")
}

pub fn plex_login(username: &str, password: &str) -> PlexUser {
    let response = plex_agent()
        .post(PLEX_USER_SIGN_IN_ENDPOINT)
        .send_form(&plex_login_params(username, password))
        .expect("Unable to send plex login request");
    response.into_json::<PlexUserResponse>().expect("Unable to deserialize to json").user
}
