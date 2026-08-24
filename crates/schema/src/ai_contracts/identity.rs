use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

macro_rules! ai_identifier {
    ($name:ident, $label:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                (!value.trim().is_empty()).then_some(Self(value))
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
                Self::parse(value)
                    .ok_or_else(|| serde::de::Error::custom(concat!($label, " must not be empty")))
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSubjectIdentity {
    pub family_id: AiFamilyId,
    pub child_profile_id: Option<AiChildProfileId>,
    pub device_id: Option<AiDeviceId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiActorIdentity {
    pub actor_id: AiActorId,
    pub role: AiActorRole,
    pub subject: Option<AiSubjectIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSchemaIdentity {
    pub schema_version: AiSchemaVersion,
    pub family: AiFamilyId,
    pub request_id: AiRequestId,
    pub subject: AiSubjectIdentity,
}
