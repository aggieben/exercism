use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl ResistorColor {
    fn try_from_u32(value: u32) -> Result<ResistorColor,&'static str> {
        use crate::ResistorColor::*;
        match value {
            0 => Ok(Black),
            1 => Ok(Brown),
            2 => Ok(Red),
            3 => Ok(Orange),
            4 => Ok(Yellow),
            5 => Ok(Green),
            6 => Ok(Blue),
            7 => Ok(Violet),
            8 => Ok(Grey),
            9 => Ok(White),
            _ => Err("invalid ResistorColor value")
        }
    }

    fn to_string(value: &ResistorColor) -> &'static str {
        use crate::ResistorColor::*;
        match value {
            Black => "Black",
            Blue => "Blue",
            Brown => "Brown",
            Green => "Green",
            Grey => "Grey",
            Orange => "Orange",
            Red => "Red",
            Violet => "Violet",
            White => "White",
            Yellow => "Yellow",
        }
    }
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ResistorColor::to_string(self))
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::try_from_u32(value) {
        Ok(resistor) => ResistorColor::to_string(&resistor).to_owned(),
        Err(_) => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    (0..=9).map(|i| { match ResistorColor::try_from_u32(i) {
        Err(msg) => panic!("{}", msg),
        Ok(resistor) => resistor
    }}).collect()
}
