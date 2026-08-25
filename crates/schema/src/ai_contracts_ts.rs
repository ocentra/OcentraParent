use super::ai_contracts::AI_CONTRACT_SCHEMA_VERSION;

const AI_CONTRACT_SCHEMA_VERSION_PLACEHOLDER: &str = "__AI_CONTRACT_SCHEMA_VERSION__";

const AI_CONTRACTS_TYPESCRIPT: &str = r#"/**
 * Generated from crates/schema/src/ai_contracts/*.rs.
 * Rust constructors and deserializers are authoritative; these declarations
 * describe the serialized edge shape only.
 */
export const AiContractSchemaVersion = "__AI_CONTRACT_SCHEMA_VERSION__" as const;
export type AiContractSchemaVersion = typeof AiContractSchemaVersion;

type AiIdentifier<Name extends string> = string & { readonly __aiIdentifier: Name };

export type AiSchemaVersion = AiIdentifier<"AiSchemaVersion">;
export type AiFamilyId = AiIdentifier<"AiFamilyId">;
export type AiChildProfileId = AiIdentifier<"AiChildProfileId">;
export type AiDeviceId = AiIdentifier<"AiDeviceId">;
export type AiActorId = AiIdentifier<"AiActorId">;
export type AiSourceId = AiIdentifier<"AiSourceId">;
export type AiAdapterId = AiIdentifier<"AiAdapterId">;
export type AiRequestId = AiIdentifier<"AiRequestId">;
export type AiWorkItemId = AiIdentifier<"AiWorkItemId">;
export type AiResultId = AiIdentifier<"AiResultId">;
export type AiEvidenceReferenceId = AiIdentifier<"AiEvidenceReferenceId">;
export type AiPolicyReferenceId = AiIdentifier<"AiPolicyReferenceId">;
export type AiRuleId = AiIdentifier<"AiRuleId">;
export type AiMemoryReferenceId = AiIdentifier<"AiMemoryReferenceId">;
export type AiGraphReferenceId = AiIdentifier<"AiGraphReferenceId">;
export type AiGraphNodeId = AiIdentifier<"AiGraphNodeId">;
export type AiJournalEntryId = AiIdentifier<"AiJournalEntryId">;
export type AiJournalStreamId = AiIdentifier<"AiJournalStreamId">;
export type AiExplanationId = AiIdentifier<"AiExplanationId">;
export type AiPromptTemplateId = AiIdentifier<"AiPromptTemplateId">;
export type AiPromptVersion = AiIdentifier<"AiPromptVersion">;
export type AiRuntimeReferenceId = AiIdentifier<"AiRuntimeReferenceId">;
export type AiProviderId = AiIdentifier<"AiProviderId">;
export type AiModelId = AiIdentifier<"AiModelId">;
export type AiCapabilityId = AiIdentifier<"AiCapabilityId">;
export type AiAuthorizationReferenceId = AiIdentifier<"AiAuthorizationReferenceId">;
export type AiRemoteAssistantRequestId = AiIdentifier<"AiRemoteAssistantRequestId">;
export type AiRemoteAssistantResultId = AiIdentifier<"AiRemoteAssistantResultId">;
export type AiTimestamp = AiIdentifier<"AiTimestamp">;
export type AiDigest = AiIdentifier<"AiDigest">;

export type AiAuthorityBoundary =
  | "evidence-only"
  | "deterministic-policy-required"
  | "manual-review-required";
export type AiCustodyState =
  | "child-local-encrypted"
  | "parent-local-encrypted"
  | "parent-authorized-redacted"
  | "ephemeral-local"
  | "deleted"
  | "unavailable";
export type AiRetentionState = "active" | "expired" | "tombstoned" | "deleted" | "manual-required";
export type AiRedactionState =
  | "not-applicable"
  | "redacted"
  | "fully-redacted"
  | "rejected-private-payload";
export type AiDegradedState =
  | "none"
  | "missing-evidence"
  | "invalid-output"
  | "timeout"
  | "model-unavailable"
  | "provider-unavailable"
  | "custody-unavailable"
  | "manual-required";
