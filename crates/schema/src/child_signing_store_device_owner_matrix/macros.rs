macro_rules! matrix_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                (!value.trim().is_empty()).then_some(Self(value))
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

macro_rules! matrix_string_enums {
    ($($name:ident { variants: [$($variant:ident),+ $(,)?], values: [$($value:ident),+ $(,)?] $(,)? }),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
            #[serde(rename_all = "kebab-case")]
            pub enum $name {
                $( $variant, )+
            }

            impl $name {
                pub const fn as_str(&self) -> &'static str {
                    const VALUES: &[&str] = &[$($value),+];
                    VALUES[*self as usize]
                }
            }
        )+
    };
}
