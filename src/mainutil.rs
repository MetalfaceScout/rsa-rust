use std::{io::Read, usize};
use clio::Input;

use crate::InputArgGroup;

pub fn parse_input_group(input: InputArgGroup) -> String {
    let input_string;

    match input.file {
        Some(mut f) => {
            let mut string_buf = String::new();
            let res = f.read_to_string(&mut string_buf);
            match res {
                Ok(_) => input_string = string_buf,
                Err(e) => {panic!("Failed to read input, either specify a file or include input on stdin. Error: {e}")}
            }
        }
        None => {
            match input.input {
                Some(s) => {
                    input_string = s;
                }
                None => {
                    panic!("Somehow, there was no file or input specified.");
                }
            }
        }
    }
    
    return input_string;
}

pub fn split_string_at_n(n: usize, string: String) -> Vec<String> {
    let mut input_string_vec = Vec::new();
    let mut temp_string = String::new();
    let mut counter = 0usize;
    for char in string.bytes() {
        if counter > n {
            temp_string.push(char::from(char));
            input_string_vec.push(temp_string.clone());
            temp_string.clear();
            counter = 0;
        } else {
            temp_string.push(char::from(char));
            counter += 1;
        }
    }
    if input_string_vec.len() != 0 {
        input_string_vec.push(temp_string);
    }
    return input_string_vec;
}

pub fn read_key(mut input: Input) -> String {

    let mut ret_text = String::new();
    let res = input.read_to_string(&mut ret_text);
    match res {
        Ok(u) => {println!("Read {u} bytes")},
        Err(e) => {
            panic!("Could not read from file at {} \n{}", input.path().to_string(), e);
        }
    }
    return ret_text;
}