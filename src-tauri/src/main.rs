// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod disks;

use std::{fs, io};
use std::fs::{FileType, read_dir};
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use serde::Serialize;
use tauri::{Manager, Wry};

use disks::disks;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct EntryDescriptor {
    pub entry_type: String,
    pub name: String,
    pub size: i128,
}

pub fn filetype_as_string(file_type: FileType) -> String {
    if file_type.is_file() { String::from("file") }
    else if file_type.is_dir() { String::from("directory") }
    else if file_type.is_symlink() { String::from("symlink") }
    else { unreachable!() }
}

#[tauri::command]
fn entries(mut path: String) -> Vec<EntryDescriptor> {
    path.insert_str(0, "\\\\?\\");
    let dir = fs::read_dir(path);
    let dir = match dir {
        Ok(val) => val,
        Err(_err) => return vec![],
    };
    let entries: Vec<_> = dir
        .into_iter()
        .map(|x| x.unwrap())
        .map(|entry| {
            let meta = entry.metadata().unwrap();

            EntryDescriptor {
                name: entry.file_name().into_string().unwrap(),
                entry_type: filetype_as_string(meta.file_type()),
                size: if meta.is_file() { meta.file_size() as i128 } else { -1 }
            }
        })
        .collect();

    entries
}

#[tauri::command]
async fn dirSize(path: String) -> u64 {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(
                    match fs::read_dir(file.path()) {
                        Ok(val) => val,
                        Err(_err) => return Ok(0),
                    }
                ).unwrap_or(0),
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    let dir = match fs::read_dir(path) {
        Ok(val) => val,
        Err(err) => return 0,
    };
    dir_size(dir).unwrap()
}

#[tauri::command]
async fn show_main(window: tauri::Window<Wry>) {
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![disks, entries, dirSize, show_main])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
