use std::{error::Error, fmt::Display};

fn main() {
    let one = 0;
    let two = 5;
    let result = match do_the_thing(one, two) {
        Ok(result) => result,
        Err(error) => panic!("{error}"),
    };
    println!("The result is {result}");
}

fn do_the_thing(one: i32, two: i32) -> Result<i32, MyError> {
    if one == 0 || two == 0 {
        Err(MyError::CannotBeZero)
    } else {
        Ok(one + two)
    }
}

#[derive(Debug)]
enum MyError {
    CannotBeZero,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::CannotBeZero => "One of the numbers must be above 0",
        };

        write!(f, "An error occurred: {message}")
    }
}

impl Error for MyError {}
