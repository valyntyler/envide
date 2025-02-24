use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    NoEqualsSign,
    EmptyKey,
    EmptyVal,
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub key: &'a str,
    pub val: &'a str,
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = ParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (key, val) = value.split_once('=').ok_or(ParseError::NoEqualsSign)?;

        if key.is_empty() { Err(ParseError::EmptyKey)? }
        if val.is_empty() { Err(ParseError::EmptyVal)? }

        Ok(Entry {
            key,
            val,
        })
    }
}

impl<'a> fmt::Display for Entry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.val)
    }
}
