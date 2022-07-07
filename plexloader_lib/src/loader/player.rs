use std::process::Command;
use crate::utils::construct_token_header;
use std::process::{Child};
use crate::MediaPlaybackError;

pub fn play_media_mpv(playback_link: &str, media_title: &str ,server_token: &str) -> Result<Child, MediaPlaybackError> {
    let header_arg = format!("--http-header-fields-add={}", construct_token_header(server_token));
    let media_title_arg = format!("--force-media-title={}", media_title);
    let child = Command::new("mpv")
        .arg(header_arg)
        .arg(media_title_arg)
        .arg(playback_link)
        .spawn()?;
    Ok(child)
}
