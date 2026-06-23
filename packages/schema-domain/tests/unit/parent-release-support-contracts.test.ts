import { describe, expect, it } from 'vitest';

import { ParentDesktopReleaseSupportIncidentHandoffSchema } from '../../src/parent-desktop-release-support-incident';
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

describe('parent release-support contracts centralized in schema-domain', () => {
  it('parses a support-safe incident handoff through the shared schema owner', () => {
    const parsed = ParentDesktopReleaseSupportIncidentHandoffSchema.parse(IncidentHandoffFixture);

    expect(parsed.metadata.status).toBe('triage-ready');
    expect(parsed.parentConsent.disclosureState).toBe('shown-before-export');
    expect(parsed.supportBundleManifest.custodyBoundary).toBe('parent-exported-local-bundle');
    expect(parsed.manualProductionSupportStates.supportBackendUploadState).toBe('not-implemented');
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
