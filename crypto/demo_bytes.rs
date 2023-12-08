



struct Bytes<const T: usize>([u8; T]);

impl<const T: usize> Bytes<T> {
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    fn fmt_as_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;

        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<const T: usize> Deref for Bytes<T> {
    type Target = [u8; T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const T: usize> Default for Bytes<T> {
    fn default() -> Self {
        Self([0u8; T])
    }
}

impl<const T: usize> fmt::Debug for Bytes<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_as_hex(f)
    }
}

impl<const T: usize> fmt::Display for Bytes<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_as_hex(f)
    }
}

