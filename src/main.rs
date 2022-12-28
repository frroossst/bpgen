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

    println!("boilerplate: powered with <3 by Rust");

    if args.clean != ""
        {
        println!("{:?}", args.clean)
        }

    }
