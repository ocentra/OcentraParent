/* generated from crates/schema/src/parent_ui_bridge.rs */

import {
  ParentScreenSettingsCommandRuntime,
  ParentScreenSettingsUpdateKind,
  type ParentUnknownRecord,
} from './parent-ui-bridge';

export const ParentScreenSettingsUpdateStatus = { Accepted: 'accepted', Rejected: 'rejected' } as const;
type ParentScreenSettingsUpdateKindValue = (typeof ParentScreenSettingsUpdateKind)[keyof typeof ParentScreenSettingsUpdateKind];
type ParentScreenSettingsUpdateStatus = (typeof ParentScreenSettingsUpdateStatus)[keyof typeof ParentScreenSettingsUpdateStatus];

export interface ParentScreenAnalysisParentSetting {
  readonly schemaVersion: number; readonly screenAnalysisEnabled: boolean; readonly analysisMode: string;
  readonly cadenceCaptureEnabled: boolean; readonly cadenceSeconds: number; readonly strictModeEnabled: boolean;
  readonly triggerCaptureEnabled: boolean; readonly enabledTriggers: readonly string[]; readonly allowedCaptureScope: string;
  readonly ocrTextEnabled: boolean; readonly ocrTextSnippetLimit: number; readonly redactionMode: string;
  readonly ocrTextRetentionMode: string; readonly credentialSuppressionEnabled: boolean; readonly piiRedactionEnabled: boolean;
  readonly temporaryImageTtlSeconds: number; readonly maxRetryCount: number; readonly deleteAfterSuccess: boolean;
  readonly deleteAfterExpiry: boolean; readonly retainRawImage: boolean; readonly policyUseEnabled: boolean;
  readonly changedByParentRef: string; readonly changedAt: string; readonly settingVersion: number; readonly reason: string | null;
}

interface ParentScreenEvidenceRemoteBoundarySetting {
  readonly schemaVersion: number; readonly parentSettingRef: string; readonly settingVersion: number;
  readonly rawScreenshotRetentionMode: string; readonly liveViewMode: string; readonly rawScreenshotRemoteUploadEnabled: boolean;
  readonly remoteSummaryMode: string; readonly remoteSummaryRedactedOnly: boolean; readonly parentApprovedRemoteSummary: boolean;
  readonly remoteSummaryApprovalRef: string | null; readonly remoteSummaryDestinationCustodyState: string;
  readonly changedByParentRef: string; readonly changedAt: string; readonly reason: string | null;
}

type ParentScreenEvidenceSettingsUiIntentKey =
  | 'disabledLocalSummary'
  | 'observeOnlyLocalSummary'
  | 'strictDryRunLocalSummary'
  | 'approvedRawRetentionLocalTtl';

interface ParentScreenEvidenceSettingsUiIntent {
  readonly intentKey: ParentScreenEvidenceSettingsUiIntentKey; readonly label: string; readonly detail: string;
  readonly setting: ParentScreenAnalysisParentSetting; readonly remoteBoundarySetting: ParentScreenEvidenceRemoteBoundarySetting;
}

export interface ParentScreenEvidenceSettingsUiProof {
  readonly title: string; readonly note: string; readonly intentLegend: string; readonly draftHeading: string;
  readonly draftTriggerHeading: string; readonly retentionHeading: string; readonly serviceCommandHeading: string;
  readonly serviceApplyActionLabel: string; readonly serviceRefreshActionLabel: string; readonly servicePendingStatus: string;
  readonly serviceAcceptedStatus: string; readonly serviceRejectedStatus: string; readonly serviceDisconnectedStatus: string;
  readonly serviceNoResponseStatus: string; readonly validationStatusLabel: string; readonly validationStatusValue: string;
  readonly defaultIntentKey: ParentScreenEvidenceSettingsUiIntentKey; readonly intents: readonly ParentScreenEvidenceSettingsUiIntent[];
}

export interface ParentScreenControlSettingsPortalMetric { readonly label: string; readonly value: string; readonly detail: string; }
export interface ParentScreenControlSettingsPortalGate {
  readonly label: string; readonly status: string; readonly statusText: string; readonly capabilityState: string;
  readonly runtimeOwner: string; readonly detail: string; readonly sourceDocument: string;
}

