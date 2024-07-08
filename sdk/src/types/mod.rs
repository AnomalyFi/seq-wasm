/// Address equivalent to address defined in hypersdk.
pub struct Address([u8; Self::LEN]);

impl Address {
    pub const LEN: usize = 33;
    /// Constructor function for Address
    #[must_use]
    pub fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }
    /// returns the address as a slice of bytes
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

impl Default for Address {
    /// Returns a new Address with all bytes set to 0. Equivalent to Empty Address in hypersdk.
    #[must_use]
    fn default() -> Self {
        Self([0u8; Self::LEN])
    }
}

// ID equivalent to ID defined in hypersdk.
pub struct ID([u8; Self::LEN]);

impl ID {
    pub const LEN: usize = 32;
    /// Constructor function for ID
    #[must_use]
    pub fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }
    /// returns the ID as a slice of bytes
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
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

impl Default for ID {
    /// Returns a new ID with all bytes set to 0. Equivalent to Empty ID in hypersdk.
    #[must_use]
    fn default() -> Self {
        Self([0u8; Self::LEN])
    }
}
