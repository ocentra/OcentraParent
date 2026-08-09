pub mod compiler {
    pub const FIELD_COMPILED_ARTIFACT_ID: &str = "policy_compiler.compiled_artifact_id";
    pub const FIELD_CONSUMER_POLICY_VERSION: &str = "policy_compiler.consumer_policy_version";
    pub const FIELD_EVIDENCE_CUSTODY_REQUIREMENTS: &str =
        "policy_compiler.evidence_custody_requirements";
    pub const FIELD_SUPPORT_MATRIX_DOMAIN: &str = "policy_compiler.support_matrix.domain";
    pub const FIELD_SUPPORT_MATRIX_TARGET_KIND: &str = "policy_compiler.support_matrix.target_kind";
    pub const FIELD_SOURCE_STATUS: &str = "policy_compiler.source_status";
    pub const REASON_UNSUPPORTED_TARGET: &str = "unsupported-target";
    pub const REASON_MANUAL_REQUIRED_TARGET: &str = "manual-required-target";
    pub const REASON_ENFORCEMENT_HANDOFF_REQUIRED: &str = "enforcement-handoff-required";
    pub const NO_CLAIM_COMPILED_ARTIFACT_NOT_SOURCE_TRUTH: &str =
        "compiled-artifact-not-source-truth";
    pub const NO_CLAIM_RUNTIME_MUTATION: &str = "runtime-mutation-not-claimed";
    pub const NO_CLAIM_ENFORCEMENT: &str = "enforcement-not-claimed";
    pub const NO_CLAIM_UI_DELIVERY: &str = "ui-delivery-not-claimed";
    pub const NO_CLAIM_PLATFORM_SUPPORT: &str = "platform-support-not-claimed";
    pub const VALUE_SOURCE_POLICY_VERSION_PREFIX: &str = "source ";
    pub const VALUE_CONSUMER_POLICY_VERSION_SEPARATOR: &str = " != consumer ";
}

pub mod conflict {
    pub const REASON_OVERLAPPING_ACTIONS: &str = "overlapping-actions";
    pub const REASON_EQUAL_PRIORITY: &str = "equal-priority-conflict";
    pub const REASON_UNKNOWN_DEVICE_TARGET: &str = "unknown-device-target";
    pub const REASON_TIMEZONE_BOUNDARY: &str = "timezone-boundary-conflict";
    pub const REASON_AMBIGUOUS_LOCAL_TIME: &str = "ambiguous-local-time";
    pub const REASON_NONEXISTENT_LOCAL_TIME: &str = "nonexistent-local-time";
    pub const REASON_CLOCK_SKEW: &str = "clock-skew";
}

pub mod source {
    pub const FIELD_DOCUMENT_ID: &str = "policy_source.document_id";
    pub const FIELD_HOUSEHOLD_ID: &str = "policy_source.household_id";
    pub const FIELD_ACTOR_ID: &str = "policy_source.actor_id";
    pub const FIELD_ACTOR_STATE: &str = "policy_source.actor_state";
    pub const FIELD_STATUS: &str = "policy_source.status";
    pub const FIELD_CHILD_PROFILE_ID: &str = "policy_source.child_profile_id";
    pub const FIELD_DEVICE_ID: &str = "policy_source.device_id";
    pub const FIELD_RULE_ID: &str = "policy_source.rule_id";
    pub const FIELD_TARGET_REFERENCE_ID: &str = "policy_source.target_reference_id";
    pub const FIELD_SCHEDULE_ID: &str = "policy_source.schedule_id";
    pub const FIELD_TIMEZONE_NAME: &str = "policy_source.timezone_name";
    pub const FIELD_SCHEDULE_STARTS_AT: &str = "policy_source.schedule.starts_at";
    pub const FIELD_SCHEDULE_ENDS_AT: &str = "policy_source.schedule.ends_at";
    pub const FIELD_SCHEDULE_TIME_BUDGET: &str = "policy_source.schedule.time_budget";
    pub const FIELD_SCHEDULE_BUDGET_WINDOW_MINUTES: &str =
        "policy_source.schedule.time_budget.budget_window_minutes";
    pub const FIELD_SCHEDULE_RESET_LOCAL_TIME: &str =
        "policy_source.schedule.time_budget.reset.local_time";
    pub const FIELD_SCHEDULE_RESET_DAY: &str = "policy_source.schedule.time_budget.reset.day";
    pub const FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES: &str =
        "policy_source.schedule.time_budget.carryover.max_minutes";
    pub const FIELD_SCHEDULE_EFFECTIVE_FROM: &str =
        "policy_source.schedule.time_budget.effective_from";
    pub const FIELD_SCHEDULE_EFFECTIVE_UNTIL: &str =
        "policy_source.schedule.time_budget.effective_until";
    pub const FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES: &str =
        "policy_source.schedule.time_budget.bonus_expiry_minutes";
    pub const FIELD_REASON_CODE: &str = "policy_source.reason_code";
    pub const FIELD_AUDIT_REFERENCE_ID: &str = "policy_source.audit_reference_id";
    pub const FIELD_POLICY_VERSION: &str = "policy_source.policy_version";
    pub const FIELD_SUPERSEDED_BY_POLICY_VERSION: &str =
        "policy_source.superseded_by_policy_version";
    pub const FIELD_ROLLED_BACK_POLICY_VERSION: &str =
        "policy_source.rollback_ref.rolled_back_policy_version";
    pub const FIELD_RESTORED_DOCUMENT_ID: &str = "policy_source.rollback_ref.restored_document_id";
    pub const FIELD_RESTORED_POLICY_VERSION: &str =
        "policy_source.rollback_ref.restored_policy_version";
    pub const FIELD_AUDIT_REFERENCE_IDS: &str = "policy_source.audit_reference_ids";
    pub const FIELD_SOURCE_SURFACE: &str = "policy_source.source_surface";
    pub const FIELD_ACTOR_ROLE: &str = "policy_source.actor_role";
    pub const FIELD_RULE_SCHEDULE_ID: &str = "policy_source.rule.schedule_id";
    pub const FIELD_RULES: &str = "policy_source.rules";

