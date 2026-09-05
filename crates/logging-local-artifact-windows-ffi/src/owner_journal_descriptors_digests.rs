use std::fmt::Display;

use sha2::{Digest, Sha256};

use super::super::{JournalText, REQUEST_DESCRIPTOR_DOMAIN};

pub(crate) trait DescriptorText: Display {
    fn update_hasher(&self, hasher: &mut Sha256);
}

pub(crate) trait DescriptorOutputInput<D> {}

impl DescriptorText for str {
    fn update_hasher(&self, hasher: &mut Sha256) {
        hasher.update(self.as_bytes());
    }
}

impl DescriptorOutputInput<String> for str {}

impl DescriptorText for JournalText<'_> {
    fn update_hasher(&self, hasher: &mut Sha256) {
        hasher.update(self.as_str().as_bytes());
    }
}

impl DescriptorOutputInput<String> for JournalText<'_> {}

impl DescriptorText for String {
    fn update_hasher(&self, hasher: &mut Sha256) {
        hasher.update(self.as_bytes());
    }
}

impl DescriptorOutputInput<String> for String {}

pub(in crate::owner_journal) fn request_descriptor<O, P>(
    operation: &O,
    relative_path: &P,
    payload: Option<&[u8]>,
) -> super::super::DescriptorDigest
where
    O: DescriptorText + ?Sized,
    P: Display + ?Sized,
{
    let mut hasher = Sha256::new();
    hasher.update(REQUEST_DESCRIPTOR_DOMAIN.as_bytes());
    operation.update_hasher(&mut hasher);
    hasher.update([0]);
    hasher.update(relative_path.to_string().as_bytes());
    hasher.update([0]);
    if let Some(payload) = payload {
        hasher.update(payload);
    }
    let digest: [u8; 32] = hasher.finalize().into();
    super::super::DescriptorDigest(digest)
}

pub(crate) trait HexInput {
    fn update_hasher(&self, hasher: &mut Sha256);
}

pub(crate) trait HexOutputInput<D> {}

impl HexInput for [u8] {
    fn update_hasher(&self, hasher: &mut Sha256) {
        hasher.update(self);
    }
}

impl HexOutputInput<String> for [u8] {}

pub(in crate::owner_journal) fn payload_digest<P>(payload: &P) -> super::super::HexText
where
    P: HexInput + ?Sized,
{
    let mut hasher = Sha256::new();
    payload.update_hasher(&mut hasher);
    let digest: [u8; 32] = hasher.finalize().into();
    let mut output = super::super::HexText::new();
    for byte in digest {
        output.push(char::from(
            super::super::HEX_DIGITS.as_bytes()[(byte >> 4) as usize],
        ));
        output.push(char::from(
            super::super::HEX_DIGITS.as_bytes()[(byte & 0x0f) as usize],
        ));
    }
    output
}
