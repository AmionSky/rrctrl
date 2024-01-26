/// Null-terminated UTF-16 string for FFI with a serde type of string.
pub struct WString(Vec<u16>);

impl WString {
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self(
            value
                .as_ref()
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect(),
        )
    }

    pub fn from_uft16(value: &[u16]) -> Self {
        let mut utf16 = Vec::with_capacity(value.len() + 1);
        utf16.extend(value);
        utf16.push(0);
        Self(utf16)
    }

    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    pub fn value(&self) -> &[u16] {
        &self.0[..self.len()]
    }
}

impl PartialEq for WString {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
