# Screen Plan External Evidence Runbook

Use real live-device or live-host artifacts only. Attach artifacts under `output/screen-plan-proof/external-gates/artifacts/`, calculate the SHA-256 digest, and copy `manual-evidence-manifest.template.json` to `manual-evidence-manifest.json` only after all fields are true for the artifact.

Authenticated-account social proof must be operator-consented, must redact account identifiers, must cite local OCR/VLM/AI analysis, must cite policy dry-run consumption, and must cite raw image deletion/custody. Public social/feed proof is not enough for this gate.

## macos-live-capture-permission

- platform: macos
- evidence kind: platform-permission-prompt-screenshot
- workpack: 10 macOS capture adapter plan/proof
- requirement: real macOS ScreenCaptureKit session with Screen Recording permission, display/window pixels, OCR, and deletion proof
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef

## linux-desktop-session-capture

- platform: linux-wayland
- evidence kind: platform-session-recording
- workpack: 11 Linux capture adapter plan/proof
- requirement: real Linux X11 or Wayland portal desktop-session capture with deletion proof
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef

## android-physical-mediaprojection-capture

- platform: android-mediaprojection
- evidence kind: physical-device-capture-recording
- workpack: 12 Android MediaProjection adapter plan/proof
- requirement: real physical Android MediaProjection capture, stop callback, deletion, and local OCR proof
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef

## ios-physical-replaykit-capture

- platform: ios-replaykit
- evidence kind: physical-device-capture-recording
- workpack: 13 iOS ReplayKit adapter plan/proof
- requirement: real physical iOS ReplayKit or broadcast-extension capture with deletion proof
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef

## live-view-platform-prompt

- platform: android-mediaprojection
- evidence kind: platform-permission-prompt-screenshot
- workpack: 28 Live view optional mode
- requirement: real live-view platform prompt artifact, not ordinary capture-only permission evidence
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef, liveViewRuntimeProofRef, viewerAuditProofRef

## live-view-physical-device-parity

- platform: android-mediaprojection
- evidence kind: physical-device-capture-recording
- workpack: 28 Live view optional mode
- requirement: physical-device parity for live view transport/custody/deletion behavior
- required proof refs: localCaptureProofRef, localAnalysisProofRef, rawImageDeletionProofRef, liveViewRuntimeProofRef, viewerAuditProofRef

## live-view-hosted-relay-infrastructure

- platform: hosted-relay
- evidence kind: hosted-relay-proof
- workpack: 28 Live view optional mode
- requirement: hosted relay infrastructure proof with end-to-end encrypted custody and no raw-frame retention
- required proof refs: relayEncryptionProofRef, relayNoRetentionProofRef, viewerAuditProofRef

## live-view-privacy-legal-approval

- platform: policy-approval
- evidence kind: privacy-legal-approval
- workpack: 28 Live view optional mode
- requirement: privacy/legal approval record for optional live view
- required proof refs: privacyApprovalRecordRef, approverRoleRef, approvalScopeRef

## authenticated-account-social-capture

- platform: authenticated-social
- evidence kind: authenticated-account-capture-proof
- workpack: 30 Test suite, Playwright, rollout, PR gate
- requirement: real logged-in social/feed account capture with operator consent, redacted account identifiers, local OCR/VLM analysis, policy dry-run, and raw image deletion proof
- required proof refs: localCaptureProofRef, localAnalysisProofRef, policyDryRunProofRef, rawImageDeletionProofRef
