import { describe, expect, it } from 'vitest';
import type { GeneratedPortalScreenSummaryPanelSnapshot } from '../../src/generated-portal-contracts';
import { createScreenSummaryPanelIntent } from '../../src/screen-summary-panel';

describe('screen summary panel intent', () => {
  it('renders generated screen summary rows for the parent portal', () => {
    const intent = createScreenSummaryPanelIntent(screenSummaryPanel());

    expect(intent.loadState).toBe('Ready');
    expect(intent.summaryDetails).toContainEqual({ label: 'Rows returned', value: '1' });
    expect(intent.summaryDetails).toContainEqual({ label: 'Capability', value: 'Available' });
    expect(intent.summaryDetails).toContainEqual({ label: 'Deleted evidence', value: 'deleted' });
    expect(intent.rows).toHaveLength(1);
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Evidence references',
      value: 'screen-summary-ref | screen-audit-ref',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Policy check',
      value: 'screen-policy-decision-ref',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Decision action',
      value: 'allow',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Reason',
      value: 'local-ai-screen-summary | stricter-parent-rule-checked',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Parent explanation refs',
      value: 'screen-parent-explanation-ref',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Enforcement handoff',
      value: 'Not claimed',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'OCR snippets',
      value: 'Homework research page [redacted]',
    });
    expect(intent.rows[0]?.details).toContainEqual({
      label: 'Redaction notes',
      value: 'credentialLikeTextRedacted | piiLikeTextRedacted',
    });
    expect(JSON.stringify(intent.rows[0])).not.toContain('student@example.com');
    expect(JSON.stringify(intent.rows[0])).not.toContain('hunter2');
  });

  it('keeps unavailable screen rows visible without inventing data', () => {
    const unavailable = createScreenSummaryPanelIntent(null);

    expect(unavailable.loadState).toBe('Unavailable');
    expect(unavailable.rows).toHaveLength(0);
    expect(unavailable.summaryDetails).toContainEqual({
      label: 'Product claim',
      value: 'No family setting is configured for this area yet.',
    });
  });
});

function screenSummaryPanel(): GeneratedPortalScreenSummaryPanelSnapshot {
  return {
    eyebrow: 'Activity kind',
    title: 'Screen analysis',
    body: 'Stored activity',
    loadState: 'Ready',
    summaryDetails: [
      { label: 'Status', value: 'Ready' },
      { label: 'Generated at', value: '2026-06-06T22:20:00Z' },
      { label: 'Rows returned', value: '1' },
      { label: 'Capability', value: 'Available' },
      { label: 'Custody', value: 'Child device query store' },
      { label: 'Deleted evidence', value: 'deleted' },
      { label: 'Model', value: 'windows-media-ocr | screen-ocr-v1 | screen-queue-job-1' },
      { label: 'Product claim', value: 'No family setting is configured for this area yet.' },
    ],
    rows: [
      {
        title: 'School research window',
        details: [
          { label: 'Status', value: 'Ready' },
          { label: 'Event ID', value: 'screen-row-school-research' },
          { label: 'Source', value: 'timedCadence' },
          { label: 'Capability', value: 'Available' },
          { label: 'Runtime reference', value: 'winrt-ocr-runtime' },
          { label: 'Model', value: 'windows-media-ocr | screen-ocr-v1 | screen-queue-job-1' },
          { label: 'Provider', value: 'localOcr' },
          { label: 'Level', value: '0.72' },
          { label: 'Activity kind', value: 'school' },
          { label: 'Custody', value: 'Child device query store' },
          { label: 'Deleted evidence', value: 'deleted' },
          { label: 'Policy check', value: 'screen-policy-decision-ref' },
          { label: 'Decision action', value: 'allow' },
          { label: 'Enforcement handoff', value: 'Not claimed' },
          { label: 'Evidence references', value: 'screen-summary-ref | screen-audit-ref' },
          { label: 'Reason codes', value: 'screen-school-allow' },
          { label: 'Parent rule context refs', value: 'screen-parent-rule-ref' },
          { label: 'Reason', value: 'local-ai-screen-summary | stricter-parent-rule-checked' },
          { label: 'OCR snippets', value: 'Homework research page [redacted]' },
          { label: 'Redaction notes', value: 'credentialLikeTextRedacted | piiLikeTextRedacted' },
          { label: 'Parent explanation refs', value: 'screen-parent-explanation-ref' },
          { label: 'Product claim', value: 'No family setting is configured for this area yet.' },
        ],
      },
    ],
    emptyMessage: 'No recent activity is available yet.',
    productClaim: 'No family setting is configured for this area yet.',
  };
}
