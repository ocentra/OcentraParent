import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameSourceFreshnessPolicyReadinessIdSchema as SchemaDomainAppGameSourceFreshnessPolicyReadinessIdSchema,
  AppGameSourceFreshnessPolicyReadinessSchema as SchemaDomainAppGameSourceFreshnessPolicyReadinessSchema,
  AppGameSourceFreshnessPolicyRequestSchema as SchemaDomainAppGameSourceFreshnessPolicyRequestSchema,
  AppGameSourceFreshnessRequirementResultSchema as SchemaDomainAppGameSourceFreshnessRequirementResultSchema,
  type AppGameSourceFreshnessPolicyReadiness as SchemaDomainAppGameSourceFreshnessPolicyReadiness,
  type AppGameSourceFreshnessPolicyRequest as SchemaDomainAppGameSourceFreshnessPolicyRequest,
  type AppGameSourceFreshnessRequirementResult as SchemaDomainAppGameSourceFreshnessRequirementResult,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import {
  appGameSourceFreshnessRequirementFailure,
  appGameSourceFreshnessRowsForRequirement,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption-rules';
import {
  AppGameSourceFreshnessAdapterDispatchState,
  AppGameSourceFreshnessCapabilityStatus,
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessReadModelState,
  AppGameSourceFreshnessReasonCode,
  AppGameSourceFreshnessRequirementState,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const AppGameSourceFreshnessPolicyReadinessIdSchema = SchemaDomainAppGameSourceFreshnessPolicyReadinessIdSchema;
const AppGameSourceFreshnessPolicyRequestSchema = SchemaDomainAppGameSourceFreshnessPolicyRequestSchema;
const AppGameSourceFreshnessRequirementResultSchema = SchemaDomainAppGameSourceFreshnessRequirementResultSchema;
const AppGameSourceFreshnessPolicyReadinessSchema = SchemaDomainAppGameSourceFreshnessPolicyReadinessSchema;

type AppGameSourceFreshnessPolicyRequest = SchemaDomainAppGameSourceFreshnessPolicyRequest;
type AppGameSourceFreshnessRequirementResult = SchemaDomainAppGameSourceFreshnessRequirementResult;
type AppGameSourceFreshnessPolicyReadiness = SchemaDomainAppGameSourceFreshnessPolicyReadiness;

type AppGameSourceFreshnessRequirementKindValue = AppGameSourceFreshnessPolicyRequest['requiredSources'][number];
type ParentTimestampValue = AppGameSourceFreshnessPolicyReadiness['evaluatedAt'];

const decodeAppGameSourceFreshnessPolicyReadinessId = Schema.decodeUnknownSync(
  AppGameSourceFreshnessPolicyReadinessIdSchema
);
const decodeParentTimestamp = Schema.decodeUnknownSync(ParentTimestampSchema);

export function evaluateAppGameSourceFreshnessPolicyReadiness(
  requestInput: unknown,
  readinessIdInput: unknown,
  evaluatedAtInput: unknown
): AppGameSourceFreshnessPolicyReadiness {
  const request = AppGameSourceFreshnessPolicyRequestSchema.parse(requestInput);
  const readinessId = decodeAppGameSourceFreshnessPolicyReadinessId(readinessIdInput);
  const evaluatedAt = decodeParentTimestamp(evaluatedAtInput);
  const requirementResults = request.requiredSources.map((requirementKind) =>
    evaluateRequirement(request, requirementKind, evaluatedAt)
  );
  const policyEvidenceRefs = [...new Set(requirementResults.flatMap((result) => result.sourceEvidenceRefs))];
  const allSatisfied = requirementResults.every(
    (result) => result.requirementState === AppGameSourceFreshnessRequirementState.Satisfied
  );

  return AppGameSourceFreshnessPolicyReadinessSchema.parse({
    schemaVersion: request.schemaVersion,
    readinessId,
    request,
    readinessState: allSatisfied
      ? AppGameSourceFreshnessPolicyReadinessState.PolicyReady
      : AppGameSourceFreshnessPolicyReadinessState.ManualRequired,
    requirementResults,
    policyEvidenceRefs,
    policyCompileAllowed: allSatisfied,
    adapterDispatchState: AppGameSourceFreshnessAdapterDispatchState.NotDispatched,
    directAdapterCallRequested: false,
    rawPrivateSourceRowsIncluded: false,
    evaluatedAt,
  });
}

function evaluateRequirement(
  request: AppGameSourceFreshnessPolicyRequest,
  requirementKind: AppGameSourceFreshnessRequirementKindValue,
  evaluatedAt: ParentTimestampValue
): AppGameSourceFreshnessRequirementResult {
  const rows = appGameSourceFreshnessRowsForRequirement(request.sourceStatusRows, requirementKind);

  if (rows.length === 0) {
    return AppGameSourceFreshnessRequirementResultSchema.parse({
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementState.Missing,
      reasonCode: AppGameSourceFreshnessReasonCode.MissingSourceStatusRow,
      matchedSourceKinds: [],
      sourceEvidenceRefs: [],
      lastObservedAt: null,
    });
  }

  const freshRow = rows.find(
    (row) =>
      row.state === AppGameSourceFreshnessReadModelState.Ready &&
      row.capabilityStatus === AppGameSourceFreshnessCapabilityStatus.Available &&
      row.rowCount > 0 &&
      row.evidence.length > 0 &&
      appGameSourceFreshnessRequirementFailure(row, evaluatedAt, request.maxSourceAgeMs) === null
  );

  if (freshRow !== undefined) {
    return AppGameSourceFreshnessRequirementResultSchema.parse({
      requirementKind,
      requirementState: AppGameSourceFreshnessRequirementState.Satisfied,
      reasonCode: null,
      matchedSourceKinds: [freshRow.sourceKind],
      sourceEvidenceRefs: freshRow.evidence,
      lastObservedAt: freshRow.lastObservedAt,
    });
  }

  const firstRow = rows[0];

  if (firstRow === undefined) {
    throw new Error('Expected app/game source freshness rows after empty-row guard');
  }

  const firstFailure = appGameSourceFreshnessRequirementFailure(firstRow, evaluatedAt, request.maxSourceAgeMs) ?? {
    requirementState: AppGameSourceFreshnessRequirementState.MissingEvidence,
    reasonCode: AppGameSourceFreshnessReasonCode.MissingSourceEvidence,
  };

  return AppGameSourceFreshnessRequirementResultSchema.parse({
    requirementKind,
    requirementState: firstFailure.requirementState,
    reasonCode: firstFailure.reasonCode,
    matchedSourceKinds: rows.map((row) => row.sourceKind),
    sourceEvidenceRefs: rows.flatMap((row) => row.evidence),
    lastObservedAt: firstRow.lastObservedAt,
  });
}
