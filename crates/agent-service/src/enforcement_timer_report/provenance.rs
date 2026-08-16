use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalProvenance;

pub(super) fn for_phase(phase: JournalDispatchPhase) -> EnforcementAuditJournalProvenance {
    match phase {
        JournalDispatchPhase::BeforeDispatch => EnforcementAuditJournalProvenance::AcceptedIntent,
        JournalDispatchPhase::AfterDispatch => EnforcementAuditJournalProvenance::AdapterResult,
    }
}
