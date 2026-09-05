use super::*;

impl IntentRecord {
    pub(crate) fn transaction_with_staged(
        &self,
        staged: Vec<StagedMutation>,
    ) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        match &mut next {
            Self::Transaction {
                staged: current_staged,
                ..
            } => *current_staged = staged,
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }

    pub(crate) fn transaction_with_phase(
        &self,
        phase: TransactionPhase,
    ) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        match &mut next {
            Self::Transaction {
                phase: current_phase,
                ..
            } => *current_phase = phase,
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }
}
