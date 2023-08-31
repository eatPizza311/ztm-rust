use crate::domain::clip::ClipError;
use derive_more::From;
use rocket::request::FromParam;
use rocket::{UriDisplayPath, UriDisplayQuery};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// From trait will automatically convert a String into ShortCode
#[derive(Debug, Clone, Deserialize, Serialize, From, UriDisplayPath, UriDisplayQuery)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];
        // random number generator
        let mut rng = thread_rng();
        // 10 character long string, this is more efficient because it allocate exact amount of memory
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            )
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for ShortCode {
    fn default() -> Self {
        Self::new()
    }
}

// Convert ShortCode into a String
impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

// Convert borrowed str into a ShortCode
impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        // this is benefit from From trait
        ShortCode(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // into functionality is implemented in the From<&str>
        Ok(Self(s.into()))
    }
}

impl<'r> FromParam<'r> for ShortCode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(ShortCode::from(param))
    }
}
