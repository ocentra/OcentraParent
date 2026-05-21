import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiEvidenceContextBuildResultSchema,
  LocalAiEvidenceContextSummarySchema,
  LocalAiRejectedFieldSchema,
  LocalAiStoredEvidenceContextBuildInputSchema,
  type LocalAiContextBuildState,
  type LocalAiContextReasonCode,
  type LocalAiEvidenceContext,
  type LocalAiEvidenceContextBuildResult,
  type LocalAiEvidenceContextKind,
  type LocalAiEvidenceContextRefId,
  type LocalAiEvidenceContextSummary,
  type LocalAiEvidenceContextSourceRef,
  type LocalAiEvidenceContextValidationSummary,
  type LocalAiEvidenceCustody,
  type LocalAiRejectedField,
  type LocalAiStoredEvidenceContextBuildInput,
} from './local-ai-context';

const decodeRejectedField = Schema.decodeUnknownSync(LocalAiRejectedFieldSchema);
const decodeContextSummary = Schema.decodeUnknownSync(LocalAiEvidenceContextSummarySchema);

const EvidenceReferencesField = decodeRejectedField('evidenceReferences');
const HostedCustodySummary = decodeContextSummary(
  'ocentra-hosted non-activity metadata cannot source child-activity evidence'
);
const HostedCustodyGate = decodeContextSummary('rejected forbidden custody before local model input');
const UnallowedCustodySummary = decodeContextSummary('evidence custody was not allowed by context request');
const UnallowedCustodyGate = decodeContextSummary('rejected unallowed custody before local model input');
const NoEvidenceSummary = decodeContextSummary('no child-activity evidence selected for context');
const InsufficientEvidenceGate = decodeContextSummary('insufficient stored evidence for local model input');
const StoredEvidenceSummary = decodeContextSummary('context uses only child-device or parent-owned stored evidence');
const PartialContextGate = decodeContextSummary('partial context built with explicit missing evidence kinds');
const ReadyContextGate = decodeContextSummary('ready stored-evidence context for local-only model input');

function refIdsForKind(
  evidenceReferences: readonly LocalAiEvidenceContextSourceRef[],
  evidenceKind: LocalAiEvidenceContextKind
): LocalAiEvidenceContextRefId[] {
  return evidenceReferences
    .filter((reference) => reference.evidenceKind === evidenceKind)
    .map((reference) => reference.evidenceRefId);
}

function uniqueCustodyLabels(evidenceReferences: readonly LocalAiEvidenceContextSourceRef[]): LocalAiEvidenceCustody[] {
  return [...new Set(evidenceReferences.map((reference) => reference.custody))];
}

function uniqueReasonCodes(
  evidenceReferences: readonly LocalAiEvidenceContextSourceRef[],
  reasonKey: 'degradedReasons' | 'unknownReasons'
): LocalAiContextReasonCode[] {
  return [...new Set(evidenceReferences.flatMap((reference) => reference[reasonKey]))];
}

function validationSummary(
  input: LocalAiStoredEvidenceContextBuildInput,
  forbiddenCustodyReferenceCount: number
): LocalAiEvidenceContextValidationSummary {
  const sourceEvidenceReferenceCount = input.evidenceReferences.reduce(
    (count, reference) => count + reference.sourceEvidenceReferences.length,
    0
  );
  return {
    evidenceReferenceCount: input.evidenceReferences.length,
    sourceEvidenceReferenceCount,
    runtimeReferenceCount: input.runtimeReferences.length,
    memoryReferenceCount: input.memoryReferences.length,
    graphReferenceCount: input.graphReferences.length,
    forbiddenCustodyReferenceCount,
  };
}

