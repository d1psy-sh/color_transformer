use std::fmt;

/// This is the struct which decupples the scheme from the input file type
/// so this represents a scheme in general
pub struct Scheme {
    // TODO: add the fields
    pub white: String,
    pub red: String,
    pub pink: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub aqua: String,
    pub cyan: String,
    pub purple: String,
    pub violet: String,
    pub orange: String,
    pub brown: String,

    pub seagreen: String,
    pub turquoise: String,

    pub foreground: String,
    pub background: String,

    pub cursor_color: String,
    pub selection_background: String,
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

