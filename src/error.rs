use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AlgoError {
    InputError,
}

impl std::error::Error for AlgoError {}

impl fmt::Display for AlgoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgoError::InputError => write!(f, "Algorithm Input Error"),
        }
    }
}