function contextForInput(
  input: LocalAiStoredEvidenceContextBuildInput,
  forbiddenCustodyReferenceCount: number
): LocalAiEvidenceContext {
  return {
    schemaVersion: input.request.schemaVersion,
    contextId: input.contextId,
    requestId: input.request.requestId,
    childProfile: input.request.childProfile,
    device: input.request.device,
    evidenceReferences: input.evidenceReferences,
    browserEvidenceRefs: refIdsForKind(input.evidenceReferences, 'browser'),
    appGameEvidenceRefs: refIdsForKind(input.evidenceReferences, 'app-game'),
    networkFlowEvidenceRefs: refIdsForKind(input.evidenceReferences, 'network-flow'),
    screenSummaryRefs: refIdsForKind(input.evidenceReferences, 'screen-summary'),
    parentRuleReferences: input.request.parentRuleReferences,
    recentActivitySummaryRefs: refIdsForKind(input.evidenceReferences, 'recent-activity'),
    memoryReferences: input.memoryReferences,
    graphReferences: input.graphReferences,
    localModelRuntimeRefs: input.runtimeReferences.map((reference) => reference.runtimeReferenceId),
    promptVersion: input.request.promptVersion,
    custodyLabels: uniqueCustodyLabels(input.evidenceReferences),
    degradedReasons: uniqueReasonCodes(input.evidenceReferences, 'degradedReasons'),
    unknownReasons: uniqueReasonCodes(input.evidenceReferences, 'unknownReasons'),
    validationSummary: validationSummary(input, forbiddenCustodyReferenceCount),
  };
}

function resultFor(
  input: LocalAiStoredEvidenceContextBuildInput,
  state: LocalAiContextBuildState,
  context: LocalAiEvidenceContext | null,
  rejectedFields: LocalAiRejectedField[],
  missingEvidenceKinds: LocalAiEvidenceContextKind[],
  degradedSourceRefs: LocalAiEvidenceContextRefId[],
  custodyBoundarySummary: LocalAiEvidenceContextSummary,
  validationGateSummary: LocalAiEvidenceContextSummary
): LocalAiEvidenceContextBuildResult {
  return LocalAiEvidenceContextBuildResultSchema.parse({
    schemaVersion: input.request.schemaVersion,
    requestId: input.request.requestId,
    state,
    context,
    rejectedFields,
    missingEvidenceKinds,
    degradedSourceRefs,
    custodyBoundarySummary,
    validationGateSummary,
    auditEvidenceReferences: input.evidenceReferences.map((reference) => reference.evidence),
  });
}

export function buildLocalAiEvidenceContext(input: unknown): LocalAiEvidenceContextBuildResult {
  const parsed = LocalAiStoredEvidenceContextBuildInputSchema.parse(input);
  const forbiddenCustodyReferences = parsed.evidenceReferences.filter(
    (reference) => reference.custody === 'ocentra-hosted-non-activity'
  );
  if (forbiddenCustodyReferences.length > 0) {
    return resultFor(
      parsed,
      'rejected',
      null,
      [EvidenceReferencesField],
      [],
      forbiddenCustodyReferences.map((reference) => reference.evidenceRefId),
      HostedCustodySummary,
      HostedCustodyGate
    );
  }
  const allowedCustody = new Set(parsed.request.allowedCustody);
  const unallowedCustodyReferences = parsed.evidenceReferences.filter(
    (reference) => !allowedCustody.has(reference.custody)
  );
  if (unallowedCustodyReferences.length > 0) {
    return resultFor(
      parsed,
      'rejected',
      null,
      [EvidenceReferencesField],
      [],
      unallowedCustodyReferences.map((reference) => reference.evidenceRefId),
      UnallowedCustodySummary,
      UnallowedCustodyGate
    );
  }

  const missingEvidenceKinds = parsed.request.requiredEvidenceKinds.filter(
    (evidenceKind) => refIdsForKind(parsed.evidenceReferences, evidenceKind).length === 0
  );
  const context =
    parsed.evidenceReferences.length > 0 ? contextForInput(parsed, forbiddenCustodyReferences.length) : null;
  if (parsed.evidenceReferences.length === 0) {
    return resultFor(
      parsed,
      'insufficient',
      null,
      [],
      missingEvidenceKinds,
      [],
      NoEvidenceSummary,
      InsufficientEvidenceGate
    );
  }
  if (missingEvidenceKinds.length > 0) {
    return resultFor(
      parsed,
      'partial',
      context,
      [],
      missingEvidenceKinds,
      [],
      StoredEvidenceSummary,
      PartialContextGate
    );
  }
  return resultFor(parsed, 'ready', context, [], [], [], StoredEvidenceSummary, ReadyContextGate);
}
