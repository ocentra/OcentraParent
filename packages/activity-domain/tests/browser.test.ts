import { describe, expect, it } from 'vitest';
import {
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
  BrowserUrlSchema,
  decodeBrowserUrl,
} from '../src/browser';

describe('browser evidence contracts', () => {
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
});

describe('browser managed session status contracts', () => {
  it('accepts managed browser session status without leaking raw bridge endpoints', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      checkedAt: '2026-05-21T03:30:00Z',
      managedBrowserSessionId: 'managed-browser-session-1',
      browserFamily: BrowserFamily.Chrome,
      browserChannel: BrowserChannel.Stable,
      browserVersion: '125.0.0',
      profileId: 'managed-profile-child',
      profilePathRef: 'managed-profile-redacted',
      processId: 4242,
      bridgeKind: BrowserBridgeKind.ChromiumDevtoolsProtocol,
      bridgeEndpointRef: 'managed-loopback-devtools-redacted',
      managedState: BrowserManagedState.BridgeConnected,
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
      degradedReason: null,
      startedAt: '2026-05-21T03:29:50Z',
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.managedState).toBe('bridge-connected');
      expect(parsed.data.bridgeEndpointRef).toBe('managed-loopback-devtools-redacted');
    }
  });

  it('accepts bridge-missing managed browser status as typed degraded state', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      checkedAt: '2026-05-21T03:30:00Z',
      managedBrowserSessionId: null,
      browserFamily: null,
      browserChannel: null,
      browserVersion: null,
      profileId: null,
      profilePathRef: null,
      processId: null,
      bridgeKind: null,
      bridgeEndpointRef: null,
      managedState: BrowserManagedState.NotInstalled,
      capabilityStatus: BrowserCapabilityStatus.ManagedProfileMissing,
      degradedReason: 'managed-browser-executable-missing',
      startedAt: null,
      custodyLabel: BrowserCustodyLabel.Unavailable,
      queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.capabilityStatus).toBe('managed-profile-missing');
      expect(parsed.data.browserFamily).toBeNull();
    }
  });
});
