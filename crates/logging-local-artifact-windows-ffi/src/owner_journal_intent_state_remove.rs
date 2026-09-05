use super::*;

impl IntentRecord {
    pub(crate) fn remove_with_phase(&self, phase: RemovePhase) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        match &mut next {
            Self::Remove {
                phase: current_phase,
                ..
            } => *current_phase = phase,
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }
}
