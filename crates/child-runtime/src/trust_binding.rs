use ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildAgentTrustBindingError {
    Unavailable,
    Invalid,
}

/// Production callers provide a family-owned current-state query, not an
/// identity DTO.  The returned binding is opaque and cannot be forged or
/// replayed by copying caller-supplied strings.
pub trait ChildAgentTrustBindingSource: Send + Sync {
    fn current_trust_binding(
        &self,
    ) -> Result<CurrentChildDeviceTrustBinding, ChildAgentTrustBindingError>;
}
