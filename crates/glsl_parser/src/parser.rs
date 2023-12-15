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
    pub set:u32,
    pub binding:u32,
    pub is_uniform:bool
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
    fn get_descriptor_data(vector: &Vec<&str>) -> (u32, Option<u32>) {
        let line = vector.join(" ");

        let start_index = line.find("(");

        let end_index = line.find(")");

        let content: Vec<Vec<&str>> = line[line.find("(").unwrap() + 1..line.find(")").unwrap()]
            .split(',')
            .map(|item| item.split_whitespace().collect())
            .collect();

        if content.len() > 1 {
            return (
                content[0][2].parse::<u32>().unwrap(),
                Some(content[1][2].parse::<u32>().unwrap()),
            );
        } else {
            return (content[0][2].parse::<u32>().unwrap(), None);
        }
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
                    if line.contains("uniform") && !line.contains("push_constant") &&  line.contains("sampler") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                            let word_size:Vec<char> = vector[index - 1].chars().collect();
                            let size:String = word_size[2..word_size.len()].iter().collect();
                        self.descriptor_data.insert(
                            words[uniform_pos + 1].to_owned(),
                            DescriptorData{
                                size: size.parse::<u32>().unwrap(),
                                set: Parser::get_descriptor_data(&words).0,
                                binding: Parser::get_descriptor_data(&words).1.unwrap(),
                                is_uniform: false,
                            },
                        );
                    }
                    if !line.contains("sampler") && line.contains("uniform") && !line.contains("push_constant") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                            let word_size:Vec<char> = vector[index - 1].chars().collect();
                            let size:String = word_size[2..word_size.len()].iter().collect();
                            self.descriptor_data.insert(
                                words[uniform_pos + 1].to_owned(),
                                DescriptorData{
                                    size: size.parse::<u32>().unwrap(),
                                    set: Parser::get_descriptor_data(&words).0,
                                    binding: Parser::get_descriptor_data(&words).1.unwrap(),
                                    is_uniform: true,
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
                    if line.contains("uniform") && !line.contains("push_constant") &&  line.contains("sampler") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                        self.descriptor_data.insert(
                            words[uniform_pos + 1].to_owned(),
                            DescriptorData{
                                size: 0,
                                set: Parser::get_descriptor_data(&words).0,
                                binding: Parser::get_descriptor_data(&words).1.unwrap(),
                                is_uniform: false,
                            },
                        );
                    }
                    if !line.contains("sampler") && line.contains("uniform") && !line.contains("push_constant") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                            let word_size:Vec<char> = vector[index - 1].chars().collect();
                            let size:String = word_size[2..word_size.len()].iter().collect();
                        self.descriptor_data.insert(
                            words[uniform_pos + 1].to_owned(),
                            DescriptorData{
                                size: size.parse::<u32>().unwrap(),
                                set: Parser::get_descriptor_data(&words).0,
                                binding: Parser::get_descriptor_data(&words).1.unwrap(),
                                is_uniform: true,
                            },
                        );
                    }
                }
            }
        }
    }
}
