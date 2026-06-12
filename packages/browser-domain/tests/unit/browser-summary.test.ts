import { describe, expect, it } from 'vitest';
import {
  BrowserActiveProofSource,
  BrowserActiveTabState,
  BrowserCustodyLabel,
  BrowserEvidenceRecentSummarySchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamily,
} from '../../src/browser';

describe('browser evidence recent summary contracts', () => {
  it('accepts empty latest-browser summaries without inventing evidence rows', () => {
    const parsed = BrowserEvidenceRecentSummarySchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      returned: 0,
      latestEventId: null,
      latestObservedAt: null,
      browserEvidenceId: null,
      sourceId: null,
      adapterId: null,
      managedBrowserSessionId: null,
      browserFamily: null,
      activeState: null,
      activeProofSource: null,
      url: null,
      origin: null,
      domain: null,
      title: null,
      capabilityStatus: null,
      custodyLabel: null,
    });

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.returned).toBe(0);
      expect(parsed.data.url).toBeNull();
    }
  });

  it('rejects unsupported browser capability spellings', () => {
    const parsed = BrowserEvidenceRecentSummarySchema.safeParse({
      schemaVersion: BrowserEvidenceSchemaVersion,
      returned: 1,
      latestEventId: 'activity-url-observed-1',
      latestObservedAt: '2026-05-21T01:00:00Z',
      browserEvidenceId: 'browser-evidence-1',
      sourceId: 'managed-chromium-devtools',
      adapterId: 'managed-chromium-devtools-adapter',
      managedBrowserSessionId: 'managed-browser-session-1',
      browserFamily: BrowserFamily.Chrome,
      activeState: BrowserActiveTabState.Unknown,
      activeProofSource: BrowserActiveProofSource.TargetListOnly,
      url: 'https://example.test/learn',
      origin: 'https://example.test',
      domain: 'example.test',
      title: 'Example learning page',
      capabilityStatus: 'maybe-captured',
      custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    });

    expect(parsed.success).toBe(false);
  });
});
