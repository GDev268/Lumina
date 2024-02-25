use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read}
};

use simple_crypt::decrypt;

const LUMINA_KEY: &[u8; 58] = b"cgvnhfjxcnmvmjtyurd34563245567878690hfgjcmxfghyuhjkfuiojlg";

#[derive(Debug, PartialEq,Clone, Copy)]
pub enum LuminaFileType {
    Json,
    Jpg,
    Png,
    Fbx,
    Gltf,
    None,
}

#[derive(Debug,Clone)]
pub struct LuminaFile {
    pub file_type: LuminaFileType,
    pub file_name: String,
    pub file_content: Vec<u8>,
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

#[derive(Debug,Clone)]
pub struct LuminaDirectory {
    pub name: String,
    pub files: Vec<LuminaFile>,
}

pub struct Loader {
    pub directories: HashMap<String, LuminaDirectory>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            directories: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, file_path: String) {
    match File::open(&file_path) {
        Ok(mut file) => {
            let mut file_content = Vec::new();
            match file.read_to_end(&mut file_content) {
                Ok(_) => {
                    let mut cursor = std::io::Cursor::new(file_content);

                    loop {
                        let mut directory_name_len_bytes = [0u8; 4];
                        if cursor.read_exact(&mut directory_name_len_bytes).is_err() {
                            break;
                        }

                        let directory_name_len =
                            u32::from_le_bytes(directory_name_len_bytes) as usize;

                        let mut dir_name_bytes = vec![0u8; directory_name_len];
                        cursor.read_exact(&mut dir_name_bytes).unwrap();

                        // Decrypt the directory name
                        let decrypted_dir_name = decrypt(&dir_name_bytes, LUMINA_KEY)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                            .unwrap();

                        let directory_name =
                            String::from_utf8_lossy(&decrypted_dir_name).into_owned();

                        let mut num_files_bytes = [0u8; 4];
                        cursor.read_exact(&mut num_files_bytes).unwrap();
                        let num_files = u32::from_le_bytes(num_files_bytes) as usize;

                        let mut files: Vec<LuminaFile> = Vec::with_capacity(num_files);

                        for _ in 0..num_files {
                            let mut file_type_byte = [0u8; 1];
                            let mut name_size_bytes = [0u8; 4];
                            let mut content_size_bytes = [0u8; 4];

                            cursor.read_exact(&mut file_type_byte).unwrap();
                            cursor.read_exact(&mut name_size_bytes).unwrap();
                            cursor.read_exact(&mut content_size_bytes).unwrap();

                            let file_type = match file_type_byte[0] {
                                b'J' => LuminaFileType::Json,
                                b'P' => LuminaFileType::Png,
                                b'F' => LuminaFileType::Fbx,
                                b'G' => LuminaFileType::Gltf,
                                b'E' => LuminaFileType::Jpg,
                                _ => {
                                    println!("ERROR: Couldn't find a file type!");
                                    LuminaFileType::None
                                }
                            };

                            let name_size = u32::from_le_bytes(name_size_bytes) as usize;
                            let content_size =
                                u32::from_le_bytes(content_size_bytes) as usize;

                            let mut name_bytes = vec![0u8; name_size];
                            cursor.read_exact(&mut name_bytes).unwrap();

                            let decrypted_file_name =
                                decrypt(&name_bytes, LUMINA_KEY)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                                    .unwrap();
                            let file_name =
                                String::from_utf8_lossy(&decrypted_file_name).into_owned();

                            let mut content = vec![0u8; content_size];
                            cursor.read_exact(&mut content).unwrap();

                            // Decrypt the file content
                            let decrypted_content = decrypt(
                                &content,
                                LUMINA_KEY,
                            )
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                            .unwrap();

                            files.push(LuminaFile {
                                file_type,
                                file_name,
                                file_content: decrypted_content,
                            });
                        }

                        self.directories.insert(
                            directory_name.clone(),
                            LuminaDirectory {
                                name: directory_name,
                                files,
                            },
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Error reading file content: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file {}: {}", file_path, err);
        }
    }
}

    /*pub fn load_data(&self,manager:&Query,device:Arc<Device>) {
        let entities = String::from_utf8(self.directories.get("directory1").unwrap().files[0].file_content.clone()).unwrap();

        let entities_json:Value = serde_json::from_str(&entities).unwrap();
        
        if let Some(lights) = entities_json.get("lights").and_then(|arr| arr.as_array()) {
            // Iterate over each model object
            for light_value in lights {
                let light_gm = manager.spawn();

                let mut light = Light::new();

                light.change_color(glam::vec3(1.0,1.0,1.0));
                light.change_intensity(light_value.get("intensity").unwrap().as_f64().unwrap() as f32);
                light.change_light_type(light_value.get("light_type").unwrap().as_u64().unwrap() as u32);
                light.change_range(light_value.get("range").unwrap().as_f64().unwrap() as f32);
                light.change_spot_size(light_value.get("spot_size").unwrap().as_f64().unwrap() as f32);
                light.change_range(light_value.get("range").unwrap().as_f64().unwrap() as f32);


                
           }
        }
    }*/
}
