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

pub(super) fn audit_provenance_matches(
    fields: &LogFields,
    provenance: Option<EnforcementAuditProvenance>,
) -> bool {
    match provenance {
        Some(EnforcementAuditProvenance::AppGameAdapterDispatch) => {
            fields.get(constants::field::SOURCE_READ_MODEL_ID)
                == Some(&LogFieldValue::String(
                    APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string(),
                ))
                && fields.get(constants::field::EXECUTION_COMMAND_NAME)
                    == Some(&LogFieldValue::String(
                        APP_GAME_ADAPTER_DISPATCH_EXECUTE_COMMAND.to_string(),
                    ))
        }
        None => {
            fields.get(constants::field::SOURCE_READ_MODEL_ID).is_none()
                && fields
                    .get(constants::field::EXECUTION_COMMAND_NAME)
                    .is_none()
        }
    }
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
