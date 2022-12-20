use std::{fs::File, io::Write};
use crate::{example_c};

pub fn parse_language_and_type(lang_cmp: String, type_of: Option<String>)
    {
    let lcmp = lang_cmp.as_str();
    match lcmp
        {
        "c" => { write_to_file(example_c::get_hello_world(), "main.c") }, 
        _ => { panic!() },
        } 

    }

pub fn write_to_file(wht: &str, whr: &str)
    {
    let mut fobj = File::create(whr).expect("failed to create file");
    fobj.write_all(wht.as_bytes()).expect("failed to write");
    }
