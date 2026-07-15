use serde_json::{json, Value};

#[path = "billing_entitlement_runtime_proof_state.rs"]
mod billing_entitlement_runtime_proof_state;

pub const BILLING_ENTITLEMENT_RUNTIME_PROOF_SCHEMA_VERSION: &str =
    "billing-entitlement-runtime-proof";

const TIMESTAMP: &str = "2026-06-04T23:34:57.000Z";
const EXPIRY_TIMESTAMP: &str = "2026-06-11T23:34:57.000Z";
const RETRY_TIMESTAMP: &str = "2026-06-05T00:34:57.000Z";

pub fn billing_entitlement_runtime_proof_read_model() -> Value {
    let runtime_provider_unavailable_failure =
        billing_entitlement_runtime_proof_state::runtime_provider_unavailable_failure();
    let runtime_stale_snapshot_failure =
        billing_entitlement_runtime_proof_state::runtime_stale_snapshot_failure();
    let runtime_payment_required_failure =
        billing_entitlement_runtime_proof_state::runtime_payment_required_failure();
    let runtime_validation_failure =
        billing_entitlement_runtime_proof_state::runtime_validation_failure();

    json!({
        "schemaVersion": BILLING_ENTITLEMENT_RUNTIME_PROOF_SCHEMA_VERSION,
        "snapshotConsumptions": runtime_snapshot_consumptions(
            &runtime_stale_snapshot_failure,
            &runtime_payment_required_failure,
            &runtime_provider_unavailable_failure,
            &runtime_validation_failure,
        ),
        "deviceLimitConsumptions": runtime_device_limit_consumptions(
            &runtime_payment_required_failure,
            &runtime_stale_snapshot_failure,
            &runtime_validation_failure,
        ),
        "failureConsumptions": runtime_failure_consumptions(
            &runtime_provider_unavailable_failure,
            &runtime_stale_snapshot_failure,
            &runtime_payment_required_failure,
            &runtime_validation_failure,
        ),
        "nonClaims": runtime_non_claims(),
        "stripeSdkClaim": "not-included",
        "providerExecutionClaim": "not-implemented",
        "providerContactClaim": "manual-required",
        "refundCreditClaim": "manual-required",
        "productionBillingClaim": "not-claimed",
        "portalUiClaim": "not-implemented",
        "childCustodyClaim": "signed-snapshot-consumption-contract",
        "childActivityCustodyClaim": "not-included",
        "updatedAt": TIMESTAMP,
    })
}

fn runtime_snapshot_consumptions(
    runtime_stale_snapshot_failure: &Value,
    runtime_payment_required_failure: &Value,
    runtime_provider_unavailable_failure: &Value,
    runtime_validation_failure: &Value,
) -> Value {
    json!([
        snapshot_consumption(
            "runtime-snapshot-active",
            "snapshot-active",
            "signed-local-snapshot",
            &entitlement_snapshot(
                "entitlement-snapshot-family-1-active",
                "active",
                "signed-local-snapshot",
                "schema-valid-local",
                &Value::Null,
            ),
            &Value::Null,
        ),
        snapshot_consumption(
            "runtime-snapshot-stale",
            "snapshot-stale",
            "signed-local-snapshot",
            &entitlement_snapshot(
                "entitlement-runtime-expired",
                "expired",
                "signed-local-snapshot",
                "schema-valid-local",
                runtime_stale_snapshot_failure,
            ),
            runtime_stale_snapshot_failure,
        ),
        snapshot_consumption(
            "runtime-snapshot-payment-required",
            "payment-required",
            "signed-local-snapshot",
            &entitlement_snapshot(
                "entitlement-runtime-past-due",
                "past-due",
                "signed-local-snapshot",
                "schema-valid-local",
                runtime_payment_required_failure,
            ),
            runtime_payment_required_failure,
        ),
        snapshot_consumption(
            "runtime-snapshot-provider-unavailable",
            "provider-unavailable",
            "unavailable",
            &entitlement_snapshot(
                "entitlement-runtime-unavailable",
                "unavailable",
                "unavailable",
                "unavailable",
                runtime_provider_unavailable_failure,
            ),
            runtime_provider_unavailable_failure,
        ),
        snapshot_consumption(
            "runtime-snapshot-manual-review",
            "manual-review",
            "manual-support-review",
            &entitlement_snapshot(
                "entitlement-runtime-manual-review",
                "unknown",
                "manual-admin-review",
                "manual-required",
                runtime_validation_failure,
            ),
            runtime_validation_failure,
        ),
    ])
}

fn runtime_device_limit_consumptions(
    runtime_payment_required_failure: &Value,
    runtime_stale_snapshot_failure: &Value,
    runtime_validation_failure: &Value,
) -> Value {
    json!([
        device_limit_consumption(
            "runtime-device-allowed",
            "allowed",
            "accepted-local",
            &Value::Null
        ),
        device_limit_consumption(
            "runtime-device-denied",
            "denied",
            "blocked-new-device",
            runtime_payment_required_failure,
        ),
        device_limit_consumption(
            "runtime-device-grace",
            "grace",
            "accepted-grace",
            runtime_stale_snapshot_failure,
        ),
        device_limit_consumption(
            "runtime-device-manual",
            "manual-review",
            "manual-required",
            runtime_validation_failure,
        ),
    ])
}

