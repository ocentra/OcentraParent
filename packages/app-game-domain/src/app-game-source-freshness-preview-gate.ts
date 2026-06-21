import {
  AppGamePolicyCompiledDecisionSchema,
  type AppGamePolicyCompiledDecision,
} from '@ocentra-parent/schema-domain/app-game-policy-target-compiler';
import {
  AppGameSourceFreshnessPreviewGateOptionsSchema as SchemaDomainAppGameSourceFreshnessPreviewGateOptionsSchema,
  AppGameSourceFreshnessPreviewGateReadModelSchema as SchemaDomainAppGameSourceFreshnessPreviewGateReadModelSchema,
  AppGameSourceFreshnessPreviewGateRowSchema as SchemaDomainAppGameSourceFreshnessPreviewGateRowSchema,
  AppGameSourceFreshnessPreviewGateEntrySchema as SchemaDomainAppGameSourceFreshnessPreviewGateEntrySchema,
  type AppGameSourceFreshnessPreviewGateReadModel as SchemaDomainAppGameSourceFreshnessPreviewGateReadModel,
  type AppGameSourceFreshnessPreviewGateRow as SchemaDomainAppGameSourceFreshnessPreviewGateRow,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-preview-gate';
import { type AppGameSourceFreshnessPolicyReadiness } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import {
  type AppGamePolicyPreviewHandoffOptions,
  buildAppGamePolicyPreviewHandoffRow,
} from './app-game-policy-preview-handoff';
import {
  AppGamePolicyPreviewTargetDomain,
  appGamePolicyPreviewTargetDomainForKind,
} from './app-game-policy-preview-handoff-rules';
import {
  AppGameSourceFreshnessPolicyReadinessState,
  AppGameSourceFreshnessPolicyTargetKind,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption-values';
import {
  AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
  AppGameSourceFreshnessPreviewGateState,
  AppGameSourceFreshnessPreviewGateStatus,
  countAppGameSourceFreshnessPreviewGateReadModelRows,
} from '@ocentra-parent/schema-domain/app-game-source-freshness-preview-gate-rules';
const AppGameSourceFreshnessPreviewGateOptionsSchema =
  SchemaDomainAppGameSourceFreshnessPreviewGateOptionsSchema;
const AppGameSourceFreshnessPreviewGateEntrySchema =
  SchemaDomainAppGameSourceFreshnessPreviewGateEntrySchema;
const AppGameSourceFreshnessPreviewGateRowSchema = SchemaDomainAppGameSourceFreshnessPreviewGateRowSchema;
const AppGameSourceFreshnessPreviewGateReadModelSchema =
  SchemaDomainAppGameSourceFreshnessPreviewGateReadModelSchema;

type AppGameSourceFreshnessPreviewGateRow = SchemaDomainAppGameSourceFreshnessPreviewGateRow;
type AppGameSourceFreshnessPreviewGateReadModel = SchemaDomainAppGameSourceFreshnessPreviewGateReadModel;

export function buildAppGameSourceFreshnessPreviewGateReadModel(
  optionsInput: unknown,
  entriesInput: readonly unknown[]
): AppGameSourceFreshnessPreviewGateReadModel {
  const options = AppGameSourceFreshnessPreviewGateOptionsSchema.parse(optionsInput);
  const rows = entriesInput.map((entry) => buildAppGameSourceFreshnessPreviewGateRow(options, entry));
  return AppGameSourceFreshnessPreviewGateReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    gateId: options.gateId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    ...countAppGameSourceFreshnessPreviewGateReadModelRows(rows),
    ...AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
  });
}

