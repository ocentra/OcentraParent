use super::AiParentAuthorization;
use crate::ai_contracts::identity::{
    AiActorIdentity, AiAuthorizationReferenceId, AiFamilyId, AiTimestamp,
};

impl AiParentAuthorization {
    pub(crate) fn issue(
        authorization_reference_id: AiAuthorizationReferenceId,
        actor: AiActorIdentity,
        authorized_at: AiTimestamp,
        expires_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        if !actor.is_parent_authority()
            || actor.subject().is_none()
            || !authorized_at.precedes(&expires_at)
        {
            return Err("AI remote authorization is not parent-issued or has an invalid lifetime");
        }
        Ok(Self {
            authorization_reference_id,
            actor,
            authorized_at,
            expires_at,
        })
    }

    pub fn authorization_reference_id(&self) -> &AiAuthorizationReferenceId {
        &self.authorization_reference_id
    }

    pub fn actor(&self) -> &AiActorIdentity {
        &self.actor
    }

    pub fn family_id(&self) -> Option<&AiFamilyId> {
        self.actor.subject().map(|subject| subject.family_id())
    }

    pub fn authorized_at(&self) -> &AiTimestamp {
        &self.authorized_at
    }

    pub fn expires_at(&self) -> &AiTimestamp {
        &self.expires_at
    }
}
