import { describe, expect, it } from 'vitest';
import {
  BrowserActiveProofSource,
  BrowserActiveTabState,
  BrowserBridgeKind,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserEvidenceSchemaVersion,
  BrowserFamily,
  BrowserManagedSessionStatusSchema,
  BrowserManagedState,
  BrowserQueryVisibilityLabel,
  BrowserTabEvidenceSchema,
  BrowserUnmanagedDetectionConfidence,
  BrowserUnmanagedDetectionReason,
  BrowserUnmanagedProcessEvidenceSchema,
  BrowserUnmanagedProcessKind,
  BrowserUrlSchema,
  decodeBrowserUrl,
} from '../src/browser';

describe('browser tab evidence contracts', () => {
  it('accepts managed Chromium tab evidence with explicit active-state certainty', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      browserEvidenceId: 'browser-evidence-1',
      observedAt: '2026-05-21T01:00:00Z',
      freshUntil: '2026-05-21T01:00:30Z',
      sourceId: 'managed-chromium-devtools',
      adapterId: 'managed-chromium-devtools-adapter',
      deviceId: 'local-dev-agent',
      browserFamily: BrowserFamily.Edge,
      browserChannel: BrowserChannel.Stable,
      managedBrowserSessionId: 'managed-browser-session-1',
      profileId: 'managed-profile-child',
      processId: 4242,
      windowId: null,
      tabId: null,
      targetId: 'target-1',
      activeState: BrowserActiveTabState.Unknown,
      activeProofSource: BrowserActiveProofSource.TargetListOnly,
      url: 'https://example.test/learn',
      origin: 'https://example.test',
      domain: 'example.test',
      title: 'Example learning page',
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
      degradedReason: null,
      staleAt: '2026-05-21T01:00:30Z',
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.browserFamily).toBe('edge');
      expect(parsed.data.activeState).toBe('unknown');
      expect(parsed.data.capabilityStatus).toBe('tab-list-only');
      expect(parsed.data.url).toBe('https://example.test/learn');
    }
  });

  it('rejects browser tab evidence with a non-URL value', () => {
    const parsed = BrowserUrlSchema.safeParse('not a browser url');

    expect(parsed.success).toBe(false);
    expect(decodeBrowserUrl('https://example.test/path')).toBe('https://example.test/path');
  });

  it('rejects browser tab evidence when URL fields are not mapper-normalized', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      browserEvidenceId: 'browser-evidence-1',
      observedAt: '2026-05-21T01:00:00Z',
      freshUntil: '2026-05-21T01:00:30Z',
      sourceId: 'managed-chromium-devtools',
      adapterId: 'managed-chromium-devtools-adapter',
      deviceId: 'local-dev-agent',
      browserFamily: BrowserFamily.Chrome,
      browserChannel: BrowserChannel.Stable,
      managedBrowserSessionId: 'managed-browser-session-1',
      profileId: 'managed-profile-child',
      processId: 4242,
      windowId: null,
      tabId: 'browser-tab-target-1',
      targetId: 'target-1',
      activeState: BrowserActiveTabState.Unknown,
      activeProofSource: BrowserActiveProofSource.TargetListOnly,
      url: 'HTTPS://child:secret@Example.Test:443/learn?Video=1',
      origin: 'https://example.test:443',
      domain: 'example.test',
      title: null,
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
      degradedReason: null,
      staleAt: '2026-05-21T01:00:30Z',
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
    });

    expect(parsed.success).toBe(false);
  });
});

describe('browser managed session status contracts', () => {
  it('accepts managed browser session status without leaking raw bridge endpoints', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse(bridgeConnectedStatus());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.managedState).toBe('bridge-connected');
      expect(parsed.data.bridgeEndpointRef).toBe('managed-loopback-devtools-redacted');
    }
  });

  it('accepts bridge-missing managed browser status as typed degraded state', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse(browserMissingStatus());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.capabilityStatus).toBe('managed-profile-missing');
      expect(parsed.data.browserFamily).toBeNull();
    }
  });

  it('accepts unmanaged browser process status as bypass evidence without a managed session', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse(unmanagedBrowserStatus());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.managedBrowserSessionId).toBeNull();
      expect(parsed.data.managedState).toBe('installed-supported');
      expect(parsed.data.capabilityStatus).toBe('unmanaged-browser');
      expect(parsed.data.degradedReason).toBe('managed-browser-unmanaged-process');
      expect(parsed.data.unmanagedProcessName).toBe('chrome.exe');
      expect(parsed.data.unmanagedDetectionReason).toBe('supported-browser-outside-managed-session');
    }
  });
});

