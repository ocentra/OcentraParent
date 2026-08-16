use super::super::GrantWireField;
use super::GrantWireValue;

pub(in crate::authenticated_delivery_grant::wire) struct GrantWireFields {
    values: [Option<GrantWireValue>; 20],
}

impl GrantWireFields {
    pub(in crate::authenticated_delivery_grant::wire) fn new() -> Self {
        Self {
            values: std::array::from_fn(|_| None),
        }
    }

    pub(in crate::authenticated_delivery_grant::wire) fn insert(
        &mut self,
        field: GrantWireField,
        value: GrantWireValue,
        names: &[&'static str],
    ) -> Result<(), String> {
        if self.values[field.0].replace(value).is_some() {
            return Err(format!("duplicate field `{}`", names[field.0]));
        }
        Ok(())
    }

    pub(super) fn required(
        &mut self,
        index: usize,
        names: &[&'static str],
    ) -> Result<GrantWireValue, String> {
        self.values[index]
            .take()
            .ok_or_else(|| format!("missing field `{}`", names[index]))
    }
}
