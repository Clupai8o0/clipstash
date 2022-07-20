use super::super::ClipError;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub struct ShortCode(String); // code at the end of the url

impl ShortCode {
  pub fn new() -> Self {
    use rand::prelude::*;
    let allowed_chars = [
      'a', 'b', 'c', 'd', '1', '2', '3', '4'
    ];

    let mut rng = thread_rng();
    let mut shortcode = String::with_capacity(10); // generates string with max no. of characters

    for _ in 0..10 {
      shortcode.push(
        *allowed_chars
          .choose(&mut rng) // random character out of the array
          .expect("sampling array should have values")
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

// since it will work with the website often, conversions will make things easier
impl From<ShortCode> for String {
  fn from(shortcode: ShortCode) -> Self {
    shortcode.0
  }
}
impl From<&str> for ShortCode {
  fn from(shortcode: &str) -> Self {
    ShortCode(shortcode.to_owned())
  }
}

impl FromStr for ShortCode {
  type Err = ClipError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(s.into()))
  }
}