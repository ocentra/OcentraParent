use std::io;

use serde::{ser::SerializeMap, Serialize, Serializer};

pub(crate) fn serialize<T: Serialize>(event: &T) -> io::Result<Vec<u8>> {
    let value = serde_json::to_value(event)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    serde_json::to_vec(&CanonicalJsonValue(&value))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

struct CanonicalJsonValue<'a>(&'a serde_json::Value);

impl Serialize for CanonicalJsonValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            serde_json::Value::Null => serializer.serialize_unit(),
            serde_json::Value::Bool(value) => serializer.serialize_bool(*value),
            serde_json::Value::Number(value) => value.serialize(serializer),
            serde_json::Value::String(value) => serializer.serialize_str(value),
            serde_json::Value::Array(values) => serialize_array(values, serializer),
            serde_json::Value::Object(values) => serialize_object(values, serializer),
        }
    }
}

fn serialize_array<S>(values: &[serde_json::Value], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    values
        .iter()
        .map(CanonicalJsonValue)
        .collect::<Vec<_>>()
        .serialize(serializer)
}

fn serialize_object<S>(
    values: &serde_json::Map<String, serde_json::Value>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut entries = values.iter().collect::<Vec<_>>();
    entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    let mut map = serializer.serialize_map(Some(entries.len()))?;
    for (key, value) in entries {
        map.serialize_entry(key, &CanonicalJsonValue(value))?;
    }
    map.end()
}
