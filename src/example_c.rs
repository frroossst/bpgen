pub fn get_hello_world() -> &'static str
    {
    "#include <stdio.h>
int main()
    {
    printf(\"hello world\\n\");
    return 0;
    }"
    }
