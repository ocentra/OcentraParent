use ocentra_eventing::error::EventingError;

pub(super) fn parse_nonempty_text_id(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}

impl super::StorageCustodyAggregateId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        Ok(Self(parse_nonempty_text_id(
            "storage_custody.aggregate_id",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for super::StorageCustodyAggregateId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<super::StorageCustodyAggregateId> for String {
    fn from(value: super::StorageCustodyAggregateId) -> Self {
        value.0
    }
}

impl std::fmt::Display for super::StorageCustodyAggregateId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
