#[derive(Debug, Default)]
pub struct Config {
    // Future configuration fields
}

impl Config {
    pub fn new() -> Result<Self, crate::error::Error> {
        Ok(Self::default())
    }
}
