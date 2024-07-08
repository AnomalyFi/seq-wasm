pub struct Address([u8; Self::LEN]);

impl Address {
    pub const LEN: usize = 33;
    // Constructor function for Address
    #[must_use]
    pub fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
    fn ne(&self, other: &Self) -> bool {
        self.0[..] != other.0[..]
    }
}

pub struct ID([u8; Self::LEN]);

impl ID {
    pub const LEN: usize = 32;
    // Constructor function for ID
    #[must_use]
    pub fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    #[must_use]
    pub fn empty_id() -> Self {
        Self([0u8; Self::LEN])
    }
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
    fn ne(&self, other: &Self) -> bool {
        self.0[..] != other.0[..]
    }
}
