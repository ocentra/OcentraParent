use std::fmt::{Display, Formatter};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
                Self::VARIANTS
                    .iter()
                    .position(|candidate| *candidate == value)
                    .map(|index| Self::VALUES[index])
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
                $name::parse(value.as_str())
                    .ok_or_else(|| de::Error::unknown_variant(value.as_str(), $name::VARIANTS))
            }
        }
    };
}

macro_rules! ios_string_enums {
    ($($name:ident { $($variant:ident => $const_ident:ident),+ $(,)? }),+ $(,)?) => {
        $(
            ios_string_enum!($name { $($variant => $const_ident,)+ });
        )+
    };
}
