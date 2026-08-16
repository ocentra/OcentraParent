use ocentra_eventing::error::EventingError;

pub(crate) fn validate_entitlement_snapshot_text_id(
    value: String,
    field: &'static str,
) -> Result<String, EventingError> {
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }

    Ok(value)
}

#[macro_export]
macro_rules! entitlement_snapshot_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                Ok(Self(
                    $crate::entitlement_snapshot_text_id::validate_entitlement_snapshot_text_id(
                        value.into(),
                        $field,
                    )?,
                ))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}
