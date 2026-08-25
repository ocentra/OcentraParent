//! Bounded construction and inspection for text crossing the Windows ABI.

use crate::{Error, InputFault, Result, MAX_WIDE_CHARS};

/// UTF-8 text copied from or prepared for a Windows UTF-16 API.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowsText(String);

/// A validated HKLM-relative registry path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryPath(WindowsText);

/// A validated registry value name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryValueName(WindowsText);

/// A validated SCM service name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceName(WindowsText);

impl WindowsText {
    pub fn try_from_str(value: &str) -> Result<Self> {
        validate_text(value)?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_utf16(value: &[u16], fault: InputFault) -> Result<Self> {
        let value = String::from_utf16(value).map_err(|_utf16_error| Error::InvalidInput(fault))?;
        validate_text(&value)?;
        Ok(Self(value))
    }

    pub(crate) fn wide_nul(&self) -> Result<Vec<u16>> {
        validate_text(&self.0)?;
        Ok(self.0.encode_utf16().chain(core::iter::once(0)).collect())
    }

    pub(crate) fn join(components: &[Self]) -> Result<Self> {
        let mut joined = String::new();
        for component in components {
            if !joined.is_empty() {
                joined.push('\\');
            }
            joined.push_str(component.as_str());
        }
        Self::try_from_str(&joined)
    }
}

impl RegistryPath {
    pub fn try_from_str(value: &str) -> Result<Self> {
        let text = WindowsText::try_from_str(value)?;
        let components = split_components(&text)?;
        if components.is_empty() || components.len() > 64 {
            return Err(Error::InvalidInput(InputFault::RegistryPathDepthInvalid));
        }
        Ok(Self(text))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn components(&self) -> Result<Vec<WindowsText>> {
        split_components(&self.0)
    }
}

impl RegistryValueName {
    pub fn try_from_str(value: &str) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::InvalidInput(InputFault::WindowsTextInvalid));
        }
        Ok(Self(WindowsText::try_from_str(value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn wide_nul(&self) -> Result<Vec<u16>> {
        self.0.wide_nul()
    }
}

impl ServiceName {
    pub fn try_from_str(value: &str) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::InvalidInput(InputFault::WindowsTextInvalid));
        }
        Ok(Self(WindowsText::try_from_str(value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn wide_nul(&self) -> Result<Vec<u16>> {
        self.0.wide_nul()
    }

    pub(crate) fn text(&self) -> WindowsText {
        self.0.clone()
    }
}

fn validate_text(value: &str) -> Result<()> {
    if value.contains('\0') {
        return Err(Error::InvalidInput(InputFault::WindowsTextInvalid));
    }
    if value.encode_utf16().count() >= MAX_WIDE_CHARS {
        return Err(Error::BufferTooLarge);
    }
    Ok(())
}

fn split_components(path: &WindowsText) -> Result<Vec<WindowsText>> {
    let mut components = Vec::new();
    for component in path.as_str().split('\\').filter(|value| !value.is_empty()) {
        if component.chars().all(|value| value == '.') && component.len() <= 2 {
            return Err(Error::InvalidInput(InputFault::RegistryPathTraversal));
        }
        components.push(WindowsText::try_from_str(component)?);
    }
    Ok(components)
}
