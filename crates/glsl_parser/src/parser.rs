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
    types:Vec<String>,
    pub vert_structs:HashMap<String,Vec<(String,String)>>,
    pub vert_push_constants:HashMap<String,Vec<(String,String)>>,
    pub vert_descriptors:HashMap<String,Vec<(String,String)>>,
    pub frag_structs:HashMap<String,Vec<(String,String)>>,
    pub frag_push_constants:HashMap<String,Vec<(String,String)>>,
    pub frag_descriptors:HashMap<String,Vec<(String,String)>>
}

impl Parser{
    pub fn new() -> Self{
        let mut types:Vec<String> = Vec::new();

        types.push(String::from("int"));
        types.push(String::from("uint"));
        types.push(String::from("float"));
        types.push(String::from("double"));
        types.push(String::from("bool"));
        types.push(String::from("bvec2"));
        types.push(String::from("bvec3"));
        types.push(String::from("bvec4"));
        types.push(String::from("ivec2"));
        types.push(String::from("ivec3"));
        types.push(String::from("ivec4"));
        types.push(String::from("uvec2"));
        types.push(String::from("uvec3"));
        types.push(String::from("uvec4"));
        types.push(String::from("vec2"));
        types.push(String::from("vec3"));
        types.push(String::from("vec4"));
        types.push(String::from("dvec2"));
        types.push(String::from("dvec3"));
        types.push(String::from("dvec4"));
        types.push(String::from("mat2"));
        types.push(String::from("mat3"));
        types.push(String::from("mat4"));
        types.push(String::from("sampler1D"));
        types.push(String::from("sampler2D"));
        types.push(String::from("sampler3D"));
        types.push(String::from("samplerCube"));
        types.push(String::from("sampler2DRect"));
        types.push(String::from("sampler1DArray"));
        types.push(String::from("sampler2DArray"));
        types.push(String::from("samplerCubeArray"));
        
        return Self{
            types,
            vert_structs: HashMap::new(),
            vert_push_constants: HashMap::new(),
            vert_descriptors: HashMap::new(),
            frag_structs: HashMap::new(),
            frag_push_constants: HashMap::new(),
            frag_descriptors: HashMap::new(),
        };
    }

    pub fn verify_parse(&self,cur_type:INSERT_TYPE,value:String,is_vertex:bool) -> bool{
        let mut value_pool:Vec<&str> = Vec::new();
        match(cur_type){
            INSERT_TYPE::PUSH => {
                if is_vertex {
                    self.vert_push_constants.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }else{
                    self.frag_push_constants.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }
            }
            INSERT_TYPE::DESCRIPTOR => {
                if is_vertex {
                    self.vert_descriptors.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }else{
                    self.frag_descriptors.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }
            }
            INSERT_TYPE::STRUCT => {
                if is_vertex {
                    self.vert_structs.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }else{
                    self.vert_structs.get(&value).unwrap().iter().for_each(|(value,_)| value_pool.push(value.deref()));
                }
            }

            _ => return false
        }
    
        while !value_pool.is_empty() {
            let mut delete_pool:Vec<usize> = Vec::new();
            for i in 0..value_pool.len(){
                let mut finished = false;
                let value = String::from(value_pool[i]);
                if self.types.contains(&value){
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

    fn decompose_structs(&mut self){
        for (_,fields) in self.vert_push_constants.iter_mut(){
            for i in 0..fields.len(){
               if self.vert_structs.contains_key(&fields[i].1){
                    if let Some(reverse_fields) = self.vert_structs.get(&fields[i].1){
                        for field in reverse_fields.iter().rev(){
                            if fields.len() <= i + 1{
                                fields.push(field.to_owned());
                            }
                            else{
                                fields.insert(i + 1, field.to_owned());
                            }
                        }

                        fields.remove(i);
                    }
                } 
            }
            println!("{:?}",fields);
        }
    }

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
                        INSERT_TYPE::PUSH => self.vert_push_constants.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::DESCRIPTOR => self.vert_descriptors.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
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
                            self.vert_push_constants.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }
                        else{
                            cur_type = INSERT_TYPE::DESCRIPTOR;
                            cur_value = String::from(words[uniform_pos + 1]);
                            self.vert_descriptors.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }                       

                        inside_struct = true;
                    }
                    else{
                        if line.contains("(push_constant)"){
                            self.vert_push_constants.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
                        }
                        else{
                            self.vert_descriptors.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
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

        for (value,_) in self.vert_push_constants.iter(){
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), true){
                finished = false;
                println!("AAAAA2")

            }
        }

        for (value,_) in self.vert_descriptors.iter(){
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), true){
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished{panic!("Shader parser failed!")}


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
                        INSERT_TYPE::PUSH => self.frag_push_constants.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
                        INSERT_TYPE::DESCRIPTOR => self.frag_descriptors.get_mut(&cur_value).unwrap().push((String::from(words[0]),String::from(words[1]))),
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
                            cur_type = INSERT_TYPE::PUSH;
                            cur_value = String::from(words[uniform_pos + 1]);
                            self.frag_push_constants.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }
                        else{
                            cur_type = INSERT_TYPE::DESCRIPTOR;
                            cur_value = String::from(words[uniform_pos + 1]);
                            self.frag_descriptors.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        }                       

                        inside_struct = true;
                    }
                    else{
                        if line.contains("(push_constant)"){
                            self.frag_push_constants.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
                        }
                        else{
                            self.frag_descriptors.insert(String::from(words[uniform_pos + 2]), vec![(String::from(words[uniform_pos + 1]),String::default())]);
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

        for (value,_) in self.vert_push_constants.iter(){
            if !self.verify_parse(INSERT_TYPE::PUSH, value.to_owned(), true){
                finished = false;
                println!("AAAAA2")

            }
        }

        for (value,_) in self.vert_descriptors.iter(){
            if !self.verify_parse(INSERT_TYPE::DESCRIPTOR, value.to_owned(), true){
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished{panic!("Shader parser failed!")}
    
        self.decompose_structs();
    }
}