export type AiDurabilityState =
  | "durable"
  | "append-pending"
  | "replay-only"
  | "not-durable"
  | "manual-required";
export type AiValidationState = "accepted" | "rejected" | "manual-required";

export type AiEvidenceKind =
  | "browser"
  | "app"
  | "game"
  | "network"
  | "screen-summary"
  | "activity"
  | "parent-rule"
  | "audit";
export type AiProvenanceKind =
  | "direct-observation"
  | "derived-from-evidence"
  | "derived-from-result"
  | "parent-authored-rule";
export type AiContextBuildState = "ready" | "partial" | "rejected" | "manual-required";
export type AiReferenceValidationState =
  | "validated"
  | "missing-source"
  | "custody-blocked"
  | "stale"
  | "rejected";

export type AiMemoryReferenceKind =
  | "recent-activity"
  | "evidence-memory"
  | "semantic-memory"
  | "policy-memory";
export type AiGraphNodeKind = "evidence" | "activity" | "result" | "memory" | "policy-rule";
export type AiGraphEdgeKind = "supports" | "derived-from" | "related-to" | "governed-by";

export type AiJournalEntryKind =
  | "work-lifecycle"
  | "context-built"
  | "result-validated"
  | "explanation-published"
  | "remote-assistant";
export type AiJournalPayloadKind =
  | "work-item"
  | "evidence-context"
  | "result"
  | "explanation"
  | "remote-assistant";

export type AiResultKind = "observation" | "classification" | "summary" | "explanation" | "no-claim";
export type AiOutputValidationState =
  | "schema-valid"
  | "schema-invalid"
  | "evidence-missing"
  | "confidence-invalid"
  | "policy-handoff-required"
  | "manual-required";
export type AiExplanationSurface = "parent-read-model" | "child-safety-internal" | "audit-record";
export type AiExplanationState = "ready" | "degraded" | "unavailable" | "manual-required";

export type AiActorRole =
  | "parent"
  | "child-agent"
  | "local-runtime"
  | "parent-assistant"
  | "remote-assistant"
  | "system";
export type AiWorkKind =
  | "context-build"
  | "classification"
  | "explanation"
  | "memory-derivation"
  | "graph-derivation"
  | "parent-assistant"
  | "remote-assistant";
export type AiWorkState =
  | "queued"
  | "claimed"
  | "running"
  | "succeeded"
  | "failed"
  | "cancelled"
  | "timed-out"
  | "manual-required";

export type AiRemoteAssistantState =
  | "disabled"
  | "awaiting-parent-authorization"
  | "authorized"
  | "submitted"
  | "succeeded"
  | "degraded"
  | "manual-required";
export type AiRemoteAssistantSafetyBoundary = "parent-report-only" | "outside-child-safety-blocking-path";
export type AiRemoteAssistantRedactionPolicy =
  | "references-only"
  | "redacted-summaries"
  | "no-child-payload";

export type AiText = string & { readonly __aiRawText: "local-only" };
export type AiUntrustedText = string & { readonly __aiUntrustedText: "wire-only" };
export interface AiSafeText {
  text: AiText;
  redaction: Extract<AiRedactionState, "redacted" | "fully-redacted">;
}

export interface AiSubjectIdentity {
  familyId: AiFamilyId;
  childProfileId: AiChildProfileId | null;
  deviceId: AiDeviceId | null;
}
export interface AiActorIdentity {
  actorId: AiActorId;
  role: AiActorRole;
  subject: AiSubjectIdentity | null;
}
export interface AiSchemaIdentity {
  schemaVersion: AiSchemaVersion;
  family: AiFamilyId;
  requestId: AiRequestId;
  subject: AiSubjectIdentity;
}

