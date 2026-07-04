use std::fmt::{Display, Formatter};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

macro_rules! ios_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! ios_string_enum {
    ($name:ident { $($variant:ident => $const_ident:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(usize)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl $name {
            pub const TYPE_NAME: &'static str = stringify!($name);
            pub const VARIANTS: &'static [&'static str] = &[
                $(
                    $const_ident,
                )+
            ];
            pub const VALUES: &'static [Self] = &[
                $(
                    Self::$variant,
                )+
            ];

            pub fn as_str(&self) -> &'static str {
                Self::VARIANTS[*self as usize]
            }

            pub fn parse(value: &str) -> Option<Self> {
                for (index, candidate) in Self::VARIANTS.iter().enumerate() {
                    if *candidate == value {
                        return Some(Self::VALUES[index]);
                    }
                }
                None
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                match $name::parse(value.as_str()) {
                    Some(parsed) => Ok(parsed),
                    None => Err(de::Error::unknown_variant(value.as_str(), $name::VARIANTS)),
                }
            }
        }
    };
}
