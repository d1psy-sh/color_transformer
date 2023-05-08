use serde::Deserialize;
use crate::scheme::Scheme;

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

pub fn parse_win_scheme(input: String) -> Scheme {
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
    /// this should go into kitty and windows file
    fn parser_test() {
        todo!()
    }
}
