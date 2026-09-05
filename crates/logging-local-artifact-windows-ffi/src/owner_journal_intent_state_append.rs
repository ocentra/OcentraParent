use super::*;

impl IntentRecord {
    pub(crate) fn append_with_state(
        &self,
        target_identity: Option<IdentityRecord>,
        temp_name: Option<impl std::fmt::Display>,
        phase: AppendPhase,
    ) -> Result<Self, ArtifactError> {
        let mut next = self.clone();
        let temp_name = temp_name.map(|value| value.to_string());
        match &mut next {
            Self::Append {
                target_identity: current_identity,
                temp_name: current_temp,
                phase: current_phase,
                ..
            } => {
                *current_identity = target_identity;
                *current_temp = temp_name;
                *current_phase = phase;
            }
            _ => return Err(ArtifactError::RecoveryRequired),
        }
        Ok(next)
    }
}
