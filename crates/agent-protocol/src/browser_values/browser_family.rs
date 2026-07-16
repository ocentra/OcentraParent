use super::protocol_lookup;
use crate::{constants, BrowserFamily};

impl BrowserFamily {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (constants::browser::FAMILY_EDGE, Self::Edge),
                (constants::browser::FAMILY_CHROME, Self::Chrome),
                (constants::browser::FAMILY_BRAVE, Self::Brave),
                (constants::browser::FAMILY_FIREFOX, Self::Firefox),
                (constants::browser::FAMILY_OPERA, Self::Opera),
                (
                    constants::browser::FAMILY_UNKNOWN_CHROMIUM,
                    Self::UnknownChromium,
                ),
                (constants::browser::FAMILY_UNKNOWN, Self::Unknown),
            ],
        )
    }
}
