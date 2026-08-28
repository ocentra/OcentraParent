use std::error::Error;

use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceRef, ActivityObserver, ActivitySource,
    ActivitySubject, ActivitySubjectKind,
};

use crate::activity_store_path::ActivityDbPath;

type TestResult = Result<(), Box<dyn Error>>;

#[tokio::test]
async fn policy_request_resolution_persistence_rejects_missing_store_parent() -> TestResult {
    let path = ActivityDbPath(
        std::env::temp_dir()
            .join(format!(
                "ocentra-policy-resolution-missing-parent-{}",
                std::process::id()
            ))
            .join("activity.sqlite"),
    );
    let result =
        crate::policy_request_resolution_persistence::persist_activity_event(path, test_event())
            .await;
    assert_eq!(
        result.err(),
        Some(crate::policy_request_resolution_persistence::ActivityPersistenceError::Unavailable)
    );
    Ok(())
}

fn test_event() -> ActivityEvent {
    ActivityEvent {
        schema_version: ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION,
        event_id: "audit.policy-request.test".to_string(),
        observed_at: "2026-06-18T00:10:00Z".to_string(),
        source: ActivitySource {
            device_id: "local-dev-agent".to_string(),
            platform: "windows".to_string(),
            observer: ActivityObserver::AgentService,
            source_id: "policy-request-parent-resolution".to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: "child-profile-1".to_string(),
            display_name: None,
        },
        fields: ocentra_parent_agent_protocol::logging::LogFields::new(),
        evidence: Vec::<ActivityEvidenceRef>::new(),
    }
}
