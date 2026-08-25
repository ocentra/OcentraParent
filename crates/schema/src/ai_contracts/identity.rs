use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::AI_CONTRACT_SCHEMA_VERSION;

mod actor;
mod schema;
mod subject;
mod timestamp;

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 256
        && !value.chars().any(char::is_control)
        && value == value.trim()
}

fn invalid_identifier<E>(label: &'static str) -> E
where
    E: serde::de::Error,
{
    E::custom(label)
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
                let error = invalid_identifier::<D::Error>(concat!($label, " is empty or invalid"));
                Self::parse(value).ok_or(error)
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiActorIdentity {
    actor_id: AiActorId,
    role: AiActorRole,
    subject: Option<AiSubjectIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSchemaIdentity {
    schema_version: AiSchemaVersion,
    family: AiFamilyId,
    request_id: AiRequestId,
    subject: AiSubjectIdentity,
}
