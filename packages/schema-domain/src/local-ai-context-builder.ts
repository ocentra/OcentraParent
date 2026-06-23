import { Schema } from './effect';
import {
  LocalAiStoredEvidenceContextBuildInputSchema,
  type LocalAiEvidenceContext,
  type LocalAiEvidenceContextBuildResult,
  type LocalAiStoredEvidenceContextBuildInput,
} from './ai-context';
import { LocalAiEvidenceContextSummarySchema, LocalAiRejectedFieldSchema } from './ai-context-primitives';
import { selectLocalAiEvidenceContextInput, type LocalAiEvidenceContextSelection } from './local-ai-context-selection';
import { contextForInput, resultFor } from './local-ai-context-result';

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

function selectedInputFor(
  parsed: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection
): LocalAiStoredEvidenceContextBuildInput {
  return {
    ...parsed,
    request: {
      ...parsed.request,
      parentRuleContextReferences: selection.selectedParentRuleContextReferences,
    },
    evidenceReferences: selection.selectedEvidenceReferences,
    runtimeReferences: selection.selectedRuntimeReferences,
    memoryReferences: selection.selectedMemoryReferences,
    graphReferences: selection.selectedGraphReferences,
  };
}

function contextForSelection(
  selectedInput: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection
): LocalAiEvidenceContext | null {
  if (selectedInput.evidenceReferences.length === 0) {
    return null;
  }
  return contextForInput(
    selectedInput,
    {
      forbiddenCustodyReferenceCount: selection.forbiddenCustodyReferences.length,
      unallowedCustodyReferenceCount: selection.unallowedCustodyReferences.length,
      ungroundedParentRuleReferenceCount: selection.ungroundedParentRuleContextReferences.length,
    },
    selection.additionalDegradedReasons
  );
}

function hostedCustodyResult(
  parsed: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection
): LocalAiEvidenceContextBuildResult {
  return resultFor(
    parsed,
    'rejected',
    null,
    [EvidenceReferencesField],
    [],
    selection.forbiddenCustodyReferences.map((reference) => reference.evidenceRefId),
    HostedCustodySummary,
    HostedCustodyGate
  );
}

function unallowedCustodyResult(
  parsed: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection
): LocalAiEvidenceContextBuildResult {
  return resultFor(
    parsed,
    'rejected',
    null,
    [EvidenceReferencesField],
    selection.missingEvidenceKinds,
    selection.degradedSourceRefs,
    UnallowedCustodySummary,
    UnallowedCustodyGate
  );
}

function insufficientResult(
  parsed: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection
): LocalAiEvidenceContextBuildResult {
  return resultFor(
    parsed,
    'insufficient',
    null,
    [],
    selection.missingEvidenceKinds,
    [],
    NoEvidenceSummary,
    InsufficientEvidenceGate
  );
}

function partialResult(
  parsed: LocalAiStoredEvidenceContextBuildInput,
  selection: LocalAiEvidenceContextSelection,
  context: LocalAiEvidenceContext | null
): LocalAiEvidenceContextBuildResult {
  return resultFor(
    parsed,
    'partial',
    context,
    selection.unallowedCustodyReferences.length > 0 ? [EvidenceReferencesField] : [],
    selection.missingEvidenceKinds,
    selection.degradedSourceRefs,
    selection.unallowedCustodyReferences.length > 0 ? UnallowedCustodySummary : StoredEvidenceSummary,
    PartialContextGate
  );
}

export function buildLocalAiEvidenceContext(input: unknown): LocalAiEvidenceContextBuildResult {
  const parsed = LocalAiStoredEvidenceContextBuildInputSchema.parse(input);
  const selection = selectLocalAiEvidenceContextInput(parsed);
  if (selection.forbiddenCustodyReferences.length > 0) {
    return hostedCustodyResult(parsed, selection);
  }
  const selectedInput = selectedInputFor(parsed, selection);
  if (selectedInput.evidenceReferences.length === 0 && selection.unallowedCustodyReferences.length > 0) {
    return unallowedCustodyResult(parsed, selection);
  }
  const context = contextForSelection(selectedInput, selection);
  if (selectedInput.evidenceReferences.length === 0) {
    return insufficientResult(parsed, selection);
  }
  if (selection.missingEvidenceKinds.length > 0 || selection.additionalDegradedReasons.length > 0) {
    return partialResult(parsed, selection, context);
  }
  return resultFor(parsed, 'ready', context, [], [], [], StoredEvidenceSummary, ReadyContextGate);
}
