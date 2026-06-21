import type {
  ParentDesktopReleaseSupportIncidentDataClass,
  ParentDesktopReleaseSupportIncidentDiagnosticReferenceKind,
  ParentDesktopReleaseSupportIncidentExcludedDataClass,
  ParentDesktopReleaseSupportIncidentHandoff,
} from './parent-desktop-release-support-incident';

const RequiredIncludedDataClasses = [
  'release-version',
  'commit-id',
  'platform-family',
  'package-runtime-state',
  'service-health-state',
  'route-state',
  'capability-state',
  'degraded-state',
  'redaction-summary',
  'manual-proof-reference',
  'incident-status',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportIncidentDataClass>;

const RequiredExcludedDataClasses = [
  'tokens',
  'child-activity',
  'raw-urls',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'commands',
  'keystrokes',
  'clipboard-data',
  'message-contents',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportIncidentExcludedDataClass>;

const RequiredDiagnosticReferenceKinds = [
  'proof-json',
  'package-preview-workflow',
  'redaction-summary',
  'manual-runbook',
  'support-status-row',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportIncidentDiagnosticReferenceKind>;

const ForbiddenReferenceFragments = [
  'http://',
  'https://',
  'token',
  'rawurl',
  'raw-url',
  'child-activity',
  'screenshot',
  'journal',
  'sqlite',
  'private',
  'appdata',
  'documents/',
  'downloads/',
  'command',
  'keystroke',
  'clipboard',
  'message-content',
] as const;

export function parentDesktopReleaseSupportIncidentHandoffIsHonest(
  handoff: ParentDesktopReleaseSupportIncidentHandoff
): boolean {
  return (
    incidentMetadataIsManualSupport(handoff) &&
    parentConsentIsExplicit(handoff) &&
    supportBundleManifestIsSafe(handoff) &&
    diagnosticReferencesAreSupportSafe(handoff) &&
    manualProductionStatesAreExplicit(handoff)
  );
}

function incidentMetadataIsManualSupport(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  return (
    handoff.metadata.status === 'triage-ready' &&
    handoff.metadata.productionSupportState === 'manual-required' &&
    handoff.metadata.supportBackendState === 'not-implemented'
  );
}

function parentConsentIsExplicit(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  return (
    handoff.parentConsent.consentState === 'parent-approved' &&
    handoff.parentConsent.capturedBy === 'manual-export-action' &&
    handoff.parentConsent.disclosureState === 'shown-before-export' &&
    handoff.parentConsent.revocationState === 'manual-required'
  );
}

function supportBundleManifestIsSafe(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  const manifest = handoff.supportBundleManifest;
  return (
    manifest.custodyBoundary === 'parent-exported-local-bundle' &&
    manifest.destination === 'parent-controlled-support-channel' &&
    manifest.disclosureState === 'shown-before-export' &&
    manifest.retentionState === 'manual-required' &&
    requiredValuesArePresent(manifest.includedDataClasses, RequiredIncludedDataClasses) &&
    requiredValuesArePresent(manifest.excludedDataClasses, RequiredExcludedDataClasses) &&
    !manifestContainsForbiddenData(handoff)
  );
}

function manifestContainsForbiddenData(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  const manifest = handoff.supportBundleManifest;
  return (
    manifest.containsChildActivity ||
    manifest.containsRawUrls ||
    manifest.containsScreenshots ||
    manifest.containsJournals ||
    manifest.containsSqliteSnapshots ||
    manifest.containsPrivatePaths ||
    manifest.containsCommands ||
    manifest.containsKeystrokes ||
    manifest.containsClipboardData ||
    manifest.containsMessageContents
  );
}

function diagnosticReferencesAreSupportSafe(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  const kinds = handoff.diagnosticReferences.map((entry) => entry.kind);
  return (
    requiredValuesArePresent(kinds, RequiredDiagnosticReferenceKinds) &&
    handoff.diagnosticReferences.every(
      (entry) => !entry.includesSensitiveData && referenceLooksSupportSafe(entry.reference)
    )
  );
}

function referenceLooksSupportSafe(reference: string): boolean {
  const normalized = reference.toLowerCase().replaceAll('\\', '/');
  const hasWindowsPrivatePath = /^[a-z]:\/users\//u.test(normalized);
  return !hasWindowsPrivatePath && !ForbiddenReferenceFragments.some((fragment) => normalized.includes(fragment));
}

function manualProductionStatesAreExplicit(handoff: ParentDesktopReleaseSupportIncidentHandoff): boolean {
  const states = handoff.manualProductionSupportStates;
  const noBackendUploadClaim = states.nonClaims.some((claim) => claim.includes('no support backend upload'));
  const noCustodyClaim = states.nonClaims.some((claim) => claim.includes('no Ocentra-hosted child data custody'));
  const noBillingAccountClaim = states.nonClaims.some((claim) => claim.includes('no billing or public account'));

  return (
    states.supportBackendUploadState === 'not-implemented' &&
    states.supportStaffAccessState === 'manual-required' &&
    states.accountLookupState === 'not-implemented' &&
    states.billingEscalationState === 'not-implemented' &&
    states.remoteControlState === 'not-implemented' &&
    states.productionSlaState === 'manual-required' &&
    noBackendUploadClaim &&
    noCustodyClaim &&
    noBillingAccountClaim
  );
}

function requiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
