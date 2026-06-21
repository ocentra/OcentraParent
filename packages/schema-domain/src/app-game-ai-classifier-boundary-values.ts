export const AppGameAiClassifierSchemaVersion = 1;

export const AppGameAiClassifierProductKind = {
  NativeApp: "nativeApp",
  NativeGame: "nativeGame",
  UnknownApp: "unknownApp",
  UnknownGame: "unknownGame",
} as const;

export const AppGameAiClassifierSourceDigestKind = {
  InventoryEvidence: "inventoryEvidence",
  RuntimeEvidence: "runtimeEvidence",
  ForegroundEvidence: "foregroundEvidence",
  LauncherEvidence: "launcherEvidence",
  SessionSummary: "sessionSummary",
  CategoryDigest: "categoryDigest",
  UnknownApprovalDigest: "unknownApprovalDigest",
} as const;

export const AppGameAiClassifierCandidateKind = {
  CategoryCandidate: "categoryCandidate",
  SafetyCandidate: "safetyCandidate",
  GameContextCandidate: "gameContextCandidate",
  UnknownIdentityCandidate: "unknownIdentityCandidate",
  PolicyEvidenceCandidate: "policyEvidenceCandidate",
} as const;

export const AppGameAiClassifierState = {
  Candidate: "candidate",
  LowConfidenceCandidate: "lowConfidenceCandidate",
  DegradedCandidate: "degradedCandidate",
  ProviderUnavailable: "providerUnavailable",
} as const;

export const AppGameAiClassifierPolicyHandoff = {
  None: "none",
  ParentReview: "parentReview",
  PolicyPreviewOnly: "policyPreviewOnly",
  ManualReview: "manualReview",
  AskParentPreview: "askParentPreview",
} as const;

export const AppGameAiClassifierFallbackState = {
  NotNeeded: "notNeeded",
  LocalModelUnavailable: "localModelUnavailable",
  PromptRejected: "promptRejected",
  ModelOutputInvalid: "modelOutputInvalid",
  EvidenceMissing: "evidenceMissing",
  LowConfidence: "lowConfidence",
} as const;

export const AppGameAiClassifierForbiddenOutputKeys = [
  "adapterAction",
  "block",
  "directAction",
  "durationMs",
  "enforcementAction",
  "fileScanRows",
  "foregroundDurationMs",
  "hide",
  "processScanRows",
  "rawOsScanResult",
  "runningDurationMs",
  "shield",
  "suspend",
  "terminate",
] as const;

export const AppGameAiClassifierForbiddenPolicyValues = ["block", "hide", "shield", "suspend", "terminate"] as const;
