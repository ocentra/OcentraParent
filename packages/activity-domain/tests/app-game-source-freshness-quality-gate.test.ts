import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../src/kinds';
import {
  buildAppGameSourceFreshnessQualityReport,
  parseAppGameSourceFreshnessQualityRows,
} from '../src/app-game-source-freshness-quality-gate';

const EvidenceRef = {
  evidenceId: 'journal-entry-source-freshness-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:source-freshness',
  uri: null,
} as const;

const RecentObservedAt = '2026-06-05T19:58:00Z';
const GeneratedAt = '2026-06-05T20:00:00Z';
const StaleAfterMs = 10 * 60 * 1000;

function sourceRow(overrides = {}) {
  return {
    sourceKind: 'processSnapshot',
    state: 'ready',
    rowCount: 2,
    lastObservedAt: RecentObservedAt,
    capabilityStatus: 'available',
    evidence: [EvidenceRef],
    ...overrides,
  } as const;
}

describe('app-game source freshness quality states', () => {
  it('marks only recent evidenced source rows as policy-eligible fresh sources', () => {
    const report = buildAppGameSourceFreshnessQualityReport({
      generatedAt: GeneratedAt,
      staleAfterMs: StaleAfterMs,
      requiredSources: ['processSnapshot', 'foregroundWindow'],
      sourceStatusRows: [
        sourceRow(),
        sourceRow({
          sourceKind: 'foregroundWindow',
          lastObservedAt: '2026-06-05T19:30:00Z',
        }),
      ],
    });

    expect(report.summary.freshSources).toBe(1);
    expect(report.summary.staleSources).toBe(1);
    expect(report.summary.policyEligibleFreshSources).toBe(1);
    expect(report.rows[0]?.qualityState).toBe('fresh');
    expect(report.rows[0]?.adapterDispatchClaimed).toBe(false);
    expect(report.rows[1]?.reason).toBe('older-than-threshold');
  });

  it('keeps missing and empty required sources distinct from fresh coverage', () => {
    const report = buildAppGameSourceFreshnessQualityReport({
      generatedAt: GeneratedAt,
      staleAfterMs: StaleAfterMs,
      requiredSources: ['osInstalledRecord', 'storePackage'],
      sourceStatusRows: [
        sourceRow({
          sourceKind: 'storePackage',
          rowCount: 0,
          lastObservedAt: RecentObservedAt,
          evidence: [],
        }),
      ],
    });

    expect(report.summary.missingSources).toBe(1);
    expect(report.summary.emptySources).toBe(1);
    expect(report.summary.policyEligibleFreshSources).toBe(0);
    expect(report.rows.map((row) => row.qualityState)).toEqual(['missing', 'empty']);
  });
});

describe('app-game source freshness quality no-claim guards', () => {
  it('keeps manual-required and unavailable source rows out of policy eligibility', () => {
    const report = buildAppGameSourceFreshnessQualityReport({
      generatedAt: GeneratedAt,
      staleAfterMs: StaleAfterMs,
      requiredSources: ['shortcut', 'inventoryScan', 'launcherManifest'],
      sourceStatusRows: [
        sourceRow({
          sourceKind: 'shortcut',
          state: 'permission-required',
          capabilityStatus: 'permissionLimited',
        }),
        sourceRow({
          sourceKind: 'inventoryScan',
          state: 'unavailable',
          capabilityStatus: 'adapterError',
        }),
        sourceRow({
          sourceKind: 'launcherManifest',
          state: 'ready',
          capabilityStatus: 'manualRequired',
        }),
      ],
    });

    expect(report.summary.manualRequiredSources).toBe(2);
    expect(report.summary.unavailableSources).toBe(1);
    expect(report.summary.policyEligibleFreshSources).toBe(0);
    expect(report.rows.map((row) => row.reason)).toEqual(['permission-limited', 'source-error', 'permission-limited']);
  });

  it('rejects malformed source status rows before quality evaluation', () => {
    expect(() =>
      parseAppGameSourceFreshnessQualityRows([
        {
          ...sourceRow(),
          sourceKind: 'not-a-real-source',
        },
      ])
    ).toThrow();
  });
});
