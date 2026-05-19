use ocentra_parent_agent_protocol::{LogFieldValue, LogFields};

pub fn fields_from_pairs(pairs: Vec<(&str, LogFieldValue)>) -> LogFields {
    let mut fields = LogFields::new();
    for (key, value) in pairs {
        fields.insert(key.to_string(), value);
    }
    fields
}
