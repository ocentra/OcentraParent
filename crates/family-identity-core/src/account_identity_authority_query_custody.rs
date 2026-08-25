use ocentra_schema::account_identity_authority::{
    AccountIdentityAccountState, AccountIdentityBindingLifecycleState,
    AccountIdentityBindingRevocationState, AccountIdentityDeviceTrustState,
    AccountIdentityInstallState, AccountIdentityMembershipState, AccountIdentityPairingState,
    AccountIdentitySessionFreshnessState,
};

impl super::VerifiedAccountIdentityAuthority {
    pub fn session_expires_at(&self) -> &str {
        &self.provenance.session_expires_at
    }

    pub fn report_query_custody_states(
        &self,
    ) -> (
        AccountIdentityAccountState,
        AccountIdentityMembershipState,
        AccountIdentityDeviceTrustState,
        AccountIdentitySessionFreshnessState,
        AccountIdentityPairingState,
        AccountIdentityInstallState,
        AccountIdentityBindingLifecycleState,
        AccountIdentityBindingRevocationState,
    ) {
        (
            self.handoff.member.account_state,
            self.handoff.member.membership_state,
            self.handoff.member.device_trust_state,
            self.handoff.member.session_freshness_state,
            self.handoff.binding.pairing_state,
            self.handoff.binding.install_state,
            self.handoff.binding.lifecycle_state,
            self.handoff.binding.revocation_state,
        )
    }
}