export function buildAppGameSourceFreshnessPreviewGateRow(
  optionsInput: unknown,
  entryInput: unknown
): AppGameSourceFreshnessPreviewGateRow {
  const options = AppGameSourceFreshnessPreviewGateOptionsSchema.parse(optionsInput);
  const entry = AppGameSourceFreshnessPreviewGateEntrySchema.parse(entryInput);
  const previewRow = buildPreviewRow(options.policyPreviewOptions, entry.sourceReadiness, entry.compiledDecision);
  const targetDomain = sourceFreshnessTargetDomain(entry.sourceReadiness);
  return AppGameSourceFreshnessPreviewGateRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: entry.rowId,
    targetDomain,
    sourceReadinessId: entry.sourceReadiness.readinessId,
    sourcePolicyRequestId: entry.sourceReadiness.request.policyRequestId,
    sourceReadinessState: entry.sourceReadiness.readinessState,
    sourcePolicyCompileAllowed: entry.sourceReadiness.policyCompileAllowed,
    sourceRequirementStates: entry.sourceReadiness.requirementResults.map((result) => result.requirementState),
    sourceReasonCodes: entry.sourceReadiness.requirementResults.map((result) => result.reasonCode),
    sourceEvidenceRefs: entry.sourceReadiness.policyEvidenceRefs,
    compiledDecisionProvided: entry.compiledDecision !== null,
    previewStatus:
      previewRow === null ? AppGameSourceFreshnessPreviewGateStatus.ManualRequired : previewRow.previewStatus,
    gateState: previewGateState(entry.sourceReadiness, previewRow),
    previewRow,
    ...AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function buildPreviewRow(
  options: AppGamePolicyPreviewHandoffOptions,
  sourceReadiness: AppGameSourceFreshnessPolicyReadiness,
  compiledDecision: AppGamePolicyCompiledDecision | null
) {
  if (sourceReadiness.readinessState === AppGameSourceFreshnessPolicyReadinessState.ManualRequired) {
    return null;
  }

  if (compiledDecision === null) {
    throw new Error('Expected policy-ready source freshness rows to include a compiled preview decision');
  }

  const previewRow = buildAppGamePolicyPreviewHandoffRow(options, compiledDecision);
  if (previewRow.targetDomain !== sourceFreshnessTargetDomain(sourceReadiness)) {
    throw new Error('Expected source freshness target domain to match compiled policy preview target domain');
  }
  return previewRow;
}

function previewGateState(
  sourceReadiness: AppGameSourceFreshnessPolicyReadiness,
  previewRow: ReturnType<typeof buildAppGamePolicyPreviewHandoffRow> | null
) {
  if (sourceReadiness.readinessState === AppGameSourceFreshnessPolicyReadinessState.ManualRequired) {
    return AppGameSourceFreshnessPreviewGateState.SourceManualRequired;
  }

  return previewRow?.previewStatus === AppGameSourceFreshnessPreviewGateStatus.PreviewReady
    ? AppGameSourceFreshnessPreviewGateState.SourceFresh
    : AppGameSourceFreshnessPreviewGateState.CompilerManualRequired;
}

function sourceFreshnessTargetDomain(readiness: AppGameSourceFreshnessPolicyReadiness) {
  switch (readiness.request.target.targetKind) {
    case AppGameSourceFreshnessPolicyTargetKind.NativeApp:
      return AppGamePolicyPreviewTargetDomain.NativeApp;
    case AppGameSourceFreshnessPolicyTargetKind.NativeGame:
      return AppGamePolicyPreviewTargetDomain.NativeGame;
    case AppGameSourceFreshnessPolicyTargetKind.AllNativeApps:
      return AppGamePolicyPreviewTargetDomain.NativeApp;
    case AppGameSourceFreshnessPolicyTargetKind.AllNativeGames:
      return AppGamePolicyPreviewTargetDomain.NativeGame;
  }
}

export const appGameSourceFreshnessPreviewGateCompiledDecisionDomain = (decisionInput: unknown) => {
  const decision = AppGamePolicyCompiledDecisionSchema.parse(decisionInput);
  return appGamePolicyPreviewTargetDomainForKind(decision.request.target.targetKind);
};

