import { describe, expect, it } from 'vitest';
import {
  BrowserActiveTabCapability,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserEvidenceSchemaVersion,
  BrowserExactUrlCapability,
  BrowserFamily,
  BrowserInventoryInstallState,
  BrowserInventoryReadModelSchema,
  BrowserInventoryRowSchema,
  BrowserInventoryRunningState,
  BrowserManagedProfileState,
  BrowserManagementTier,
  BrowserQueryVisibilityLabel,
  BrowserSupportTier,
  BrowserUnmanagedFallbackCapability,
} from '../../src/browser';

describe('browser inventory contracts', () => {
  it('accepts managed inventory rows with target-list-only exact evidence boundaries', () => {
    const parsed = BrowserInventoryReadModelSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      generatedAt: '2026-06-02T18:40:00Z',
      limit: 10,
      returned: 1,
      latestObservedAt: '2026-06-02T18:39:59Z',
      capabilityStatus: BrowserCapabilityStatus.TabListOnly,
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
      rows: [managedEdgeInventoryRow()],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows[0].managementTier).toBe('managed');
      expect(parsed.data.rows[0].exactUrlCapability).toBe('managed-target-list-only');
      expect(parsed.data.rows[0].activeTabCapability).toBe('target-list-only');
    }
  });

  it('accepts unmanaged browser rows only as process or bypass evidence', () => {
    const parsed = BrowserInventoryRowSchema.safeParse(unmanagedChromeInventoryRow());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.managementTier).toBe('unmanaged');
      expect(parsed.data.supportTier).toBe('unmanaged-process-only');
      expect(parsed.data.exactUrlCapability).toBe('not-claimed');
      expect(parsed.data.unmanagedFallbackCapability).toBe('report-only');
    }
  });

  it('rejects unmanaged inventory rows that claim managed exact URL support', () => {
    const row = {
      ...unmanagedChromeInventoryRow(),
      exactUrlCapability: BrowserExactUrlCapability.ManagedExactUrlAvailable,
      activeTabCapability: BrowserActiveTabCapability.KnownActiveSupported,
    };

    const parsed = BrowserInventoryRowSchema.safeParse(row);

    expect(parsed.success).toBe(false);
  });

  it('rejects unsupported browsers unless the support tier is also unsupported', () => {
    const row = {
      ...unsupportedFirefoxInventoryRow(),
      supportTier: BrowserSupportTier.Candidate,
    };

    const parsed = BrowserInventoryRowSchema.safeParse(row);

    expect(parsed.success).toBe(false);
  });
});

describe('browser inventory catalog refs', () => {
  it('accepts mixed inventory catalog rows with publisher and hash refs', () => {
    const parsed = BrowserInventoryReadModelSchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      generatedAt: '2026-06-02T18:40:00Z',
      limit: 10,
      returned: 3,
      latestObservedAt: '2026-06-02T18:39:59Z',
      capabilityStatus: null,
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
      queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
      rows: [managedEdgeInventoryRow(), unmanagedChromeInventoryRow(), unsupportedFirefoxInventoryRow()],
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.rows).toHaveLength(3);
      expect(parsed.data.rows[1].publisherSignatureRef).toBe('chrome-publisher-signature-ref');
      expect(parsed.data.rows[1].fileHashRef).toBe('chrome-file-hash-ref');
      expect(parsed.data.rows[1].exactUrlCapability).toBe('not-claimed');
    }
  });

  it('rejects empty inventory identity refs', () => {
    const row = {
      ...unmanagedChromeInventoryRow(),
      fileHashRef: '',
    };

    const parsed = BrowserInventoryRowSchema.safeParse(row);

    expect(parsed.success).toBe(false);
  });
});

function managedEdgeInventoryRow() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    inventoryRowId: 'browser-inventory-edge-stable',
    scannedAt: '2026-06-02T18:39:59Z',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Edge,
    browserChannel: BrowserChannel.Stable,
    productName: 'Microsoft Edge',
    browserVersion: '125.0.0',
    installState: BrowserInventoryInstallState.Installed,
    runningState: BrowserInventoryRunningState.RunningManaged,
    managementTier: BrowserManagementTier.Managed,
    supportTier: BrowserSupportTier.ManagedTargetList,
    exactUrlCapability: BrowserExactUrlCapability.ManagedTargetListOnly,
    activeTabCapability: BrowserActiveTabCapability.TargetListOnly,
    managedProfileState: BrowserManagedProfileState.Ready,
    unmanagedFallbackCapability: BrowserUnmanagedFallbackCapability.OsBlockManualRequired,
    executablePathRef: 'edge-stable-redacted-path',
    publisherSignatureRef: null,
    fileHashRef: null,
    profileId: 'managed-browser-profile-dev',
    processId: 4242,
    capabilityStatus: BrowserCapabilityStatus.TabListOnly,
    reasonCode: 'managed-target-list-active-tab-unproved',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}

function unmanagedChromeInventoryRow() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    inventoryRowId: 'browser-inventory-chrome-unmanaged',
    scannedAt: '2026-06-02T18:39:59Z',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    productName: 'Google Chrome',
    browserVersion: null,
    installState: BrowserInventoryInstallState.CandidateRunning,
    runningState: BrowserInventoryRunningState.RunningUnmanaged,
    managementTier: BrowserManagementTier.Unmanaged,
    supportTier: BrowserSupportTier.UnmanagedProcessOnly,
    exactUrlCapability: BrowserExactUrlCapability.NotClaimed,
    activeTabCapability: BrowserActiveTabCapability.NotClaimed,
    managedProfileState: BrowserManagedProfileState.NotApplicable,
    unmanagedFallbackCapability: BrowserUnmanagedFallbackCapability.ReportOnly,
    executablePathRef: 'chrome-unmanaged-redacted-path',
    publisherSignatureRef: 'chrome-publisher-signature-ref',
    fileHashRef: 'chrome-file-hash-ref',
    profileId: null,
    processId: 5150,
    capabilityStatus: BrowserCapabilityStatus.UnmanagedBrowser,
    reasonCode: 'unmanaged-browser-process-only',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
  };
}

function unsupportedFirefoxInventoryRow() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    inventoryRowId: 'browser-inventory-firefox',
    scannedAt: '2026-06-02T18:39:59Z',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Firefox,
    browserChannel: BrowserChannel.Stable,
    productName: 'Mozilla Firefox',
    browserVersion: null,
    installState: BrowserInventoryInstallState.Installed,
    runningState: BrowserInventoryRunningState.NotRunning,
    managementTier: BrowserManagementTier.Unsupported,
    supportTier: BrowserSupportTier.Unsupported,
    exactUrlCapability: BrowserExactUrlCapability.Unsupported,
    activeTabCapability: BrowserActiveTabCapability.Unsupported,
    managedProfileState: BrowserManagedProfileState.NotApplicable,
    unmanagedFallbackCapability: BrowserUnmanagedFallbackCapability.Unsupported,
    executablePathRef: 'firefox-redacted-path',
    publisherSignatureRef: 'firefox-publisher-signature-ref',
    fileHashRef: null,
    profileId: null,
    processId: null,
    capabilityStatus: BrowserCapabilityStatus.UnsupportedBrowser,
    reasonCode: 'firefox-adapter-not-proved',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.Unavailable,
  };
}
