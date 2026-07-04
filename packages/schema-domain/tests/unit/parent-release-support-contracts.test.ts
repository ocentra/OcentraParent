import { describe, expect, it } from 'vitest';

import { ParentDesktopReleaseSupportIncidentHandoffSchema } from '../../src/parent-desktop-release-support-incident';
import {
  ParentDesktopReleaseSupportAvailabilityStateSchema,
  ParentDesktopReleaseSupportReadModelSchema,
  type ParentDesktopReleaseSupportDiagnosticField,
  type ParentDesktopReleaseSupportOperation,
  type ParentDesktopReleaseSupportSigningSurface,
} from '../../src/parent-desktop-release-support';
import {
  ParentOwnedLocalExportRuntimeKnownGaps,
  RequiredParentOwnedLocalExportRuntimeNonClaims,
  RequiredParentOwnedLocalExportRuntimeStates,
} from '../../src/parent-owned-local-export-runtime-values';

const IncidentHandoffFixture = {
  metadata: {
    incidentId: 'release-support-incident-1',
    status: 'triage-ready',
    severity: 'manual-required',
    productionSupportState: 'manual-required',
    supportBackendState: 'not-implemented',
    createdAt: '2026-06-02T05:45:00.000Z',
    updatedAt: '2026-06-02T05:45:00.000Z',
  },
  parentConsent: {
    consentState: 'parent-approved',
    capturedBy: 'manual-export-action',
    disclosureState: 'shown-before-export',
    parentActor: 'parent-release-support-proof',
    consentRecordedAt: '2026-06-02T05:45:00.000Z',
    revocationState: 'manual-required',
  },
  supportBundleManifest: {
    manifestId: 'release-support-manifest-1',
    custodyBoundary: 'parent-exported-local-bundle',
    destination: 'parent-controlled-support-channel',
    disclosureState: 'shown-before-export',
    retentionState: 'manual-required',
    includedDataClasses: [
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
    ],
    excludedDataClasses: [
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
    ],
    containsChildActivity: false,
    containsRawUrls: false,
    containsScreenshots: false,
    containsJournals: false,
    containsSqliteSnapshots: false,
    containsPrivatePaths: false,
    containsCommands: false,
    containsKeystrokes: false,
    containsClipboardData: false,
    containsMessageContents: false,
  },
  diagnosticReferences: [
    {
      kind: 'proof-json',
      reference: 'test-results/parent-desktop-release-support-proof/proof.json',
      sourceState: 'preview-only',
      includesSensitiveData: false,
    },
    {
      kind: 'package-preview-workflow',
      reference: '.github/workflows/package-preview.yml',
      sourceState: 'preview-only',
      includesSensitiveData: false,
    },
    {
      kind: 'redaction-summary',
      reference: 'release-support-redaction-summary',
      sourceState: 'manual-required',
      includesSensitiveData: false,
    },
    {
      kind: 'manual-runbook',
      reference: 'docs/expectations/release-installer.md',
      sourceState: 'manual-required',
      includesSensitiveData: false,
    },
    {
      kind: 'support-status-row',
      reference: 'support-status-row-release-support',
      sourceState: 'manual-required',
      includesSensitiveData: false,
    },
  ],
  manualProductionSupportStates: {
    supportBackendUploadState: 'not-implemented',
    supportStaffAccessState: 'manual-required',
    accountLookupState: 'not-implemented',
    billingEscalationState: 'not-implemented',
    remoteControlState: 'not-implemented',
    productionSlaState: 'manual-required',
    nonClaims: ['no support backend upload', 'no Ocentra-hosted child data custody', 'no billing or public account'],
  },
} as const;

