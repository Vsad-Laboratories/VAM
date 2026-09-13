#[derive(Debug, Default)]
pub struct Diagnostics {
    // Future diagnostic fields
}

impl Diagnostics {
    pub fn new() -> Result<Self, crate::error::Error> {
        Ok(Self::default())
    }
}
