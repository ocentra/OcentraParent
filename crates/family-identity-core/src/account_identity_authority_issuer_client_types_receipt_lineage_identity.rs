use super::account_identity_authority_issuer_client_types::AccountIdentityIssuerReceiptLineage;

impl AccountIdentityIssuerReceiptLineage {
    pub fn account_id(&self) -> &str {
        self.account_id.as_str()
    }

    pub fn household_id(&self) -> &str {
        self.household_id.as_str()
    }

    pub fn provider(&self) -> &ocentra_schema::account_identity_authority::AccountIdentityProvider {
        &self.provider
    }

    pub fn provider_subject(
        &self,
    ) -> &ocentra_schema::account_identity_authority::AccountIdentityProviderSubject {
        &self.provider_subject
    }

    pub fn member_id(&self) -> &str {
        self.member_id.as_str()
    }

    pub fn device_id(&self) -> &str {
        self.device_id.as_str()
    }

    pub fn session_id(&self) -> &str {
        self.session_id.as_str()
    }

    pub fn service(&self) -> &str {
        self.service.as_str()
    }

    pub fn service_binding_id(&self) -> &str {
        self.service_binding_id.as_str()
    }

    pub fn key_id(&self) -> &str {
        self.key_id.as_str()
    }
}
