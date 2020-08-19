use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MyError {
    #[error("foo failed with size {0}")]
    FooFailed(usize),
    #[error("failed due to another reason")]
    OtherReason,
}

pub fn library_method(input: usize) -> Result<usize, MyError> {
    if input < 10 {
        return Err(MyError::FooFailed(input));
    }

    if input == 42 {
        return Err(MyError::OtherReason);
    }

    return Ok(input * 999);
}

pub fn other_method(input: usize) -> anyhow::Result<usize> {
    // no returns! last line without a semicolon of any block returns a value.
    // the type of all branches must be equal
    if input != 30 {
        Ok(input)
    } else {
        Err(MyError::FooFailed(input).into())
    }
}

pub fn application_method(input: usize) -> anyhow::Result<usize> {
    let library_value = library_method(input)?;
    let other_value = other_method(input)?;

    Ok(library_value + other_value)
}

#[cfg(test)]
mod tests {
    use super::{library_method, MyError};

    pub fn test_foo() {
        assert_eq!(Err(MyError::FooFailed(8)), library_method(8));
        assert_eq!(Err(MyError::OtherReason), library_method(42));
        assert_eq!(Ok(9999), library_method(10));
    }
}
