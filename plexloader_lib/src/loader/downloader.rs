use std::process::Command;
use crate::utils::construct_token_header;
use std::process::{Child};
use std::path::Path;
use crate::MediaDownloadError;

pub fn download_aria(download_link: &str, dir_path: &Path ,out_file_path: &str, server_token: &str) -> Result<Child, MediaDownloadError> {
    let header_arg = format!("--header={}", construct_token_header(server_token));
    let outfile_arg = format!("-o {}", out_file_path);
    let dir_arg = format!("-d {}", dir_path.display());
    let child = Command::new("aria2c")
        .arg("-x 16")
        .arg("-s 16")
        .arg(header_arg)
        .arg(outfile_arg)
        .arg(dir_arg)
        .arg(download_link)
        .spawn()?;
    Ok(child)
}
