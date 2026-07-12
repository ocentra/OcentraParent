use crate::BrowserInterventionTargetType;

mod protocol_pairs;

impl BrowserInterventionTargetType {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_pairs::find(value)
    }
}
