use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Write},
};

use lazy_static::lazy_static;
use serde_json::{json, Value};
use simple_crypt::encrypt;

const LUMINA_KEY: &[u8; 58] = b"cgvnhfjxcnmvmjtyurd34563245567878690hfgjcmxfghyuhjkfuiojlg";

#[derive(Debug, PartialEq)]
pub enum LuminaFileType {
    Json,
    Jpg,
    Png,
    Fbx,
    Gltf,
    None,
}

#[derive(Debug)]
pub struct LuminaFile {
    file_type: LuminaFileType,
    file_name: String,
    file_content: Vec<u8>,
}

impl LuminaFile {
    pub fn new(file_type: LuminaFileType, file_name: String, file_content: Vec<u8>) -> Self {
        Self {
            file_type,
            file_name,
            file_content,
        }
    }
}

#[derive(Debug)]
struct LuminaDirectory {
    name: String,
    files: Vec<LuminaFile>,
}

pub struct Saver {
    pub json: Value,
    directories: HashMap<String, LuminaDirectory>,
}

impl Saver {
    pub fn new() -> Self {
        let json: Value = json!({
            "project_name": "",
            "skybox": {
                "x": "",
                "-x": "",
                "y": "",
                "-y": "",
                "z": "",
                "-z": "",
            },
            "game_objects": [],
            "lights": [],
            "models": [],
            "transforms": []
        });

        Self {
            json,
            directories: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Value) {
        self.json["entities"]
            .as_array_mut()
            .unwrap()
            .push(entity);
    }

    pub fn modify_array_value(&mut self,value:&str,data:Vec<Value>){
        if let Some(object) = self.json.as_object_mut() {
            if object.contains_key(value) {
                object.insert(value.to_string(), Value::Array(data));
            }
        }
    }

    pub fn modify_project_name(&mut self,project_name:&str) {
        self.json["project_name"] = json!(project_name);
    }

    pub fn modify_skybox(&mut self, skybox_images: [String; 6]) {
        self.json["skybox"]["x"] = json!(skybox_images[0]);
        self.json["skybox"]["-x"] = json!(skybox_images[1]);
        self.json["skybox"]["y"] = json!(skybox_images[2]);
        self.json["skybox"]["-y"] = json!(skybox_images[3]);
        self.json["skybox"]["z"] = json!(skybox_images[4]);
        self.json["skybox"]["-z"] = json!(skybox_images[5]);
    }

    pub fn create_directory(&mut self, name: &str) {
        self.directories.insert(
            name.to_string(),
            LuminaDirectory {
                name: name.to_string(),
                files: Vec::new(),
            },
        );
    }

    pub fn insert_file_into_directory(&mut self, directory_name: &str, file: LuminaFile) {
        self.directories
            .get_mut(&directory_name.to_string())
            .unwrap()
            .files
            .push(file);
    }

    pub fn save_data(&mut self) {
        let file_name = "./".to_string() + &self.json["project_name"].as_str().unwrap() + ".lumin";

        self.create_directory("gameData");
        self.insert_file_into_directory("gameData", LuminaFile { file_type: LuminaFileType::Json, file_name: "scene.json".to_string(), file_content: serde_json::to_vec(&self.json).unwrap() });

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name)
            .unwrap();

        for (name, directory) in self.directories.iter_mut() {
            let encrypted_dir_name = encrypt(
                &directory.name.as_bytes(),
                LUMINA_KEY,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .unwrap();

            file.write_all(&(encrypted_dir_name.len() as u32).to_le_bytes())
                .unwrap();
            file.write_all(&encrypted_dir_name).unwrap();

            file.write_all(&(directory.files.len() as u32).to_le_bytes())
                .unwrap();
            for file_info in directory.files.iter() {
                let file_type_byte = match file_info.file_type {
                    LuminaFileType::Json => b'J',
                    LuminaFileType::Png => b'P',
                    LuminaFileType::Fbx => b'F',
                    LuminaFileType::Gltf => b'G',
                    LuminaFileType::Jpg => b'E',
                    LuminaFileType::None => 0,
                };

                let encrypted_file_name = encrypt(
                    &file_info.file_name.as_bytes(),
                    LUMINA_KEY,
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                .unwrap();

                file.write_all(&[file_type_byte]).unwrap();
                file.write_all(&(encrypted_file_name.len() as u32).to_le_bytes())
                    .unwrap();

                let encrypted_content = encrypt(
                    &file_info.file_content,
                    LUMINA_KEY,
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                .unwrap();

                let content_len = encrypted_content.len() as u32;
                file.write_all(&content_len.to_le_bytes()).unwrap();

                file.write_all(&encrypted_file_name).unwrap();
                file.write_all(&encrypted_content).unwrap();
            }
        }
    }
}
