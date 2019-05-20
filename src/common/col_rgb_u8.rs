use std::fmt;

#[derive(Clone)]
pub struct ColRgbU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Debug for ColRgbU8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}-{:02X}-{:02X}", self.r, self.g, self.b)
    }
}