fn runtime_failure_consumptions(
    runtime_provider_unavailable_failure: &Value,
    runtime_stale_snapshot_failure: &Value,
    runtime_payment_required_failure: &Value,
    runtime_validation_failure: &Value,
) -> Value {
    json!([
        failure_consumption(
            "runtime-failure-provider-unavailable",
            runtime_provider_unavailable_failure,
            "unavailable-local-safety",
            &[
                "account-entitlement-snapshot-consumption",
                "billing-failure-state-consumption",
            ],
        ),
        failure_consumption(
            "runtime-failure-stale-snapshot",
            runtime_stale_snapshot_failure,
            "accepted-grace",
            &[
                "account-entitlement-snapshot-consumption",
                "device-limit-decision-consumption",
            ],
        ),
        failure_consumption(
            "runtime-failure-payment-required",
            runtime_payment_required_failure,
            "blocked-new-device",
            &[
                "account-entitlement-snapshot-consumption",
                "device-limit-decision-consumption",
            ],
        ),
        failure_consumption(
            "runtime-failure-validation-failed",
            runtime_validation_failure,
            "manual-required",
            &["billing-failure-state-consumption"],
        ),
    ])
}

fn runtime_non_claims() -> Value {
    json!([
        "no-stripe-sdk",
        "no-live-provider-execution",
        "no-provider-contact",
        "no-refund-credit-runtime",
        "no-child-activity-custody",
        "no-production-billing-claim",
        "no-portal-ui",
    ])
}

fn entitlement_snapshot(
    snapshot_id: &str,
    subscription_status: &str,
    source: &str,
    signature_state: &str,
    failure_state: &Value,
) -> Value {
    json!({
        "schemaVersion": "billing-entitlement-contract-proof",
        "snapshotId": snapshot_id,
        "family": {
            "familyId": "family-billing-entitlement-proof-1",
        },
        "parentAccount": {
            "parentAccountId": "parent-account-billing-entitlement-proof-1",
        },
        "planId": "family-plus-monthly",
        "subscriptionStatus": subscription_status,
        "source": source,
        "signatureState": signature_state,
        "generatedAt": "2026-06-03T09:57:32.000Z",
        "expiresAt": EXPIRY_TIMESTAMP,
        "deviceLimit": 5,
        "baseChildDeviceLimit": 1,
        "activeReferralCredits": 2,
        "paidExtraChildDeviceSeats": 2,
        "effectiveChildDeviceLimit": 5,
        "featureDecisions": [
            feature_decision("multi-device-sync", "available", "within-plan", false, "unchanged"),
            feature_decision("advanced-reports", "available", "within-plan", false, "unchanged"),
            feature_decision("cloud-relay", "grace", "snapshot-stale", false, "grace-with-local-safety"),
            feature_decision(
                "local-evidence-capture",
                "local-only",
                "within-plan",
                true,
                "local-only",
            ),
            feature_decision(
                "evidence-export-access",
                "local-only",
                "within-plan",
                true,
                "local-only",
            ),
        ],
        "failureState": failure_state,
    })
}

fn feature_decision(
    feature_code: &str,
    decision: &str,
    reason_code: &str,
    safety_critical: bool,
    local_safety_behavior: &str,
) -> Value {
    json!({
        "featureCode": feature_code,
        "decision": decision,
        "reasonCode": reason_code,
        "safetyCritical": safety_critical,
        "localSafetyBehavior": local_safety_behavior,
        "evidenceExportAccess": "retained",
        "childActivityCustody": "not-included",
    })
}

fn snapshot_consumption(
    boundary_id: &str,
    runtime_state: &str,
    source: &str,
    entitlement_snapshot: &Value,
    failure_state: &Value,
) -> Value {
    json!({
        "schemaVersion": BILLING_ENTITLEMENT_RUNTIME_PROOF_SCHEMA_VERSION,
        "boundaryId": boundary_id,
        "operation": "account-entitlement-snapshot-consumption",
        "runtimeState": runtime_state,
        "source": source,
        "entitlementSnapshot": entitlement_snapshot,
        "localSafetyBehavior": local_safety_behavior(failure_state),
        "evidenceExportAccess": "retained",
        "childActivityCustody": "not-included",
        "failureState": failure_state,
        "auditReference": format!("audit-{boundary_id}"),
    })
}

fn device_limit_consumption(
    boundary_id: &str,
    decision: &str,
    consumption_state: &str,
    failure_state: &Value,
) -> Value {
    json!({
        "schemaVersion": BILLING_ENTITLEMENT_RUNTIME_PROOF_SCHEMA_VERSION,
        "boundaryId": boundary_id,
        "operation": "device-limit-decision-consumption",
        "deviceLimitDecision": required_device_limit_decision(decision),
        "consumptionState": consumption_state,
        "localSafetyBehavior": local_safety_behavior(failure_state),
        "evidenceExportAccess": "retained",
        "childActivityCustody": "not-included",
        "failureState": failure_state,
        "auditReference": format!("audit-{boundary_id}"),
    })
}

