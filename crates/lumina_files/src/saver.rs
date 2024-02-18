use std::{collections::HashMap, fs::OpenOptions};

use lazy_static::lazy_static;
use serde_json::{json, Value};

const LUMINA_KEY: &[u8; 58] = b"cgvnhfjxcnmvmjtyurd34563245567878690hfgjcmxfghyuhjkfuiojlg";

#[derive(Debug, PartialEq)]
pub enum LuminaFileType {
    Json,
    Png,
    Fbx,
    None
}

#[derive(Debug)]
pub struct LuminaFile {
    file_type: LuminaFileType,
    file_name: String,
    file_content: Vec<u8>,
}

impl LuminaFile {
    pub fn new(file_type:LuminaFileType,file_name:String,file_content:Vec<u8>) -> Self {
        Self { file_type, file_name, file_content}
    }
}   

#[derive(Debug)]
struct LuminaDirectory {
    name: String,
    files: Vec<LuminaFile>,
}

pub struct Saver {
    json: Value,
    directories: HashMap<String, LuminaDirectory>,
}

impl Saver {
    pub fn new() -> Self {
        let json: Value = json!({
            "project_name": "",
            "skybox": {
                "x": "",
                "y": "",
                "-x": "",
                "-y": "",
            },
            "components": []
        });

        Self {
            json,
            directories: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Value) {
        self.json["components"]
            .as_array_mut()
            .unwrap()
            .push(component);
    }

    pub fn modify_skybox(&mut self, skybox_images: [String; 4]) {
        self.json["skybox"]["x"] = json!(skybox_images[0]);
        self.json["skybox"]["y"] = json!(skybox_images[1]);
        self.json["skybox"]["-x"] = json!(skybox_images[2]);
        self.json["skybox"]["-y"] = json!(skybox_images[3]);
    }

    pub fn create_directory(&mut self,name:&str) {
        self.directories.insert(name.to_string(), LuminaDirectory { name: name.to_string(), files: Vec::new() });
    }

    pub fn insert_file_into_directory(&mut self,directory_name:&str,file:LuminaFile) {
        self.directories.get_mut(&directory_name.to_string()).unwrap().files.push(file);
    }

    pub fn create_project_file(&self) {
        let file_name = (self.json["project_name"].to_string() + ".lumin").as_str();
    
    }

    /*pub fn combine_to_binary(&self) {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(bi)
    }*/
}
