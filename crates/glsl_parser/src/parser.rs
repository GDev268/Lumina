use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Deref;

#[derive(Debug)]
pub enum INSERT_TYPE{
    PUSH,
    DESCRIPTOR,
    STRUCT,
    EMPTY,
}


pub struct Parser{
    types:HashMap<String,usize>,
    pub vert_structs:HashMap<String,Vec<(String,String)>>,
    pub frag_structs:HashMap<String,Vec<(String,String)>>,
    pub glsl_push_constants:HashMap<String,Vec<(String,String)>>,
    pub glsl_descriptors:HashMap<String,Vec<(String,String)>>,
    pub descriptor_data:HashMap<String,(u32,Option<u32>)>,
    pub value_sizes:HashMap<String,(usize,Option<u16>)>
}

impl Parser{
    /**
     * Criaçao da classe "Parser" para poder converter ficheiros de tipo shader e devolver hashmaps contendo essa mesma informaçao
     */
    pub fn new() -> Self{
        let mut types:HashMap<String,usize> = HashMap::new();

        types.insert(String::from("int"),4);
        types.insert(String::from("uint"),4);
        types.insert(String::from("float"),4);
        types.insert(String::from("bool"),1);
        types.insert(String::from("bvec2"),2);
        types.insert(String::from("bvec3"),3);
        types.insert(String::from("bvec4"),4);
        types.insert(String::from("ivec2"),8);
        types.insert(String::from("ivec3"),12);
        types.insert(String::from("ivec4"),16);
        types.insert(String::from("uvec2"),8);
        types.insert(String::from("uvec3"),12);
        types.insert(String::from("uvec4"),16);
        types.insert(String::from("vec2"),8);
        types.insert(String::from("vec3"),12);
        types.insert(String::from("vec4"),16);
        types.insert(String::from("mat2"),32);
        types.insert(String::from("mat3"),48);
        types.insert(String::from("mat4"),64);
        types.insert(String::from("sampler1D"),0);
        types.insert(String::from("sampler2D"),0);
        types.insert(String::from("sampler3D"),0);
        types.insert(String::from("samplerCube"),0);
        types.insert(String::from("sampler2DRect"),0);
        types.insert(String::from("sampler1DArray"),0);
        types.insert(String::from("sampler2DArray"),0);
        types.insert(String::from("samplerCubeArray"),0);

        
        return Self{
            types,
            vert_structs: HashMap::new(),
            frag_structs: HashMap::new(),
            glsl_descriptors:HashMap::new(),
            glsl_push_constants:HashMap::new(),
            descriptor_data:HashMap::new(),
            value_sizes:HashMap::new() 
        };
    }

    /**
     * Conversao de uma string que contem o tipo de data para um numero que possui o tamanho da variavel
     */
    pub fn convert_to_size(&self,field_types:&String) -> usize{
        return *self.types.get(field_types).unwrap_or(&(0 as usize));
    }

    /**
     * Verificar se a analise do ficheiro foi bem sucedida e sem falhas
     */
    pub fn verify_parse(&self,cur_type:INSERT_TYPE,value:String,is_vertex:bool) -> bool{
        let mut value_pool:Vec<&str> = Vec::new();
        match(cur_type){
            INSERT_TYPE::PUSH => {
                self.glsl_push_constants.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));  
            }
            INSERT_TYPE::DESCRIPTOR => {
                self.glsl_descriptors.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
            }
            INSERT_TYPE::STRUCT => {
                if is_vertex {
                        self.vert_structs.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }else{
                    self.frag_structs.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }
            }