const RuntimeReadModel = {
  schemaVersion: 'parent-desktop-release-support-proof',
  observerAuthority: observerAuthority(),
  mobileBridgeBoundary: {
    parentMobileState: 'scaffold',
    childAndroidAgentState: 'manual-required',
    childIosAgentState: 'manual-required',
    parentMobileClaim: 'parent mobile bridge is a parent shell route boundary only',
    childAgentNonClaim: 'child Android and child iOS agent parity is not claimed by parent desktop release support',
  },
  packageRuntimeEvidence: {
    packageFrontendSource: 'built-portal-dist',
    backendBoundary: 'rust-service-boundary',
    serviceLaunchOwner: 'package-service-manager',
    serviceHealthState: 'implemented',
    connectOrDegradeState: 'degraded',
    fixedAgentAddress: '127.0.0.1:4477',
    portOwnership: 'fixed-loopback',
    portConflictPolicy: 'no-foreign-process-reclaim',
    processOwnership: 'parent-shell-only',
    blankWindowGuard: 'frontend-dist-required',
    updateRollbackPosture: 'signed-channel-required',
    artifactState: 'not-checked-local',
    supportDiagnosticState: 'preview-only',
    nonClaim: 'CI package preview is not signing not production not store distribution proof',
  },
  updateStates: [
    updateState('scaffold', 'unavailable', 'scaffold', 'unavailable', 'unavailable', 'signature-required', 'rollback-unavailable', 'unavailable', 'recorded', 'recorded'),
    updateState('unsigned-preview', 'available', 'unsigned-preview', 'verified', 'manual-required', 'signature-required', 'rollback-unavailable', 'unavailable', 'recorded', 'recorded'),
    updateState('signature-required', 'manual-required', 'signature-required', 'verified', 'manual-required', 'signature-required', 'manual-required', 'manual-required', 'manual-required', 'manual-required'),
    updateState('production', 'manual-required', 'production-promotion-required', 'manual-required', 'manual-required', 'signature-required', 'manual-required', 'manual-required', 'manual-required', 'manual-required'),
  ],
  signingStoreStates: signingStoreStates(),
  platformCapabilityMatrix: platformRows(),
  ciArtifactProof: {
    workflowName: 'Package Preview',
    runStatus: 'pending',
    artifactState: 'pending',
    packageReadinessClaim: 'manual-required',
    checkedBy: 'node scripts/test/parent-desktop-release-support-proof.mjs',
    runUrl: null,
  },
  supportDiagnostics: {
    outputState: 'preview-only',
    entries: diagnosticEntries(),
    redactedFields: [
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
    ],
  },
  supportIncidentHandoff: IncidentHandoffFixture,
  manualRunbook: manualRunbook(),
  productionReadinessGate: productionReadinessGate(),
  updaterRollbackRunbookProof: updaterRollbackRunbookProof(),
  updatedAt: '2026-06-28T17:00:00.000Z',
} as const;

describe('parent release-support contracts centralized in schema-domain', () => {
  it('parses a support-safe incident handoff through the shared schema owner', () => {
    const parsed = ParentDesktopReleaseSupportIncidentHandoffSchema.parse(IncidentHandoffFixture);

    expect(parsed.metadata.status).toBe('triage-ready');
    expect(parsed.parentConsent.disclosureState).toBe('shown-before-export');
    expect(parsed.supportBundleManifest.custodyBoundary).toBe('parent-exported-local-bundle');
    expect(parsed.manualProductionSupportStates.supportBackendUploadState).toBe('not-implemented');
  });

  it('accepts WP08 release-support state with explicit update, rollback, checksum, signature, and negative evidence fields', () => {
    const parsed = ParentDesktopReleaseSupportReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.updateStates.find((entry) => entry.channel === 'scaffold')).toMatchObject({
      updateAvailabilityState: 'unavailable',
      checksumState: 'unavailable',
      signatureState: 'unavailable',
      rollbackAvailabilityState: 'unavailable',
      teardownEvidenceState: 'recorded',
      revertEvidenceState: 'recorded',
    });
    expect(parsed.updateStates.find((entry) => entry.channel === 'unsigned-preview')).toMatchObject({
      updateAvailabilityState: 'available',
      checksumState: 'verified',
      signatureState: 'manual-required',
      rollbackAvailabilityState: 'unavailable',
      teardownEvidenceState: 'recorded',
      revertEvidenceState: 'recorded',
    });
    expect(parsed.updateStates.find((entry) => entry.channel === 'production')).toMatchObject({
      updateAvailabilityState: 'manual-required',
      rollbackAvailabilityState: 'manual-required',
    });
    expect(parsed.updaterRollbackRunbookProof.runbookStatus.requiredSections).toContain('teardown-revert-evidence');
  });

  it('keeps update and rollback availability states explicit at the shared contract surface', () => {
    expect(ParentDesktopReleaseSupportAvailabilityStateSchema.parse('available')).toBe('available');
    expect(ParentDesktopReleaseSupportAvailabilityStateSchema.parse('unavailable')).toBe('unavailable');
    expect(ParentDesktopReleaseSupportAvailabilityStateSchema.parse('manual-required')).toBe('manual-required');
  });
});

