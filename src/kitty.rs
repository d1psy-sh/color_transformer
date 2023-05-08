use crate::scheme::Scheme;

// NOTE: a colleciton would be better for the job
#[derive(Debug)]
pub struct KittyScheme {
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,

    pub cursor: String,
    pub cursor_text_color: String,

    pub selection_background: String,
    pub selection_forground: String,

    pub foreground: String,
    pub background: String,
}

pub fn deserialize_kitty_scheme(input: String) -> std::collections::HashMap<String, String> {
    let mut lines = input
        .lines()
        .into_iter()
        .map(|line| {
            // if first letter is '#' then we ditch the line
            // if the line is empty we ditch it too
            let first = line.chars().nth(0);
            match first {
                Some(char) => {
                    if char == '#' {
                        ""
                    } else {
                        line
                    }
                }
                None => "",
            }
        })
        .filter(|line| line != &"")
        .collect::<Vec<&str>>();
    let scheme_map = lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(' ');
            let key = split.next().expect("[Kitty Parse Scheme]: no key");
            let value = split.next().expect("[Kitty Parse Scheme]: no value");
            (key.to_string(), value.to_string())
        })
        .collect::<std::collections::HashMap<String, String>>();
    scheme_map
}

fn parse_kitty_color(map: std::collections::HashMap<&str, &str>, mut scheme: KittyScheme) {
    todo!("use a collection here")
}

pub fn parse_kitty_scheme(input: String) -> Scheme {
    todo!()
}
