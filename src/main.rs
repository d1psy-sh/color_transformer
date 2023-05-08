use std::fs;

use color_transformer::{
    args::{parse_args, SchemeType},
    kitty,
    scheme::Scheme,
    windows,
};

fn main() {
    let infos = parse_args();
    let scheme = parse_to_scheme(
        fs::read_to_string(infos.input_file).expect("Unable to read file"),
        infos.scheme_type,
    );
    println!("{:?}", scheme);
}

/// this func should parse a kitty or win scheme to a general scheme
fn parse_to_scheme(input: String, scheme_type: SchemeType) -> Scheme {
    match scheme_type {
        SchemeType::Win => windows::parse_win_scheme(input),
        SchemeType::Kitty => kitty::parse_kitty_scheme(input),
    }
    // TODO: there are some misses in the colors so that is a thing we have to fix with like some
    // calculations and some tricks leaving it for now
    // TODO: add the rest of the colors
}

#[cfg(test)]
mod tests {
    #[test]
    /// this should go into kitty and windows file
    fn parser_test() {
        todo!()
    }
}
