#![forbid(unsafe_code)]

//! Parent/controller runtime ownership boundary.
//!
//! This crate owns parent desktop/mobile runtime orchestration,
//! controller-side event handling, local assistant handoff, discovery entry
//! points, and parent-visible service state. Child evidence logic must stay in
//! child runtime feature crates.

mod agent_service_client;
pub mod data_custody_backup_runtime;
pub(crate) mod data_custody_backup_runtime_job_ledger;
pub(crate) mod data_custody_backup_runtime_lifecycle;
pub(crate) mod data_custody_backup_runtime_persistence;
pub(crate) mod data_custody_backup_runtime_ports;
pub(crate) mod data_custody_backup_runtime_reconciliation;
pub(crate) mod data_custody_backup_runtime_schedule;
pub(crate) mod data_custody_backup_runtime_schedule_execute;
pub(crate) mod data_custody_backup_runtime_schedule_execute_artifact;
pub(crate) mod data_custody_backup_runtime_schedule_execute_authority;
pub(crate) mod data_custody_backup_runtime_schedule_execute_finish;
pub(crate) mod data_custody_backup_runtime_schedule_execute_helpers;
pub(crate) mod data_custody_parent_runtime_clock;
pub mod data_custody_restore_runtime;
pub(crate) mod data_custody_restore_runtime_binding;
pub(crate) mod data_custody_restore_runtime_dispatch;
pub(crate) mod data_custody_restore_runtime_dispatch_apply;
pub(crate) mod data_custody_restore_runtime_dispatch_preflight;
pub mod data_custody_restore_runtime_executor;
pub(crate) mod data_custody_restore_runtime_ledger;
pub(crate) mod data_custody_restore_runtime_ledger_event_stage;
pub(crate) mod data_custody_restore_runtime_ledger_validation;
pub(crate) mod data_custody_restore_runtime_receipts;
pub(crate) mod data_custody_restore_runtime_reconciliation;
pub(crate) mod data_custody_restore_runtime_reconciliation_sections;
pub(crate) mod data_custody_restore_runtime_reconciliation_validation;
pub(crate) mod data_custody_restore_runtime_recovery;
pub(crate) mod data_custody_restore_runtime_rollback;
pub(crate) mod data_custody_restore_runtime_rollback_dispatch;
pub(crate) mod data_custody_restore_runtime_stage;
pub mod data_custody_runtime_eventing;
pub(crate) mod data_custody_runtime_eventing_identity;
pub(crate) mod data_custody_runtime_eventing_identity_backup;
pub(crate) mod data_custody_runtime_eventing_identity_kind;
pub(crate) mod data_custody_runtime_eventing_validation;
pub(crate) mod data_custody_runtime_eventing_validation_payload;
pub mod device_trust_bootstrap_runtime;
pub mod device_trust_bootstrap_runtime_status;
pub mod parent_service_health;
pub mod parent_ui_bridge;
pub mod policy_control_dispatch;
pub mod policy_control_update_flow;
pub(crate) mod setup_first_run;
pub mod tracking_child_check_in_request_flow;
pub mod tracking_config_update_flow;
pub mod tracking_dispatch;

pub const CRATE_NAME: &str = "ocentra-parent-runtime-core";
