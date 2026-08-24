use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::{validate_contract_schema_version, AI_CONTRACT_SCHEMA_VERSION};

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 256
        && !value.chars().any(char::is_control)
        && value == value.trim()
}

macro_rules! ai_identifier {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                valid_identifier(&value).then_some(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(value).ok_or_else(|| {
                    serde::de::Error::custom(concat!($label, " is empty or invalid"))
                })
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

ai_identifier!(AiSchemaVersion, "AI schema version");
ai_identifier!(AiFamilyId, "AI family id");
ai_identifier!(AiChildProfileId, "AI child profile id");
ai_identifier!(AiDeviceId, "AI device id");
ai_identifier!(AiActorId, "AI actor id");
ai_identifier!(AiSourceId, "AI source id");
ai_identifier!(AiAdapterId, "AI adapter id");
ai_identifier!(AiRequestId, "AI request id");
ai_identifier!(AiWorkItemId, "AI work item id");
ai_identifier!(AiResultId, "AI result id");
ai_identifier!(AiEvidenceReferenceId, "AI evidence reference id");
ai_identifier!(AiPolicyReferenceId, "AI policy reference id");
ai_identifier!(AiRuleId, "AI rule id");
ai_identifier!(AiMemoryReferenceId, "AI memory reference id");
ai_identifier!(AiGraphReferenceId, "AI graph reference id");
ai_identifier!(AiGraphNodeId, "AI graph node id");
ai_identifier!(AiJournalEntryId, "AI journal entry id");
ai_identifier!(AiJournalStreamId, "AI journal stream id");
ai_identifier!(AiExplanationId, "AI explanation id");
ai_identifier!(AiPromptTemplateId, "AI prompt template id");
ai_identifier!(AiPromptVersion, "AI prompt version");
ai_identifier!(AiRuntimeReferenceId, "AI runtime reference id");
ai_identifier!(AiProviderId, "AI provider id");
ai_identifier!(AiModelId, "AI model id");
ai_identifier!(AiCapabilityId, "AI capability id");
ai_identifier!(AiAuthorizationReferenceId, "AI authorization reference id");
ai_identifier!(AiRemoteAssistantRequestId, "AI remote assistant request id");
ai_identifier!(AiRemoteAssistantResultId, "AI remote assistant result id");
ai_identifier!(AiTimestamp, "AI timestamp");
ai_identifier!(AiDigest, "AI digest");

fn ascii_two_digits(value: &[u8]) -> Option<u8> {
    match value {
        [tens, ones] if tens.is_ascii_digit() && ones.is_ascii_digit() => {
            Some((tens - b'0') * 10 + (ones - b'0'))
        }
        _ => None,
    }
}

impl AiSchemaVersion {
    pub fn current() -> Self {
        Self(AI_CONTRACT_SCHEMA_VERSION.to_owned())
    }

    pub fn is_current(&self) -> bool {
        self.as_str() == AI_CONTRACT_SCHEMA_VERSION
    }

    pub fn parse_current(value: impl Into<String>) -> Option<Self> {
        let value = Self::parse(value)?;
        value.is_current().then_some(value)
    }
}

impl AiTimestamp {
    pub(crate) fn is_well_formed(&self) -> bool {
        let value = self.as_str().as_bytes();
        if value.len() < 20
            || value.get(4) != Some(&b'-')
            || value.get(7) != Some(&b'-')
            || value.get(10) != Some(&b'T')
            || value.get(13) != Some(&b':')
            || value.get(16) != Some(&b':')
        {
            return false;
        }
        let numeric = [
            &value[0..4],
            &value[5..7],
            &value[8..10],
            &value[11..13],
            &value[14..16],
            &value[17..19],
        ];
        if numeric
            .iter()
            .any(|part| !part.iter().all(u8::is_ascii_digit))
        {
            return false;
        }
        let hour = ascii_two_digits(&value[11..13]).unwrap_or(u8::MAX);
        let minute = ascii_two_digits(&value[14..16]).unwrap_or(u8::MAX);
        let second = ascii_two_digits(&value[17..19]).unwrap_or(u8::MAX);
        let month = ascii_two_digits(&value[5..7]).unwrap_or(0);
        let day = ascii_two_digits(&value[8..10]).unwrap_or(0);
        if !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || hour >= 24
            || minute >= 60
            || second >= 60
        {
            return false;
        }
        let mut offset_start = 19;
        if value.get(19) == Some(&b'.') {
            offset_start += 1;
            let fraction_end = value[offset_start..]
                .iter()
                .position(|byte| *byte == b'Z' || *byte == b'+' || *byte == b'-')
                .map(|index| offset_start + index);
            let Some(fraction_end) = fraction_end else {
                return false;
            };
            if fraction_end == offset_start
                || !value[offset_start..fraction_end]
                    .iter()
                    .all(u8::is_ascii_digit)
            {
                return false;
            }
            offset_start = fraction_end;
        }
        match value.get(offset_start..) {
            Some([b'Z']) => true,
            Some([b'+' | b'-', hour_tens, hour_ones, b':', minute_tens, minute_ones]) => {
                ascii_two_digits(&[*hour_tens, *hour_ones]).is_some_and(|value| value <= 23)
                    && ascii_two_digits(&[*minute_tens, *minute_ones])
                        .is_some_and(|value| value <= 59)
            }
            _ => false,
        }
    }

    pub(crate) fn precedes(&self, other: &Self) -> bool {
        self.is_well_formed() && other.is_well_formed() && self.as_str() < other.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiActorRole {
    Parent,
    ChildAgent,
    LocalRuntime,
    ParentAssistant,
    RemoteAssistant,
    System,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSubjectIdentity {
    family_id: AiFamilyId,
    child_profile_id: Option<AiChildProfileId>,
    device_id: Option<AiDeviceId>,
}

impl AiSubjectIdentity {
    pub fn new(
        family_id: AiFamilyId,
        child_profile_id: Option<AiChildProfileId>,
        device_id: Option<AiDeviceId>,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            family_id,
            child_profile_id,
            device_id,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn child_profile_id(&self) -> Option<&AiChildProfileId> {
        self.child_profile_id.as_ref()
    }

    pub fn device_id(&self) -> Option<&AiDeviceId> {
        self.device_id.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiSubjectIdentityFields {
    family_id: AiFamilyId,
    child_profile_id: Option<AiChildProfileId>,
    device_id: Option<AiDeviceId>,
}

impl<'de> Deserialize<'de> for AiSubjectIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiSubjectIdentityFields::deserialize(deserializer)?;
        Self::new(fields.family_id, fields.child_profile_id, fields.device_id)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiActorIdentity {
    actor_id: AiActorId,
    role: AiActorRole,
    subject: Option<AiSubjectIdentity>,
}

impl AiActorIdentity {
    pub fn new(
        actor_id: AiActorId,
        role: AiActorRole,
        subject: Option<AiSubjectIdentity>,
    ) -> Result<Self, &'static str> {
        if matches!(role, AiActorRole::Parent) {
            return Err("parent authority identity requires a trusted issuer");
        }
        let subject_required = !matches!(role, AiActorRole::System);
        if subject_required != subject.is_some() {
            return Err("AI actor subject does not match the actor role");
        }
        Ok(Self {
            actor_id,
            role,
            subject,
        })
    }

    pub(crate) fn trusted_parent(actor_id: AiActorId, subject: AiSubjectIdentity) -> Self {
        Self {
            actor_id,
            role: AiActorRole::Parent,
            subject: Some(subject),
        }
    }

    pub fn actor_id(&self) -> &AiActorId {
        &self.actor_id
    }

    pub fn role(&self) -> AiActorRole {
        self.role
    }

    pub fn subject(&self) -> Option<&AiSubjectIdentity> {
        self.subject.as_ref()
    }

    pub(crate) fn is_parent_authority(&self) -> bool {
        matches!(self.role, AiActorRole::Parent)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiActorIdentityFields {
    actor_id: AiActorId,
    role: AiActorRole,
    subject: Option<AiSubjectIdentity>,
}

impl<'de> Deserialize<'de> for AiActorIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiActorIdentityFields::deserialize(deserializer)?;
        Self::new(fields.actor_id, fields.role, fields.subject).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSchemaIdentity {
    schema_version: AiSchemaVersion,
    family: AiFamilyId,
    request_id: AiRequestId,
    subject: AiSubjectIdentity,
}

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
