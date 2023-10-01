use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use glam::*;

fn get_size_of<T>(value: &T) -> usize {
    std::mem::size_of_val(value)
}

struct adsa{
    asddss:u32,
    sad:f64,
    ds:String
}


enum GLSL_TYPES{
    int,
    uint,
    float,
    double,
    bool,
    vec2,
    vec3,
    mat4,
}

enum INSERT_TYPE{
    PUSH,
    DESCRIPTOR,
    STRUCT,
    EMPTY,
}

/*pub fn parse_output(field:String,glsl_types:&HashMap<String,Box<dyn std::any::Any>>,glsl_structs:&HashMap<String,Vec<String>>,glsl_outputs:&mut HashMap<String,Vec<String>>){
    let mut the_type = "";
  
    let words:Vec<&str> = field.split_whitespace().collect();
    println!("{:?}",words);

    for (key, value) in glsl_structs{
        
    } 
 
}

pub fn parse_struct(field:&str,glsl_types:HashMap<String,Box<dyn std::any::Any>>,glsl_structs:&HashMap<String,Vec<String>>) {


}*/

pub fn new() -> HashMap<String,Box<dyn std::any::Any>> {
    let mut glsl_types:HashMap<String,Box<dyn std::any::Any>> = HashMap::new();

    glsl_types.insert(String::from("int"), Box::new(()));
    glsl_types.insert(String::from("uint"),Box::new(()));
    glsl_types.insert(String::from("float"),Box::new(()));
    glsl_types.insert(String::from("double"), Box::new(()));
    glsl_types.insert(String::from("bool"), Box::new(()));
    glsl_types.insert(String::from("bvec2"), Box::new(()));
    glsl_types.insert(String::from("bvec3"), Box::new(()));
    glsl_types.insert(String::from("bvec4"), Box::new(()));
    glsl_types.insert(String::from("ivec2"), Box::new(()));
    glsl_types.insert(String::from("ivec3"), Box::new(()));
    glsl_types.insert(String::from("ivec4"), Box::new(()));
    glsl_types.insert(String::from("uvec2"), Box::new(()));
    glsl_types.insert(String::from("uvec3"), Box::new(()));
    glsl_types.insert(String::from("uvec4"), Box::new(()));
    glsl_types.insert(String::from("vec2"), Box::new(()));
    glsl_types.insert(String::from("vec3"), Box::new(()));
    glsl_types.insert(String::from("vec4"), Box::new(()));
    glsl_types.insert(String::from("dvec2"), Box::new(()));
    glsl_types.insert(String::from("dvec3"), Box::new(()));
    glsl_types.insert(String::from("dvec4"), Box::new(()));
    glsl_types.insert(String::from("mat2"), Box::new(()));
    glsl_types.insert(String::from("mat3"), Box::new(()));
    glsl_types.insert(String::from("mat4"), Box::new(()));
    glsl_types.insert(String::from("sampler1D"), Box::new(()));
    glsl_types.insert(String::from("sampler2D"), Box::new(()));
    glsl_types.insert(String::from("sampler3D"), Box::new(()));
    glsl_types.insert(String::from("samplerCube"), Box::new(()));
    glsl_types.insert(String::from("sampler2DRect"), Box::new(()));
    glsl_types.insert(String::from("sampler1DArray"), Box::new(()));
    glsl_types.insert(String::from("sampler2DArray"), Box::new(()));
    glsl_types.insert(String::from("samplerCubeArray"), Box::new(()));
    
    return glsl_types
}

struct test{
    test1:glam::Mat4,
    test2:glam::Mat4
}