describe('parent release-support contract rejection paths', () => {

  it('rejects hidden checksum or signature truth inside an update row', () => {
    const hiddenChecksum = withUpdateChannel('unsigned-preview', { checksumState: 'manual-required' });
    const hiddenSignature = withUpdateChannel('scaffold', { signatureState: 'manual-required' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(hiddenChecksum).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(hiddenSignature).success).toBe(false);
  });

  it('rejects missing teardown or revert evidence for negative rollback paths', () => {
    const missingPreviewTeardown = withUpdateChannel('unsigned-preview', { teardownEvidenceState: 'missing' });
    const missingPreviewRevert = withUpdateChannel('scaffold', { revertEvidenceState: 'missing' });
    const missingRunbookEvidence = withUpdaterRollbackRow('unsigned-preview', { teardownEvidenceState: 'missing' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingPreviewTeardown).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingPreviewRevert).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingRunbookEvidence).success).toBe(false);
  });

  it('rejects observer write, approval, or controller operations presented as completed', () => {
    for (const operation of ['write-policy', 'approve-request', 'take-controller'] as const) {
      const claim = withObserverOperation(operation, { result: 'completed', rejectionReason: null });

      expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(claim).success).toBe(false);
    }
  });

  it('rejects release-signing surfaces marked implemented without manual proof boundaries', () => {
    for (const surface of [
      'windows-code-signing',
      'macos-notarization',
      'google-play',
      'testflight',
      'app-store',
    ] as const) {
      const claim = withSigningSurface(surface, { state: 'implemented', credentialState: 'implemented' });

      expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(claim).success).toBe(false);
    }
  });

  it('rejects support diagnostics that leak raw URLs or command-line details', () => {
    const leakedUrl = withDiagnostic('route', { value: 'https://example.invalid/private', redactionState: 'safe' });
    const leakedCommandLine = withDiagnostic('service', {
      value: 'command line contained child browser URL',
      redactionState: 'safe',
    });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(leakedUrl).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(leakedCommandLine).success).toBe(false);
  });
});

describe('parent-owned local export value surfaces centralized in schema-domain', () => {
  it('preserves the required runtime states, non-claims, and known gaps', () => {
    expect(RequiredParentOwnedLocalExportRuntimeStates).toEqual([
      'export-queued',
      'export-running',
      'export-written',
      'delete-requested',
      'delete-confirmed',
      'delete-failed',
      'offline-queued',
      'manual-required',
    ]);
    expect(RequiredParentOwnedLocalExportRuntimeNonClaims).toContain('no-ocentra-family-data-custody');
    expect(ParentOwnedLocalExportRuntimeKnownGaps).toContain(
      'Retention scheduler and parent-visible status controls remain future work before broader product export/delete claims.'
    );
  });
});

function observerAuthority() {
  return [
    authority('read-service-state', 'completed', null),
    authority('read-route-state', 'completed', null),
    authority('write-policy', 'rejected', 'observer-read-only'),
    authority('approve-request', 'rejected', 'observer-read-only'),
    authority('take-controller', 'disabled', 'observer-read-only'),
  ] as const;
}

function authority(
  operation: ParentDesktopReleaseSupportOperation,
  result: 'completed' | 'rejected' | 'disabled',
  rejectionReason: 'observer-read-only' | null
) {
  return {
    operation,
    result,
    authorityRole: 'observer',
    rejectionReason,
    proofRequirement: `${operation} must preserve parent observer read-only authority`,
  };
}

function updateState(
  channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production',
  updateAvailabilityState: 'available' | 'unavailable' | 'manual-required',
  packageState: string,
  checksumState: 'verified' | 'unavailable' | 'manual-required',
  signatureState: 'verified' | 'unavailable' | 'manual-required',
  signingState: string,
  rollbackState: string,
  rollbackAvailabilityState: 'available' | 'unavailable' | 'manual-required',
  teardownEvidenceState: 'recorded' | 'missing' | 'manual-required',
  revertEvidenceState: 'recorded' | 'missing' | 'manual-required'
) {
  return {
    channel,
    updateAvailabilityState,
    packageState,
    checksumState,
    signatureState,
    signingState,
    rollbackState,
    rollbackAvailabilityState,
    teardownEvidenceState,
    revertEvidenceState,
    productionPromotionState: 'production-promotion-required',
    proofRequirement: `${channel} update and rollback states must keep checksum signature and teardown or revert evidence explicit without implying production release`,
  };
}

function signingStoreStates() {
  return (['windows-code-signing', 'macos-notarization', 'google-play', 'testflight', 'app-store'] as const).map(
    (surface) => ({
      surface,
      state: 'manual-required',
      credentialState: 'manual-required',
      proofRequirement: `${surface} remains manual-required until real credentials and artifacts exist`,
    })
  );
}

