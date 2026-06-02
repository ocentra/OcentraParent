import type {
  ParentDesktopReleaseSupportOperation,
  ParentDesktopReleaseSupportTarget,
} from '../src/parent-desktop-release-support';

export const RuntimeReadModel = {
  schemaVersion: 'parent-desktop-release-support-proof',
  observerAuthority: observerAuthority(),
  mobileBridgeBoundary: {
    parentMobileState: 'scaffold',
    childAndroidAgentState: 'manual-required',
    childIosAgentState: 'manual-required',
    parentMobileClaim: 'parent mobile bridge is a parent shell route boundary only',
    childAgentNonClaim: 'child Android and child iOS agent parity is not claimed by parent desktop release support',
  },
  updateStates: [
    updateState('scaffold', 'scaffold', 'signature-required', 'rollback-unavailable', 'production-promotion-required'),
    updateState(
      'unsigned-preview',
      'unsigned-preview',
      'signature-required',
      'rollback-unavailable',
      'production-promotion-required'
    ),
    updateState(
      'signature-required',
      'signature-required',
      'signature-required',
      'rollback-unavailable',
      'production-promotion-required'
    ),
    updateState(
      'production',
      'production-promotion-required',
      'signature-required',
      'rollback-unavailable',
      'production-promotion-required'
    ),
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
    redactedFields: ['secrets', 'tokens', 'raw journals', 'SQLite contents', 'private child data'],
  },
  manualRunbook: manualRunbook(),
  updatedAt: '2026-06-02T05:45:00.000Z',
} as const;

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
  packageState: string,
  signingState: string,
  rollbackState: string,
  productionPromotionState: string
) {
  return {
    channel,
    packageState,
    signingState,
    rollbackState,
    productionPromotionState,
    proofRequirement: `${channel} update state must not imply signed production rollback`,
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
  target: ParentDesktopReleaseSupportTarget,
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
