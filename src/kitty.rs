use crate::scheme::Scheme;
use std::collections::HashMap;

pub fn deserialize_kitty_scheme(input: String) -> std::collections::HashMap<String, String> {
    let lines = input
        .lines()
        .map(|line| {
            // if first letter is '#' then we ditch the line
            // if the line is empty we ditch it too
            let first = line.chars().next();
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
    lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(' ');
            let key = split.next().expect("[Kitty Parse Scheme]: no key");
            let value = split.next().expect("[Kitty Parse Scheme]: no value");
            (key.to_string(), value.to_string())
        })
        .collect::<std::collections::HashMap<String, String>>()
}

pub fn parse_kitty_scheme(input: String) -> Scheme {
    let conf_colors = deserialize_kitty_scheme(input);
    let default_colors = HashMap::from([
        ("color0", "#868686".to_string()),
        ("color1", "#ff6600".to_string()),
        ("color10", "#00ff00".to_string()),
        ("color11", "#ffff00".to_string()),
        ("color12", "#0000ff".to_string()),
        ("color13", "#ff00ff".to_string()),
        ("color14", "#00ffff".to_string()),
        ("color15", "#e5e5e5".to_string()),
        ("color2", "#ccff04".to_string()),
        ("color3", "#ffcc00".to_string()),
        ("color4", "#44b3cc".to_string()),
        ("color5", "#9933cc".to_string()),
        ("color6", "#44b3cc".to_string()),
        ("color7", "#f4f4f4".to_string()),
        ("color8", "#545454".to_string()),
        ("color9", "#ff0000".to_string()),
        ("cursor", "#ffffff".to_string()),
        ("cursor_text_color", "#000000".to_string()),
        ("background", "#000000".to_string()),
        ("foreground", "#ffffff".to_string()),
        ("selection_background", "#b4d5ff".to_string()),
        ("selection_foreground", "#000000".to_string()),
    ]);
    Scheme {
        white: conf_colors
            .get("color15")
            .unwrap_or(&default_colors["color15"])
            .to_string(),
        red: conf_colors
            .get("color1")
            .unwrap_or(&default_colors["color1"])
            .to_string(),
        pink: conf_colors
            .get("color13")
            .unwrap_or(&default_colors["color13"])
            .to_string(),
        green: conf_colors
            .get("color2")
            .unwrap_or(&default_colors["color2"])
            .to_string(),
        yellow: conf_colors
            .get("color3")
            .unwrap_or(&default_colors["color3"])
            .to_string(),
        blue: conf_colors
            .get("color4")
            .unwrap_or(&default_colors["color4"])
            .to_string(),
        aqua: conf_colors
            .get("color6")
            .unwrap_or(&default_colors["color6"])
            .to_string(),
        cyan: conf_colors
            .get("color12")
            .unwrap_or(&default_colors["color12"])
            .to_string(),
        purple: conf_colors
            .get("color5")
            .unwrap_or(&default_colors["color5"])
            .to_string(),
        violet: conf_colors
            .get("color9")
            .unwrap_or(&default_colors["color9"])
            .to_string(),
        orange: conf_colors
            .get("color10")
            .unwrap_or(&default_colors["color10"])
            .to_string(),
        brown: conf_colors
            .get("color11")
            .unwrap_or(&default_colors["color11"])
            .to_string(),
        seagreen: conf_colors
            .get("color14")
            .unwrap_or(&default_colors["color14"])
            .to_string(),
        turquoise: conf_colors
            .get("color7")
            .unwrap_or(&default_colors["color7"])
            .to_string(),
        foreground: conf_colors
            .get("foreground")
            .unwrap_or(&default_colors["foreground"])
            .to_string(),
        background: conf_colors
            .get("background")
            .unwrap_or(&default_colors["background"])
            .to_string(),
        cursor_color: conf_colors
            .get("cursor")
            .unwrap_or(&default_colors["cursor"])
            .to_string(),
        selection_background: conf_colors
            .get("selection_background")
            .unwrap_or(&default_colors["selection_background"])
            .to_string(),
    }
}

// TODO: add tests