    pub const SURFACE_PARENT_PORTAL: &str = "parent-portal";
    pub const SURFACE_PARENT_COMPANION: &str = "parent-companion";
    pub const SURFACE_AI_PREVIEW: &str = "ai-preview";
    pub const SURFACE_DOMAIN_CACHE: &str = "domain-cache";

    pub const ROLE_PARENT: &str = "parent";
    pub const ROLE_CO_PARENT: &str = "co-parent";
    pub const ROLE_OBSERVER: &str = "observer";
    pub const ROLE_CHILD: &str = "child";
    pub const ROLE_SUPPORT: &str = "support";

    pub const ACTOR_STATE_ACTIVE: &str = "active";
    pub const ACTOR_STATE_REVOKED: &str = "revoked";

    pub const STATUS_DRAFT: &str = "draft";
    pub const STATUS_PREVIEW: &str = "preview";
    pub const STATUS_CONFIRMED: &str = "confirmed";
    pub const STATUS_QUEUED: &str = "queued";
    pub const STATUS_DELIVERED: &str = "delivered";
    pub const STATUS_ACKNOWLEDGED: &str = "acknowledged";
    pub const STATUS_ACTIVE: &str = "active";
    pub const STATUS_PARTIALLY_ACTIVE: &str = "partially-active";
    pub const STATUS_REJECTED: &str = "rejected";
    pub const STATUS_SUPERSEDED: &str = "superseded";
    pub const STATUS_ROLLED_BACK: &str = "rolled-back";
    pub const STATUS_STALE: &str = "stale";
    pub const STATUS_EXPIRED: &str = "expired";
    pub const STATUS_MANUAL_REQUIRED: &str = "manual-required";

    pub const VALUE_STALE_POLICY_VERSION_PREFIX: &str = "stale policy version ";
    pub const VALUE_STALE_POLICY_VERSION_SEPARATOR: &str = " behind ";
    pub const VALUE_DUPLICATE_SOURCE_TRUTH_PREFIX: &str = "duplicate source truth for household ";
    pub const VALUE_DUPLICATE_SOURCE_TRUTH_VERSION_SEPARATOR: &str = " version ";
    pub const VALUE_MISSING_AUDIT_REFERENCE_FOR_STATUS_PREFIX: &str =
        "missing audit reference for status ";
    pub const VALUE_MISSING_AUDIT_REFERENCES_FOR_STATUS_PREFIX: &str =
        "missing audit references for status ";
    pub const VALUE_ACTIVE_POLICY_HAS_NO_RULES: &str = "active policy has no rules";
    pub const VALUE_ACTIVE_POLICY_REQUIRES_ACKNOWLEDGED_DELIVERY: &str =
        "active policy requires acknowledged delivery for every target";
    pub const VALUE_REPLACEMENT_POLICY_VERSION_PREFIX: &str = "replacement policy version ";
    pub const VALUE_MUST_BE_NEWER_THAN_SEPARATOR: &str = " must be newer than ";
    pub const VALUE_RESTORED_POLICY_VERSION_PREFIX: &str = "restored policy version ";
    pub const VALUE_MUST_BE_OLDER_THAN_SEPARATOR: &str = " must be older than ";
}