fn failure_consumption(
    boundary_id: &str,
    failure_state: &Value,
    consumption_state: &str,
    consumed_for: &[&str],
) -> Value {
    json!({
        "schemaVersion": BILLING_ENTITLEMENT_RUNTIME_PROOF_SCHEMA_VERSION,
        "boundaryId": boundary_id,
        "operation": "billing-failure-state-consumption",
        "failureState": failure_state,
        "consumedFor": consumed_for,
        "localSafetyBehavior": local_safety_behavior(failure_state),
        "evidenceExportAccess": "retained",
        "childActivityCustody": "not-included",
        "consumptionState": consumption_state,
        "auditReference": format!("audit-{boundary_id}"),
    })
}

fn required_device_limit_decision(decision: &str) -> Value {
    match decision {
        "allowed" => allowed_device_limit_decision(),
        "denied" => denied_device_limit_decision(),
        "grace" => grace_device_limit_decision(),
        "manual-review" => manual_review_device_limit_decision(),
        _ => Value::Null,
    }
}

fn allowed_device_limit_decision() -> Value {
    json!({
        "schemaVersion": "billing-entitlement-contract-proof",
        "decisionId": "device-limit-allowed-1",
        "requestedDevice": {
            "deviceId": "windows-child-device-1",
            "childProfileId": "child-billing-entitlement-proof-1",
            "label": "windows-child-device-1 activation",
            "platform": "windows",
        },
        "entitlementSnapshotId": "entitlement-snapshot-family-1-active",
        "activeDeviceCount": 4,
        "planDeviceLimit": 5,
        "requestedDeviceAlreadyTrusted": false,
        "decision": "allowed",
        "reasonCode": "within-plan",
        "deviceActivationBehavior": "allow-new-device",
        "auditReference": "audit-device-limit-allowed-1",
        "existingLocalSafetyBehavior": "unchanged",
    })
}

fn denied_device_limit_decision() -> Value {
    json!({
        "schemaVersion": "billing-entitlement-contract-proof",
        "decisionId": "device-limit-denied-1",
        "requestedDevice": {
            "deviceId": "android-child-device-6",
            "childProfileId": "child-billing-entitlement-proof-1",
            "label": "android-child-device-6 activation",
            "platform": "android",
        },
        "entitlementSnapshotId": "entitlement-snapshot-family-1-active",
        "activeDeviceCount": 5,
        "planDeviceLimit": 5,
        "requestedDeviceAlreadyTrusted": false,
        "decision": "denied",
        "reasonCode": "limit-exceeded",
        "deviceActivationBehavior": "deny-new-device",
        "auditReference": "audit-device-limit-denied-1",
        "existingLocalSafetyBehavior": "grace-with-local-safety",
    })
}

fn grace_device_limit_decision() -> Value {
    json!({
        "schemaVersion": "billing-entitlement-contract-proof",
        "decisionId": "device-limit-grace-1",
        "requestedDevice": {
            "deviceId": "ios-child-device-2",
            "childProfileId": "child-billing-entitlement-proof-1",
            "label": "ios-child-device-2 activation",
            "platform": "ios",
        },
        "entitlementSnapshotId": "entitlement-snapshot-family-1-active",
        "activeDeviceCount": 5,
        "planDeviceLimit": 5,
        "requestedDeviceAlreadyTrusted": false,
        "decision": "grace",
        "reasonCode": "snapshot-stale",
        "deviceActivationBehavior": "grace-existing-devices",
        "auditReference": "audit-device-limit-grace-1",
        "existingLocalSafetyBehavior": "grace-with-local-safety",
    })
}

fn manual_review_device_limit_decision() -> Value {
    json!({
        "schemaVersion": "billing-entitlement-contract-proof",
        "decisionId": "device-limit-manual-1",
        "requestedDevice": {
            "deviceId": "android-child-device-7",
            "childProfileId": "child-billing-entitlement-proof-1",
            "label": "android-child-device-7 activation",
            "platform": "android",
        },
        "entitlementSnapshotId": "entitlement-snapshot-family-1-active",
        "activeDeviceCount": 5,
        "planDeviceLimit": 5,
        "requestedDeviceAlreadyTrusted": false,
        "decision": "manual-review",
        "reasonCode": "manual-review",
        "deviceActivationBehavior": "manual-review-required",
        "auditReference": "audit-device-limit-manual-1",
        "existingLocalSafetyBehavior": "grace-with-local-safety",
    })
}

fn local_safety_behavior(failure_state: &Value) -> &str {
    failure_state
        .get("localSafetyBehavior")
        .and_then(Value::as_str)
        .unwrap_or("unchanged")
}
