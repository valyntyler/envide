use core::fmt;

#[derive(Debug)]
pub struct Entry {
    pub key: String,
    pub val: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.val)
    }
}
