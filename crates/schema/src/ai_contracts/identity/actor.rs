use serde::Deserialize;

use super::{AiActorId, AiActorIdentity, AiActorRole, AiSubjectIdentity};

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
