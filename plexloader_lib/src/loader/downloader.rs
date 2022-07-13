use std::process::Command;
use crate::utils::construct_token_header;
use std::process::{Child};
use std::path::{Path, PathBuf};
use crate::MediaDownloadError;
use crate::utils::fs::create_dir;

use crate::{PlexVideoResource, PlexDirectoryResource, PlexDirectoryChild};

pub fn download_video_resource(video: &PlexVideoResource, download_dir: &Path) -> Result<(), MediaDownloadError> {
    let mut download_dir = PathBuf::from(download_dir);
    download_dir.push(&video.title);
    create_dir(&download_dir)?;
    let mut aria_proc = download_aria(&video.resource_path, &download_dir, &video.file_name, &video.access_token)?;
    aria_proc.wait()?;
    Ok(())
}

fn download_directory_child(child: &PlexDirectoryChild, download_dir: &Path, access_token: &str) -> Result<(), MediaDownloadError> {
    let mut aria_proc = download_aria(&child.resource_path, &download_dir, &child.file_name, access_token)?;
    aria_proc.wait()?;
    Ok(())
}

pub fn download_directory_resource(directory: &PlexDirectoryResource, download_dir: &Path) -> Result<(), MediaDownloadError> {
    let mut download_dir = PathBuf::from(download_dir);
    if directory.title.contains("/") {
        let paths = directory.title.split("/").collect::<Vec<&str>>();
        download_dir.push(paths[0]);
        download_dir.push(paths[1]);
        create_dir(&download_dir)?;
        for child in &directory.children {
            download_directory_child(child, &download_dir, &directory.access_token)?; 
        }
    }
    else {
        download_dir.push(&directory.title);
        create_dir(&download_dir)?;
        for child in &directory.children {
            download_directory_child(child, &download_dir, &directory.access_token)?; 
        }
    }
    Ok(())
}

fn download_aria(download_link: &str, dir_path: &Path ,out_file_path: &str, server_token: &str) -> Result<Child, MediaDownloadError> {
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
