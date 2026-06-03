import type {
  ParentDesktopReleaseSupportAuthorityOperation,
  ParentDesktopReleaseSupportCapabilityRow,
  ParentDesktopReleaseSupportCiArtifactProof,
  ParentDesktopReleaseSupportDiagnosticField,
  ParentDesktopReleaseSupportDiagnostics,
  ParentDesktopReleaseSupportManualRunbookEntry,
  ParentDesktopReleaseSupportMobileBridgeBoundary,
  ParentDesktopReleaseSupportOperation,
  ParentDesktopReleaseSupportPackageRuntimeEvidence,
  ParentDesktopReleaseSupportReadModel,
  ParentDesktopReleaseSupportSigningStoreState,
  ParentDesktopReleaseSupportSigningSurface,
  ParentDesktopReleaseSupportTarget,
  ParentDesktopReleaseSupportUpdateState,
} from './parent-desktop-release-support';
import { parentDesktopReleaseSupportIncidentHandoffIsHonest } from './parent-desktop-release-support-incident-guards';

const RequiredOperations = [
  'read-service-state',
  'read-route-state',
  'write-policy',
  'approve-request',
  'take-controller',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportOperation>;
const RequiredTargets = [
  'parent-desktop',
  'parent-mobile',
  'child-desktop',
  'child-android',
  'child-ios',
  'relay',
  'signing',
  'store',
  'support',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportTarget>;
const RequiredSigningSurfaces = [
  'windows-code-signing',
  'macos-notarization',
  'google-play',
  'testflight',
  'app-store',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportSigningSurface>;
const RequiredDiagnosticFields = [
  'version',
  'commit',
  'platform',
  'package',
  'service',
  'route',
  'capability',
  'degraded-state',
] as const satisfies ReadonlyArray<ParentDesktopReleaseSupportDiagnosticField>;
const RequiredRedactedFieldLabels = [
  'tokens',
  'child activity',
  'raw urls',
  'screenshots',
  'journals',
  'SQLite snapshots',
  'private paths',
  'command lines',
  'keystrokes',
  'clipboard data',
  'message contents',
] as const;

export function parentDesktopReleaseSupportReadModelIsHonest(readModel: ParentDesktopReleaseSupportReadModel): boolean {
  return (
    observerAuthorityIsReadOnly(readModel.observerAuthority) &&
    mobileBoundaryIsSeparate(readModel.mobileBridgeBoundary) &&
    packageRuntimeEvidenceIsHonest(readModel.packageRuntimeEvidence) &&
    updateStatesAreHonest(readModel.updateStates) &&
    signingStoreStatesAreManual(readModel.signingStoreStates) &&
    matrixCoversRequiredTargets(readModel.platformCapabilityMatrix) &&
    ciArtifactProofIsHonest(readModel.ciArtifactProof) &&
    supportDiagnosticsAreRedacted(readModel.supportDiagnostics) &&
    parentDesktopReleaseSupportIncidentHandoffIsHonest(readModel.supportIncidentHandoff) &&
    manualRunbookCoversRequiredTargets(readModel.manualRunbook)
  );
}

function packageRuntimeEvidenceIsHonest(evidence: ParentDesktopReleaseSupportPackageRuntimeEvidence): boolean {
  return (
    packageRuntimeBoundaryIsHonest(evidence) &&
    packageRuntimeStatesAreHonest(evidence) &&
    evidence.supportDiagnosticState !== 'unavailable' &&
    packageRuntimeNonClaimIsHonest(evidence)
  );
}

function packageRuntimeBoundaryIsHonest(evidence: ParentDesktopReleaseSupportPackageRuntimeEvidence): boolean {
  return [
    evidence.packageFrontendSource === 'built-portal-dist',
    evidence.backendBoundary === 'rust-service-boundary',
    evidence.serviceLaunchOwner === 'package-service-manager',
    evidence.fixedAgentAddress.includes('127.0.0.1:4477'),
    evidence.portOwnership === 'fixed-loopback',
    evidence.portConflictPolicy === 'no-foreign-process-reclaim',
    evidence.processOwnership === 'parent-shell-only',
    evidence.blankWindowGuard === 'frontend-dist-required',
    evidence.updateRollbackPosture === 'signed-channel-required',
  ].every((entry) => entry);
}

function packageRuntimeStatesAreHonest(evidence: ParentDesktopReleaseSupportPackageRuntimeEvidence): boolean {
  const serviceStateIsHonest =
    evidence.serviceHealthState === 'implemented' ||
    evidence.serviceHealthState === 'degraded' ||
    evidence.serviceHealthState === 'manual-required';
  const connectStateIsHonest =
    evidence.connectOrDegradeState === 'implemented' || evidence.connectOrDegradeState === 'degraded';
  return serviceStateIsHonest && connectStateIsHonest;
}

function packageRuntimeNonClaimIsHonest(evidence: ParentDesktopReleaseSupportPackageRuntimeEvidence): boolean {
  return evidence.nonClaim.includes('not signing') && evidence.nonClaim.includes('not production');
}

function observerAuthorityIsReadOnly(
  operations: ReadonlyArray<ParentDesktopReleaseSupportAuthorityOperation>
): boolean {
  const byOperation = new Map(operations.map((entry) => [entry.operation, entry] as const));
  if (byOperation.size !== operations.length || !RequiredOperations.every((operation) => byOperation.has(operation))) {
    return false;
  }

  return operations.every((entry) => {
    if (entry.operation === 'read-service-state' || entry.operation === 'read-route-state') {
      return entry.authorityRole === 'observer' && entry.result === 'completed' && entry.rejectionReason === null;
    }
    return (
      entry.authorityRole === 'observer' &&
      (entry.result === 'rejected' || entry.result === 'disabled') &&
      entry.rejectionReason === 'observer-read-only'
    );
  });
}

function mobileBoundaryIsSeparate(boundary: ParentDesktopReleaseSupportMobileBridgeBoundary): boolean {
  return (
    boundary.parentMobileState !== 'implemented' &&
    boundary.childAndroidAgentState !== 'implemented' &&
    boundary.childIosAgentState !== 'implemented' &&
    boundary.childAgentNonClaim.includes('child Android') &&
    boundary.childAgentNonClaim.includes('child iOS')
  );
}

function updateStatesAreHonest(states: ReadonlyArray<ParentDesktopReleaseSupportUpdateState>): boolean {
  const byChannel = new Map(states.map((entry) => [entry.channel, entry] as const));
  if (!byChannel.has('unsigned-preview') || !byChannel.has('production') || byChannel.size !== states.length) {
    return false;
  }

  return states.every((entry) => {
    if (entry.channel === 'unsigned-preview') {
      return (
        entry.packageState === 'unsigned-preview' &&
        entry.signingState === 'signature-required' &&
        entry.rollbackState === 'rollback-unavailable'
      );
    }
    if (entry.channel === 'production') {
      return (
        entry.packageState === 'production-promotion-required' &&
        entry.signingState === 'signature-required' &&
        entry.productionPromotionState === 'production-promotion-required'
      );
    }
    return entry.rollbackState !== 'rollback-available';
  });
}

function signingStoreStatesAreManual(states: ReadonlyArray<ParentDesktopReleaseSupportSigningStoreState>): boolean {
  const bySurface = new Map(states.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === states.length &&
    RequiredSigningSurfaces.every((surface) => bySurface.has(surface)) &&
    states.every(
      (entry) =>
        entry.state !== 'implemented' &&
        entry.state !== 'ci-artifact-present' &&
        (entry.credentialState === 'manual-required' || entry.credentialState === 'signature-required')
    )
  );
}

function matrixCoversRequiredTargets(rows: ReadonlyArray<ParentDesktopReleaseSupportCapabilityRow>): boolean {
  const byTarget = new Map(rows.map((entry) => [entry.target, entry] as const));
  return (
    byTarget.size === rows.length &&
    RequiredTargets.every((target) => byTarget.has(target)) &&
    (byTarget.get('parent-desktop')?.proofLevel === 'preview-only' ||
      byTarget.get('parent-desktop')?.proofLevel === 'manual-required') &&
    byTarget.get('child-android')?.proofLevel === 'manual-required' &&
    byTarget.get('child-ios')?.proofLevel === 'manual-required' &&
    byTarget.get('relay')?.capabilityState === 'not-implemented'
  );
}

function ciArtifactProofIsHonest(proof: ParentDesktopReleaseSupportCiArtifactProof): boolean {
  if (proof.packageReadinessClaim === 'ready') {
    return proof.runStatus === 'success' && proof.artifactState === 'present' && proof.runUrl !== null;
  }
  return proof.artifactState !== 'present' || proof.runStatus === 'success';
}

function supportDiagnosticsAreRedacted(diagnostics: ParentDesktopReleaseSupportDiagnostics): boolean {
  const fields = new Set(diagnostics.entries.map((entry) => entry.field));
  if (!RequiredDiagnosticFields.every((field) => fields.has(field))) {
    return false;
  }

  const forbiddenValues = ['secret', 'token', 'raw-journal', 'sqlite', 'child-private-data'];
  const expandedForbiddenValues = [
    ...forbiddenValues,
    'rawurl',
    'raw url',
    'http://',
    'https://',
    'screenshot',
    'journal',
    'private path',
    'privatepath',
    'command line',
    'commandline',
    'keystroke',
    'clipboard',
    'message content',
    'messagecontent',
    'child activity',
    'childactivity',
  ];
  const redactedLabels = new Set(diagnostics.redactedFields.map((field) => field.toLowerCase()));
  const requiredRedactionsPresent = RequiredRedactedFieldLabels.every((field) =>
    redactedLabels.has(field.toLowerCase())
  );

  return (
    requiredRedactionsPresent &&
    diagnostics.entries.every((entry) => {
      const value = entry.value.toLowerCase();
      return (
        entry.redactionState === 'redacted' || !expandedForbiddenValues.some((forbidden) => value.includes(forbidden))
      );
    })
  );
}

function manualRunbookCoversRequiredTargets(entries: ReadonlyArray<ParentDesktopReleaseSupportManualRunbookEntry>) {
  const byTarget = new Map(entries.map((entry) => [entry.target, entry] as const));
  return (
    byTarget.size === entries.length &&
    RequiredTargets.every((target) => byTarget.has(target)) &&
    entries.every((entry) => entry.knownGaps.length > 0)
  );
}
