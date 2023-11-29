use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;
use std::ops::Index;

#[derive(Debug)]
pub enum INSERT_TYPE {
    CONSTANT,
    UNIFORM,
    STRUCT,
    EMPTY,
}

pub struct Parser {
    types: HashMap<String, usize>,
    pub wgsl_constants: HashMap<String, Vec<(String, String)>>,
    pub wgsl_uniforms: HashMap<String, Vec<(String, String)>>,
    pub descriptor_data: HashMap<String, (u32, Option<u32>)>,
    pub value_sizes: HashMap<String, (usize, Option<u16>)>,
}

impl Parser {
    /**
     * Criaçao da classe "Parser" para poder converter ficheiros de tipo shader e devolver hashmaps contendo essa mesma informaçao
     */
    pub fn new() -> Self {
        let mut types: HashMap<String, usize> = HashMap::new();

        types.insert(String::from("int"), std::mem::size_of::<i32>());
        types.insert(String::from("uint"), std::mem::size_of::<u32>());
        types.insert(String::from("float"), std::mem::size_of::<f32>());
        types.insert(String::from("bool"), std::mem::size_of::<bool>());
        types.insert(String::from("bvec2"), std::mem::size_of::<glam::BVec2>());
        types.insert(String::from("bvec3"), std::mem::size_of::<glam::BVec3>());
        types.insert(String::from("bvec4"), std::mem::size_of::<glam::BVec4>());
        types.insert(String::from("ivec2"), std::mem::size_of::<glam::IVec2>());
        types.insert(String::from("ivec3"), std::mem::size_of::<glam::IVec3>());
        types.insert(String::from("ivec4"), std::mem::size_of::<glam::IVec4>());
        types.insert(String::from("uvec2"), std::mem::size_of::<glam::UVec2>());
        types.insert(String::from("uvec3"), std::mem::size_of::<glam::UVec3>());
        types.insert(String::from("uvec4"), std::mem::size_of::<glam::UVec4>());
        types.insert(String::from("vec2"), std::mem::size_of::<glam::Vec2>());
        types.insert(String::from("vec3"), std::mem::size_of::<glam::Vec3>());
        types.insert(String::from("vec4"), std::mem::size_of::<glam::Vec4>());
        types.insert(String::from("mat2"), std::mem::size_of::<glam::Mat2>());
        types.insert(String::from("mat3"), std::mem::size_of::<glam::Mat3>());
        types.insert(String::from("mat4"), std::mem::size_of::<glam::Mat4>());
        types.insert(String::from("sampler1D"), 0);
        types.insert(String::from("sampler2D"), 0);
        types.insert(String::from("sampler3D"), 0);
        types.insert(String::from("samplerCube"), 0);
        types.insert(String::from("sampler2DRect"), 0);
        types.insert(String::from("sampler1DArray"), 0);
        types.insert(String::from("sampler2DArray"), 0);
        types.insert(String::from("samplerCubeArray"), 0);

        return Self {
            types,
            wgsl_constants: HashMap::new(),
            wgsl_uniforms: HashMap::new(),
            descriptor_data: HashMap::new(),
            value_sizes: HashMap::new(),
        };
    }

    /**
     * Conversao de uma string que contem o tipo de data para um numero que possui o tamanho da variavel
     */
    pub fn convert_to_size(&self, field_types: &String) -> usize {
        return *self.types.get(field_types).unwrap_or(&(0 as usize));
    }

    /**
     * Verificar se a analise do ficheiro foi bem sucedida e sem falhas
     */
    pub fn verify_parse(
        &self,
        cur_type: INSERT_TYPE,
        value: String,
        structs: HashMap<String, Vec<(String, String)>>,
    ) -> bool {
        let mut value_pool: Vec<&str> = Vec::new();
        match (cur_type) {
            INSERT_TYPE::CONSTANT => {
                self.wgsl_constants
                    .get(&value)
                    .unwrap()
                    .iter()
                    .for_each(|(value, _)| value_pool.push(value.deref()));
            }
            INSERT_TYPE::UNIFORM => {
                self.wgsl_uniforms
                    .get(&value)
                    .unwrap()
                    .iter()
                    .for_each(|(value, _)| value_pool.push(value.deref()));
            }
            INSERT_TYPE::STRUCT => {
                structs
                    .get(&value)
                    .unwrap()
                    .iter()
                    .for_each(|(value, _)| value_pool.push(value.deref()));
            }

            _ => return false,
        }

        while !value_pool.is_empty() {
            let mut delete_pool: Vec<usize> = Vec::new();
            for i in 0..value_pool.len() {
                let mut finished = false;
                let value = String::from(value_pool[i]);
                if self.types.contains_key(&value) {
                    delete_pool.push(i);
                    finished = true;
                } else {
                    if structs.contains_key(&value) {
                        delete_pool.push(i);
                        finished = true;
                    }
                }

                if !finished {
                    return false;
                }
            }

            for &index in delete_pool.iter() {
                if index < value_pool.len() {
                    value_pool.remove(index);
                }
            }
        }

        return true;
    }

