use super::AiParentAuthorization;
use crate::ai_contracts::identity::{
    AiActorIdentity, AiAuthorizationReferenceId, AiFamilyId, AiTimestamp,
};

impl AiParentAuthorization {
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
