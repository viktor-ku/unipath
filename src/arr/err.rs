use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ArrError {
    TooLong,
    Empty,
    Impossible,
}

impl fmt::Display for ArrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrError::TooLong => {
                write!(f, "Does not fit into any of the arr brackets")
            }

            ArrError::Empty => {
                write!(f, "Empty")
            }

            ArrError::Impossible => {
                write!(f, "Impossible")
            }
        }
    }
}

impl Error for ArrError {}
