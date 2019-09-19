use std::io::Error as IoError;
use std::str::ParseBoolError;
use std::num::{ParseIntError, ParseFloatError};
use xml::reader::Error as XmlError;

/// A custom error type that abstracts over
/// other errors (such as IO/XML errors) and
/// can conveniently be used in conjunction with
/// `Result`.
#[derive(Debug)]
pub enum SCError {
	Io(IoError),
	Xml(XmlError),
	ParseInt(ParseIntError),
	ParseFloat(ParseFloatError),
	ParseBool(ParseBoolError),
	Custom(String)
}

impl From<IoError> for SCError {
	fn from(error: IoError) -> Self { Self::Io(error) }
}

impl From<XmlError> for SCError {
	fn from(error: XmlError) -> Self { Self::Xml(error) }
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
