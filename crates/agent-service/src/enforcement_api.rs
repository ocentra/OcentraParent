use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    enforcement_os_adapter_product_proof_read_model::product_control_spine::v08_enforcement_product_control_spine_read_model,
    enforcement_os_adapter_product_proof_read_model::product_control_spine::GeneratedAtText,
    enforcement_policy_dispatch_read_model::{
        v08_enforcement_policy_dispatch_read_model, DispatchText,
    },
    enforcement_timer_state_path::{enforcement_timer_state_path, EnforcementTimerStatePath},
    event_builder::build_event,
    time::timestamp_now,
};

#[path = "enforcement_api/enforcement_broad_adapter_proof_payload.rs"]
mod enforcement_broad_adapter_proof_payload;
#[path = "enforcement_api/enforcement_broad_adapter_proof_read_model.rs"]
pub(crate) mod enforcement_broad_adapter_proof_read_model;
#[path = "enforcement_api/enforcement_broad_adapter_proof_report.rs"]
pub(crate) mod enforcement_broad_adapter_proof_report;
#[path = "enforcement_api/enforcement_command_execution.rs"]
mod enforcement_command_execution;
#[path = "enforcement_api/enforcement_integrity_runtime_audit_read_model.rs"]
pub(crate) mod enforcement_integrity_runtime_audit_read_model;
#[path = "enforcement_api/enforcement_pre_action_journal.rs"]
pub(crate) mod enforcement_pre_action_journal;
#[path = "enforcement_api/enforcement_product_control_payload.rs"]
mod enforcement_product_control_payload;
#[path = "enforcement_api/enforcement_report_payload.rs"]
mod enforcement_report_payload;
#[path = "enforcement_api/enforcement_supported_adapter_runtime_proof_read_model.rs"]
pub(crate) mod enforcement_supported_adapter_runtime_proof_read_model;
#[path = "enforcement_api/enforcement_supported_adapter_runtime_proof_report.rs"]
pub(crate) mod enforcement_supported_adapter_runtime_proof_report;
#[path = "enforcement_api/integrity_alert_status_bridge_read_model.rs"]
pub(crate) mod integrity_alert_status_bridge_read_model;
#[path = "enforcement_api/notification_provider_status_boundary_read_model.rs"]
pub(crate) mod notification_provider_status_boundary_read_model;

use self::enforcement_product_control_payload::{
    enforcement_policy_dispatch_payload, enforcement_product_control_spine_payload,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementJournalPaths {
    pub journal_path: PathBuf,
    pub key_path: PathBuf,
    pub store_path: PathBuf,
    pub timer_state_path: EnforcementTimerStatePath,
}

impl EnforcementJournalPaths {
    pub(crate) fn from_environment() -> Self {
        Self {
            journal_path: activity_journal_path().into(),
            key_path: activity_journal_key_path().into(),
            store_path: activity_db_path().into(),
            timer_state_path: enforcement_timer_state_path(),
        }
    }
}

pub async fn build_enforcement_audit_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_enforcement_audit_report_with_paths(command, EnforcementJournalPaths::from_environment())
        .await
}

pub async fn build_enforcement_product_control_spine_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at: String = timestamp_now();
    let read_model =
        v08_enforcement_product_control_spine_read_model(GeneratedAtText(generated_at));
    build_event(
        constants::event_id::ENFORCEMENT_PRODUCT_CONTROL_SPINE_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementProductControlSpineReported,
        LogLevel::Info,
        enforcement_product_control_spine_payload(&read_model),
        None,
    )
}

pub async fn build_enforcement_policy_dispatch_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at: String = timestamp_now();
    let read_model = v08_enforcement_policy_dispatch_read_model(DispatchText(generated_at));
    build_event(
        constants::event_id::ENFORCEMENT_POLICY_DISPATCH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementPolicyDispatchReported,
        LogLevel::Info,
        enforcement_policy_dispatch_payload(&read_model),
        None,
    )
}

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    enforcement_command_execution::build_enforcement_audit_report_with_paths(command, paths).await
}
