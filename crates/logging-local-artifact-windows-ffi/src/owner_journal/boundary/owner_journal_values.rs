//! Boundary conversions for journal-owned wire values.

use std::path::Path;

use super::{
    DescriptorDigest, DescriptorOutput, HexOutput, HexText, JournalPath, JournalText, NameOutput,
    NameText, HEX_DIGITS,
};

impl JournalText<'_> {
    pub(in crate::owner_journal) fn as_str(&self) -> &str {
        self.0
    }
}

impl JournalPath {
    pub(in crate::owner_journal) fn as_path(&self) -> &Path {
        &self.0
    }
}

impl DescriptorOutput for String {
    fn from_descriptor(value: DescriptorDigest) -> Self {
        let mut output = String::with_capacity(value.0.len() * 2);
        for byte in value.0 {
            output.push(char::from(HEX_DIGITS.as_bytes()[(byte >> 4) as usize]));
            output.push(char::from(HEX_DIGITS.as_bytes()[(byte & 0x0f) as usize]));
        }
        output
    }
}

impl HexOutput for String {
    fn from_hex(value: HexText) -> Self {
        value.0
    }
}

impl NameOutput for String {
    fn from_name(value: NameText) -> Self {
        value.0
    }
}

impl HexText {
    pub(in crate::owner_journal) fn new() -> Self {
        Self(String::with_capacity(64))
    }

    pub(in crate::owner_journal) fn push(&mut self, value: char) {
        self.0.push(value);
    }
}

impl NameText {
    pub(in crate::owner_journal) fn new() -> Self {
        Self(String::new())
    }

    pub(in crate::owner_journal) fn push(&mut self, value: char) {
        self.0.push(value);
    }

    pub(in crate::owner_journal) fn push_text(&mut self, value: &str) {
        self.0.push_str(value);
    }
}
