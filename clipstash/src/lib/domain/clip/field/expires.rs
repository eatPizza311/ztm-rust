use crate::domain::clip::ClipError;
use crate::domain::time::Time;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(s) {
                Ok(time) => Ok(Self::new(time)),
                // use .into to turn chrono::ParseError into a ClipError
                Err(e) => Err(e.into()),
            }
        }
    }
}
