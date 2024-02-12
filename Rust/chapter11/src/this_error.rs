use num_traits::NumOps;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SampleError {
    #[error(transparent)]
    IntError(#[from] ParseIntError),
    #[error(transparent)]
    FloatError(#[from] ParseFloatError),
}

#[allow(dead_code)]
pub fn parse_03<T: NumOps + FromStr>(value: String) -> Result<T, SampleError>
where
    SampleError: From<<T as FromStr>::Err>,
{
    let result = value
        .parse::<T>()
        .map_err(|error| SampleError::from(error))?;
    Ok(result)
}

// impl From<ParseIntError> for SampleError {
//     fn from(value: ParseIntError) -> Self {
//         Self::IntError(value.to_string())
//     }
// }

// impl From<ParseFloatError> for SampleError {
//     fn from(value: ParseFloatError) -> Self {
//         Self::FloatError(value.to_string())
//     }
// }

#[allow(dead_code)]
pub fn use_parse_03() {
    let result = parse_03::<i32>(String::from("123")).unwrap();
    println!("{:?}", result);
    let result = parse_03::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result.to_string());
    let result = parse_03::<f32>(String::from("123")).unwrap();
    println!("{:?}", result);
    let result = parse_03::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result.to_string());
}
