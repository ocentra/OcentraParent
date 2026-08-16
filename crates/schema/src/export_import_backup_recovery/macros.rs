macro_rules! export_import_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                parse_text_identifier(value).map(Self)
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

macro_rules! export_import_text_identifiers {
    ($($name:ident),+ $(,)?) => {
        $(export_import_text_identifier!($name);)+
    };
}

macro_rules! export_import_string_enum {
    ($name:ident, $rename_all:literal { $($(#[$variant_meta:meta])* $variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[repr(usize)]
        #[serde(rename_all = $rename_all)]
        pub enum $name {
            $($(#[$variant_meta])* $variant,)+
        }
    };
}

macro_rules! export_import_string_enums {
    ($($name:ident, $rename_all:literal { $($(#[$variant_meta:meta])* $variant:ident),+ $(,)? }),+ $(,)?) => {
        $(export_import_string_enum!($name, $rename_all { $($(#[$variant_meta])* $variant,)+ });)+
    };
}

macro_rules! export_import_string_enum_as_str_values {
    ($($name:ident { variants: [$($variant:ident),+ $(,)?], values: [$($value:expr),+ $(,)?] $(,)? }),+ $(,)?) => {
        $(
            impl $name {
                pub const VARIANTS: &'static [&'static str] = &[$($value),+];

                pub fn as_str(&self) -> &'static str {
                    let _ = stringify!($($variant),+);
                    Self::VARIANTS[*self as usize]
                }
            }
        )+
    };
}