            _ => return false
        }
    
        while !value_pool.is_empty() {
            let mut delete_pool:Vec<usize> = Vec::new();
            for i in 0..value_pool.len(){
                let mut finished = false;
                let value = String::from(value_pool[i]);
                if self.types.contains_key(&value){
                    delete_pool.push(i);
                    finished = true;
                }
                else{
                    if is_vertex{
                        if self.vert_structs.contains_key(&value){
                            delete_pool.push(i);
                            finished = true;
                        }
                    }
                    else{
                        if self.frag_structs.contains_key(&value){
                            delete_pool.push(i);
                            finished = true;
                        }
                    }
                }

                if !finished{
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
    fn decompose_structs(fields: &HashMap<String, Vec<(String, String)>>, check_struct: &HashMap<String, Vec<(String, String)>>) -> HashMap<String, Vec<(String, String)>> {
        let mut result: HashMap<String, Vec<(String, String)>> = HashMap::new();
    
        for (name, fields) in fields.iter() {
            let mut new_fields = Vec::new();
    
            for i in 0..fields.len() {
                if check_struct.contains_key(&fields[i].0) {
                    let pre_word: String = fields[i].1.to_string() + ".";
                    if let Some(reverse_fields) = check_struct.get(&fields[i].0) {

                        for field in reverse_fields.iter().rev() {
                            let push_field = (field.0.clone(), (pre_word.to_string() + field.1.as_str()));
                            new_fields.push(push_field.to_owned());
                        }
                    }
                }
                else{
                    new_fields.push(fields[i].clone());

                }
    
           }
    
            result.insert(name.clone(), new_fields);
        }

        result
    }

    /**
     * Obter as informaçoes de um "Descriptor" sendo elas o "binding" e o "set"
     */
    fn get_descriptor_data(vector:&Vec<&str>) -> (u32,Option<u32>) {
        let line = vector.join(" ");

        let start_index = line.find("(");

        let end_index = line.find(")");

        let content:Vec<Vec<&str>> = line[line.find("(").unwrap() + 1..line.find(")").unwrap()].split(',')
        .map(|item| item.split_whitespace().collect())
        .collect();
                 
        if content.len() > 1 {
            return (content[0][2].parse::<u32>().unwrap(),Some(content[1][2].parse::<u32>().unwrap()));
        }
        else{
            return (content[0][2].parse::<u32>().unwrap(),None);
        }

    } 

    /**
     * Converter os ficheiros shader "Vertex" e "Fragment" a partir do caminho fornecido 
     */
    pub fn parse_shader(&mut self,vert_path:&str,frag_path:&str){
        let mut inside_struct = false;
        let mut cur_value = String::new();
        let mut cur_type:INSERT_TYPE = INSERT_TYPE::EMPTY;

 
        //VERT SHADER

        let vert = File::open(&vert_path).unwrap();
        let mut buf_reader = BufReader::new(vert);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector:Vec<String> = contents.split("\n").map(|line| line.replace(";", "")).collect();
        
        for line in vector{
            if !line.trim().is_empty(){
            if line.contains("//") || line.contains("*/") || line.contains("/*"){
                
            }
            else{
                if line.contains("}") && inside_struct {
                    inside_struct = false;
                }

                if inside_struct{
                    let words:Vec<&str> = line.split_whitespace().collect();
                    match cur_type{
                        INSERT_TYPE::PUSH => self.glsl_push_constants.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::DESCRIPTOR => self.glsl_descriptors.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::STRUCT => self.vert_structs.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        _ => println!("ERROR: Invalid Type!")
                    };  
                }

                if line.contains("struct"){
                    let words:Vec<&str> = line.split_whitespace().collect();
                    let uniform_pos = words.iter().position(|&word| word == "struct").expect("Failed to get the position");
                   
                    if line.contains("{"){
                        cur_type = INSERT_TYPE::STRUCT;
                        cur_value = String::from(words[uniform_pos + 1]);
                        self.vert_structs.insert(String::from(words[uniform_pos + 1]), Vec::new());
                    }
                
                    inside_struct = true;
                }

                if line.contains("uniform"){
                    let words:Vec<&str> = line.split_whitespace().collect();
                    let uniform_pos = words.iter().position(|&word| word == "uniform").expect("Failed to get the position");

                    if line.contains("{"){
                        if line.contains("(push_constant)"){
                            cur_type = INSERT_TYPE::PUSH;
                            cur_value = String::from(words[uniform_pos + 1]);
                            self.glsl_push_constants.insert(String::from(words[uniform_pos + 1]), Vec::new());
                            inside_struct = true;    
                        }
                        else{
                            cur_type = INSERT_TYPE::DESCRIPTOR;
                            cur_value = String::from(words[uniform_pos + 1]);
 
                            self.descriptor_data.insert(words[uniform_pos + 1].to_owned(), Parser::get_descriptor_data(&words));
                            self.glsl_descriptors.insert(String::from(words[uniform_pos + 1]), Vec::new());
                            inside_struct = true;
                        }                       

                    }
                    else{
                        if line.contains("(push_constant)"){
                            self.glsl_push_constants.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]); 
                        }
                        else{
                            self.descriptor_data.insert(words[uniform_pos + 1].to_owned(), Parser::get_descriptor_data(&words));
                            self.glsl_descriptors.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
                        }
                    }
                }
            }
            }
        }
        
        let mut finished = true;
        for (value,_) in self.vert_structs.iter(){
            if !self.verify_parse(INSERT_TYPE::STRUCT, value.to_owned(), true){
                finished = false;
                println!("AAAAA1")
            }
        }

        if !finished{panic!("Shader parser failed! 1")}

        for (value,_) in self.glsl_push_constants.iter(){
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), true){
                finished = false;
                println!("AAAAA2")

            }
        }

        if !finished{panic!("Shader parser failed! 2")}

        for (value,_) in self.glsl_descriptors.iter(){
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), true){
                finished = false;
                println!("AAAAA3")
            }
        }
 
        if !finished{panic!("Shader parser failed! 3")}
        self.glsl_descriptors = Parser::decompose_structs(&mut self.glsl_descriptors,&self.vert_structs);
        self.glsl_push_constants = Parser::decompose_structs(&mut self.glsl_push_constants,&self.vert_structs);

        //FRAG SHADER

        let frag = File::open(&frag_path).unwrap();
        let mut buf_reader = BufReader::new(frag);
        let mut contents = String::new();


        buf_reader.read_to_string(&mut contents).unwrap();

        let vector:Vec<String> = contents.split("\n").map(|line| line.replace(";", "")).collect();
        
        for line in vector{
            if !line.trim().is_empty(){
            if line.contains("//") || line.contains("*/") || line.contains("/*"){
                
            }
            else{
                if line.contains("}") && inside_struct {
                    inside_struct = false;
                }

                if inside_struct{
                    let words:Vec<&str> = line.split_whitespace().collect();
                    match cur_type{
                        INSERT_TYPE::PUSH => self.glsl_push_constants.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::DESCRIPTOR => self.glsl_descriptors.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::STRUCT => self.frag_structs.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        _ => println!("ERROR: Invalid Type!")
                    };  
                }

                if line.contains("struct"){
                    let words:Vec<&str> = line.split_whitespace().collect();
                    let uniform_pos = words.iter().position(|&word| word == "struct").expect("Failed to get the position");
                   
                    if line.contains("{"){
                        cur_type = INSERT_TYPE::STRUCT;
                        cur_value = String::from(words[uniform_pos + 1]);
                        self.frag_structs.insert(String::from(words[uniform_pos + 1]), Vec::new());
                    }

                
                    inside_struct = true;
                }

                if line.contains("uniform"){
                    let words:Vec<&str> = line.split_whitespace().collect();
                    let uniform_pos = words.iter().position(|&word| word == "uniform").expect("Failed to get the position");

                    if line.contains("{"){
                        if line.contains("(push_constant)"){
                            if !self.glsl_push_constants.contains_key(words[uniform_pos + 1]) || !self.glsl_push_constants.len() > 1 {
                                cur_type = INSERT_TYPE::PUSH;
                                cur_value = String::from(words[uniform_pos + 1]);
                                self.glsl_push_constants.insert(String::from(words[uniform_pos + 1]), Vec::new());
                                inside_struct = true
                            }
                        }
                        else{
                            if !self.glsl_descriptors.contains_key(words[uniform_pos + 1]) {
                                cur_type = INSERT_TYPE::DESCRIPTOR;
                                cur_value = String::from(words[uniform_pos + 1]);

                                self.descriptor_data.insert(words[uniform_pos + 1].to_owned(), Parser::get_descriptor_data(&words));
                                self.glsl_descriptors.insert(String::from(words[uniform_pos + 1]), Vec::new());
                                inside_struct = true;
                            }
                        }                       

                    }
                    else{
                        if line.contains("(push_constant)"){
                            if !self.glsl_push_constants.contains_key(words[uniform_pos + 1]) || !self.glsl_push_constants.len() > 1 {
                                self.glsl_push_constants.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
                            }
                        }
                        else{
                            if !self.glsl_descriptors.contains_key(words[uniform_pos + 1]) {
                                self.descriptor_data.insert(words[uniform_pos + 1].to_owned(), Parser::get_descriptor_data(&words));
                                self.glsl_descriptors.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
                            }
                        }
                    }
                }
                
                }
            }
        }

        println!("{:?}\n{:?}",self.glsl_descriptors,self.glsl_push_constants);

        let mut finished = true;
        for (value,_) in self.frag_structs.iter(){
            if !self.verify_parse(INSERT_TYPE::STRUCT, value.to_owned(), false){
                finished = false;
                println!("AAAAA1")
            }
        }

        if !finished{panic!("Shader parser failed! 1")}


        for (value,_) in self.glsl_push_constants.iter(){
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), false){
                finished = false;
                println!("AAAAA2")

            }
        }

        if !finished{panic!("Shader parser failed! 2")}

        for (value,_) in self.glsl_descriptors.iter(){
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), false){
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished{panic!("Shader parser failed! 3")}

        self.glsl_descriptors = Parser::decompose_structs(&mut self.glsl_descriptors,&self.frag_structs);
        self.glsl_push_constants = Parser::decompose_structs(&mut self.glsl_push_constants,&self.frag_structs);

    }
}


