#[derive(Clone, Copy, Debug)]
pub enum Colour {
    White = 0b000,
    Red = 0b001,
    Green = 0b010,
    Blue = 0b011,
    Black = 0b100
}

impl From<char> for Colour {
    fn from(char: char) -> Self {
        match char.to_ascii_uppercase() {
            'W' => Colour::White,
            'R' => Colour::Red,
            'G' => Colour::Green,
            'B' => Colour::Blue,
            'N' => Colour::Black,
            _ => panic!("Expected values in [W, R, G, B, N]")
        }
    }
}

impl Colour {
    // pub fn convert_to_char(&self) -> char {
    //     match self {
    //         Colour::White => 'W',
    //         Colour::Red => 'R',
    //         Colour::Green => 'G',
    //         Colour::Blue => 'B',
    //         Colour::Black => 'N',
    //     }
    // }
}

impl From<u16> for Colour {
    fn from(number: u16) -> Self {
        match number {
            0b000 => Colour::White,
            0b001 => Colour::Red,
            0b010 => Colour::Green,
            0b011 => Colour::Blue,
            0b100 => Colour::Black,
            _ => panic!("Invalid conversion to colour")
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Red => write!(f, "Red"),
            Colour::White => write!(f, "White"),
            Colour::Green => write!(f, "Green"),
            Colour::Blue => write!(f, "Blue"),
            Colour::Black => write!(f, "Black"),
        }
    }
}