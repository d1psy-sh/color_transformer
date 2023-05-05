use std::fmt;
use std::fs;
use std::fs::write;

use clap::Parser;
use serde::Deserialize;

fn main() {
    let infos = parse_args();
    let scheme = parse_to_scheme(
        fs::read_to_string(infos.input_file).expect("Unable to read file"),
        infos.scheme_type,
    );
    println!("{:?}", scheme);
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
        input_file: "test_data/win_test.json".to_string(),
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
    white: String,
    red: String,
    pink: String,
    green: String,
    yellow: String,
    blue: String,
    aqua: String,
    cyan: String,
    purple: String,
    violet: String,
    orange: String,
    brown: String,

    seagreen: String,
    turquoise: String,

    foreground: String,
    background: String,

    cursor_color: String,
    selection_background: String,
}

impl fmt::Debug for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write every field in a new line for better readability
        fmt::Display::fmt(
            &format!(
                r#"Scheme:
        white: {0},
        red: {1},
        pink: {2},
        green: {3},
        yellow: {4},
        blue: {5},
        aqua: {6},
        cyan: {7},
        purple: {8},
        violet: {9},
        orange: {10},
        brown: {11},

        seagreen: {12},
        turquoise: {13},

        cursor_color: {14},
        selection_background: {15},

        foreground: {16},
        background: {17},
        "#,
                self.white,
                self.red,
                self.pink,
                self.green,
                self.yellow,
                self.blue,
                self.aqua,
                self.cyan,
                self.purple,
                self.violet,
                self.orange,
                self.brown,
                self.seagreen,
                self.turquoise,
                self.cursor_color,
                self.selection_background,
                self.foreground,
                self.background,
            ),
            f,
        )
    }
}

// TODO: implement those
impl Scheme {
    fn new() -> Self {
        Self {
            white: String::new(),
            red: String::new(),
            pink: String::new(),
            green: String::new(),
            yellow: String::new(),
            blue: String::new(),
            aqua: String::new(),
            cyan: String::new(),
            purple: String::new(),
            violet: String::new(),
            orange: String::new(),
            brown: String::new(),

            seagreen: String::new(),
            turquoise: String::new(),

            cursor_color: String::new(),
            selection_background: String::new(),

            foreground: String::new(),
            background: String::new(),
        }
    }
    //         r#"require("colorbuddy").setup()
    //
    // Color.new('white',     '{0}')
    // Color.new('red',       '{1}')
    // Color.new('pink',      '{2}')
    // Color.new('green',     '{3}')
    // Color.new('yellow',    '{4}')
    // Color.new('blue',      '{5}')
    // Color.new('aqua',      '{6}')
    // Color.new('cyan',      '{7}')
    // Color.new('purple',    '{8}')
    // Color.new('violet',    '{9}')
    // Color.new('orange',    '{10}')
    // Color.new('brown',     '{11}')
    //
    // Color.new('seagreen',  '{12}')
    // Color.new('turquoise', '{13}')"#,
}

impl Default for Scheme {
    fn default() -> Self {
        Self::new()
    }
}

/// this func should parse a kitty or win scheme to a general scheme
fn parse_to_scheme(input: String, scheme_type: SchemeType) -> Scheme {
    match scheme_type {
        SchemeType::Win => parse_win_scheme(input),
        SchemeType::Kitty => parse_kitty_scheme(input),
    }
    // TODO: there are some misses in the colors so that is a thing we have to fix with like some
    // calculations and some tricks leaving it for now
    // TODO: add the rest of the colors
}

fn parse_kitty_scheme(input: String) -> Scheme {
    todo!()
}

fn parse_win_scheme(input: String) -> Scheme {
    let win_scheme = serde_json::from_str::<WinTermScheme>(&input)
        .expect("Error while parsing the win scheme json");
    Scheme {
        white: win_scheme.white,
        red: win_scheme.red,
        purple: win_scheme.purple,
        blue: win_scheme.blue,
        green: win_scheme.green,
        yellow: win_scheme.yellow,
        cyan: win_scheme.cyan,
        background: win_scheme.background,
        foreground: win_scheme.foreground,
        selection_background: win_scheme.selection_background,
        cursor_color: win_scheme.cursor_color,
        orange: win_scheme.bright_yellow,
        pink: win_scheme.bright_red,
        aqua: win_scheme.bright_blue,
        brown: win_scheme.bright_black,
        seagreen: win_scheme.bright_green,
        violet: win_scheme.bright_purple,
        turquoise: win_scheme.bright_cyan,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        todo!()
    }
}
