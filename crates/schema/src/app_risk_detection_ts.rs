use super::app_risk_detection::{
    sample_app_risk_detection_matrix, APP_RISK_DETECTION_SCHEMA_VERSION,
};

const APP_RISK_DETECTION_MATRIX_JSON_EXPECT_MESSAGE: &str = "app risk detection matrix json";
const APP_RISK_DETECTION_INDENT_UNIT: &str = " ";
const APP_RISK_DETECTION_JSON_KEY_VALUE_SEPARATOR: &str = "\": ";
const APP_RISK_DETECTION_TYPESCRIPT_LINE_BREAK: &str = "\n";
const APP_RISK_DETECTION_JSON_DOUBLE_QUOTE: char = '"';
const APP_RISK_DETECTION_TYPESCRIPT_SINGLE_QUOTE: char = '\'';

pub fn app_risk_detection_contracts_typescript() -> String {
    let matrix_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_app_risk_detection_matrix()),
        APP_RISK_DETECTION_MATRIX_JSON_EXPECT_MESSAGE,
    );
    let matrix_typescript = json_object_to_typescript_literal(&matrix_json);

    format!(
        r#"/* generated from crates/schema/src/app_risk_detection.rs */

export const AppRiskDetectionContractRuntime = {{
  SchemaVersion: '{schema_version}',
}} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentEvidenceReferenceKind = 'activity-event';

export const GeneratedAppRiskDetectionRiskSignal = {{
  VpnProxy: 'vpnProxy',
  RemoteDesktop: 'remoteDesktop',
  DownloadTorrent: 'downloadTorrent',
  InstallerUpdater: 'installerUpdater',
  AiChatbot: 'aiChatbot',
  SocialVideoMessaging: 'socialVideoMessaging',
  UnknownRisk: 'unknownRisk',
}} as const;
export type GeneratedAppRiskDetectionRiskSignal =
  (typeof GeneratedAppRiskDetectionRiskSignal)[keyof typeof GeneratedAppRiskDetectionRiskSignal];

export const GeneratedAppRiskDetectionSourceKind = {{
  KnownCatalog: 'knownCatalog',
  ExecutableName: 'executableName',
  PublisherMetadata: 'publisherMetadata',
  ExecutableHash: 'executableHash',
  LocalAiDigest: 'localAiDigest',
  ParentOverride: 'parentOverride',
}} as const;
export type GeneratedAppRiskDetectionSourceKind =
  (typeof GeneratedAppRiskDetectionSourceKind)[keyof typeof GeneratedAppRiskDetectionSourceKind];

export const GeneratedAppRiskDetectionCandidateState = {{
  CatalogMatch: 'catalogMatch',
  HeuristicCandidate: 'heuristicCandidate',
  AiCandidate: 'aiCandidate',
  ParentReviewCandidate: 'parentReviewCandidate',
  ParentDisplayOverride: 'parentDisplayOverride',
}} as const;
export type GeneratedAppRiskDetectionCandidateState =
  (typeof GeneratedAppRiskDetectionCandidateState)[keyof typeof GeneratedAppRiskDetectionCandidateState];

export const GeneratedAppRiskDetectionPublisherTrustState = {{
  KnownPublisher: 'knownPublisher',
  UnknownPublisher: 'unknownPublisher',
  MissingPublisher: 'missingPublisher',
  UnverifiedPublisher: 'unverifiedPublisher',
  ParentTrusted: 'parentTrusted',
}} as const;
export type GeneratedAppRiskDetectionPublisherTrustState =
  (typeof GeneratedAppRiskDetectionPublisherTrustState)[keyof typeof GeneratedAppRiskDetectionPublisherTrustState];

export const GeneratedAppRiskDetectionPolicyCandidateAction = {{
  None: 'none',
  Observe: 'observe',
  Warn: 'warn',
  AskParent: 'askParent',
  ManualReview: 'manualReview',
}} as const;
export type GeneratedAppRiskDetectionPolicyCandidateAction =
  (typeof GeneratedAppRiskDetectionPolicyCandidateAction)[keyof typeof GeneratedAppRiskDetectionPolicyCandidateAction];

export const GeneratedAppRiskDetectionConfidenceBand = {{
  High: 'high',
  Medium: 'medium',
  Low: 'low',
  Review: 'review',
}} as const;
export type GeneratedAppRiskDetectionConfidenceBand =
  (typeof GeneratedAppRiskDetectionConfidenceBand)[keyof typeof GeneratedAppRiskDetectionConfidenceBand];

export const GeneratedAppRiskDetectionPolicyTargetKind = {{
  RiskApp: 'risk-app',
}} as const;
export type GeneratedAppRiskDetectionPolicyTargetKind =
  (typeof GeneratedAppRiskDetectionPolicyTargetKind)[keyof typeof GeneratedAppRiskDetectionPolicyTargetKind];

export const GeneratedAppRiskDetectionAskParentRouting = {{
  Available: 'available',
  ManualReview: 'manual-review',
  NotRouted: 'not-routed',
}} as const;
export type GeneratedAppRiskDetectionAskParentRouting =
  (typeof GeneratedAppRiskDetectionAskParentRouting)[keyof typeof GeneratedAppRiskDetectionAskParentRouting];

export const GeneratedAppRiskDetectionSurfaceState = {{
  RiskDisclosureReady: 'riskdisclosure-ready',
}} as const;
export type GeneratedAppRiskDetectionSurfaceState =
  (typeof GeneratedAppRiskDetectionSurfaceState)[keyof typeof GeneratedAppRiskDetectionSurfaceState];

export const GeneratedAppRiskDetectionNoContentClaimState = {{
  NoContentCaptured: 'no-content-captured',
}} as const;
export type GeneratedAppRiskDetectionNoContentClaimState =
  (typeof GeneratedAppRiskDetectionNoContentClaimState)[keyof typeof GeneratedAppRiskDetectionNoContentClaimState];

export const GeneratedAppRiskDetectionRiskSignalValues = [
  'vpnProxy',
  'remoteDesktop',
  'downloadTorrent',
  'installerUpdater',
  'aiChatbot',
  'socialVideoMessaging',
  'unknownRisk',
] as const satisfies readonly GeneratedAppRiskDetectionRiskSignal[];

export const GeneratedAppRiskDetectionSourceKindValues = [
  'knownCatalog',
  'executableName',
  'publisherMetadata',
  'executableHash',
  'localAiDigest',
  'parentOverride',
] as const satisfies readonly GeneratedAppRiskDetectionSourceKind[];

export const GeneratedAppRiskDetectionCandidateStateValues = [
  'catalogMatch',
  'heuristicCandidate',
  'aiCandidate',
  'parentReviewCandidate',
  'parentDisplayOverride',
] as const satisfies readonly GeneratedAppRiskDetectionCandidateState[];

export const GeneratedAppRiskDetectionPublisherTrustStateValues = [
  'knownPublisher',
  'unknownPublisher',
  'missingPublisher',
  'unverifiedPublisher',
  'parentTrusted',
] as const satisfies readonly GeneratedAppRiskDetectionPublisherTrustState[];

export const GeneratedAppRiskDetectionPolicyCandidateActionValues = [
  'none',
  'observe',
  'warn',
  'askParent',
  'manualReview',
] as const satisfies readonly GeneratedAppRiskDetectionPolicyCandidateAction[];

export const GeneratedAppRiskDetectionConfidenceBandValues = [
  'high',
  'medium',
  'low',
  'review',
] as const satisfies readonly GeneratedAppRiskDetectionConfidenceBand[];

export const GeneratedAppRiskDetectionPolicyTargetKindValues = [
  'risk-app',
] as const satisfies readonly GeneratedAppRiskDetectionPolicyTargetKind[];

export const GeneratedAppRiskDetectionAskParentRoutingValues = [
  'available',
  'manual-review',
  'not-routed',
] as const satisfies readonly GeneratedAppRiskDetectionAskParentRouting[];

export const GeneratedAppRiskDetectionSurfaceStateValues = [
  'riskdisclosure-ready',
] as const satisfies readonly GeneratedAppRiskDetectionSurfaceState[];

export const GeneratedAppRiskDetectionNoContentClaimStateValues = [
  'no-content-captured',
] as const satisfies readonly GeneratedAppRiskDetectionNoContentClaimState[];

export interface GeneratedParentEvidenceReference {{
  evidenceReferenceId: string;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: string;
}}

export interface GeneratedAppRiskDetectionParentOverride {{
  parentDisplayLabel: string;
  policyCandidateAction: GeneratedAppRiskDetectionPolicyCandidateAction;
  rawIdentityChanged: boolean;
}}

export interface GeneratedAppRiskDetectionSurfaceDisclosure {{
  surfaceState: GeneratedAppRiskDetectionSurfaceState;
  confidencePercent: number;
  sourceEvidenceCount: number;
  noContentClaimState: GeneratedAppRiskDetectionNoContentClaimState;
}}

export interface GeneratedAppRiskDetectionCandidate {{
  schemaVersion: GeneratedParentContractSchemaVersion;
  candidateId: string;
  platform: GeneratedParentPlatform;
  inventoryEntryRef: string | null;
  identityRef: string | null;
  riskSignal: GeneratedAppRiskDetectionRiskSignal;
  sourceKind: GeneratedAppRiskDetectionSourceKind;
  candidateState: GeneratedAppRiskDetectionCandidateState;
  publisherTrustState: GeneratedAppRiskDetectionPublisherTrustState;
  confidence: number;
  confidenceBand: GeneratedAppRiskDetectionConfidenceBand;
  evidenceReferences: readonly GeneratedParentEvidenceReference[];
  sourceRefs: readonly string[];
  localAiDigestRef: string | null;
  parentOverride: GeneratedAppRiskDetectionParentOverride | null;
  policyCandidateAction: GeneratedAppRiskDetectionPolicyCandidateAction;
  policyTargetKind: GeneratedAppRiskDetectionPolicyTargetKind;
  askParentRouting: GeneratedAppRiskDetectionAskParentRouting;
  notDirectEnforcement: boolean;
  noContentClaim: boolean;
  surfaceDisclosure: GeneratedAppRiskDetectionSurfaceDisclosure;
  lastCheckedAt: string;
}}

export interface GeneratedAppRiskDetectionMatrix {{
  schemaVersion: GeneratedParentContractSchemaVersion;
  matrixId: string;
  generatedAt: string;
  candidates: readonly GeneratedAppRiskDetectionCandidate[];
}}

export const GeneratedAppRiskDetectionMatrix = {matrix_json} as const satisfies GeneratedAppRiskDetectionMatrix;
"#,
        schema_version = APP_RISK_DETECTION_SCHEMA_VERSION,
        matrix_json = matrix_typescript,
    )
}

