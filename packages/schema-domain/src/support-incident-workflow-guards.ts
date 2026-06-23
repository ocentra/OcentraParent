import type {
  SupportIncidentWorkflowDataClass,
  SupportIncidentWorkflowEntry,
  SupportIncidentWorkflowEntryCandidate,
  SupportIncidentWorkflowState,
} from './support-incident-workflow.js';
import { supportProofHasAnyClaimUpgrade, supportProofRequiredValuesArePresent } from './support-proof-contract.js';

export function supportIncidentWorkflowEntryIsSafe(
  entry: SupportIncidentWorkflowEntryCandidate,
  requiredDataClasses: ReadonlyArray<SupportIncidentWorkflowDataClass>
): boolean {
  return (
    !supportIncidentWorkflowHasClaimUpgrade(entry) &&
    supportProofRequiredValuesArePresent(entry.disclosedDataClasses, requiredDataClasses) &&
    supportIncidentWorkflowRefsArePresent(entry) &&
    supportIncidentWorkflowStatesAreCoherent(entry)
  );
}

export function supportIncidentWorkflowCoversRequiredStates(entries: readonly SupportIncidentWorkflowEntry[]): boolean {
  const states = new Set(entries.map((entry) => entry.workflowState));
  return [
    'parent-consent-gate',
    'privacy-legal-disclosure-required',
    'redaction-audit-review',
    'backend-upload-manual-required',
    'billing-escalation-manual-required',
    'account-lookup-manual-required',
  ].every((state) => states.has(state as SupportIncidentWorkflowState));
}

function supportIncidentWorkflowHasClaimUpgrade(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return supportProofHasAnyClaimUpgrade([
    entry.containsTokens,
    entry.containsChildActivity,
    entry.containsRawUrls,
    entry.containsScreenshots,
    entry.containsJournals,
    entry.containsSqliteSnapshots,
    entry.containsPrivatePaths,
    entry.containsCommandLines,
    entry.containsKeystrokes,
    entry.containsClipboardData,
    entry.containsMessageContents,
    entry.providerSecretPresent,
    entry.backendUploadExecuted,
    entry.billingProviderContacted,
    entry.accountLookupExecuted,
    entry.remoteSupportSessionStarted,
    entry.productionSlaClaimed,
    entry.ocentraHostedChildActivityCustody,
  ]);
}

function supportIncidentWorkflowRefsArePresent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return (
    entry.consentRefs.length > 0 &&
    entry.privacyLegalRefs.length > 0 &&
    entry.redactionRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.manualProofRequirements.length > 0
  );
}

function supportIncidentWorkflowStatesAreCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return (
    supportIncidentConsentStateIsCoherent(entry) &&
    supportIncidentDisclosureStateIsCoherent(entry) &&
    supportIncidentUploadStateIsCoherent(entry) &&
    supportIncidentBillingStateIsCoherent(entry) &&
    supportIncidentAccountStateIsCoherent(entry)
  );
}

function supportIncidentConsentStateIsCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return entry.workflowState !== 'parent-consent-gate' || entry.parentConsentState === 'required';
}

function supportIncidentDisclosureStateIsCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return (
    entry.workflowState === 'parent-consent-gate' ||
    (entry.parentConsentState === 'parent-approved' &&
      entry.privacyDisclosureState === 'disclosed-before-export' &&
      entry.legalDisclosureState === 'disclosed-before-export')
  );
}

function supportIncidentUploadStateIsCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return entry.workflowState !== 'backend-upload-manual-required' || entry.backendUploadState === 'manual-required';
}

function supportIncidentBillingStateIsCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return (
    entry.workflowState !== 'billing-escalation-manual-required' ||
    (entry.billingEscalationState === 'manual-required' && entry.billingRefs.length > 0)
  );
}

function supportIncidentAccountStateIsCoherent(entry: SupportIncidentWorkflowEntryCandidate): boolean {
  return (
    entry.workflowState !== 'account-lookup-manual-required' ||
    (entry.accountLookupState === 'manual-required' && entry.accountRefs.length > 0)
  );
}
