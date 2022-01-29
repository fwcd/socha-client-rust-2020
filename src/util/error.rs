use std::io::Error as IoError;
use std::str::ParseBoolError;
use std::num::{ParseIntError, ParseFloatError};
use xml::reader::Error as XmlReaderError;
use xml::writer::Error as XmlWriterError;

/// A custom error type that abstracts over
/// other errors (such as IO/XML errors) and
/// can conveniently be used in conjunction with
/// `Result`.
#[derive(Debug)]
pub enum SCError {
    Io(IoError),
    XmlReader(XmlReaderError),
    XmlWriter(XmlWriterError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    ParseBool(ParseBoolError),
    Custom(String)
}

impl From<IoError> for SCError {
    fn from(error: IoError) -> Self { Self::Io(error) }
}

impl From<XmlReaderError> for SCError {
    fn from(error: XmlReaderError) -> Self { Self::XmlReader(error) }
}

impl From<XmlWriterError> for SCError {
    fn from(error: XmlWriterError) -> Self { Self::XmlWriter(error) }
}

impl From<ParseIntError> for SCError {
    fn from(error: ParseIntError) -> Self { Self::ParseInt(error) }
}

impl From<ParseFloatError> for SCError {
    fn from(error: ParseFloatError) -> Self { Self::ParseFloat(error) }
}

impl From<ParseBoolError> for SCError {
    fn from(error: ParseBoolError) -> Self { Self::ParseBool(error) }
}

impl From<String> for SCError {
    fn from(error: String) -> Self { Self::Custom(error) }
}

impl<'a> From<&'a str> for SCError {
    fn from(error: &'a str) -> Self { Self::Custom(error.to_owned()) }
}