interface ParentScreenControlSettingsPortalProof {
  readonly title: string; readonly note: string; readonly metrics: readonly ParentScreenControlSettingsPortalMetric[];
  readonly gates: readonly ParentScreenControlSettingsPortalGate[];
}

interface ParentScreenOptionalVisibilityRawRetentionSetting {
  readonly sourceLabel: string; readonly custodyState: string; readonly retentionBehavior: string; readonly [key: string]: unknown;
}
interface ParentScreenOptionalVisibilityLiveViewSetting {
  readonly sourceLabel: string; readonly custodyState: string; readonly transportMode: string; readonly [key: string]: unknown;
}
interface ParentScreenOptionalVisibilityPermissionGate { readonly permissionEvidenceKind: string; readonly [key: string]: unknown; }

export interface ParentScreenOptionalVisibilityCapabilityStatus {
  readonly schemaVersion: number; readonly checkedAt: string; readonly capabilityKind: string; readonly parentSettingRef: string;
  readonly readinessState: string; readonly rawRetentionSetting: ParentScreenOptionalVisibilityRawRetentionSetting | null;
  readonly liveViewSetting: ParentScreenOptionalVisibilityLiveViewSetting | null;
  readonly liveViewPermissionGate: ParentScreenOptionalVisibilityPermissionGate | null;
  readonly runtimeProofRef: string | null; readonly deletionProofRef: string | null; readonly transportProofRef: string | null;
  readonly childDisclosureReady: boolean; readonly childDeviceCapabilityReady: boolean; readonly productModeReady: boolean;
  readonly rawFramesRetained: boolean; readonly rawRemoteUploadAllowed: boolean; readonly remoteInputAllowed: boolean; readonly reason: string;
}

interface ParentScreenOptionalVisibilityCapabilityProof {
  readonly schemaVersion: number; readonly generatedAt: string; readonly proofId: string;
  readonly rows: readonly ParentScreenOptionalVisibilityCapabilityStatus[]; readonly nonClaims: readonly string[];
}

export interface ParentScreenSettingsUpdateResponse {
  readonly schemaVersion: number; readonly requestId: string; readonly kind: ParentScreenSettingsUpdateKindValue;
  readonly status: ParentScreenSettingsUpdateStatus; readonly setting: ParentScreenAnalysisParentSetting | null;
  readonly auditEventId: string | null; readonly rejectionReason: string | null; readonly message: string | null;
}

