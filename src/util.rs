pub type Result<T> = std::result::Result<T, String>;

pub trait ToErrString<T> {
    fn to_err_string(self) -> Result<T>;
}

impl <T> ToErrString<T> for zbus::Result<T> {
    fn to_err_string(self) -> Result<T> {
        self.or_else(|err| Err(err.to_string()))
    }
}