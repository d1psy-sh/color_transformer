use std::fmt;
use std::fs;

use clap::Parser;
use serde::Deserialize;

fn main() {
    let infos = parse_args();
    let nvim_scheme =
        parse(fs::read_to_string(&infos.input_file).expect("Error while reading scheme file"));
    println!("{:?}; {:?}", infos, nvim_scheme);
}

// A simple colorscheme transpiler from windows and kitty terminal themes to
// a full nvim colorscheme using nvim-colorbuddy from tjdevries
#[derive(Debug, Parser)]
struct Args {
    /// The input file
    #[arg(short, long)]
    input: Option<String>,
    /// The output file
    #[arg(short, long)]
    output: Option<String>,
    /// The colorscheme type windowes terminal (json) or kitty (conf)
    #[arg(short = 't', long, default_value = "win")]
    scheme_type: Option<String>,
}

#[derive(Debug)]
enum SchemeType {
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

#[derive(Debug)]
struct Infos {
    input_file: String,
    output_file: String,
    scheme_type: SchemeType,
}

fn parse_args() -> Infos {
    let args = Args::parse();
    // here come the sane default the belong in a default method I know
    // TODO: refactor in a default method
    let mut infos = Infos {
        input_file: "test.json".to_string(),
        output_file: "out_test.json".to_string(),
        scheme_type: SchemeType::Win,
    };
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
            "win" => infos.scheme_type = SchemeType::Win,
            "kitty" => infos.scheme_type = SchemeType::Kitty,
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

/// This is the struct for serde to deserialize the scheme json
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinTermScheme {
    pub name: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub purple: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_purple: String,
    pub bright_cyan: String,
    pub bright_white: String,
    pub background: String,
    pub foreground: String,
    pub selection_background: String,
    pub cursor_color: String,
}

/// This is the struct which decupples the scheme from the input file type
/// so this represents a scheme in general
struct Scheme {
    // TODO: add the fields
}

// TODO: implement those
impl Scheme {
    fn new() -> Self {
        Self {}
    }
    /// this function parses the scheme at hand to a nvim scheme using colorbuddy.nvim
    // TODO: implement this (replaces the parse function or the parse func can invoke this one)
    fn to_nvim() -> String {
        todo!()
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Self::new()
    }
}

fn parse(input: String) -> String {
    let win_scheme: WinTermScheme =
        serde_json::from_str(&input).expect("Error while parsing scheme file");

    // TODO: there are some misses in the colors so that is a thing we have to fix with like some
    // calculations and some tricks leaving it for now
    let nvim_scheme = format!(
        r#"require("colorbuddy").setup()

Color.new('white',     '{0}')
Color.new('red',       '{1}')
Color.new('pink',      '{2}')
Color.new('green',     '{3}')
Color.new('yellow',    '{4}')
Color.new('blue',      '{5}')
Color.new('aqua',      '{6}')
Color.new('cyan',      '{7}')
Color.new('purple',    '{8}')
Color.new('violet',    '{9}')
Color.new('orange',    '{10}')
Color.new('brown',     '{11}')

Color.new('seagreen',  '{12}')
Color.new('turquoise', '{13}')"#,
        win_scheme.white,
        win_scheme.red,
        "#aa00aa",
        win_scheme.green,
        win_scheme.yellow,
        win_scheme.blue,
        "#00ffff",
        win_scheme.cyan,
        win_scheme.purple,
        "#ff00ff",
        win_scheme.bright_red,
        win_scheme.bright_yellow,
        win_scheme.bright_green,
        "#00ff00"
    );
    nvim_scheme
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        let input = r#"{
  "name": "3024 Night",
  "black": "\#090300",
  "red": "\#db2d20",
  "green": "\#01a252",
  "yellow": "\#fded02",
  "blue": "\#01a0e4",
  "purple": "\#a16a94",
  "cyan": "\#b5e4f4",
  "white": "\#a5a2a2",
  "brightBlack": "\#5c5855",
  "brightRed": "\#e8bbd0",
  "brightGreen": "\#3a3432",
  "brightYellow": "\#4a4543",
  "brightBlue": "\#807d7c",
  "brightPurple": "\#d6d5d4",
  "brightCyan": "\#cdab53",
  "brightWhite": "\#f7f7f7",
  "background": "\#090300",
  "foreground": "\#a5a2a2",
  "selectionBackground": "\#4a4543",
  "cursorColor": "\#a5a2a2"
}"#
        .to_string();
        let expected = "ahh todo".to_string();
        let scheme = super::parse(input);
        assert_eq!(
            expected, scheme,
            "These two are not matching expected is {}\n, and we got {}",
            expected, scheme
        )
    }
}
