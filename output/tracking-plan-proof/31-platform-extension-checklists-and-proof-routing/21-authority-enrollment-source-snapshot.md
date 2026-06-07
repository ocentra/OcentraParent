# WP31 Tracking Authority Enrollment Manual-Required Proof

- Branch: codex/tracking-plan-full-continuation-a
- Commit: b3680e0d7bf70eda44a47710b1052c8530a9fc35
- Evidence: test-results/tracking-authority-enrollment-manual-required-proof/proof.json
- Status: authority_required

## Required Evidence Rows

- android-device-owner: authority-required; required refs: tracking-authority-android-device-owner-device-identity-proof, tracking-authority-android-device-owner-enrollment-state-proof, tracking-authority-android-device-owner-approved-capability-proof, tracking-authority-android-device-owner-parent-visible-consent-proof
- android-managed-profile: authority-required; required refs: tracking-authority-android-managed-profile-device-identity-proof, tracking-authority-android-managed-profile-enrollment-state-proof, tracking-authority-android-managed-profile-approved-capability-proof, tracking-authority-android-managed-profile-parent-visible-consent-proof
- ios-family-controls-entitlement: authority-required; required refs: tracking-authority-ios-family-controls-entitlement-device-identity-proof, tracking-authority-ios-family-controls-entitlement-enrollment-state-proof, tracking-authority-ios-family-controls-entitlement-approved-capability-proof, tracking-authority-ios-family-controls-entitlement-parent-visible-consent-proof
- ios-app-review-approval: authority-required; required refs: tracking-authority-ios-app-review-approval-device-identity-proof, tracking-authority-ios-app-review-approval-enrollment-state-proof, tracking-authority-ios-app-review-approval-approved-capability-proof, tracking-authority-ios-app-review-approval-parent-visible-consent-proof
- desktop-managed-policy: manual-required; required refs: tracking-authority-desktop-managed-policy-device-identity-proof, tracking-authority-desktop-managed-policy-enrollment-state-proof, tracking-authority-desktop-managed-policy-approved-capability-proof, tracking-authority-desktop-managed-policy-parent-visible-consent-proof

## Non-Claims

- No authority enrollment is claimed.
- No hard-control runtime is claimed.
- No physical-device behavior is claimed.
- No provider delivery, production worker, or product-ready tracking is claimed.
