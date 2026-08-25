use serde::Deserialize;

use super::{AiFamilyId, AiRequestId, AiSchemaIdentity, AiSchemaVersion, AiSubjectIdentity};
use crate::ai_contracts::validate_contract_schema_version;

impl AiSchemaIdentity {
    pub fn new(
        schema_version: AiSchemaVersion,
        family: AiFamilyId,
        request_id: AiRequestId,
        subject: AiSubjectIdentity,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if family != *subject.family_id() {
            return Err("AI schema identity family does not match its subject");
        }
        Ok(Self {
            schema_version,
            family,
            request_id,
            subject,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family(&self) -> &AiFamilyId {
        &self.family
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn subject(&self) -> &AiSubjectIdentity {
        &self.subject
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiSchemaIdentityFields {
    schema_version: AiSchemaVersion,
    family: AiFamilyId,
    request_id: AiRequestId,
    subject: AiSubjectIdentity,
}

impl<'de> Deserialize<'de> for AiSchemaIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiSchemaIdentityFields::deserialize(deserializer)?;
        Self::new(
            fields.schema_version,
            fields.family,
            fields.request_id,
            fields.subject,
        )
        .map_err(serde::de::Error::custom)
    }
}
