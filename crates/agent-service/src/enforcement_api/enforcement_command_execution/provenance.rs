use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::{
    APP_GAME_ADAPTER_DISPATCH_EXECUTE_COMMAND, APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentCommandName;

#[derive(Clone, Copy, Debug)]
pub(super) enum EnforcementAuditProvenance {
    AppGameAdapterDispatch,
}

pub(super) fn enforcement_audit_provenance(
    command: &AgentCommandName,
) -> Option<EnforcementAuditProvenance> {
    match command {
        AgentCommandName::AgentActivityAppGameAdapterDispatchExecute => {
            Some(EnforcementAuditProvenance::AppGameAdapterDispatch)
        }
        _ => None,
    }
}

pub(super) fn record_audit_provenance(
    fields: &mut LogFields,
    provenance: Option<EnforcementAuditProvenance>,
) {
    provenance
        .iter()
        .for_each(|provenance| provenance.record(fields));
}

impl EnforcementAuditProvenance {
    fn record(self, fields: &mut LogFields) {
        let source_read_model_id = match self {
            Self::AppGameAdapterDispatch => APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID,
        };
        fields.insert(
            constants::field::SOURCE_READ_MODEL_ID.to_string(),
            LogFieldValue::String(source_read_model_id.to_string()),
        );
        fields.insert(
            constants::field::EXECUTION_COMMAND_NAME.to_string(),
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_EXECUTE_COMMAND.to_string()),
        );
    }
}