const ParentScreenEvidenceSettingsWritableUiProofValue = {"defaultIntentKey":"disabledLocalSummary","draftHeading":"Draft mode","draftTriggerHeading":"Triggers and custody","intentLegend":"Intent","intents":[{"detail":"No cadence capture, trigger capture, strict mode, or policy use can run while disabled.","intentKey":"disabledLocalSummary","label":"Keep screen analysis disabled","remoteBoundarySetting":{"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-disabled","liveViewMode":"disabled","parentApprovedRemoteSummary":false,"parentSettingRef":"screen-settings-ui-parent-disabled","rawScreenshotRemoteUploadEnabled":false,"rawScreenshotRetentionMode":"disabled","reason":"local screen summary settings do not enable raw retention or live view","remoteSummaryApprovalRef":null,"remoteSummaryDestinationCustodyState":"unavailable","remoteSummaryMode":"disabled","remoteSummaryRedactedOnly":true,"schemaVersion":1,"settingVersion":1},"setting":{"allowedCaptureScope":"unsupported","analysisMode":"observeOnly","cadenceCaptureEnabled":false,"cadenceSeconds":300,"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-disabled","credentialSuppressionEnabled":true,"deleteAfterExpiry":true,"deleteAfterSuccess":true,"enabledTriggers":[],"maxRetryCount":0,"ocrTextEnabled":false,"ocrTextRetentionMode":"disabled","ocrTextSnippetLimit":0,"piiRedactionEnabled":false,"policyUseEnabled":false,"reason":"parent kept local screen summaries disabled","redactionMode":"disabled","retainRawImage":false,"schemaVersion":1,"screenAnalysisEnabled":false,"settingVersion":1,"strictModeEnabled":false,"temporaryImageTtlSeconds":300,"triggerCaptureEnabled":false}},{"detail":"Five-minute local summaries can be reviewed by the parent, but policy handoff remains disabled.","intentKey":"observeOnlyLocalSummary","label":"Enable observe-only summaries","remoteBoundarySetting":{"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-observe","liveViewMode":"disabled","parentApprovedRemoteSummary":false,"parentSettingRef":"screen-settings-ui-parent-observe","rawScreenshotRemoteUploadEnabled":false,"rawScreenshotRetentionMode":"disabled","reason":"local screen summary settings do not enable raw retention or live view","remoteSummaryApprovalRef":null,"remoteSummaryDestinationCustodyState":"unavailable","remoteSummaryMode":"disabled","remoteSummaryRedactedOnly":true,"schemaVersion":1,"settingVersion":2},"setting":{"allowedCaptureScope":"activeWindow","analysisMode":"observeOnly","cadenceCaptureEnabled":true,"cadenceSeconds":300,"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-observe","credentialSuppressionEnabled":true,"deleteAfterExpiry":true,"deleteAfterSuccess":true,"enabledTriggers":["foregroundAppChange","policyAmbiguity"],"maxRetryCount":2,"ocrTextEnabled":true,"ocrTextRetentionMode":"redactedSnippets","ocrTextSnippetLimit":3,"piiRedactionEnabled":true,"policyUseEnabled":false,"reason":"parent enabled observe-only local screen summaries","redactionMode":"localSensitiveText","retainRawImage":false,"schemaVersion":1,"screenAnalysisEnabled":true,"settingVersion":2,"strictModeEnabled":false,"temporaryImageTtlSeconds":300,"triggerCaptureEnabled":true}},{"detail":"One-minute cadence, selected triggers, local OCR, redaction, and policy dry-run become explicit parent intent.","intentKey":"strictDryRunLocalSummary","label":"Enable strict dry-run review","remoteBoundarySetting":{"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-strict","liveViewMode":"disabled","parentApprovedRemoteSummary":false,"parentSettingRef":"screen-settings-ui-parent-strict","rawScreenshotRemoteUploadEnabled":false,"rawScreenshotRetentionMode":"disabled","reason":"local screen summary settings do not enable raw retention or live view","remoteSummaryApprovalRef":null,"remoteSummaryDestinationCustodyState":"unavailable","remoteSummaryMode":"disabled","remoteSummaryRedactedOnly":true,"schemaVersion":1,"settingVersion":3},"setting":{"allowedCaptureScope":"activeWindow","analysisMode":"policyDryRun","cadenceCaptureEnabled":true,"cadenceSeconds":60,"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-strict","credentialSuppressionEnabled":true,"deleteAfterExpiry":true,"deleteAfterSuccess":true,"enabledTriggers":["foregroundAppChange","managedBrowserUrlChange","appGameForegroundStart","policyAmbiguity"],"maxRetryCount":2,"ocrTextEnabled":true,"ocrTextRetentionMode":"redactedSnippets","ocrTextSnippetLimit":5,"piiRedactionEnabled":true,"policyUseEnabled":true,"reason":"parent enabled strict local screen summary dry run","redactionMode":"localSensitiveText","retainRawImage":false,"schemaVersion":1,"screenAnalysisEnabled":true,"settingVersion":3,"strictModeEnabled":true,"temporaryImageTtlSeconds":300,"triggerCaptureEnabled":true}},{"detail":"Parent-approved local raw screenshot retention uses a short TTL and keeps delete-after-success and delete-after-expiry required.","intentKey":"approvedRawRetentionLocalTtl","label":"Approve local short-TTL retention","remoteBoundarySetting":{"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-raw-retention-local-ttl","liveViewMode":"disabled","parentApprovedRemoteSummary":false,"parentSettingRef":"screen-settings-ui-parent-raw-retention-local-ttl","rawScreenshotRemoteUploadEnabled":false,"rawScreenshotRetentionMode":"parentApprovedLocalShortTtl","reason":"parent approved local short TTL raw screenshot retention without raw remote upload","remoteSummaryApprovalRef":null,"remoteSummaryDestinationCustodyState":"unavailable","remoteSummaryMode":"disabled","remoteSummaryRedactedOnly":true,"schemaVersion":1,"settingVersion":4},"setting":{"allowedCaptureScope":"activeWindow","analysisMode":"policyDryRun","cadenceCaptureEnabled":true,"cadenceSeconds":60,"changedAt":"2026-06-04T23:50:00Z","changedByParentRef":"screen-settings-ui-parent-raw-retention-local-ttl","credentialSuppressionEnabled":true,"deleteAfterExpiry":true,"deleteAfterSuccess":true,"enabledTriggers":["foregroundAppChange","managedBrowserUrlChange","appGameForegroundStart","policyAmbiguity"],"maxRetryCount":2,"ocrTextEnabled":true,"ocrTextRetentionMode":"redactedSnippets","ocrTextSnippetLimit":5,"piiRedactionEnabled":true,"policyUseEnabled":true,"reason":"parent approved local short TTL raw screenshot retention","redactionMode":"localSensitiveText","retainRawImage":true,"schemaVersion":1,"screenAnalysisEnabled":true,"settingVersion":4,"strictModeEnabled":true,"temporaryImageTtlSeconds":120,"triggerCaptureEnabled":true}}],"note":"Parent Settings can build a schema-valid local screen-summary intent and submit it to the child service command path.","retentionHeading":"Remote boundary","serviceAcceptedStatus":"service accepted persisted setting","serviceApplyActionLabel":"Save selected screen setting","serviceCommandHeading":"Service command","serviceDisconnectedStatus":"service command unavailable while disconnected","serviceNoResponseStatus":"no service settings response yet","servicePendingStatus":"waiting for service response","serviceRefreshActionLabel":"Refresh persisted screen setting","serviceRejectedStatus":"service rejected setting","title":"Writable screen settings proof","validationStatusLabel":"Parser status","validationStatusValue":"schema-valid local parent intent"} as const satisfies ParentScreenEvidenceSettingsUiProof;
const ParentScreenControlSettingsPortalProofValue = {"gates":[{"capabilityState":"unavailable","detail":"Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.","label":"Allow Ocentra-hosted processing of child screen images?","runtimeOwner":"parent-owned-storage","sourceDocument":"docs/screen-evidence-analysis-schema-proposal.md","status":"unavailable","statusText":"unavailable / unavailable"},{"capabilityState":"unavailable","detail":"Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.","label":"Show raw screenshots in parent reports by default?","runtimeOwner":"portal-only","sourceDocument":"docs/screen-evidence-analysis-schema-proposal.md","status":"unavailable","statusText":"unavailable / unavailable"},{"capabilityState":"unavailable","detail":"Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.","label":"Retain raw screenshots or recordings?","runtimeOwner":"parent-owned-storage","sourceDocument":"docs/screen-evidence-analysis-schema-proposal.md","status":"unavailable","statusText":"unavailable / unavailable"},{"capabilityState":"available","detail":"Require validated summary, evidence refs, deletion proof, and deterministic policy before policy use.","label":"Allow screen summaries to be used by policy?","runtimeOwner":"os-adapter","sourceDocument":"docs/screen-evidence-analysis-schema-proposal.md","status":"proof-required","statusText":"proof-required / available"},{"capabilityState":"available","detail":"Portal renders authored intent; child agent owns capture gating, queue, analysis, compile, and audit.","label":"Use local OCR/vision returns schema-valid output;?","runtimeOwner":"local-ai-runtime","sourceDocument":"docs/screen-evidence-analysis-capability-guide.md","status":"needs-effect-wiring","statusText":"needs-effect-wiring / available"}],"metrics":[{"detail":"Screen settings parsed from the current capability guide and schema proposal.","label":"Catalog settings","value":"474"},{"detail":"Parent-facing Screen categories available for read-only rendering.","label":"Catalog tabs","value":"11"},{"detail":"Strict behavior requires platform capture, local analysis, deletion, or policy proof before use.","label":"Proof-required controls","value":"68"},{"detail":"Raw retention, hosted processing, hidden capture, continuous recording, and unsupported sensitive states fail closed.","label":"Unavailable sensitive modes","value":"9"}],"note":"Read-only Settings proof from the Screen control catalog; child runtime owns capture, queue, local analysis, policy handoff, and audit.","title":"Screen settings and capability proof"} as const satisfies ParentScreenControlSettingsPortalProof;
const ParentScreenOptionalVisibilityCapabilityStatusProofValue = {"generatedAt":"2026-06-07T05:55:00Z","nonClaims":["This proof proves raw screenshot retention readiness only after explicit parent approval, runtime proof, deletion proof, child disclosure readiness, and child device readiness.","This proof does not enable live-view transport, relay, cache, or remote input.","This proof does not satisfy privacy/legal approval or physical platform live-view prompt screenshots."],"proofId":"screen-optional-visibility-capability-status-proof","rows":[{"capabilityKind":"rawScreenshotRetention","checkedAt":"2026-06-07T05:55:00Z","childDeviceCapabilityReady":false,"childDisclosureReady":false,"deletionProofRef":null,"liveViewPermissionGate":null,"liveViewSetting":null,"parentSettingRef":"screen-parent-retention-capability-disabled","productModeReady":false,"rawFramesRetained":false,"rawRemoteUploadAllowed":false,"rawRetentionSetting":{"approvalRef":null,"auditRef":null,"changedAt":"2026-06-07T05:55:00Z","custodyState":"unavailable","deleteAfterTtl":false,"deleteOnParentDisable":true,"deleteProofRequired":false,"disclosureState":"notRequired","explicitParentApproval":false,"exportRef":null,"mode":"disabled","parentSettingRef":"screen-parent-retention-capability-disabled","rawScreenshotRemoteUploadEnabled":false,"reason":"raw screenshot retention is disabled by default","retentionBehavior":"noRawRetention","schemaVersion":1,"settingId":"screen-retention-capability-disabled","settingVersion":1,"sourceLabel":"unavailable","ttlSeconds":null},"readinessState":"disabled","reason":"raw screenshot retention is disabled by default","remoteInputAllowed":false,"runtimeProofRef":null,"schemaVersion":1,"transportProofRef":null},{"capabilityKind":"rawScreenshotRetention","checkedAt":"2026-06-07T05:55:00Z","childDeviceCapabilityReady":false,"childDisclosureReady":false,"deletionProofRef":null,"liveViewPermissionGate":null,"liveViewSetting":null,"parentSettingRef":"screen-parent-retention-capability-local-ttl","productModeReady":false,"rawFramesRetained":false,"rawRemoteUploadAllowed":false,"rawRetentionSetting":{"approvalRef":"screen-retention-capability-approval","auditRef":"screen-retention-capability-audit","changedAt":"2026-06-07T05:55:00Z","custodyState":"child-device-temp-queue","deleteAfterTtl":true,"deleteOnParentDisable":true,"deleteProofRequired":true,"disclosureState":"requiredShown","explicitParentApproval":true,"exportRef":null,"mode":"localShortTtl","parentSettingRef":"screen-parent-retention-capability-local-ttl","rawScreenshotRemoteUploadEnabled":false,"reason":"parent approved local short TTL raw screenshot retention","retentionBehavior":"deleteAfterTtl","schemaVersion":1,"settingId":"screen-retention-capability-local-ttl","settingVersion":1,"sourceLabel":"rawScreenshotRetention","ttlSeconds":300},"readinessState":"manualRequired","reason":"raw screenshot retention needs runtime and deletion proof before product readiness","remoteInputAllowed":false,"runtimeProofRef":null,"schemaVersion":1,"transportProofRef":null},{"capabilityKind":"rawScreenshotRetention","checkedAt":"2026-06-07T05:55:00Z","childDeviceCapabilityReady":true,"childDisclosureReady":true,"deletionProofRef":"output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json","liveViewPermissionGate":null,"liveViewSetting":null,"parentSettingRef":"screen-parent-retention-capability-local-ttl-runtime","productModeReady":true,"rawFramesRetained":false,"rawRemoteUploadAllowed":false,"rawRetentionSetting":{"approvalRef":"screen-retention-runtime-approval","auditRef":"screen-retention-runtime-audit","changedAt":"2026-06-07T05:55:00Z","custodyState":"child-device-temp-queue","deleteAfterTtl":true,"deleteOnParentDisable":true,"deleteProofRequired":true,"disclosureState":"requiredShown","explicitParentApproval":true,"exportRef":null,"mode":"localShortTtl","parentSettingRef":"screen-parent-retention-capability-local-ttl-runtime","rawScreenshotRemoteUploadEnabled":false,"reason":"parent approved local short TTL raw screenshot retention with runtime and deletion proof","retentionBehavior":"deleteAfterTtl","schemaVersion":1,"settingId":"screen-retention-capability-local-ttl-runtime","settingVersion":2,"sourceLabel":"rawScreenshotRetention","ttlSeconds":120},"readinessState":"ready","reason":"raw screenshot retention is ready only with parent approval, runtime proof, deletion proof, child disclosure, and child device readiness","remoteInputAllowed":false,"runtimeProofRef":"output/screen-plan-proof/screen-settings-service-command/proof-summary.json","schemaVersion":1,"transportProofRef":null},{"capabilityKind":"liveView","checkedAt":"2026-06-07T05:55:00Z","childDeviceCapabilityReady":false,"childDisclosureReady":false,"deletionProofRef":null,"liveViewPermissionGate":null,"liveViewSetting":{"approvalRef":null,"cacheRawFrames":false,"changedAt":"2026-06-07T05:55:00Z","custodyState":"unavailable","disclosureState":"notRequired","explicitParentApproval":false,"frameRetentionBehavior":"noFrameRetention","liveViewMode":"disabled","parentSettingRef":"screen-parent-live-capability-disabled","platformProofRef":null,"platformProofState":"notRequired","reason":"live view is disabled by default","remoteInputControlAllowed":false,"schemaVersion":1,"sessionRecordingAllowed":false,"settingId":"screen-live-capability-disabled","settingVersion":1,"sourceLabel":"unavailable","stopOrRevokeAuditRequired":true,"transportMode":"none","viewerAuditRef":null},"parentSettingRef":"screen-parent-live-capability-disabled","productModeReady":false,"rawFramesRetained":false,"rawRemoteUploadAllowed":false,"rawRetentionSetting":null,"readinessState":"disabled","reason":"live view is disabled by default","remoteInputAllowed":false,"runtimeProofRef":null,"schemaVersion":1,"transportProofRef":null},{"capabilityKind":"liveView","checkedAt":"2026-06-07T05:55:00Z","childDeviceCapabilityReady":false,"childDisclosureReady":false,"deletionProofRef":null,"liveViewPermissionGate":{"cacheRawFrames":false,"checkedAt":"2026-06-07T05:55:00Z","custodyState":"live-lan-child-agent","explicitViewerDisclosure":true,"frameRetentionBehavior":"noFrameRetention","liveTransportProofRef":null,"liveViewMode":"lanOnlyView","permissionEvidenceKind":"screen-capture-only","platform":"android-mediaprojection","platformProofRef":"screen-live-capability-capture-only-proof","platformProofState":"operatorVerified","productLiveViewReady":false,"reason":"capture-only permission cannot satisfy live-view readiness","remoteInputControlAllowed":false,"schemaVersion":1,"sessionRecordingAllowed":false,"sourceLabel":"liveView","transportMode":"lanMutualAuth","viewerAuditRef":"screen-live-capability-audit"},"liveViewSetting":{"approvalRef":"screen-live-capability-approval","cacheRawFrames":false,"changedAt":"2026-06-07T05:55:00Z","custodyState":"live-lan-child-agent","disclosureState":"requiredShown","explicitParentApproval":true,"frameRetentionBehavior":"noFrameRetention","liveViewMode":"lanOnlyView","parentSettingRef":"screen-parent-live-capability-lan","platformProofRef":"screen-live-capability-platform-proof","platformProofState":"operatorVerified","reason":"parent approved LAN live view but capture-only evidence is insufficient","remoteInputControlAllowed":false,"schemaVersion":1,"sessionRecordingAllowed":false,"settingId":"screen-live-capability-lan","settingVersion":1,"sourceLabel":"liveView","stopOrRevokeAuditRequired":true,"transportMode":"lanMutualAuth","viewerAuditRef":"screen-live-capability-audit"},"parentSettingRef":"screen-parent-live-capability-lan","productModeReady":false,"rawFramesRetained":false,"rawRemoteUploadAllowed":false,"rawRetentionSetting":null,"readinessState":"blocked","reason":"live view remains blocked until live-view permission and transport proof are present","remoteInputAllowed":false,"runtimeProofRef":null,"schemaVersion":1,"transportProofRef":null}],"schemaVersion":1} as const satisfies ParentScreenOptionalVisibilityCapabilityProof;