pub fn app_risk_detection_contract_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/app_risk_detection.rs */

import {
  GeneratedAppRiskDetectionCandidateState,
  GeneratedAppRiskDetectionNoContentClaimState,
  GeneratedAppRiskDetectionPolicyCandidateAction,
  GeneratedAppRiskDetectionSourceKind,
  type GeneratedAppRiskDetectionCandidate,
} from './app-riskdetection-contracts';

export function appRiskDetectionCandidateIsHonestGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return (
    appRiskDetectionCandidateCitesEvidenceGenerated(candidate) &&
    appRiskDetectionCandidateStateMatchesSourceGenerated(candidate) &&
    appRiskDetectionUnknownPublisherLowersConfidenceGenerated(candidate) &&
    appRiskDetectionAiCandidateCitesDigestGenerated(candidate) &&
    appRiskDetectionParentOverrideIsDisplayOnlyGenerated(candidate) &&
    appRiskDetectionCandidateCannotDirectlyEnforceGenerated(candidate) &&
    appRiskDetectionSurfaceDisclosureMatchesEvidenceGenerated(candidate)
  );
}

function appRiskDetectionCandidateCitesEvidenceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return candidate.evidenceReferences.length > 0 && candidate.sourceRefs.length > 0;
}

function appRiskDetectionCandidateStateMatchesSourceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  switch (candidate.sourceKind) {
    case GeneratedAppRiskDetectionSourceKind.KnownCatalog:
      return (
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.CatalogMatch &&
        candidate.confidence >= 0.7
      );
    case GeneratedAppRiskDetectionSourceKind.ExecutableName:
    case GeneratedAppRiskDetectionSourceKind.PublisherMetadata:
    case GeneratedAppRiskDetectionSourceKind.ExecutableHash:
      return (
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.HeuristicCandidate ||
        candidate.candidateState === GeneratedAppRiskDetectionCandidateState.ParentReviewCandidate
      );
    case GeneratedAppRiskDetectionSourceKind.LocalAiDigest:
      return candidate.candidateState === GeneratedAppRiskDetectionCandidateState.AiCandidate;
    case GeneratedAppRiskDetectionSourceKind.ParentOverride:
      return candidate.candidateState === GeneratedAppRiskDetectionCandidateState.ParentDisplayOverride;
  }
}