pub fn main() {
    let test = File::open("simple_shader.vert").unwrap();
    
    let mut buf_reader = BufReader::new(test);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap();

    let vector:Vec<String> = contents.split("\n").map(|line| line.replace(";", "")).collect();
    let mut glsl_descriptors:HashMap<String,Vec<String>> = HashMap::new();
    let mut glsl_push_constants:HashMap<String,Vec<String>> = HashMap::new();
    let mut glsl_structs:HashMap<String,Vec<String>> = HashMap::new();
    let mut glsl_types:HashMap<String,Box<dyn std::any::Any>> = new();

    let num:adsa = adsa { asddss: 24, sad: 42.3, ds: String::from("222222") };
    println!("{:?} | {:?}",get_size_of(&num),std::mem::size_of::<adsa>());

    let mut inside_struct = false;
    let mut cur_value = String::new();
    let mut cur_type:INSERT_TYPE = INSERT_TYPE::EMPTY;

    for line in vector{
        if line.contains("//") || line.contains("*/") || line.contains("/*"){

        }
        else{
            if line.contains("}") && inside_struct{
                inside_struct = false;
            }

            if inside_struct{
                let words:Vec<&str> = line.split_whitespace().collect();
               
                match cur_type{
                    INSERT_TYPE::PUSH => glsl_push_constants.get_mut(&cur_value).unwrap().push(String::from(words[0])),
                    INSERT_TYPE::DESCRIPTOR => glsl_descriptors.get_mut(&cur_value).unwrap().push(String::from(words[0])),
                    INSERT_TYPE::STRUCT => glsl_structs.get_mut(&cur_value).unwrap().push(String::from(words[0])),
                    _ => println!("ERROR: Invalid Type!")
                };

            }

            if line.contains("struct"){
                let words:Vec<&str> = line.split_whitespace().collect();
                let uniform_pos = words.iter().position(|&word| word == "struct").expect("Failed to get the position");


                if line.contains("{"){
                    println!("Struct Name: {:?}",words[uniform_pos + 1]);
                    cur_type = INSERT_TYPE::STRUCT; 
                    cur_value = String::from(words[uniform_pos + 1]); 
                    glsl_structs.insert(String::from(words[uniform_pos + 1]), Vec::new());
                    

                }

                inside_struct = true;

            }

            if line.contains("uniform"){
                let words:Vec<&str> = line.split_whitespace().collect();
                let uniform_pos = words.iter().position(|&word| word == "uniform").expect("Failed to get the position");
    

                if line.contains("{"){

                    if line.contains("(push_constant)"){
                        println!("Push Constant: {:?}",words[uniform_pos + 1]);
                        cur_type = INSERT_TYPE::PUSH; 
                        cur_value = String::from(words[uniform_pos + 1]); 
                        glsl_push_constants.insert(String::from(words[uniform_pos + 1]), Vec::new());
                    }
                    else{
                        cur_type = INSERT_TYPE::DESCRIPTOR; 
                        cur_value = String::from(words[uniform_pos + 1]); 
                        glsl_descriptors.insert(String::from(words[uniform_pos + 1]), Vec::new());
                        println!("Descriptor {:?}",words[uniform_pos + 1]);
                    }
                    
                    inside_struct = true;
                }else{
                    if line.contains("(push_constant)"){
                        println!("Push Constant: {:?}",words[uniform_pos + 2]);
                        glsl_push_constants.insert(String::from(words[uniform_pos + 2]), vec![String::from(words[uniform_pos + 1])]);

                    }
                    else{
                        println!("Descriptor {:?}",words[uniform_pos + 2]);
                        glsl_descriptors.insert(String::from(words[uniform_pos + 2]), vec![String::from(words[uniform_pos + 1])]);

                    }
                    //parse_ou(line);
                }
            }


        }
    }

    println!("Pushes: {:?}",glsl_push_constants);
    println!("Descriptors: {:?}",glsl_descriptors);
    println!("Structs: {:?}",glsl_structs);
}

//PLAN HAVING THE GLSL_TYPES ENUM HAVING THE TYPE NAME WITH THEIR RESPECTIVE RUST TYPE EQUAL
//THEN PARSING FROM STRING TO ENUM AND THEN PUTTING IN AN POOL WITHIN THE SHADER WITH THE
//DESCRIPTOR/PUSH_CONSTANT NAME AND THEN ADDING THE LINE WHERE FROM THE VARIABLE THAT THIER TYPE
//WAS PARSED AND THEN VERYFING IF THE TYPE IS WITHIN THE PUSH/DESCRIPTOR AND IN THE RIGHT POSITION
//AND IF NOT PASS AN WARNING MESSAGE WITH AN DEFAULT VALUE FROM THAT TYPE
//
//UPDATES ON THE CREATION OF THE GLSL PARSER 22-09-2023:
//NOW THE GLSL_PARSER WILL PARSER THE INPUTS FROM VULKAN (DESCRIPTORS/PUSH_CONSTANTS) AND THEIR
//NESTED STRUCTS WOULD BE READ UNTIL IT REACHES IT'S SIMPLICITY INCLUDING OF THE TYPES AND THE
//STRUCTS WOULD BE SAVED IN AN HASHMAP
//THE VULKAN INPUTS AND STRUCTS WOULD BE AN HASHMAP WITH AN STRING OF THE NAME AN VECTOR OF STRINGS THAT WOULD 
//DIRECT TO ANOTHER HASHMAP THAT WOULD CONTAIN THE ENUM VALUE

