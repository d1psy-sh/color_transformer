use clap::Parser;
use std::fmt;

// A simple colorscheme transpiler from windows and kitty terminal themes to
// a full nvim colorscheme using nvim-colorbuddy from tjdevries
#[derive(Debug, Parser)]
pub struct Args {
    /// The input file
    #[arg(short, long)]
    pub input: Option<String>,
    /// The output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// The colorscheme type windowes terminal (json) or kitty (conf)
    #[arg(short = 't', long, default_value = "win")]
    pub scheme_type: Option<String>,
}

pub fn parse_args() -> Infos {
    let args = Args::parse();
    // here come the sane default the belong in a default method I know
    let mut infos = Infos::default();
    if args.input.is_some() {
        infos.input_file = args.input.unwrap();
    } else {
        println!(
            "No input file provided, using default: {}",
            infos.input_file
        );
    }
    if args.output.is_some() {
        infos.output_file = args.output.unwrap();
    } else {
        println!(
            "No output file provided, using default: {}",
            infos.output_file
        );
    }
    if args.scheme_type.is_some() {
        match args.scheme_type.unwrap().as_str() {
            "win" | "w" => infos.scheme_type = SchemeType::Win,
            "kitty" | "k" => infos.scheme_type = SchemeType::Kitty,
            _ => println!(
                "Invalid scheme type! There is:\n\n\t- \"win\"\n\t-\"kitty\"\nUsing default: {}",
                infos.scheme_type
            ),
        }
    } else {
        println!(
            "No output file provided, using default: {}",
            infos.scheme_type
        );
    }
    infos
}

#[derive(Debug)]
pub struct Infos {
    pub input_file: String,
    pub output_file: String,
    pub scheme_type: SchemeType,
}

impl Infos {
    fn new() -> Self {
        Infos {
            input_file: "test.json".to_string(),
            output_file: "test.lua".to_string(),
            scheme_type: SchemeType::Win,
        }
    }
}

impl Default for Infos {
    fn default() -> Self {
        Infos::new()
    }
}

#[derive(Debug)]
pub enum SchemeType {
    Win,
    Kitty,
}

impl fmt::Display for SchemeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SchemeType::Win => write!(f, "win"),
            SchemeType::Kitty => write!(f, "kitty"),
        }
    }
}

