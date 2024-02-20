use num_traits::NumOps;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

#[allow(dead_code)]
pub fn parse_02<T: NumOps + FromStr>(value: String) -> Result<T, SampleError>
where
    SampleError: From<<T as FromStr>::Err>,
{
    let result = value
        .parse::<T>()
        .map_err(|error| SampleError::from(error))?;
    Ok(result)
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

#[allow(dead_code)]
pub fn use_parse_02() {
    let result = parse_02::<i32>(String::from("123")).unwrap();
    println!("{:?}", result);
    let result = parse_02::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result.to_string());
    let result = parse_02::<f32>(String::from("123")).unwrap();
    println!("{:?}", result);
    let result = parse_02::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result.to_string());
}
