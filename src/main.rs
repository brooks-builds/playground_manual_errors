use std::{error::Error, fmt::Display};

fn main() {
    let result = match add_them(0, 2) {
        Ok(sum) => sum,
        Err(error) => panic!("{error}"),
    };
    dbg!(result);
}

#[derive(Debug)]
enum CustomError {
    CannotBeZero,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::CannotBeZero => "number provided cannot be zero",
        };

        write!(f, "Error: {message}")
    }
}

fn add_them(a: i32, b: i32) -> Result<i32, CustomError> {
    if a == 0 || b == 0 {
        return Err(CustomError::CannotBeZero);
    }

    Ok(a + b)
}