function platformRows() {
  return [
    platformRow('parent-desktop', 'unsigned-preview', 'implemented', 'preview-only', 'preview-only'),
    platformRow('parent-mobile', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('child-desktop', 'preview-only', 'implemented', 'preview-only', 'manual-required'),
    platformRow('child-android', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('child-ios', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('relay', 'not-implemented', 'not-implemented', 'not-implemented', 'not-ready'),
    platformRow('signing', 'signature-required', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('store', 'manual-required', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('support', 'preview-only', 'preview-only', 'preview-only', 'preview-only'),
  ] as const;
}

function platformRow(
  target: string,
  packageState: string,
  serviceState: string,
  capabilityState: string,
  proofLevel: 'ready' | 'preview-only' | 'manual-required' | 'not-ready'
) {
  return {
    target,
    packageState,
    serviceState,
    routeState: target === 'relay' ? 'not-implemented' : 'preview-only',
    capabilityState,
    proofLevel,
    nonClaim: `${target} state is limited to the named proof level and does not upgrade unsupported platform behavior`,
  };
}

function diagnosticEntries() {
  return (
    [
      ['version', '0.1.1'],
      ['commit', '5995a7c5ec8da33bbfb21aac28ac79e4d1038cf5'],
      ['platform', 'windows'],
      ['package', 'parent-desktop unsigned preview'],
      ['service', 'loopback service reachable or explicitly unavailable'],
      ['route', 'local route or unavailable route state'],
      ['capability', 'observer read-only release support'],
      ['degraded-state', 'signing store relay and rollback are manual-required'],
    ] as const
  ).map(([field, value]) => ({
    field,
    value,
    redactionState: 'safe',
  }));
}

function productionReadinessGate() {
  return {
    gate: 'v8-production-release-support-readiness',
    packagePreviewArtifacts: packagePreviewArtifacts(),
    supportDiagnosticsState: 'preview-only',
    supportRunbookState: 'manual-required',
    updaterRollbackExecutionState: 'rollback-unavailable',
    signingStoreProofState: 'manual-required',
    productionPublishingState: 'production-promotion-required',
    claimBoundary:
      'V8 readiness gate is package preview support readiness not production publishing not signing not store upload proof',
    proofReferences: [
      'test-results/parent-desktop-release-support-proof/proof.json',
      '.github/workflows/package-preview.yml',
    ],
    manualRequiredGaps: [
      'windows signing',
      'macOS notarization',
      'Google Play signing',
      'TestFlight device proof',
      'App Store proof',
      'production updater rollback',
      'production support runbook',
    ],
  };
}

function packagePreviewArtifacts() {
  return (
    [
      'ocentra-parent-windows-x64-preview',
      'ocentra-parent-linux-amd64-preview',
      'ocentra-parent-macos-preview',
      'ocentra-parent-android-preview',
      'ocentra-parent-ios-simulator-preview',
    ] as const
  ).map((artifactName) => ({
    artifactName,
    runStatus: 'pending',
    artifactState: 'pending',
    packageReadinessClaim: 'manual-required',
    manualProofRequirement: `${artifactName} requires manual platform signing or store proof before production readiness`,
  }));
}

function updaterRollbackRunbookProof() {
  return {
    proof: 'v8-updater-rollback-runbook-status',
    updaterRows: updaterRollbackRows(),
    runbookStatus: {
      draftRunbookState: 'preview-only',
      productionRunbookState: 'manual-required',
      rollbackTriageState: 'manual-required',
      requiredSections: [
        'rollback-triage',
        'rollback-failure-status',
        'teardown-revert-evidence',
        'diagnostics-redaction',
        'manual-platform-proof',
        'support-escalation-boundary',
      ],
      proofReferences: [
        'docs/expectations/release-installer.md',
        'docs/expectations/roadmap-v8-production-hardening.md',
        'test-results/parent-desktop-release-support-proof/proof.json',
      ],
      nonClaim: 'release support runbook status is preview-only not production support execution not update execution',
    },
    claimBoundary:
      'updater rollback runbook proof is not production update execution not signing not store upload proof',
    manualRequiredGaps: [
      'signed update channel',
      'production rollback execution',
      'rollback failure smoke',
      'published support runbook',
      'support escalation execution',
    ],
  };
}

function updaterRollbackRows() {
  return (['scaffold', 'unsigned-preview', 'signature-required', 'production'] as const).map((channel) => ({
    channel,
    updateAvailabilityState: updaterRollbackUpdateAvailabilityState(channel),
    checksumState: updaterRollbackChecksumState(channel),
    signatureState: updaterRollbackSignatureState(channel),
    rollbackState: updaterRollbackRollbackState(channel),
    rollbackAvailabilityState: updaterRollbackRollbackAvailabilityState(channel),
    teardownEvidenceState: updaterRollbackEvidenceState(channel),
    revertEvidenceState: updaterRollbackEvidenceState(channel),
    failureStatusState: 'manual-required',
    manualRequiredState: 'manual-required',
    proofRequirement:
      channel === 'production'
        ? 'production channel requires signed production update channel and manual proof before rollback execution teardown or revert evidence'
        : `${channel} channel requires teardown or revert evidence and manual proof before rollback execution or failure status claim`,
  }));
}

function updaterRollbackUpdateAvailabilityState(channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production') {
  if (channel === 'unsigned-preview') {
    return 'available';
  }

  if (channel === 'scaffold') {
    return 'unavailable';
  }

  return 'manual-required';
}

function updaterRollbackChecksumState(channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production') {
  if (channel === 'unsigned-preview' || channel === 'signature-required') {
    return 'verified';
  }

  if (channel === 'scaffold') {
    return 'unavailable';
  }

  return 'manual-required';
}

function updaterRollbackSignatureState(channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production') {
  return channel === 'scaffold' ? 'unavailable' : 'manual-required';
}

function updaterRollbackRollbackState(channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production') {
  return channel === 'scaffold' || channel === 'unsigned-preview' ? 'rollback-unavailable' : 'manual-required';
}

function updaterRollbackRollbackAvailabilityState(
  channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production'
) {
  return channel === 'scaffold' || channel === 'unsigned-preview' ? 'unavailable' : 'manual-required';
}

function updaterRollbackEvidenceState(channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production') {
  return channel === 'scaffold' || channel === 'unsigned-preview' ? 'recorded' : 'manual-required';
}

function manualRunbook() {
  return (
    [
      'parent-desktop',
      'parent-mobile',
      'child-desktop',
      'child-android',
      'child-ios',
      'relay',
      'signing',
      'store',
      'support',
    ] as const
  ).map((target) => ({
    target,
    hostOrDevice: `${target} named manual host or device`,
    commandOrUiAction: `${target} package launch or UI proof action`,
    permissions: `${target} permissions and entitlement state recorded`,
    packageVersion: '0.1.1',
    logsScreenshotsProofJson: `test-results/manual-platform-proof/${target}.json`,
    knownGaps: [`${target} requires manual proof before production claim`],
  }));
}

function withObserverOperation(operation: ParentDesktopReleaseSupportOperation, patch: object) {
  return {
    ...RuntimeReadModel,
    observerAuthority: RuntimeReadModel.observerAuthority.map((entry) =>
      entry.operation === operation ? { ...entry, ...patch } : entry
    ),
  };
}

function withUpdateChannel(
  channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production',
  patch: object
) {
  return {
    ...RuntimeReadModel,
    updateStates: RuntimeReadModel.updateStates.map((entry) =>
      entry.channel === channel ? { ...entry, ...patch } : entry
    ),
  };
}

function withUpdaterRollbackRow(
  channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production',
  patch: object
) {
  return {
    ...RuntimeReadModel,
    updaterRollbackRunbookProof: {
      ...RuntimeReadModel.updaterRollbackRunbookProof,
      updaterRows: RuntimeReadModel.updaterRollbackRunbookProof.updaterRows.map((entry) =>
        entry.channel === channel ? { ...entry, ...patch } : entry
      ),
    },
  };
}

function withSigningSurface(surface: ParentDesktopReleaseSupportSigningSurface, patch: object) {
  return {
    ...RuntimeReadModel,
    signingStoreStates: RuntimeReadModel.signingStoreStates.map((entry) =>
      entry.surface === surface ? { ...entry, ...patch } : entry
    ),
  };
}

function withDiagnostic(field: ParentDesktopReleaseSupportDiagnosticField, patch: object) {
  return {
    ...RuntimeReadModel,
    supportDiagnostics: {
      ...RuntimeReadModel.supportDiagnostics,
      entries: RuntimeReadModel.supportDiagnostics.entries.map((entry) =>
        entry.field === field ? { ...entry, ...patch } : entry
      ),
    },
  };
}
