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

type SampleResult<T> = Result<T, anyhow::Error>;

#[allow(dead_code)]
fn parse_04<T: NumOps + FromStr>(value: String) -> SampleResult<T>
where
    SampleError: From<<T as FromStr>::Err>,
{
    let result = value.parse::<T>().map_err(|error| {
        let context = format!("Failed to parse value: {}", value);
        let err = SampleError::from(error);
        anyhow::Error::new(err).context(context)
    })?;
    Ok(result)
}

#[allow(dead_code)]
pub fn use_parse04() {
    let result = parse_04::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result);
    println!("{:?}", result.source().unwrap());
}
