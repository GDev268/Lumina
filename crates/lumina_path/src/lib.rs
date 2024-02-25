use std::fs;

use lumina_files::loader::{Loader, LuminaFile};

#[derive(Debug,Clone)]
pub struct Path {
    new_path: String,
    raw_path: String,
    raw:bool
}

impl Path {
    pub fn get_new_path(&self) -> &str {
        self.new_path.as_str()
    }

    pub fn get_raw_path(&self) -> &str {
        self.raw_path.as_str()
    }

    pub fn set_new_path(&mut self,new_path:&str) {
        self.new_path = new_path.to_string();
    }

    pub fn set_raw_path(&mut self,new_path:&str) {
        self.raw_path = new_path.to_string();
    }

    pub fn is_raw_path(&self) -> bool {
        self.raw
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            new_path: "".to_string(),
            raw_path: "".to_string(),
            raw: false
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
            raw:true
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn load_image(file_name:&str,loader:Loader) -> LuminaFile{
    loader.directories.get("textures").unwrap().files.iter().find(|file| file_name == file.file_name).unwrap().clone()
}

pub fn get_raw_model(file_path: &str) -> Option<Path> {
    if let Ok(metadata) = fs::metadata(file_path) {
        let mut split_path: Vec<&str> = file_path.split("/").collect();

        let file_name = split_path[split_path.len() - 1];

        Some(Path {
            new_path: file_name.to_string(),
            raw_path: file_path.to_string(),
            raw:true
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn get_scene_data(file_name:&str,loader:Loader) -> LuminaFile{
    loader.directories.get("sceneData").unwrap().files.iter().find(|file| file_name == file.file_name).unwrap().clone()
}