export interface AiEvidenceProvenance {
  provenanceKind: AiProvenanceKind;
  familyId: AiFamilyId;
  sourceId: AiSourceId;
  adapterId: AiAdapterId;
  sourceSchemaVersion: AiSchemaVersion;
  observedAt: AiTimestamp;
  ingestedAt: AiTimestamp | null;
  sourceEvidenceReferenceId: AiEvidenceReferenceId | null;
  sourceResultId: AiResultId | null;
  sourceRuleId: AiRuleId | null;
}
export interface AiEvidenceReference {
  evidenceReferenceId: AiEvidenceReferenceId;
  familyId: AiFamilyId;
  evidenceKind: AiEvidenceKind;
  provenance: AiEvidenceProvenance;
  custody: AiCustodyState;
  retention: AiRetentionState;
  redaction: AiRedactionState;
  confidence: number | null;
  validation: AiReferenceValidationState;
}
export interface AiRuleReference {
  policyReferenceId: AiPolicyReferenceId;
  familyId: AiFamilyId;
  ruleId: AiRuleId;
  ruleVersion: AiSchemaVersion;
  sourceEvidenceReferenceId: AiEvidenceReferenceId;
}
export interface AiPromptReference {
  templateId: AiPromptTemplateId;
  version: AiPromptVersion;
  task: AiSafeText;
}
export interface AiRuntimeReference {
  runtimeReferenceId: AiRuntimeReferenceId;
  providerId: AiProviderId;
  modelId: AiModelId;
  capabilityIds: AiCapabilityId[];
  observedAt: AiTimestamp;
}
export interface AiEvidenceContextRequest {
  identity: AiSchemaIdentity;
  requestedEvaluation: AiSafeText;
  requestedAt: AiTimestamp;
  requiredEvidence: AiEvidenceKind[];
  allowedCustody: AiCustodyState[];
  parentRules: AiRuleReference[];
  prompt: AiPromptReference;
  runtime: AiRuntimeReference | null;
}
export interface AiEvidenceContext {
  schemaVersion: AiSchemaVersion;
  requestId: AiRequestId;
  familyId: AiFamilyId;
  childProfileId: AiChildProfileId | null;
  deviceId: AiDeviceId | null;
  evidence: AiEvidenceReference[];
  parentRules: AiRuleReference[];
  memory: AiMemoryReference[];
  graph: AiGraphReference[];
  prompt: AiPromptReference;
  runtime: AiRuntimeReference | null;
  custody: AiCustodyState[];
  authorityBoundary: AiAuthorityBoundary;
  degradedState: AiDegradedState;
}
export interface AiEvidenceContextBuildResult {
  requestId: AiRequestId;
  state: AiContextBuildState;
  validation: AiValidationState;
  context: AiEvidenceContext | null;
  rejectedReferences: AiEvidenceReferenceId[];
  missingEvidence: AiEvidenceKind[];
  degradedState: AiDegradedState;
}

export interface AiProvenanceLink {
  familyId: AiFamilyId;
  sourceEvidenceReferenceIds: AiEvidenceReferenceId[];
  sourceResultId: AiResultId | null;
  sourceDigest: AiDigest | null;
}
export interface AiMemoryReference {
  memoryReferenceId: AiMemoryReferenceId;
  familyId: AiFamilyId;
  kind: AiMemoryReferenceKind;
  provenance: AiProvenanceLink;
  generatedAt: AiTimestamp;
  expiresAt: AiTimestamp | null;
  confidence: number;
  custody: AiCustodyState;
  retention: AiRetentionState;
}
export interface AiGraphReference {
  graphReferenceId: AiGraphReferenceId;
  familyId: AiFamilyId;
  nodeKind: AiGraphNodeKind;
  targetNodeId: AiGraphNodeId;
  edgeKind: AiGraphEdgeKind;
  sourceMemoryReferenceId: AiMemoryReferenceId | null;
  sourceEvidenceReferenceIds: AiEvidenceReferenceId[];
  sourceResultId: AiResultId | null;
  generatedAt: AiTimestamp;
  expiresAt: AiTimestamp | null;
  custody: AiCustodyState;
  retention: AiRetentionState;
}

