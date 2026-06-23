import {
  AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema,
  type AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow as SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffMatchesParentSurfaceIntent,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-handoff-rules';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema;
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowIdSchema;
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema;
const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema;
const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema;
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema;

type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow;
type AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff =
  SchemaDomainAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff(
  optionsInput: unknown,
  parentSurfaceIntentInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptionsSchema.parse(optionsInput);
  const intent =
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.parse(parentSurfaceIntentInput);
  const rows = intent.rows.map((row) => buildServiceReadinessRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceParentSurfaceIntentId: intent.intentId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: intent.nativeAppRowCount,
    nativeGameRowCount: intent.nativeGameRowCount,
    serviceReadApiProofRequiredCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceReadinessHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision
    ).length,
    serviceReadinessHandoffNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
  });
}

function buildServiceReadinessRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
  intentRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow {
  const serviceReadinessHandoffState = serviceReadinessStateForParentSurfaceIntent(intentRow);
  const serviceReadApiProofRequired =
    serviceReadinessHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.ServiceReadApiProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${intentRow.rowId}:service-readiness-handoff`,
    sourceParentSurfaceIntentRowId: intentRow.rowId,
    sourceAuditRollbackReadModelRowId: intentRow.sourceAuditRollbackReadModelRowId,
    sourceAuditRollbackHandoffRowId: intentRow.sourceAuditRollbackHandoffRowId,
    sourceSchedulerPersistenceRowId: intentRow.sourceSchedulerPersistenceRowId,
    targetDomain: intentRow.targetDomain,
    serviceReadinessHandoffState,
    parentSurfaceProofRequired: intentRow.parentSurfaceProofRequired,
    serviceReadinessProofRequired: serviceReadApiProofRequired,
    serviceReadApiProofRequired,
    requiredProofRefs: serviceReadApiProofRequired
      ? [...intentRow.requiredProofRefs, options.serviceReadinessProofRef, options.serviceReadApiProofRef]
      : intentRow.requiredProofRefs,
    sourceEvidenceRefs: intentRow.sourceEvidenceRefs,
    serviceReadApiRef: options.serviceReadApiRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function serviceReadinessStateForParentSurfaceIntent(
  intentRow: AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffMatchesParentSurfaceIntent(
        intentRow.parentSurfaceIntentState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState.BlockedByCompilerDecision;
}
export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffState };
