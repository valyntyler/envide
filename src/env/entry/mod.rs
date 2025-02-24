use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    NoEqualsSign,
    EmptyKey,
    EmptyVal,
}

#[derive(Debug)]
pub struct Entry {
    pub key: String,
    pub val: String,
}

impl TryFrom<&str> for Entry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (key, val) = value.split_once('=').ok_or(ParseError::NoEqualsSign)?;

        if key.is_empty() { return Err(ParseError::EmptyKey)?; }
        if val.is_empty() { return Err(ParseError::EmptyVal)?; }

        Ok(Entry {
            key: key.to_string(),
            val: val.to_string()
        })
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.val)
    }
}