export interface AiJournalPayloadReference {
  payloadKind: AiJournalPayloadKind;
  requestId: AiRequestId;
  workItemId: AiWorkItemId | null;
  resultId: AiResultId | null;
  explanationId: AiExplanationId | null;
}
export interface AiJournalEntry {
  journalEntryId: AiJournalEntryId;
  streamId: AiJournalStreamId;
  sequence: number;
  kind: AiJournalEntryKind;
  payload: AiJournalPayloadReference;
  custody: AiCustodyState;
  retention: AiRetentionState;
  redaction: AiRedactionState;
  durability: AiDurabilityState;
  occurredAt: AiTimestamp;
  digest: AiDigest;
}
export interface AiJournalCursor {
  streamId: AiJournalStreamId;
  afterSequence: number;
  afterEntryId: AiJournalEntryId | null;
  durable: AiDurabilityState;
}
export interface AiJournalAppendResult {
  entry: AiJournalEntry;
  accepted: boolean;
  nextSequence: number;
  durability: AiDurabilityState;
}

export interface AiClaim {
  claimId: AiResultId;
  resultKind: AiResultKind;
  subject: AiSubjectIdentity;
  label: AiSafeText;
  confidence: number;
  evidenceReferenceIds: AiEvidenceReferenceId[];
  memoryReferenceIds: AiMemoryReferenceId[];
  graphReferenceIds: AiGraphReferenceId[];
  ruleReferenceIds: AiRuleId[];
  authorityBoundary: AiAuthorityBoundary;
}
export interface AiResultPayload {
  familyId: AiFamilyId;
  claims: AiClaim[];
  summary: AiSafeText | null;
  evidence: AiEvidenceReference[];
  memory: AiMemoryReference[];
  graph: AiGraphReference[];
  rules: AiRuleReference[];
  prompt: AiPromptReference;
  runtime: AiRuntimeReference | null;
}
export interface AiResult {
  schemaVersion: AiSchemaVersion;
  familyId: AiFamilyId;
  resultId: AiResultId;
  requestId: AiRequestId;
  workItemId: AiWorkItemId;
  generatedAt: AiTimestamp;
  validation: AiValidationState;
  outputValidation: AiOutputValidationState;
  degradedState: AiDegradedState;
  payload: AiResultPayload | null;
  explanationId: AiExplanationId | null;
  authorityBoundary: AiAuthorityBoundary;
}
export interface AiPolicyHandoff {
  resultId: AiResultId;
  requestId: AiRequestId;
  policyReferenceIds: AiPolicyReferenceId[];
  authorityBoundary: AiAuthorityBoundary;
}

export interface AiRetryPolicy {
  maxAttempts: number;
  retryAfterMs: number | null;
}
export interface AiWorkRequest {
  identity: AiSchemaIdentity;
  workItemId: AiWorkItemId;
  workKind: AiWorkKind;
  requestedAt: AiTimestamp;
  deadlineAt: AiTimestamp | null;
  retryPolicy: AiRetryPolicy;
  prompt: AiPromptReference | null;
  runtime: AiRuntimeReference | null;
}
export interface AiWorkItem {
  request: AiWorkRequest;
  state: AiWorkState;
  attempt: number;
  durability: AiDurabilityState;
  validation: AiValidationState;
  degradedState: AiDegradedState;
  lastTransitionSequence: number;
  lastTransitionAt: AiTimestamp;
  terminalReason: AiSafeText | null;
}
export interface AiWorkLifecycleRecord {
  workItemId: AiWorkItemId;
  requestId: AiRequestId;
  journalEntryId: AiJournalEntryId;
  sequence: number;
  previousState: AiWorkState | null;
  nextState: AiWorkState;
  actor: AiActorIdentity;
  occurredAt: AiTimestamp;
  durability: AiDurabilityState;
}
export interface AiDurableWorkLifecycle {
  workItemId: AiWorkItemId;
  requestId: AiRequestId;
  records: AiWorkLifecycleRecord[];
  lastSequence: number;
  durability: AiDurabilityState;
}

