use num_trates::NumOps;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

#[allow(dead_code)]
pub fn parse_02<T: NumOps + FromStr>(value: String) -> Result<T, E> {
    todo!()
}

#[derive(Debug)]
pub enum SampleError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
}
impl Error for SampleError {}
impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::IntError(err) => write!(f, "IntError: {}", err),
            SampleError::FloatError(err) => write!(f, "FloatError: {}", err),
        }
    }
}

impl From<ParseIntError> for SampleError {
    fn from(value: ParseIntError) -> Self {
        Self::IntError(value)
    }
}

impl From<ParseFloatError> for SampleError {
    fn from(value: ParseFloatError) -> Self {
        Self::FloatError(value)
    }
}
