use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;
use std::ops::Index;


/*pub struct Parser {
    types: HashMap<String, usize>,
    pub wgsl_constants_vert: HashMap<String, Vec<(String, String)>>,
    pub wgsl_constants_frag: HashMap<String, Vec<(String, String)>>,
    pub wgsl_uniforms: HashMap<String, Vec<(String, String)>>,
    pub descriptor_data: HashMap<String, (u32, Option<u32>)>,
    pub value_sizes: HashMap<String, (usize, Option<u16>)>,
}

impl Parser {
    fn parse_shader_text(
        vector: &Vec<&str>,
    ) -> (Vec<String>, String, Vec<String>, String, Vec<String>) {
        let mut vector = vector.clone();

        let mut data_contents: Vec<&str> = Vec::new();

        while vector.contains(&"#Data") {
            let vert_start = vector.iter().position(|s| *s == "#Data").unwrap_or(0);

            let vert_end = vector
                .iter()
                .enumerate()
                .filter(|(_, s)| s.contains("#end"))
                .min_by_key(|(index, _)| (*index as isize - vert_start as isize).abs())
                .map(|(index, _)| index)
                .unwrap_or(0);

            data_contents.extend_from_slice(&vector[vert_start + 1..vert_end]);
            vector.remove(vert_start);
        }

        let vs_start = vector
            .iter()
            .position(|s| s.contains("fn vs_main"))
            .unwrap_or(0);

        let vs_end = vector
            .iter()
            .enumerate()
            .filter(|(_, s)| s.contains("{"))
            .min_by_key(|(index, _)| (*index as isize - vs_start as isize).abs())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let vs_main_line: String = vector[vs_start..vs_end + 1].concat();

        let variables: String = vs_main_line[vs_main_line.chars().position(|c| c == '(').unwrap()
            + 1
            ..vs_main_line.chars().position(|c| c == ')').unwrap()]
            .replace(" ", "");

        let vs_main: Vec<&str> = variables.split(",").collect();

        let fs_start = vector
            .iter()
            .position(|s| s.contains("fn fs_main"))
            .unwrap_or(0);

        let fs_end = vector
            .iter()
            .enumerate()
            .filter(|(_, s)| s.contains("{"))
            .min_by_key(|(index, _)| (*index as isize - fs_start as isize).abs())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let fs_main_line: String = vector[fs_start..fs_end + 1].concat();

        let variables: String = fs_main_line[fs_main_line.chars().position(|c| c == '(').unwrap()
            + 1
            ..fs_main_line.chars().position(|c| c == ')').unwrap()]
            .replace(" ", "");

        let fs_main: Vec<&str> = variables.split(",").collect();

        let data_contents_string = data_contents
            .iter()
            .map(|raw| raw.to_string())
            .collect::<Vec<String>>();

        let vs_main_string = vs_main
            .iter()
            .map(|raw| raw.to_string())
            .collect::<Vec<String>>();

        let fs_main_string = fs_main
            .iter()
            .map(|raw| raw.to_string())
            .collect::<Vec<String>>();

        return (
            data_contents_string,
            vs_main_line,
            vs_main_string,
            fs_main_line,
            fs_main_string,
        );
    }

    /**
     * Converter os ficheiros shader "Vertex" e "Fragment" a partir do caminho fornecido
     */
    pub fn parse_shader(&mut self, shader_path: &str) {
        let mut inside_struct = false;
        let mut cur_value = String::new();

        let vert = File::open(&shader_path).unwrap();
        let mut buf_reader = BufReader::new(vert);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let vector: Vec<&str> = contents.split(|c| c == ';' || c == '\n').collect();

        let (data_contents, vs_main_line, vs_main, fs_main_line, fs_main) =
            Parser::parse_shader_text(&vector);

        //VERT SHADER

        let mut wgsl_structs: HashMap<String, Vec<(String, String)>> = HashMap::new();
        for (index, line) in data_contents.iter().enumerate() {
            if !line.trim().is_empty() {*/
                //if line.contains("//") || line.contains("*/") || line.contains("/*") {
                /* } else {
                    if line.contains("}") && inside_struct {
                        inside_struct = false;
                    }

                    if inside_struct {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        wgsl_structs.get_mut(&cur_value).unwrap().push((
                            String::from(words[1].replace(":", "")),
                            String::from(words[2].replace(":", "").replace(",", "")),
                        ));
                    }

                    if line.contains("struct") {
                        let words: Vec<&str> = line.split_whitespace().collect();
                        let struct_pos = words
                            .iter()
                            .position(|&word| word == "struct")
                            .expect("Failed to get the position");

                        let word = words[struct_pos + 1].replace("{", "");
                        cur_value = word.clone();
                        wgsl_structs.insert(word, Vec::new());

                        if line.contains("{") {
                            inside_struct = true;
                        }
                    }

                    if line.contains("@group") {
                        let (group, binding, name) = Parser::get_descriptor_data(line);

                        let words: Vec<&str> =
                            data_contents[index + 1].split_whitespace().collect();

                        let var: Vec<&str> = words[1].split(":").collect();

                        self.wgsl_uniforms
                            .insert(name, vec![(var[0].to_string(), var[1].to_string())]);
                    }
                }
            }
        }

        for variable in vs_main {
            let split_variable: Vec<&str> = variable.split(":").collect();
            self.wgsl_constants_vert.insert(
                split_variable[0].to_string(),
                vec![(split_variable[0].to_string(), split_variable[1].to_string())],
            );
        }

        for variable in fs_main {
            let split_variable: Vec<&str> = variable.split(":").collect();
            let mut is_from_another = false;

            is_from_another = !vs_main_line
                [vs_main_line.chars().position(|c| c == '>').unwrap()..vs_main_line.len() - 1]
                .contains(split_variable[0]);

            if !is_from_another {
                self.wgsl_constants_frag.insert(
                    split_variable[0].to_string(),
                    vec![(split_variable[0].to_string(), split_variable[1].to_string())],
                );
            }
        }

        let mut finished = true;
        for (value, _) in wgsl_structs.iter() {
            if !self.verify_parse(INSERT_TYPE::STRUCT, value.to_owned(), &wgsl_structs, false) {
                finished = false;
                println!("AAAAA1")
            }
        }

        if !finished {
            panic!("Shader parser failed! WGSL Structs")
        }

        for (value, _) in self.wgsl_constants_vert.iter() {
            if !self.verify_parse(INSERT_TYPE::CONSTANT, value.to_owned(), &wgsl_structs, true) {
                finished = false;
                println!("AAAAA2")
            }
        }

        if !finished {
            panic!("Shader parser failed! WGSL Vert Constants")
        }

        for (value, _) in self.wgsl_constants_frag.iter() {
            if !self.verify_parse(
                INSERT_TYPE::CONSTANT,
                value.to_owned(),
                &wgsl_structs,
                false,
            ) {
                finished = false;
                println!("AAAAA2")
            }
        }

        if !finished {
            panic!("Shader parser failed! WGSL Frag Constants")
        }

        for (value, _) in self.wgsl_uniforms.iter() {
            if !self.verify_parse(INSERT_TYPE::UNIFORM, value.to_owned(), &wgsl_structs, false) {
                finished = false;
                println!("AAAAA3")
            }
        }

        if !finished {
            panic!("Shader parser failed! WGSL Uniforms")
        }

        wgsl_structs = Parser::decompose_structs(&wgsl_structs, &wgsl_structs, false);
        self.wgsl_constants_vert =
            Parser::decompose_structs(&self.wgsl_constants_vert, &wgsl_structs, true);
        self.wgsl_constants_frag =
            Parser::decompose_structs(&self.wgsl_constants_frag, &wgsl_structs, true);
        self.wgsl_uniforms = Parser::decompose_structs(&self.wgsl_uniforms, &wgsl_structs, false);

    }
}*/
