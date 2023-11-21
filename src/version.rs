use std::{fmt::Display, any::type_name};

use once_cell::sync::Lazy;
use regex::Regex;
use rocket::request::FromParam;

use crate::error::from_param::FromParamError;

#[derive(Copy, Clone, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8) -> Self {
        Self {major, minor}
    }
}

impl Display for Version {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "v{}.{}", self.major, self.minor)
    }
}

impl FromParam<'_> for Version {
    type Error = FromParamError;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"v(\d+)\.(\d+)").unwrap());

        if let Some(captures) = PATTERN.captures(param) {
            let parse_to_from_param_error = |_| FromParamError::pattern_mismatch(param.to_owned(), type_name::<Version>(), "v9.2");
            
            let major = captures.get(1).unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(parse_to_from_param_error)?;

            let minor = captures.get(2).unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(parse_to_from_param_error)?;

            Ok(Version::new(major, minor))
        } else {
            Err(FromParamError::pattern_mismatch(param.to_owned(), type_name::<Version>(), "v9.2"))
        }
    }
}