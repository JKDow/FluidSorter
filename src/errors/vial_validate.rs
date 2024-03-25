use thiserror::Error;

#[derive(Error, Debug)]
pub enum VialValidateErr {
    #[error("A vial was marked as empty that was not empty")]
    EmptyVialNotEmpty, 
    #[error("Non empty vial has no known colors")]
    UnknownVialNoColor,
    #[error("Colour marked unknown when it should be known")]
    MisplacedUnknown,
    #[error("Vial capacity is not valid")]
    MismatchedCapacity,
}
