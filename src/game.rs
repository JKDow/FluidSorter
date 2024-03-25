
use std::str::FromStr;

use crate::{color::Color, errors::new_game::NewGameErr, vial::Vial};

/// # Description
/// Created from string from command line 
/// String will be in the form of color codes seperated by spaces and commas 
/// A space seperated colours in a vial and commas seperate vials 
/// # Color Codes
/// - 'pi' - pink
/// - 're' - red
/// - 'or' - orange
/// - 'lg' - light green
/// - 'dg' - dark green
/// - 'lb' - light blue
/// - 'db' - dark blue
/// - 'pu' - purple 
/// - 'gr' - grey
/// - 'un' - unknown
/// - 'em' - empty
/// # Input String
/// An input of `pi pi pu, pu pu pi, pi pu pu, em`
/// would mean that there are 4 vials
/// - pink pink purple
/// - purple purpl pink
/// - pink purple purple 
/// - empty
/// The left most color is the top color on the vial
/// # Empty/Unknown
/// If a vial is marked as empty or unknown then no other elements will be listed 
#[derive(Debug, Clone)]
pub struct Game {
    vials: Vec<Vial>, 
    num_vials: usize,
}

impl Game {
    pub fn new(input: &str, num_vials: usize, capacity: usize) -> Result<Self, NewGameErr> {
        let vials: Result<Vec<Vec<Color>>, String> = input.split(",")
            .map(|v| { v
                .split_whitespace()
                .map(Color::from_str)
                .collect::<Result<Vec<Color>, String>>()
            })
            .collect();

        let mut vials = match vials {
            Ok(vials) => vials,
            Err(e) => return Err(NewGameErr::BadColorCode(e)),
        };
        
        let vials: Vec<Vial> = vials.iter_mut()
            .map(|v| {
                v.reverse();
                Vial::new(v.to_vec(), v.len()) 
            })
            .collect();

        let mut game = Game {vials, num_vials};
        for vial in game.vials.iter_mut() {
            vial.initial_validate(capacity)?;
        }
        Ok(game)
    }

    pub fn print_state(&self) {
        for (i, vial) in self.vials.iter().enumerate() {
            println!("Vial {}", i);
            if vial.is_empty() {
                println!("  Empty");
            } else {
                for color in vial.view().iter().rev() {
                    println!("  {}", color);
                }
            }
        }
    }


}
