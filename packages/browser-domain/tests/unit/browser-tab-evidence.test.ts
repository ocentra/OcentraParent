import { describe, expect, it } from 'vitest';
import {
  BrowserActiveProofSource,
  BrowserActiveTabState,
  BrowserCapabilityStatus,
  BrowserChannel,
  BrowserCustodyLabel,
  BrowserFamily,
  BrowserQueryVisibilityLabel,
} from '@ocentra-parent/schema-domain/browser-values';
import { BrowserEvidenceSchemaVersion, BrowserTabEvidenceSchema } from '@ocentra-parent/schema-domain/browser-schemas';

describe('browser tab evidence contract', () => {
  it('accepts tab evidence with normalized URL origin and domain', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse(tabEvidence());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.origin).toBe('https://example.test');
      expect(parsed.data.domain).toBe('example.test');
      expect(parsed.data.capabilityStatus).toBe('tab-list-only');
      expect(parsed.data.activeProofSource).toBe('target-list-only');
    }
  });

  it('rejects tab evidence when origin does not match URL authority', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      origin: 'https://other.test',
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects tab evidence when domain does not match URL authority', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      domain: 'other.test',
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects evidence rows that still contain credential-bearing raw URLs', () => {
    const normalized = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      url: 'https://example.test/learn',
      origin: 'https://example.test',
      domain: 'example.test',
    });
    const raw = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      url: 'https://child:secret@example.test/learn',
      origin: 'https://child:secret@example.test',
      domain: 'example.test',
    });

    expect(normalized.success).toBe(true);
    expect(raw.success).toBe(false);
  });

  it('rejects target-list evidence promoted to known active', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      activeState: BrowserActiveTabState.KnownActive,
      activeProofSource: BrowserActiveProofSource.TargetListOnly,
    });

    expect(parsed.success).toBe(false);
  });

  it('accepts known active only with an explicit active proof source', () => {
    const parsed = BrowserTabEvidenceSchema.safeParse({
      ...tabEvidence(),
      activeState: BrowserActiveTabState.KnownActive,
      activeProofSource: BrowserActiveProofSource.CdpFocusActivation,
      capabilityStatus: BrowserCapabilityStatus.Available,
    });

    expect(parsed.success).toBe(true);
  });
});

function tabEvidence() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    browserEvidenceId: 'browser-evidence-0-target-1-2026-05-20T00:00:00Z',
    observedAt: '2026-05-20T00:00:00Z',
    freshUntil: '2026-05-20T00:01:00Z',
    sourceId: 'managed-chromium-devtools',
    adapterId: 'managed-chromium-devtools-adapter',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    managedBrowserSessionId: 'managed-browser-session-dev',
    profileId: 'managed-browser-profile-dev',
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
    staleAt: '2026-05-20T00:01:00Z',
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    queryVisibility: BrowserQueryVisibilityLabel.LiveLocal,
  };
}