describe('browser unmanaged process evidence contracts', () => {
  it('accepts unmanaged browser process evidence without exact URL fields', () => {
    const parsed = BrowserUnmanagedProcessEvidenceSchema.safeParse(unmanagedProcessEvidence());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.processName).toBe('chrome.exe');
      expect(parsed.data.processKind).toBe('supported-browser');
      expect(parsed.data.capabilityStatus).toBe('unmanaged-browser');
    }
  });

  it('does not preserve injected exact URL, social, or game fields on unmanaged evidence', () => {
    const parsed = BrowserUnmanagedProcessEvidenceSchema.safeParse({
      ...unmanagedProcessEvidence(),
      url: 'https://social.example.test/signup',
      tabId: 'browser-tab-1',
      socialAccountId: 'teen-account',
      socialRoute: 'signup',
      browserGameTitle: 'Unmanaged game',
      cloudGameTitle: 'Cloud title',
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect('url' in parsed.data).toBe(false);
      expect('tabId' in parsed.data).toBe(false);
      expect('socialAccountId' in parsed.data).toBe(false);
      expect('browserGameTitle' in parsed.data).toBe(false);
    }
  });

  it('rejects unmanaged browser evidence that drifts into managed URL capability', () => {
    const parsed = BrowserUnmanagedProcessEvidenceSchema.safeParse({
      ...unmanagedProcessEvidence(),
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects supported browser process evidence with a social route reason', () => {
    const parsed = BrowserUnmanagedProcessEvidenceSchema.safeParse({
      ...unmanagedProcessEvidence(),
      detectionReason: BrowserUnmanagedDetectionReason.PossibleSocialBypass,
    });

    expect(parsed.success).toBe(false);
  });
});

function bridgeConnectedStatus() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    checkedAt: '2026-05-21T03:30:00Z',
    managedBrowserSessionId: 'managed-browser-session-1',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    browserVersion: '125.0.0',
    profileId: 'managed-profile-child',
    profilePathRef: 'managed-profile-redacted',
    profileRootRef: 'managed-profile-root-redacted',
    profileScopeId: 'managed-profile-scope-dev',
    profileLifecycleState: 'ready',
    policyRevision: 'browser-policy-revision-dev',
    processId: 4242,
    bridgeKind: BrowserBridgeKind.ChromiumDevtoolsProtocol,
    bridgeEndpointRef: 'managed-loopback-devtools-redacted',
    managedState: BrowserManagedState.BridgeConnected,
    capabilityStatus: BrowserCapabilityStatus.TabListOnly,
    degradedReason: null,
    startedAt: '2026-05-21T03:29:50Z',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}

function browserMissingStatus() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    checkedAt: '2026-05-21T03:30:00Z',
    managedBrowserSessionId: null,
    browserFamily: null,
    browserChannel: null,
    browserVersion: null,
    profileId: null,
    profilePathRef: null,
    profileRootRef: null,
    profileScopeId: null,
    profileLifecycleState: null,
    policyRevision: null,
    processId: null,
    bridgeKind: null,
    bridgeEndpointRef: null,
    managedState: BrowserManagedState.NotInstalled,
    capabilityStatus: BrowserCapabilityStatus.ManagedProfileMissing,
    degradedReason: 'managed-browser-executable-missing',
    startedAt: null,
    custodyLabel: BrowserCustodyLabel.Unavailable,
    queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
  };
}

function unmanagedBrowserStatus() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    checkedAt: '2026-05-21T03:30:00Z',
    managedBrowserSessionId: null,
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    browserVersion: null,
    profileId: null,
    profilePathRef: null,
    profileRootRef: null,
    profileScopeId: null,
    profileLifecycleState: null,
    policyRevision: null,
    processId: 5150,
    bridgeKind: null,
    bridgeEndpointRef: null,
    unmanagedProcessName: 'chrome.exe',
    unmanagedExecutablePathRef: 'windows-browser-executable-redacted',
    unmanagedSignatureRef: 'windows-browser-signature-redacted',
    unmanagedProcessHashRef: 'windows-browser-process-hash-redacted',
    unmanagedProcessKind: BrowserUnmanagedProcessKind.SupportedBrowser,
    unmanagedDetectionConfidence: BrowserUnmanagedDetectionConfidence.High,
    unmanagedDetectionReason: BrowserUnmanagedDetectionReason.SupportedBrowserOutsideManagedSession,
    managedState: BrowserManagedState.InstalledSupported,
    capabilityStatus: BrowserCapabilityStatus.UnmanagedBrowser,
    degradedReason: 'managed-browser-unmanaged-process',
    startedAt: null,
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
  };
}

function unmanagedProcessEvidence() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    browserEvidenceId: 'browser-unmanaged-process-evidence-1',
    observedAt: '2026-05-21T03:30:00Z',
    sourceId: 'windows-process-snapshot',
    deviceId: 'local-dev-agent',
    processId: 5150,
    processName: 'chrome.exe',
    executablePathRef: 'windows-browser-executable-redacted',
    signatureRef: 'windows-browser-signature-redacted',
    processHashRef: 'windows-browser-process-hash-redacted',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    processKind: BrowserUnmanagedProcessKind.SupportedBrowser,
    detectionConfidence: BrowserUnmanagedDetectionConfidence.High,
    detectionReason: BrowserUnmanagedDetectionReason.SupportedBrowserOutsideManagedSession,
    capabilityStatus: BrowserCapabilityStatus.UnmanagedBrowser,
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
  };
}
