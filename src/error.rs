use core::fmt::Display;
use core::fmt::Formatter;

use serde::ser;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Unsupported(&'static str),
}

impl ser::Error for Error {
    fn custom<T>(message: T) -> Self
    where
        T: Display,
    {
        Self::Message(message.to_string())
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(message) => fmt.write_str(message),
            Error::Unsupported(r#type) => write!(fmt, "Unsupported type: {}", r#type),
        }
    }
}
