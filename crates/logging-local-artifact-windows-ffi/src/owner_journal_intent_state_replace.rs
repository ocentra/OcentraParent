use super::*;

impl IntentRecord {
    pub(crate) fn replace_with_state(
        &self,
        staged_identity: Option<IdentityRecord>,
        phase: ReplacePhase,
    ) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        match &mut next {
            Self::Replace {
                staged_identity: current_identity,
                phase: current_phase,
                ..
            } => {
                *current_identity = staged_identity;
                *current_phase = phase;
            }
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }

    pub(crate) fn replace_with_phase(&self, phase: ReplacePhase) -> Result<Self, ArtifactError> {
        self.replace_with_state(self.replace_staged_identity()?, phase)
    }

    fn replace_staged_identity(&self) -> Result<Option<IdentityRecord>, ArtifactError> {
        match self {
            Self::Replace {
                staged_identity, ..
            } => Ok(*staged_identity),
            _ => Err(ArtifactError::RecoveryRequired),
        }
    }
}
