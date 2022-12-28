use std::{fs, io::Write};
use crate::{example_c};

pub fn parse_language_and_type(lang_cmp: &str, type_of: Option<String>)
    {
    match lang_cmp
        {
        "c" => 
            { 
            example_c::parse_c_lang_type(type_of)
            }, 
        _ => { panic!() },
        } 

    }

pub fn remove_boilerplate_files(lang: &str, type_of: Option<String>)
    {
    match lang
        {
        "c" => { remove_files(example_c::get_hello_world_files().to_vec()) },
        _ => { unimplemented!() }
        }
    }

pub fn write_to_file(wht: &str, whr: &str)
    {
    let mut fobj = fs::File::create(whr).expect("failed to create file");
    fobj.write_all(wht.as_bytes()).expect("failed to write");
    }

fn remove_files(file_arr: Vec<&str>)
    {
    for i in file_arr
        {
        fs::remove_file(i).unwrap();
        }
    }