export const ParentScreenOptionalVisibilityCapabilityProofGeneratedAt = '2026-06-07T05:55:00Z' as const;

export function parentScreenEvidenceSettingsWritableUiProof(): ParentScreenEvidenceSettingsUiProof {
  return ParentScreenEvidenceSettingsWritableUiProofValue;
}

export function parentScreenControlSettingsPortalProof(): ParentScreenControlSettingsPortalProof {
  return ParentScreenControlSettingsPortalProofValue;
}

export function parentScreenOptionalVisibilityCapabilityStatusProof(
  generatedAt: typeof ParentScreenOptionalVisibilityCapabilityProofGeneratedAt
): ParentScreenOptionalVisibilityCapabilityProof {
  if (generatedAt !== ParentScreenOptionalVisibilityCapabilityProofGeneratedAt) {
    throw new TypeError('generatedAt must match the Rust-owned screen optional visibility proof timestamp');
  }
  return ParentScreenOptionalVisibilityCapabilityStatusProofValue;
}

export function decodeParentScreenSettingsUpdateResponse(value: unknown): ParentScreenSettingsUpdateResponse | null {
  return isParentScreenSettingsUpdateResponse(value) ? value : null;
}

function isParentScreenSettingsUpdateResponse(value: unknown): value is ParentScreenSettingsUpdateResponse {
  if (!isParentScreenRecord(value)) {
    return false;
  }
  return value['schemaVersion'] === ParentScreenSettingsCommandRuntime.SchemaVersion &&
    isParentScreenNonEmptyString(value['requestId']) &&
    isParentScreenSettingsUpdateKind(value['kind']) &&
    isParentScreenSettingsUpdateStatus(value['status']) &&
    isParentScreenAnalysisParentSettingOrNull(value['setting']) &&
    isParentScreenNullableString(value['auditEventId']) &&
    isParentScreenNullableString(value['rejectionReason']) &&
    isParentScreenNullableString(value['message']);
}

