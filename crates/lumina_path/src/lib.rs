use std::fs;

use lumina_files::loader::{Loader, LuminaFile};

#[derive(Debug,Clone)]
pub struct Path {
    new_path: String,
    raw_path: String,
}

impl Path {
    pub fn get_new_path(&self) -> &str {
        self.new_path.as_str()
    }

    pub fn get_raw_path(&self) -> &str {
        self.raw_path.as_str()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            new_path: "".to_string(),
            raw_path: "".to_string(),
        }
    }
}

pub fn get_raw_image(file_path: &str) -> Option<Path> {
    if let Ok(metadata) = fs::metadata(file_path) {
        let mut split_path: Vec<&str> = file_path.split("/").collect();

        let file_name = split_path[split_path.len() - 1];

        Some(Path {
            new_path: file_name.to_string(),
            raw_path: file_path.to_string(),
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn get_image(file_name:&str,loader:Loader) -> LuminaFile{
    loader.directories.get("textures").unwrap().files.iter().find(|file| file_name == file.file_name).unwrap().clone()
}

pub fn get_scene_data(file_name:&str,loader:Loader) -> LuminaFile{
    loader.directories.get("sceneData").unwrap().files.iter().find(|file| file_name == file.file_name).unwrap().clone()
}

