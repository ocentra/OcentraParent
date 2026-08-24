use std::sync::Arc;

use crate::custody::CustodyAdmission;
use ocentra_protected_capability_custody_protocol::request::{
    authenticated::AuthenticatedRequest, RequestKind,
};

use super::{
    authority, error_status, finalize, platform, prepare, recover, wire, BrokerCustodyOutcome,
    BrokerCustodyRuntime, BrokerPlatformSessionState, BrokerProcessAdmission, BrokerRuntimeError,
};

impl BrokerProcessAdmission {
    pub fn for_current_process() -> Result<Self, BrokerRuntimeError> {
        let executable = super::BrokerExecutableGuard::open_current_broker()?;
        let database_path = super::storage_path::open_fixed_database()?;
        Ok(Self {
            _executable: executable,
            database_path,
        })
    }
}

impl BrokerCustodyRuntime {
    /// Opens the neutral custody runtime for the fixed database selected by
    /// the isolated broker process. This is not caller-selected authority.
    pub fn open_broker_owned(
        admission: BrokerProcessAdmission,
    ) -> Result<Self, BrokerRuntimeError> {
        let database_path = admission.database_path.as_path();
        let registry_id = platform::registry_id(database_path).map_err(error_status::platform)?;
        let authority = Arc::new(authority::BrokerCurrentBindingAuthority::new(
            registry_id.clone(),
        ));
        let platform_owner = Arc::new(platform::BrokerPlatformOwner::new());
        let admission_authority = Arc::clone(&authority);
        let custody_admission = CustodyAdmission::new(platform_owner, admission_authority);
        let store = crate::custody::CustodyStore::open(database_path, custody_admission)?;
        Ok(Self {
            store,
            authority,
            registry_id,
            _process_admission: admission,
        })
    }

    /// Executes a request only after the broker validates kernel peer
    /// identity and the authenticated, expiring, one-shot transcript.
    pub fn execute_authenticated_request(
        &self,
        request: &AuthenticatedRequest,
    ) -> Result<BrokerCustodyOutcome, BrokerRuntimeError> {
        let request = request.as_untrusted();
        let locator = wire::validated_locator(request)?;
        let current = self
            .authority
            .resolve_for_request(
                locator.clone(),
                request.kind(),
                request.expected_generations(),
            )
            .map_err(error_status::authority)?;
        let generations = wire::observed(current)?;
        match request.kind() {
            RequestKind::Prepare => prepare::prepare(self, &locator, generations),
            RequestKind::Commit => finalize::finalize(
                self,
                request,
                locator,
                generations,
                crate::custody::Decision::Commit,
            ),
            RequestKind::Abort => finalize::finalize(
                self,
                request,
                locator,
                generations,
                crate::custody::Decision::Abort,
            ),
            RequestKind::Recover => recover::recover(self, &locator, generations, false),
            RequestKind::ResolveAmbiguity => recover::recover(self, &locator, generations, true),
        }
    }

    pub fn platform_session_state(&self) -> Result<BrokerPlatformSessionState, BrokerRuntimeError> {
        let (key_epoch, writer_lease_epoch, watermark) = self.store.broker_session_epochs()?;
        Ok(BrokerPlatformSessionState {
            key_epoch,
            writer_lease_epoch,
            watermark,
        })
    }
}