function isParentScreenAnalysisParentSettingOrNull(value: unknown): value is ParentScreenAnalysisParentSetting | null {
  return value === null || isParentScreenAnalysisParentSetting(value);
}

function isParentScreenAnalysisParentSetting(value: unknown): value is ParentScreenAnalysisParentSetting {
  if (!isParentScreenRecord(value)) {
    return false;
  }
  return value['schemaVersion'] === ParentScreenSettingsCommandRuntime.SchemaVersion &&
    typeof value['screenAnalysisEnabled'] === 'boolean' && typeof value['analysisMode'] === 'string' &&
    typeof value['cadenceCaptureEnabled'] === 'boolean' && typeof value['cadenceSeconds'] === 'number' &&
    typeof value['strictModeEnabled'] === 'boolean' && typeof value['triggerCaptureEnabled'] === 'boolean' &&
    Array.isArray(value['enabledTriggers']) && value['enabledTriggers'].every((trigger) => typeof trigger === 'string') &&
    typeof value['allowedCaptureScope'] === 'string' && typeof value['ocrTextEnabled'] === 'boolean' &&
    typeof value['ocrTextSnippetLimit'] === 'number' && typeof value['redactionMode'] === 'string' &&
    typeof value['ocrTextRetentionMode'] === 'string' && typeof value['credentialSuppressionEnabled'] === 'boolean' &&
    typeof value['piiRedactionEnabled'] === 'boolean' && typeof value['temporaryImageTtlSeconds'] === 'number' &&
    typeof value['maxRetryCount'] === 'number' && typeof value['deleteAfterSuccess'] === 'boolean' &&
    typeof value['deleteAfterExpiry'] === 'boolean' && typeof value['retainRawImage'] === 'boolean' &&
    typeof value['policyUseEnabled'] === 'boolean' && isParentScreenNonEmptyString(value['changedByParentRef']) &&
    isParentScreenNonEmptyString(value['changedAt']) && typeof value['settingVersion'] === 'number' &&
    isParentScreenNullableString(value['reason']);
}

function isParentScreenSettingsUpdateKind(value: unknown): value is ParentScreenSettingsUpdateKindValue {
  return value === ParentScreenSettingsUpdateKind.Get || value === ParentScreenSettingsUpdateKind.Replace;
}

function isParentScreenSettingsUpdateStatus(value: unknown): value is ParentScreenSettingsUpdateStatus {
  return value === ParentScreenSettingsUpdateStatus.Accepted || value === ParentScreenSettingsUpdateStatus.Rejected;
}

function isParentScreenNullableString(value: unknown): value is string | null {
  return value === null || typeof value === 'string';
}

function isParentScreenNonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function isParentScreenRecord(value: unknown): value is ParentUnknownRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
