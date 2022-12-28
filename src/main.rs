#![allow(unused_parens)]
use clap::Parser;
use bpgen::lang_util;



#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// Language for which boilerplate is to be generated
    #[clap(value_parser)]
    lang: Option<String>,

    /// Type of boilerplate to be generated
    #[clap(short, long, value_parser, default_value="")]
    type_of: Option<String>,

    /// Cleanup files that were created
    #[clap(short, long, value_parser, default_value="")]
    clean: String, // this value is useless
    }

fn main() 
    {
    let args = Args::parse();

    // TODO: replace with love_rust()
    println!("boilerplate: powered with <3 by Rust");

    if (args.clean != "")
        {
        // NOTE:
        // It is okay to pass args.clean as it wouldn't have reached here if it was
        // empty, validity of the value of args.clean is handled further down the 
        // stack
        lang_util::remove_boilerplate_files(&args.clean, args.type_of)
        }
    else
        {
        match args.lang
            {
            Some(lang) => { lang_util::parse_language_and_type(&lang, args.type_of)  },
            None => { eprintln!("bpgen failed, lang arg not provided") },
            }
        }

    }
