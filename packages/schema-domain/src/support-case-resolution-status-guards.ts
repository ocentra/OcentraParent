import type {
  SupportCaseResolutionDataClass,
  SupportCaseResolutionStatusEntry,
  SupportCaseResolutionStatusEntryCandidate,
  SupportCaseResolutionStatusState,
} from './support-case-resolution-status.js';
import { supportProofHasAnyClaimUpgrade, supportProofRequiredValuesArePresent } from './support-proof-contract.js';

export function supportCaseResolutionStatusEntryIsSafe(
  entry: SupportCaseResolutionStatusEntryCandidate,
  requiredDataClasses: ReadonlyArray<SupportCaseResolutionDataClass>
): boolean {
  return (
    !supportCaseResolutionHasClaimUpgrade(entry) &&
    supportProofRequiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    supportCaseResolutionRefsArePresent(entry) &&
    supportCaseResolutionStatesAreCoherent(entry)
  );
}

export function supportCaseResolutionStatusCoversRequiredStates(
  entries: readonly SupportCaseResolutionStatusEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.caseStatus));
  return [
    'case-opened',
    'triage-ready',
    'parent-update-ready',
    'escalation-manual-required',
    'response-manual-required',
    'closure-ready',
    'sla-manual-required',
  ].every((state) => states.has(state as SupportCaseResolutionStatusState));
}

function supportCaseResolutionHasClaimUpgrade(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return supportProofHasAnyClaimUpgrade([
    entry.containsTokens,
    entry.containsRawChildActivity,
    entry.containsRawUrls,
    entry.containsScreenshots,
    entry.containsJournals,
    entry.containsSqliteSnapshots,
    entry.containsPrivatePaths,
    entry.containsCommandLines,
    entry.containsKeystrokes,
    entry.containsClipboardData,
    entry.containsMessageContents,
    entry.containsProviderSecrets,
    entry.containsRemoteSupportTranscripts,
    entry.realSupportBackendUploadExecuted,
    entry.accountLookupExecuted,
    entry.billingProviderContactExecuted,
    entry.remoteSupportSessionExecuted,
    entry.productionSlaClaimed,
    entry.ocentraHostedFamilyDataDefault,
  ]);
}

function supportCaseResolutionRefsArePresent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.parentConsentRefs.length > 0 &&
    entry.incidentRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.publicationRefs.length > 0
  );
}

function supportCaseResolutionStatesAreCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    supportCaseResolutionBoundaryStateIsCoherent(entry) &&
    supportCaseResolutionReadyStateIsCoherent(entry) &&
    supportCaseResolutionEscalationStateIsCoherent(entry) &&
    supportCaseResolutionResponseStateIsCoherent(entry) &&
    supportCaseResolutionClosureStateIsCoherent(entry) &&
    supportCaseResolutionSlaStateIsCoherent(entry)
  );
}

function supportCaseResolutionBoundaryStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.parentInitiationState === 'parent-initiated' &&
    entry.parentConsentState === 'parent-approved' &&
    entry.casePayloadState === 'support-safe-status-and-refs-only' &&
    entry.custodyState === 'no-ocentra-hosted-family-data'
  );
}

function supportCaseResolutionReadyStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  if (!['case-opened', 'triage-ready', 'parent-update-ready'].includes(entry.caseStatus)) {
    return true;
  }

  return (
    entry.operatorResponseState === 'manual-required' &&
    entry.escalationState === 'not-requested' &&
    entry.allowedDestinations.includes('support-safe-case-status-boundary')
  );
}

function supportCaseResolutionEscalationStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.caseStatus !== 'escalation-manual-required' ||
    (entry.escalationState === 'manual-required' &&
      entry.escalationRefs.length > 0 &&
      entry.manualProofRequirements.length > 0 &&
      entry.allowedDestinations.includes('manual-support-operator'))
  );
}

function supportCaseResolutionResponseStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.caseStatus !== 'response-manual-required' ||
    (entry.operatorResponseState === 'manual-required' &&
      entry.responseRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function supportCaseResolutionClosureStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.caseStatus !== 'closure-ready' ||
    (entry.operatorResponseState === 'manual-required' &&
      entry.closureRefs.length > 0 &&
      entry.allowedDestinations.includes('support-safe-case-status-boundary'))
  );
}

function supportCaseResolutionSlaStateIsCoherent(entry: SupportCaseResolutionStatusEntryCandidate): boolean {
  return (
    entry.caseStatus !== 'sla-manual-required' ||
    (entry.slaState === 'manual-required' &&
      entry.slaRefs.length > 0 &&
      entry.manualProofRequirements.length > 0 &&
      entry.allowedDestinations.includes('manual-support-operator'))
  );
}
