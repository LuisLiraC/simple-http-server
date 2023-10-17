use std::convert::TryFrom;
use std::str;
use crate::Method;
use crate::ParseError;

pub struct Request {
    path: String,
    query_params: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
        unimplemented!()
    }
}

