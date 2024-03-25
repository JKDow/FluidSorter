use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug)]
pub(crate) enum Color {
    Pink,
    Red,
    Orange,
    LightGreen,
    DarkGreen, 
    LightBlue,
    DarkBlue,
    Purple,
    Grey,
    Unknown,
    Empty,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::Pink => "Pink",
            Color::Red => "Red",
            Color::Orange => "Orange",
            Color::LightGreen => "Light Green",
            Color::DarkGreen => "Dark Green",
            Color::LightBlue => "Light Blue",
            Color::DarkBlue => "Dark Blue",
            Color::Purple => "Purple",
            Color::Grey => "Grey",
            Color::Unknown => "Unknown",
            Color::Empty => "Empty",
        })
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pi" => Ok(Color::Pink),
            "re" => Ok(Color::Red),
            "or" => Ok(Color::Orange),
            "lg" => Ok(Color::LightGreen),
            "dg" => Ok(Color::DarkGreen),
            "lb" => Ok(Color::LightBlue),
            "db" => Ok(Color::DarkBlue),
            "pu" => Ok(Color::Purple),
            "gr" => Ok(Color::Grey),
            "un" => Ok(Color::Unknown),
            "em" => Ok(Color::Empty),
            _ => Err(s.to_string()),
        }
    }
}
