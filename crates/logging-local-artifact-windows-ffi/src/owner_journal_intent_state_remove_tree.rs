use super::*;

impl IntentRecord {
    pub(crate) fn remove_tree_with_phase(
        &self,
        phase: RemoveTreePhase,
    ) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        match &mut next {
            Self::RemoveTree {
                phase: current_phase,
                ..
            } => *current_phase = phase,
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }
}