pub mod preview {
    pub const FIELD_CANDIDATE_STATUS: &str = "policy_preview.candidate.status";
    pub const FIELD_CURRENT_DOCUMENT_HOUSEHOLD_ID: &str =
        "policy_preview.current_document.household_id";
    pub const FIELD_SCHEDULE_TIME: &str = "policy_preview.schedule_time";
    pub const FIELD_REQUEST_ID: &str = "policy_preview.request_id";
    pub const FIELD_EXPLANATION_CODE: &str = "policy_preview.explanation_code";
    pub const FIELD_SCHEDULE_ID: &str = "policy_preview.schedule_id";

    pub const EXPLANATION_STALE_POLICY_VERSION: &str = "stale-policy-version";
    pub const EXPLANATION_SCHEDULE_TIMEZONE_BOUNDARY: &str = "schedule-timezone-boundary";
    pub const EXPLANATION_OVERLAPPING_SCHEDULE: &str = "overlapping-schedule";
    pub const EXPLANATION_AMBIGUOUS_LOCAL_TIME: &str = "ambiguous-local-time";
    pub const EXPLANATION_NONEXISTENT_LOCAL_TIME: &str = "nonexistent-local-time";
    pub const EXPLANATION_CLOCK_SKEW: &str = "clock-skew";
    pub const EXPLANATION_UNSUPPORTED_TARGET: &str = "unsupported-target";

    pub const ERROR_STATIC_EXPLANATION_CODE: &str = "static preview explanation code";
}

pub mod request {
    pub const FIELD_REQUEST_ID: &str = "policy_request.request_id";
    pub const FIELD_SUBMISSION_KEY: &str = "policy_request.submission_key";
    pub const FIELD_APPROVAL_ID: &str = "policy_request.approval_id";
    pub const FIELD_OVERRIDE_ID: &str = "policy_request.override_id";
    pub const FIELD_ASSISTANT_PREVIEW_ID: &str = "policy_request.assistant_preview_id";
    pub const FIELD_TIMESTAMP: &str = "policy_request.timestamp";
    pub const FIELD_DURATION_MINUTES: &str = "policy_request.duration_minutes";
    pub const FIELD_AUDIT_REFERENCE_IDS: &str = "policy_request.audit_reference_ids";
    pub const FIELD_STATUS: &str = "policy_request.status";
    pub const FIELD_ORIGIN: &str = "policy_request.origin";
    pub const FIELD_ACTOR_ROLE: &str = "policy_request.actor_role";
    pub const FIELD_ACTOR_STATE: &str = "policy_request.actor_state";
    pub const FIELD_ASSISTANT_CONFIRMATION_STATE: &str =
        "policy_request.assistant_confirmation_state";
    pub const FIELD_APPROVAL_DECISION: &str = "policy_request.approval_decision";
    pub const FIELD_APPROVED_BONUS_MINUTES: &str = "policy_request.approved_bonus_minutes";
    pub const FIELD_REQUESTED_BONUS_MINUTES: &str = "policy_request.requested_bonus_minutes";
    pub const FIELD_RESOLVED_APPROVAL_ID: &str = "policy_request.resolved_approval_id";
    pub const FIELD_HOUSEHOLD_ID: &str = "policy_request.household_id";
    pub const FIELD_POLICY_VERSION: &str = "policy_request.policy_version";
    pub const FIELD_CANONICAL_CONFIRMED_REQUEST_JSON: &str =
        "policy_request.canonical_confirmed_request_json";
    pub const FIELD_CANONICAL_RESOLVED_REQUEST_JSON: &str =
        "policy_request.canonical_resolved_request_json";
    pub const FIELD_CANONICAL_TEMPORARY_OVERRIDE_JSON: &str =
        "policy_request.canonical_temporary_override_json";

