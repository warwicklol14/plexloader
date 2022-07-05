use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlexUserResponse {
    pub user: PlexUser
}

#[derive(Deserialize, Debug)]
pub struct PlexUser {
    pub email: String,
    pub username: String,

    #[serde(rename = "authToken")]
    pub auth_token: String
}
