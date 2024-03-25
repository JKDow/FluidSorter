use thiserror::Error;

use super::vial_validate::VialValidateErr;

#[derive(Error, Debug)]
pub enum NewGameErr {
    #[error("Error translating color code {0}")]
    BadColorCode(String),
    #[error("Error validating vial {0}")]
    VialValidation(#[from] VialValidateErr)
}