    pub const OVERRIDE_ID_PREFIX: &str = "policy-override:";
    pub const VALUE_DUPLICATE_SUBMISSION_KEY_PREFIX: &str = "duplicate submission key ";
    pub const VALUE_DUPLICATE_SUBMISSION_KEY_SUFFIX: &str = " with different request payload";
    pub const VALUE_ASSISTANT_PREVIEW_ONLY: &str = "assistant-preview-only";
    pub const VALUE_MISSING_OVERRIDE_FOR_RESOLVED_APPROVAL_REPLAY: &str =
        "missing override for resolved approval replay";
    pub const VALUE_DENY_OR_EXPIRE_CANNOT_CARRY_OVERRIDE_VALUES: &str =
        "deny-or-expire-cannot-carry-override-values";
    pub const VALUE_MODIFY_REQUIRES_CHANGED_OVERRIDE_VALUES: &str =
        "modify-requires-changed-override-values";
    pub const VALUE_BONUS_TIME_APPROVAL_REQUIRES_MINUTES: &str =
        "bonus-time-approval-requires-minutes";
    pub const VALUE_BONUS_TIME_REQUEST_REQUIRES_MINUTES: &str =
        "bonus-time-request-requires-minutes";
    pub const VALUE_CHILD_REQUEST_CANNOT_BE_ASSISTANT_PREVIEW: &str =
        "child-request-cannot-be-assistant-preview";
    pub const VALUE_ASSISTANT_DRAFT_REQUEST_MUST_STAY_PREVIEW_ONLY_UNTIL_PARENT_CONFIRMED: &str =
        "assistant-draft-request-must-stay-preview-only-until-parent-confirmed";
    pub const VALUE_PREVIEW_ONLY_REQUEST_CANNOT_BE_RESOLVED: &str =
        "preview-only-request-cannot-be-resolved";
    pub const VALUE_EXPIRED_REQUEST_CANNOT_BE_APPROVED: &str = "expired-request-cannot-be-approved";
    pub const VALUE_MISSING_AUDIT_REFERENCE: &str = "missing-audit-reference";

    pub const STATUS_PREVIEW_ONLY: &str = "preview-only";
    pub const STATUS_PENDING_PARENT_REVIEW: &str = "pending-parent-review";
    pub const STATUS_APPROVED: &str = "approved";
    pub const STATUS_DENIED: &str = "denied";
    pub const STATUS_MODIFIED: &str = "modified";
    pub const STATUS_EXPIRED: &str = "expired";
    pub const STATUS_REPLAY_REJECTED: &str = "replay-rejected";
}

pub mod delivery {
    pub const FIELD_DELIVERY_ID: &str = "policy_delivery.delivery_id";
    pub const FIELD_ATTEMPT_ID: &str = "policy_delivery.attempt_id";
    pub const FIELD_SEQUENCE: &str = "policy_delivery.sequence";
    pub const FIELD_STATE: &str = "policy_delivery.state";
    pub const FIELD_AUDIT_REFERENCE_IDS: &str = "policy_delivery.audit_reference_ids";
    pub const FIELD_REASON_CODE: &str = "policy_delivery.reason_code";
    pub const FIELD_SUPERSEDED_BY_POLICY_VERSION: &str =
        "policy_delivery.superseded_by_policy_version";
    pub const FIELD_ROLLBACK_REFERENCE_STATE: &str = "policy_delivery.rollback_reference_state";

    pub const STATUS_QUEUED: &str = "queued";
    pub const STATUS_DELIVERED: &str = "delivered";
    pub const STATUS_ACKNOWLEDGED: &str = "acknowledged";
    pub const STATUS_APPLIED: &str = "applied";
    pub const STATUS_REJECTED: &str = "rejected";
    pub const STATUS_SUPERSEDED: &str = "superseded";
    pub const STATUS_ROLLED_BACK: &str = "rolled-back";
    pub const STATUS_DEGRADED: &str = "degraded";
    pub const STATUS_OFFLINE: &str = "offline";

    pub const VALUE_MISSING_AUDIT_REFERENCES: &str = "missing audit references";
    pub const VALUE_CONFLICTING_REPLAY_FOR_SEQUENCE_PREFIX: &str =
        "conflicting replay for sequence ";
    pub const VALUE_CONFLICTING_REPLAY_ON_SEPARATOR: &str = " on ";
    pub const VALUE_INVALID_TRANSITION_PREFIX: &str = "invalid transition ";
    pub const VALUE_INVALID_TRANSITION_SEPARATOR: &str = " -> ";
    pub const VALUE_UNEXPECTED_REASON_CODE_PREFIX: &str = "unexpected reason code ";
    pub const VALUE_UNEXPECTED_REPLACEMENT_POLICY_VERSION_PREFIX: &str =
        "unexpected replacement policy version ";
    pub const VALUE_UNEXPECTED_ROLLBACK_REFERENCE_STATE_PREFIX: &str =
        "unexpected rollback reference state ";
    pub const VALUE_FOR_STATE_SEPARATOR: &str = " for ";
    pub const VALUE_MISSING_REASON_CODE_FOR_PREFIX: &str = "missing reason code for ";
    pub const VALUE_MISSING_REPLACEMENT_POLICY_VERSION_FOR_PREFIX: &str =
        "missing replacement policy version for ";
    pub const VALUE_REPLACEMENT_POLICY_VERSION_PREFIX: &str = "replacement policy version ";
    pub const VALUE_MUST_BE_NEWER_THAN_SEPARATOR: &str = " must be newer than ";
    pub const VALUE_MISSING_ROLLBACK_REFERENCE_STATE_FOR_PREFIX: &str =
        "missing rollback reference state for ";
}
