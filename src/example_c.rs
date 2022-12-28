use crate::lang_util;

pub fn get_hello_world() -> &'static str
    {
    "#include <stdio.h>
int main()
    {
    printf(\"hello world\\n\");
    return 0;
    }\n"
    }

pub fn get_hello_world_make_file() -> &'static str
    {
    "output: main.c
\tgcc main.c -o output"
    }

pub fn get_hello_world_files() -> [&'static str; 2]
    {
    ["main.c", "Makefile"]
    }

pub fn generate_hello_world_example()
    {
    lang_util::write_to_file(get_hello_world(), "main.c");
    lang_util::write_to_file(get_hello_world_make_file(), "Makefile");
    }

pub fn parse_c_lang_type(type_of: Option<String>)
    {
    match type_of
        {
        None => { generate_hello_world_example() }, 
        Some(val) => { unimplemented!() },
        }
    }
