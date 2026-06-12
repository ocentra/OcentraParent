import { describe, expect, it } from 'vitest';
import {
  BrowserBridgeKind,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserEvidenceSchemaVersion,
  BrowserFamily,
  BrowserManagedSessionStatusSchema,
  BrowserManagedState,
  BrowserQueryVisibilityLabel,
} from '../../src/browser';

describe('browser managed session status contracts', () => {
  it('accepts running managed status without upgrading bridge capability', () => {
    const parsed = BrowserManagedSessionStatusSchema.safeParse(runningManagedStatus());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.managedState).toBe('running-managed');
      expect(parsed.data.capabilityStatus).toBe('bridge-missing');
      expect(parsed.data.processId).toBe(5150);
      expect(parsed.data.bridgeEndpointRef).toBe('managed-loopback-devtools-redacted');
    }
  });
});

function runningManagedStatus() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    checkedAt: '2026-05-21T03:30:00Z',
    managedBrowserSessionId: 'managed-browser-session-1',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    browserVersion: null,
    profileId: 'managed-profile-child',
    profilePathRef: 'managed-profile-redacted',
    profileRootRef: 'managed-profile-root-redacted',
    profileScopeId: 'managed-profile-scope-dev',
    profileLifecycleState: 'ready',
    policyRevision: 'browser-policy-revision-dev',
    processId: 5150,
    bridgeKind: BrowserBridgeKind.ChromiumDevtoolsProtocol,
    bridgeEndpointRef: 'managed-loopback-devtools-redacted',
    managedState: BrowserManagedState.RunningManaged,
    capabilityStatus: BrowserCapabilityStatus.BridgeMissing,
    degradedReason: 'managed-browser-bridge-connect-pending',
    startedAt: '2026-05-21T03:29:50Z',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}