function appRiskDetectionUnknownPublisherLowersConfidenceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  if (
    candidate.publisherTrustState === 'knownPublisher' ||
    candidate.publisherTrustState === 'parentTrusted'
  ) {
    return true;
  }

  return candidate.confidence <= 0.5;
}

function appRiskDetectionAiCandidateCitesDigestGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  if (candidate.sourceKind !== GeneratedAppRiskDetectionSourceKind.LocalAiDigest) {
    return true;
  }

  return (
    candidate.localAiDigestRef !== null &&
    candidate.policyCandidateAction !== GeneratedAppRiskDetectionPolicyCandidateAction.None
  );
}

function appRiskDetectionParentOverrideIsDisplayOnlyGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  if (candidate.sourceKind !== GeneratedAppRiskDetectionSourceKind.ParentOverride) {
    return true;
  }

  return candidate.parentOverride !== null && candidate.parentOverride.rawIdentityChanged === false;
}

function appRiskDetectionCandidateCannotDirectlyEnforceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return candidate.notDirectEnforcement && candidate.noContentClaim;
}

function appRiskDetectionSurfaceDisclosureMatchesEvidenceGenerated(
  candidate: GeneratedAppRiskDetectionCandidate
): boolean {
  return (
    candidate.surfaceDisclosure.sourceEvidenceCount === candidate.evidenceReferences.length &&
    candidate.surfaceDisclosure.confidencePercent === Math.round(candidate.confidence * 100) &&
    candidate.surfaceDisclosure.noContentClaimState ===
      GeneratedAppRiskDetectionNoContentClaimState.NoContentCaptured
  );
}
"#
    .to_owned()
}

fn json_object_to_typescript_literal(json: &str) -> String {
    json.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            if let Some(key_and_rest) = trimmed.strip_prefix(APP_RISK_DETECTION_JSON_DOUBLE_QUOTE) {
                if let Some((key, rest)) =
                    key_and_rest.split_once(APP_RISK_DETECTION_JSON_KEY_VALUE_SEPARATOR)
                {
                    let indent = APP_RISK_DETECTION_INDENT_UNIT.repeat(line.len() - trimmed.len());
                    return format!("{indent}{key}: {rest}");
                }
            }
            line.to_owned()
        })
        .collect::<Vec<_>>()
        .join(APP_RISK_DETECTION_TYPESCRIPT_LINE_BREAK)
        .chars()
        .map(|ch| {
            if ch == APP_RISK_DETECTION_JSON_DOUBLE_QUOTE {
                APP_RISK_DETECTION_TYPESCRIPT_SINGLE_QUOTE
            } else {
                ch
            }
        })
        .collect()
}