    /**
     * Decompor estruturas de dados para o mais simplificado ate as tipos de data do hashmap "types"
     */
    fn decompose_structs(
        fields: &HashMap<String, Vec<(String, String)>>,
        check_struct: &HashMap<String, Vec<(String, String)>>,
    ) -> HashMap<String, Vec<(String, String)>> {
        let mut result: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for (name, fields) in fields.iter() {
            let mut new_fields = Vec::new();

            for i in 0..fields.len() {
                if check_struct.contains_key(&fields[i].0) {
                    let pre_word: String = fields[i].1.to_string() + ".";
                    if let Some(reverse_fields) = check_struct.get(&fields[i].0) {
                        for field in reverse_fields.iter().rev() {
                            let push_field =
                                (field.0.clone(), (pre_word.to_string() + field.1.as_str()));
                            new_fields.push(push_field.to_owned());
                        }
                    }
                } else {
                    new_fields.push(fields[i].clone());
                }
            }

            result.insert(name.clone(), new_fields);
        }

        result
    }

    /**
     * Obter as informaçoes de um "Descriptor" sendo elas o "group" e o "binding"
     */
    fn get_descriptor_data(line: &str) -> (u32, u32) {
        let mut value_1 = 0;
        let mut passed_first = false;
        for (index, character) in line.chars().enumerate() {
            if character == '(' && !passed_first {
                let group_end = line
                    .chars()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c == ')')
                    .min_by_key(|(index, _)| (*index as isize - *index as isize).abs())
                    .map(|(index, _)| index)
                    .unwrap_or(0);

                value_1 = line[index + 1..group_end].parse::<u32>().unwrap();
                passed_first = true;
            } else if character == '(' && passed_first {
                return (
                    value_1,
                    line[index + 1..line.len() - 1].parse::<u32>().unwrap(),
                );
            }
        }
        return (0, 0);
    }

    /**
     * Converter os ficheiros shader "Vertex" e "Fragment" a partir do caminho fornecido
     */
    pub fn parse_shader(&mut self, shader_path: &str) {
        let mut inside_struct = false;
        let mut cur_value = String::new();
        let mut cur_type: INSERT_TYPE = INSERT_TYPE::EMPTY;

        let vert = File::open(&shader_path).unwrap();
        let mut buf_reader = BufReader::new(vert);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector: Vec<&str> = contents.split(|c| c == ';' || c == '\n').collect();

        let vert_start = vector.iter().position(|s| *s == "#Vertex").unwrap_or(0);

        let vert_end = vector.iter().position(|s| s.contains("#end")).unwrap_or(0);

        let vert_contents = vector[vert_start..vert_end].to_vec();

        let frag_start = vector.iter().position(|s| *s == "#Fragment").unwrap_or(0);

        let frag_end = vector
            .iter()
            .enumerate()
            .filter(|(_, s)| s.contains("#end"))
            .min_by_key(|(index, _)| (*index as isize - frag_start as isize).abs())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let frag_contents = vector[frag_start..frag_end].to_vec();

        //VERT SHADER

        let mut vert_structs: HashMap<String, Vec<(String, String)>> = HashMap::new();
        for (index, line) in vert_contents.iter().enumerate() {
            if line.contains("fn") {
                break;
            }
            if !line.trim().is_empty() {
                if line.contains("//") || line.contains("*/") || line.contains("/*") {
                } else {
                    if line.contains("}") && inside_struct {
                        inside_struct = false;
                    }

                    /*if inside_struct {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        match cur_type {
                            INSERT_TYPE::CONSTANT => self
                                .wgsl_constants
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            INSERT_TYPE::UNIFORM => self
                                .wgsl_uniforms
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            INSERT_TYPE::STRUCT => vert_structs
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            _ => println!("ERROR: Invalid Type!"),
                        };
                    }*/

                    if *line == "@Constant" {
                        cur_type = INSERT_TYPE::CONSTANT;
                    }
                    if *line == "@Uniform" {
                        cur_type = INSERT_TYPE::UNIFORM;
                    }

                    if line.contains("struct") {
                        if line.contains("{") {
                            inside_struct = true;
                        }
                        match cur_type {
                            INSERT_TYPE::CONSTANT => {
                                let words: Vec<&str> = line.split_whitespace().collect();
                                let struct_pos = words
                                    .iter()
                                    .position(|&word| word == "struct")
                                    .expect("Failed to get the position");

                                let word = words[struct_pos + 1].replace("{", "");
                                cur_value = word.clone();
                                vert_structs
                                    .insert("CONSTANT-".to_owned() + word.as_str(), Vec::new());
                            }
                            INSERT_TYPE::UNIFORM => {
                                let words: Vec<&str> = line.split_whitespace().collect();
                                let struct_pos = words
                                    .iter()
                                    .position(|&word| word == "struct")
                                    .expect("Failed to get the position");

                                let word = words[struct_pos + 1].replace("{", "");
                                cur_value = word.clone();
                                vert_structs
                                    .insert("UNIFORM-".to_owned() + word.as_str(), Vec::new());
                            }
                            _ => {}
                        }
                    }

                    if line.contains("@group") {
                        let (group, binding) = Parser::get_descriptor_data(line);
                        let words: Vec<&str> =
                            vert_contents[index + 1].split_whitespace().collect();

                        let var:Vec<&str> = words[1].split(":").collect();
                        println!("{:?}", var);
                    }

                    /*
                    if line.contains("struct") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "struct")
                            .expect("Failed to get the position");

                        if line.contains("{") {
                            cur_type = INSERT_TYPE::STRUCT;
                            cur_value = String::from(words[uniform_pos + 1]);
                            vert_structs.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }

                        inside_struct = true;
                    }

                    if line.contains("uniform") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                        if line.contains("{") {
                            if line.contains("@Constants") {
                                cur_type = INSERT_TYPE::PUSH;
                                cur_value = String::from(words[uniform_pos + 1]);
                                self.glsl_push_constants
                                    .insert(String::from(words[uniform_pos + 1]), Vec::new());
                                inside_struct = true;
                            } else {
                                cur_type = INSERT_TYPE::DESCRIPTOR;
                                cur_value = String::from(words[uniform_pos + 1]);

                                self.descriptor_data.insert(
                                    words[uniform_pos + 1].to_owned(),
                                    Parser::get_descriptor_data(&words),
                                );
                                self.glsl_descriptors
                                    .insert(String::from(words[uniform_pos + 1]), Vec::new());
                                inside_struct = true;
                            }
                        } else {
                            if line.contains("(push_constant)") {
                                self.glsl_push_constants.insert(
                                    String::from(words[uniform_pos + 2]),
                                    vec![(String::from(words[uniform_pos + 1]), String::default())],
                                );
                            } else {
                                self.descriptor_data.insert(
                                    words[uniform_pos + 1].to_owned(),
                                    Parser::get_descriptor_data(&words),
                                );
                                self.glsl_descriptors.insert(
                                    String::from(words[uniform_pos + 2]),
                                    vec![(String::from(words[uniform_pos + 1]), String::default())],
                                );
                            }
                        }
                    }*/
                }
            }
        }

        /*let mut finished = true;
        for (value, _) in self.vert_structs.iter() {
            if !self.verify_parse(INSERT_TYPE::STRUCT, value.to_owned(), true) {
                finished = false;
                println!("AAAAA1")
            }
        }

        if !finished {
            panic!("Shader parser failed! 1")
        }

        for (value, _) in self.glsl_push_constants.iter() {
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), true) {
                finished = false;
                println!("AAAAA2")
            }
        }

        if !finished {
            panic!("Shader parser failed! 2")
        }

        for (value, _) in self.glsl_descriptors.iter() {
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), true) {
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished {
            panic!("Shader parser failed! 3")
        }
        self.glsl_descriptors =
            Parser::decompose_structs(&mut self.glsl_descriptors, &self.vert_structs);
        self.glsl_push_constants =
            Parser::decompose_structs(&mut self.glsl_push_constants, &self.vert_structs);*/

        //FRAG SHADER

        /*let frag = File::open(&frag_path).unwrap();
        let mut buf_reader = BufReader::new(frag);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector: Vec<String> = contents
            .split("\n")
            .map(|line| line.replace(";", ""))
            .collect();

        for line in vector {
            if line.contains("void main") {
                break;
            }
            if !line.trim().is_empty() {*/
        //if line.contains("//") || line.contains("*/") || line.contains("/*") {
        /* } else {
                    if line.contains("}") && inside_struct {
                        inside_struct = false;
                    }

                    if inside_struct {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        match cur_type {
                            INSERT_TYPE::PUSH => self
                                .glsl_push_constants
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            INSERT_TYPE::DESCRIPTOR => self
                                .glsl_descriptors
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            INSERT_TYPE::STRUCT => self
                                .frag_structs
                                .get_mut(&cur_value)
                                .unwrap()
                                .push((String::from(words[0]), String::from(words[1]))),
                            _ => println!("ERROR: Invalid Type!"),
                        };
                    }

                    if line.contains("struct") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "struct")
                            .expect("Failed to get the position");

                        if line.contains("{") {
                            cur_type = INSERT_TYPE::STRUCT;
                            cur_value = String::from(words[uniform_pos + 1]);
                            self.frag_structs
                                .insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }

                        inside_struct = true;
                    }

                    if line.contains("uniform") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let uniform_pos = words
                            .iter()
                            .position(|&word| word == "uniform")
                            .expect("Failed to get the position");

                        if line.contains("{") {
                            if line.contains("(push_constant)") {
                                if !self
                                    .glsl_push_constants
                                    .contains_key(words[uniform_pos + 1])
                                    || !self.glsl_push_constants.len() > 1
                                {
                                    cur_type = INSERT_TYPE::PUSH;
                                    cur_value = String::from(words[uniform_pos + 1]);
                                    self.glsl_push_constants
                                        .insert(String::from(words[uniform_pos + 1]), Vec::new());
                                    inside_struct = true
                                }
                            } else {
                                if !self.glsl_descriptors.contains_key(words[uniform_pos + 1]) {
                                    cur_type = INSERT_TYPE::DESCRIPTOR;
                                    cur_value = String::from(words[uniform_pos + 1]);

                                    self.descriptor_data.insert(
                                        words[uniform_pos + 1].to_owned(),
                                        Parser::get_descriptor_data(&words),
                                    );
                                    self.glsl_descriptors
                                        .insert(String::from(words[uniform_pos + 1]), Vec::new());
                                    inside_struct = true;
                                }
                            }
                        } else {
                            if line.contains("(push_constant)") {
                                if !self
                                    .glsl_push_constants
                                    .contains_key(words[uniform_pos + 1])
                                    || !self.glsl_push_constants.len() > 1
                                {
                                    self.glsl_push_constants.insert(
                                        String::from(words[uniform_pos + 2]),
                                        vec![(
                                            String::from(words[uniform_pos + 1]),
                                            String::default(),
                                        )],
                                    );
                                }
                            } else {
                                if !self.glsl_descriptors.contains_key(words[uniform_pos + 1]) {
                                    self.descriptor_data.insert(
                                        words[uniform_pos + 1].to_owned(),
                                        Parser::get_descriptor_data(&words),
                                    );
                                    self.glsl_descriptors.insert(
                                        String::from(words[uniform_pos + 2]),
                                        vec![(
                                            String::from(words[uniform_pos + 1]),
                                            String::default(),
                                        )],
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "{:?}\n{:?}",
            self.glsl_descriptors, self.glsl_push_constants
        );

        let mut finished = true;
        for (value, _) in self.frag_structs.iter() {
            if !self.verify_parse(INSERT_TYPE::STRUCT, value.to_owned(), false) {
                finished = false;
                println!("AAAAA1")
            }
        }

        if !finished {
            panic!("Shader parser failed! 1")
        }

        for (value, _) in self.glsl_push_constants.iter() {
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), false) {
                finished = false;
                println!("AAAAA2")
            }
        }

        if !finished {
            panic!("Shader parser failed! 2")
        }

        for (value, _) in self.glsl_descriptors.iter() {
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), false) {
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished {
            panic!("Shader parser failed! 3")
        }

        self.glsl_descriptors =
            Parser::decompose_structs(&mut self.glsl_descriptors, &self.frag_structs);
        self.glsl_push_constants =
            Parser::decompose_structs(&mut self.glsl_push_constants, &self.frag_structs);*/
    }
}
