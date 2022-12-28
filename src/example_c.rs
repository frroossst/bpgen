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
