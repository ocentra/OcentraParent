use serde::{Deserialize, Serialize};

use crate::constants;

use super::lifecycle::{LocalAiDegradedState, LocalAiResourceClass};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderSingletonScope {
    #[serde(rename = "physical-device")]
    PhysicalDevice,
}

impl LocalAiProviderSingletonScope {
    pub fn as_protocol_str(&self) -> &'static str {
        constants::local_ai_runtime::SINGLETON_SCOPE_PHYSICAL_DEVICE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderSchedulerLifecycle {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl LocalAiProviderSchedulerLifecycle {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::SCHEDULER_LIFECYCLE_IDLE,
        constants::local_ai_runtime::SCHEDULER_LIFECYCLE_RUNNING,
        constants::local_ai_runtime::SCHEDULER_LIFECYCLE_QUEUED,
        constants::local_ai_runtime::SCHEDULER_LIFECYCLE_DEGRADED,
        constants::local_ai_runtime::SCHEDULER_LIFECYCLE_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderSchedulerJobClass {
    #[serde(rename = "child-safety")]
    ChildSafety,
    #[serde(rename = "parent-assistant")]
    ParentAssistant,
    #[serde(rename = "parent-report")]
    ParentReport,
}

impl LocalAiProviderSchedulerJobClass {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY,
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT,
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_REPORT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LocalAiProviderSchedulerJobStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "complete")]
    Complete,
}

impl LocalAiProviderSchedulerJobStatus {
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_ACCEPTED,
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_RUNNING,
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_QUEUED,
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_DEGRADED,
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_UNAVAILABLE,
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_COMPLETE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiProviderSchedulerQueue {
    pub child_safety_queued: u16,
    pub parent_assistant_queued: u16,
    pub parent_report_queued: u16,
}

impl LocalAiProviderSchedulerQueue {
    pub fn total(&self) -> u16 {
        self.child_safety_queued + self.parent_assistant_queued + self.parent_report_queued
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiProviderSchedulerStatus {
    pub physical_device_id: String,
    pub singleton_scope: LocalAiProviderSingletonScope,
    pub provider_id: String,
    pub runtime_reference_id: String,
    pub model_id: String,
    pub model_reference: String,
    pub resource_class: LocalAiResourceClass,
    pub lifecycle_state: LocalAiProviderSchedulerLifecycle,
    pub current_job_class: Option<LocalAiProviderSchedulerJobClass>,
    pub queue: LocalAiProviderSchedulerQueue,
    pub duplicate_runtime_blocked: bool,
    pub degraded_state: LocalAiDegradedState,
    pub unavailable_reason: Option<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiProviderSchedulerDecision {
    pub physical_device_id: String,
    pub job_class: LocalAiProviderSchedulerJobClass,
    pub job_status: LocalAiProviderSchedulerJobStatus,
    pub selected_runtime_reference_id: Option<String>,
    pub queue_position: Option<u16>,
    pub unavailable_reason: Option<String>,
    pub duplicate_runtime_blocked: bool,
}
