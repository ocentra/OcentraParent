import { describe, expect, it } from 'vitest';
import {
  ActivityScreenReadModelSchema,
  ActivitySurfaceSchemaVersion,
} from '@ocentra-parent/schema-domain/activity-surface';
import { ActivityEvidenceKind } from '@ocentra-parent/schema-domain/evidence-kinds';
import { createScreenSummaryPanelIntent } from '../../src/screen-summary-panel';

describe('screen summary panel intent', () => {
  it('summarizes Activity Screen rows for the parent portal', () => {
    const intent = createScreenSummaryPanelIntent({
      ok: true,
      state: 'ready',
      value: screenReadModel(),
    });

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

  it('keeps unavailable or parser-failed screen rows visible without inventing data', () => {
    const unavailable = createScreenSummaryPanelIntent(null);
    const failed = createScreenSummaryPanelIntent({
      ok: false,
      state: 'unavailable',
      reason: 'parse-error',
    });

    expect(unavailable.loadState).toBe('Unavailable');
    expect(unavailable.rows).toHaveLength(0);
    expect(unavailable.summaryDetails).toContainEqual({
      label: 'Product claim',
      value: 'No family setting is configured for this area yet.',
    });
    expect(failed.summaryDetails).toContainEqual({ label: 'Reason', value: 'parse-error' });
  });
});

function screenReadModel() {
  return ActivityScreenReadModelSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: {
      schemaVersion: ActivitySurfaceSchemaVersion,
      scope: {
        scopeKind: 'device',
        familyId: null,
        deviceId: 'child-device-screen-summary',
      },
      requestedAt: '2026-06-06T22:19:00Z',
      rangeStart: '2026-06-06T21:20:00Z',
      rangeEnd: '2026-06-06T22:20:00Z',
    },
    state: 'ready',
    generatedAt: '2026-06-06T22:20:00Z',
    summary: 'One screen analysis row is ready for parent review',
    rows: [
      {
        rowId: 'screen-row-school-research',
        label: 'School research window',
        deviceId: 'child-device-screen-summary',
        state: 'ready',
        totalMs: 1800000,
        foregroundMs: 1800000,
        backgroundMs: 0,
        captureReason: 'timedCadence',
        captureScope: 'activeWindow',
        capabilityStatus: 'available',
        queueJobId: 'screen-queue-job-1',
        modelRuntimeRef: 'winrt-ocr-runtime',
        modelId: 'windows-media-ocr',
        providerKind: 'localOcr',
        promptOrTemplateVersion: 'screen-ocr-v1',
        primaryCategory: 'school',
        confidence: 0.72,
        imageDeletionState: 'deleted',
        rawImageRetained: false,
        policyEligible: true,
        imageDigest: 'sha256:screen-digest',
        custodyState: 'child-device-query-store',
        evidence: [
          evidenceRef('screen-summary-ref', 'sha256:screen-summary-ref'),
          evidenceRef('screen-audit-ref', 'sha256:screen-audit-ref'),
        ],
        policyDecisionRef: 'screen-policy-decision-ref',
        policyAction: 'allow',
        policyReasonCodes: ['screen-school-allow'],
        parentRuleRefs: ['screen-parent-rule-ref'],
        localModelRuntimeRefs: ['winrt-ocr-runtime'],
        parentExplanationRefs: ['screen-parent-explanation-ref'],
        explanationReasons: ['local-ai-screen-summary', 'stricter-parent-rule-checked'],
        deletionReasons: ['screen-image-deleted'],
        ocrTextSnippets: ['Homework research page [redacted]'],
        redactionNotes: ['credentialLikeTextRedacted', 'piiLikeTextRedacted'],
      },
    ],
  });
}

function evidenceRef(evidenceId: string, digest: string) {
  return {
    evidenceId,
    kind: ActivityEvidenceKind.JournalEntry,
    digest,
    uri: null,
  } as const;
}
