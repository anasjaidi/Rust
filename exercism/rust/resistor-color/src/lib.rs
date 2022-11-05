
#[derive(Debug, PartialEq, Eq)]

pub enum ResistorColor {
    Black = 1,
    Blue = 2,
    Brown = 3,
    Green = 4,
    Grey =  5,
    Orange = 6,
    Red = 7,
    Violet = 8,
    White = 9,
    Yellow = 10,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    let _return  = match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    };
    _return
}

pub fn value_to_color_string(value: u32) -> String {
    let _return  = match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range")
    };
    _return
}

pub fn colors() -> Vec<ResistorColor> {
    use ResistorColor::*;
    vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
}
