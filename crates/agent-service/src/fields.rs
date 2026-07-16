use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub struct LogFieldPair(pub &'static str, pub LogFieldValue);

pub trait IntoLogFieldPair {
    fn into_log_field_pair(self) -> LogFieldPair;
}

impl IntoLogFieldPair for LogFieldPair {
    fn into_log_field_pair(self) -> LogFieldPair {
        self
    }
}

impl IntoLogFieldPair for (&'static str, LogFieldValue) {
    fn into_log_field_pair(self) -> LogFieldPair {
        let (key, value) = self;
        LogFieldPair(key, value)
    }
}

pub fn fields_from_pairs<P>(pairs: Vec<P>) -> LogFields
where
    P: IntoLogFieldPair,
{
    let mut fields = LogFields::new();
    for pair in pairs {
        let LogFieldPair(key, value) = pair.into_log_field_pair();
        fields.insert(key.to_string(), value);
    }
    fields
}