export interface AiExplanationCitation {
  evidenceReferenceIds: AiEvidenceReferenceId[];
  memoryReferenceIds: AiMemoryReferenceId[];
  graphReferenceIds: AiGraphReferenceId[];
  label: AiSafeText;
}
export interface AiExplanationSection {
  heading: AiSafeText;
  body: AiSafeText;
  citations: AiExplanationCitation[];
}
export interface AiExplanation {
  schemaVersion: AiSchemaVersion;
  familyId: AiFamilyId;
  explanationId: AiExplanationId;
  requestId: AiRequestId;
  resultId: AiResultId;
  surface: AiExplanationSurface;
  state: AiExplanationState;
  validation: AiValidationState;
  degradedState: AiDegradedState;
  sections: AiExplanationSection[];
  evidence: AiEvidenceReference[];
  memory: AiMemoryReference[];
  graph: AiGraphReference[];
  rules: AiRuleReference[];
  prompt: AiPromptReference;
  runtime: AiRuntimeReference | null;
  authorityBoundary: AiAuthorityBoundary;
  redaction: AiRedactionState;
  retention: AiRetentionState;
  generatedAt: AiTimestamp;
}

export interface AiParentAuthorization {
  authorizationReferenceId: AiAuthorizationReferenceId;
  actor: AiActorIdentity;
  authorizedAt: AiTimestamp;
  expiresAt: AiTimestamp;
}
export interface AiRemoteAssistantSourceBundle {
  familyId: AiFamilyId;
  evidenceReferenceIds: AiEvidenceReferenceId[];
  authorization: AiParentAuthorization;
  custody: AiCustodyState;
  retention: AiRetentionState;
  redaction: AiRedactionState;
  redactionPolicy: AiRemoteAssistantRedactionPolicy;
  safetyBoundary: AiRemoteAssistantSafetyBoundary;
}
export interface AiRemoteAssistantRequest {
  schemaVersion: AiSchemaVersion;
  requestId: AiRemoteAssistantRequestId;
  sourceBundle: AiRemoteAssistantSourceBundle;
  prompt: AiPromptReference;
  runtime: AiRuntimeReference | null;
  requestedAt: AiTimestamp;
  state: AiRemoteAssistantState;
}
export interface AiRemoteAssistantWirePrompt {
  templateId: AiPromptTemplateId;
  version: AiPromptVersion;
  task: AiUntrustedText;
}
export interface AiRemoteAssistantWireRequest {
  schemaVersion: AiSchemaVersion;
  requestId: AiRemoteAssistantRequestId;
  familyId: AiFamilyId;
  authorizationReferenceId: AiAuthorizationReferenceId;
  prompt: AiRemoteAssistantWirePrompt;
  requestedAt: AiTimestamp;
  state: AiRemoteAssistantState;
}
export interface AiRemoteAssistantResult {
  schemaVersion: AiSchemaVersion;
  resultId: AiRemoteAssistantResultId;
  requestId: AiRemoteAssistantRequestId;
  familyId: AiFamilyId;
  state: AiRemoteAssistantState;
  validation: AiValidationState;
  degradedState: AiDegradedState;
  answer: AiSafeText | null;
  citedEvidenceReferenceIds: AiEvidenceReferenceId[];
  safetyBoundary: AiRemoteAssistantSafetyBoundary;
  redaction: AiRedactionState;
  retention: AiRetentionState;
  returnedAt: AiTimestamp;
}
"#;

pub fn ai_contracts_typescript() -> String {
    let body = AI_CONTRACTS_TYPESCRIPT.replace(
        AI_CONTRACT_SCHEMA_VERSION_PLACEHOLDER,
        AI_CONTRACT_SCHEMA_VERSION,
    );
    let mut output = String::from("// Rust schema version: ");
    output.push_str(AI_CONTRACT_SCHEMA_VERSION);
    output.push('\n');
    output.push_str(&body);
    output
}
