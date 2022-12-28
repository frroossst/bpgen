use clap::Parser;
use bpgen::{lang_util};



#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// Name of the singular file to be removed
    #[clap(value_parser)]
    lang: String,

    /// Type of boilerplate to be generated
    #[clap(short, long, value_parser, default_value="")]
    type_of: Option<String>,

    /// Cleanup files that were created
    #[clap(short, long, value_parser, default_value="")]
    clean: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    println!("boilerplate: powered with <3 by Rust");

    let lang_cmp = args.lang.to_lowercase();
    let type_of = args.type_of;
    let type_of_for_clean = type_of.clone();
    let lang_cmp_for_clean = lang_cmp.clone();
    let clean = args.clean;

    match lang_cmp.as_str()
        {
        "rust" => { lang_util::parse_language_and_type(lang_cmp, type_of) }
        "c" => { lang_util::parse_language_and_type(lang_cmp, type_of) }
        _ => { eprintln!("language not supported") }
        }

    match clean
        {
        Some(val) => 
            { 
            match val.as_str()
                {
                "c" => { lang_util::remove_boilerplate_files(&lang_cmp_for_clean, type_of_for_clean) },
                _ => {}
                }
            },
        None => {}
        }
    }
