use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, String>;

pub trait ToErrString<T> {
    fn to_err_string(self) -> Result<T>;
}

impl <T> ToErrString<T> for zbus::Result<T> {
    fn to_err_string(self) -> Result<T> {
        self.or_else(|err| Err(err.to_string()))
    }
}

impl <T> ToErrString<T> for std::result::Result<T, FromUtf8Error> {
    fn to_err_string(self) -> Result<T> {
        self.or_else(|err| Err(err.to_string()))
    }
}

pub trait ToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl <T> ToOption<T> for Result<T> {
    fn to_option(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            _ => None
        }
    }
}