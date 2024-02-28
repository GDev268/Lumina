use lazy_static::*;
use lumina_files::{
    loader::{Loader, LuminaFile},
    saver::LuminaFileType,
};
use std::{
    collections::HashMap,
    fs,
    hash::Hash,
    sync::{Arc, Mutex, RwLock},
};

pub static mut PATHS: Vec<(String, String)> = vec![];

#[derive(Debug, Clone)]
pub struct Path {
    new_path: String,
    raw_path: String,
    raw: bool,
}

impl Path {
    pub fn get_new_path(&self) -> &str {
        self.new_path.as_str()
    }

    pub fn get_raw_path(&self) -> &str {
        self.raw_path.as_str()
    }

    pub fn set_new_path(&mut self, new_path: &str) {
        self.new_path = new_path.to_string();
    }

    pub fn set_raw_path(&mut self, new_path: &str) {
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
            raw: true,
        }
    }
}

pub fn get_raw_image(file_path: &str) -> Option<Path> {
    if let Ok(metadata) = fs::metadata(file_path) {
        let mut split_path: Vec<&str> = file_path.split("/").collect();

        let file_name = split_path[split_path.len() - 1];
        unsafe { PATHS.push((file_path.to_string(), file_name.to_string())) };

        Some(Path {
            new_path: file_name.to_string(),
            raw_path: file_path.to_string(),
            raw: true,
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn get_new_image(file_path: &str, loader: Arc<RwLock<Loader>>) -> Option<Path> {
    let binding = loader
        .read()
        .unwrap();
    
    let file: Vec<&LuminaFile> = binding
        .directories
        .get("textures")
        .unwrap()
        .files
        .iter()
        .filter(|file| file.file_name == file_path)
        .collect();

    if !file.is_empty() {
        Some(Path {
            new_path: file_path.to_string(),
            raw_path: String::default(),
            raw: true,
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn load_image(file_name: &str, loader: Loader) -> LuminaFile {
    loader
        .directories
        .get("textures")
        .unwrap()
        .files
        .iter()
        .find(|file| file_name == file.file_name)
        .unwrap()
        .clone()
}

pub fn get_raw_model(file_path: &str) -> Option<Path> {
    if let Ok(metadata) = fs::metadata(file_path) {
        let mut split_path: Vec<&str> = file_path.split("/").collect();

        let file_name = split_path[split_path.len() - 1];

        Some(Path {
            new_path: file_name.to_string(),
            raw_path: file_path.to_string(),
            raw: true,
        })
    } else {
        println!("ERROR: Invalid image path");
        None
    }
}

pub fn get_scene_data(file_name: &str, loader: Loader) -> LuminaFile {
    loader
        .directories
        .get("sceneData")
        .unwrap()
        .files
        .iter()
        .find(|file| file_name == file.file_name)
        .unwrap()
        .clone()
}
