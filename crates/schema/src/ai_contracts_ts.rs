use super::ai_contracts::AI_CONTRACT_SCHEMA_VERSION;

const AI_CONTRACTS_TYPESCRIPT: &str = r#"/**
 * Generated edge declarations for the Rust-owned AI contract family.
 * Runtime authority remains in crates/schema.
 */
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

export type AiDegradedState =
  | "none"
  | "missing-evidence"
  | "invalid-output"
  | "timeout"
  | "model-unavailable"
  | "provider-unavailable"
  | "custody-unavailable"
  | "manual-required";

export interface AiSchemaIdentity {
  schemaVersion: string;
  family: string;
  requestId: string;
  subject: {
    familyId: string;
    childProfileId?: string;
    deviceId?: string;
  };
}

export interface AiEvidenceReference {
  evidenceReferenceId: string;
  evidenceKind: string;
  custody: AiCustodyState;
  retention: string;
  redaction: string;
  validation: string;
}

export interface AiWorkItem {
  request: unknown;
  state: string;
  attempt: number;
  durability: string;
  validation: string;
  degradedState: AiDegradedState;
}

export interface AiResult {
  schemaVersion: string;
  resultId: string;
  requestId: string;
  workItemId: string;
  validation: string;
  degradedState: AiDegradedState;
  authorityBoundary: AiAuthorityBoundary;
}

export interface AiRemoteAssistantRequest {
  schemaVersion: string;
  requestId: string;
  sourceBundle: unknown;
  state: string;
}
"#;

pub fn ai_contracts_typescript() -> String {
    let mut output = String::from("// Rust schema version: ");
    output.push_str(AI_CONTRACT_SCHEMA_VERSION);
    output.push('\n');
    output.push_str(AI_CONTRACTS_TYPESCRIPT);
    output
}
