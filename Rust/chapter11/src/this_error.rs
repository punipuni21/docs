#[derive(Debug, Error)]
pub enum SampleError {
    #[error("IntError: {0}")]
    IntError(String),
    #[error("FloatError: {0}")]
    FloatError(String),
}

impl From<ParseIntError> for SampleError {
    fn from(value: String) -> Self {
        Self::IntError(value.to_string())
    }
}

impl From<ParseFloatError> for SampleError {
    fn from(value: String) -> Self {
        Self::FloatError(value.to_string())
    }
}
