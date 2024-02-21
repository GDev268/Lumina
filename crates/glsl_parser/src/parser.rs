use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;

#[derive(Debug)]
pub enum INSERT_TYPE {
    PUSH,
    DESCRIPTOR,
    STRUCT,
    EMPTY,
}

#[derive(Debug)]
pub struct DescriptorData{
    pub size:u32,
    pub binding:u32,
    pub value:u32
}

pub struct Parser {
    pub descriptor_data: HashMap<String,DescriptorData>,
}

impl Parser {
    /**
     * Criaçao da classe "Parser" para poder converter ficheiros de tipo shader e devolver hashmaps contendo essa mesma informaçao
     */
    pub fn new() -> Self {
        return Self {
            descriptor_data: HashMap::new(),
        };
    }

    /**
     * Obter as informaçoes de um "Descriptor" sendo elas o "binding" e o "set"
     */
    fn get_descriptor_binding(vector: &Vec<&str>) -> u32 {
        let line = vector.join(" ");

        let start_index = line.find("(");

        let end_index = line.find(")");

        let content: Vec<Vec<&str>> = line[line.find("(").unwrap() + 1..line.find(")").unwrap()]
            .split(',')
            .map(|item| item.split_whitespace().collect())
            .collect();

        for words in content {
            if words.contains(&"binding") {
                return words[2].parse::<u32>().unwrap();
            }
        }

        return 0;
    }

    /**
     * Converter os ficheiros shader "Vertex" e "Fragment" a partir do caminho fornecido
     */
    pub fn parse_shader(&mut self, vert_path: &str, frag_path: &str) {
        let mut inside_struct = false;
        let mut cur_value = String::new();
        let mut cur_type: INSERT_TYPE = INSERT_TYPE::EMPTY;

        //VERT SHADER

        let vert = File::open(&vert_path).unwrap();
        let mut buf_reader = BufReader::new(vert);
        let mut contents = String::new();       

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector: Vec<String> = contents
            .split("\n")
            .map(|line| line.replace(";", ""))
            .collect();

        for (index,line) in vector.iter().enumerate() {
            if !line.trim().is_empty() {
                if line.contains("//") || line.contains("*/") || line.contains("/*") {
                } else {
                    if line.contains("uniform") && !line.contains("push_constant") && line.contains("sampler2D") || line.contains("uniform") && !line.contains("push_constant") && line.contains("samplerCube") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                            let formatted_type = vector[index - 1].replace("/", "").replace(" ", "").replace("\r", "").replace(" ", "");

                            let value = match formatted_type.to_lowercase().as_str() {
                                "color" => {1},
                                "depth" => {2},
                                "cubemap-color" => {3},
                                "cubemap-depth" => {4},
                                &_ => {1}
                            };

                        self.descriptor_data.insert(
                            words[uniform_pos + 2].to_owned(),
                            DescriptorData{
                                size: 0,
                                binding: Parser::get_descriptor_binding(&words),
                                value,
                            },
                        );
                    }
                    if !line.contains("sampler2D") && !line.contains("samplerCube") && line.contains("uniform") && !line.contains("push_constant") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                               let size = vector[index - 1].replace("/", "").replace(" ", "").replace("\r", "").replace(" ", "").parse::<u32>().unwrap();

                        self.descriptor_data.insert(
                            words[uniform_pos + 1].to_owned(),
                            DescriptorData{
                                size,
                                binding: Parser::get_descriptor_binding(&words),
                                value: 0,
                            },
                        );
                    }
                }
            }
        }

        //FRAG SHADER

        let frag = File::open(&frag_path).unwrap();
        let mut buf_reader = BufReader::new(frag);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector: Vec<String> = contents
            .split("\n")
            .map(|line| line.replace(";", ""))
            .collect();

        for (index,line) in vector.iter().enumerate() {
            if !line.trim().is_empty() {
                if line.contains("//") || line.contains("*/") || line.contains("/*") {
                } else {
                    if line.contains("uniform") && !line.contains("push_constant") && line.contains("sampler2D") || line.contains("uniform") && !line.contains("push_constant") && line.contains("samplerCube") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                        let formatted_type = vector[index - 1].replace("/", "").replace(" ", "").replace("\r", "").replace(" ", "");

                        let value = match formatted_type.to_lowercase().as_str() {
                            "color" => {1},
                            "depth" => {2},
                            "cubemap-color" => {3},
                            "cubemap-depth" => {4},
                            &_ => {1}
                        };

                        self.descriptor_data.insert(
                            words[uniform_pos + 2].to_owned(),
                            DescriptorData{
                                size: 0,
                                binding: Parser::get_descriptor_binding(&words),
                                value,
                            },
                        );
                    }
                    if !line.contains("sampler2D") && !line.contains("samplerCube") && line.contains("uniform") && !line.contains("push_constant") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                            println!("{:?}",vector[index - 1].replace("/", "").replace(" ", "").replace("\r", "").replace(" ", ""));
                            let size = vector[index - 1].replace("/", "").replace(" ", "").replace("\r", "").replace(" ", "").parse::<u32>().unwrap();

                            Parser::get_descriptor_binding(&words);

                        self.descriptor_data.insert(
                            words[uniform_pos + 1].to_owned(),
                            DescriptorData{
                                size,
                                binding: Parser::get_descriptor_binding(&words),
                                value: 0,
                            },
                        );
                    }
                }
            }
        }
    }
}
