use crate::{color::Color, errors::vial_validate::VialValidateErr};


#[derive(Debug, Clone)]
pub(crate) struct Vial {
    holding: Vec<Color>,
    capacity: usize,
}

impl Vial {
    pub(crate) fn new(colors: Vec<Color>, capacity: usize) -> Self {
        Vial { holding: colors, capacity }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.capacity
    }

    pub(crate) fn view(&self) -> &[Color] {
        &self.holding
    }

    pub(crate) fn top(&self) -> &Color {
        self.holding.last().unwrap_or(&Color::Empty)
    }

    pub(crate) fn initial_validate(&mut self, capacity: usize) -> Result<(), VialValidateErr> {
        if let Color::Empty = self.top() {
            if self.holding.len() != 1 { return Err(VialValidateErr::EmptyVialNotEmpty) }
            self.capacity = capacity;
            self.holding.clear();
            return Ok(())
        }
        if self.holding.len() == 2 {
            if let Color::Unknown = self.holding.first().unwrap() {
                match self.top() {
                    Color::Empty | Color::Unknown => return Err(VialValidateErr::UnknownVialNoColor), 
                    _ => {}
                }
                self.capacity = capacity;
                for _ in 0..(capacity-2) { self.holding.insert(0, Color::Unknown) }
                return Ok(())
            }
        }
        for color in &self.holding {
            match color {
                Color::Unknown | Color::Empty => return Err(VialValidateErr::MisplacedUnknown),
                _ => {}
            }
        }
        if self.capacity != capacity { return Err(VialValidateErr::MismatchedCapacity) }
        Ok(())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.holding.is_empty()
    }
}
