import { describe, expect, it } from 'vitest';
import {
  ScreenVlmJournalReadModelProofSchema,
  ScreenVlmJournalReadModelProofTier,
  ScreenVlmJournalReadModelSchemaVersion,
  screenVlmJournalLineFromCompletedStatus,
  screenVlmJournalReadModelProjection,
  screenVlmReadModelFromCompletedStatus,
} from '../../src/screen-vlm-journal-read-model';
import {
  screenVlmCompletedStatusFromResult,
  screenVlmQueueHandoffFromJob,
  screenVlmQueuedStatusFromHandoff,
} from '../../src/screen-vlm-execution-readiness';
import {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerMaxImagePixels,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerSchemaVersion,
  ScreenVlmWorkerTemplateVersion,
} from '../../src/screen-vlm-worker';
import { ActivitySurfaceSchemaVersion } from '../../src/activity-surface';

const EvidenceRef = {
  evidenceId: 'screen-vlm-journal-read-model-source',
  kind: 'journal-entry',
  digest: 'sha256:screen-vlm-journal-read-model-image',
  uri: null,
} as const;

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-vlm-journal',
  },
  requestedAt: '2026-06-06T00:55:00.000Z',
  rangeStart: '2026-06-06T00:00:00.000Z',
  rangeEnd: '2026-06-06T00:55:00.000Z',
} as const;

const VlmJob = ScreenVlmWorkerJobSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  queueJobId: 'screen-vlm-journal-read-model-job',
  createdAt: '2026-06-06T00:50:00.000Z',
  captureReason: 'timedCadence',
  captureScope: 'selectedWindow',
  capabilityStatus: 'ready',
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: EvidenceRef.digest,
  encryptedImageRef: 'encrypted-temp-screen-vlm-journal-read-model-image',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  prompt: 'Classify this encrypted local selected-window capture for parent-visible screen history.',
  maxImagePixels: ScreenVlmWorkerMaxImagePixels,
  providerKind: 'localVision',
  custodyState: 'child-device-temp-queue',
  localOnly: true,
  remoteAiUsed: false,
  rawImageRetained: false,
});

const VlmResult = ScreenVlmWorkerResultSchema.parse({
  schemaVersion: ScreenVlmWorkerSchemaVersion,
  vlmResultId: 'screen-vlm-journal-read-model-result',
  queueJobId: VlmJob.queueJobId,
  analyzedAt: '2026-06-06T00:50:06.000Z',
  modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
  modelId: ScreenVlmWorkerModelId,
  promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
  providerKind: 'localVision',
  captureReason: VlmJob.captureReason,
  captureScope: VlmJob.captureScope,
  capabilityStatus: 'ready',
  modelOutput: {
    primary_category: 'school',
    confidence: 0.88,
    visible_text: 'A school assignment page is visible in the selected window.',
    risk_signals: [],
  },
  summary: 'The local VLM result classified the selected-window capture as school.',
  visibleCategoryCandidates: [
    {
      category: 'school',
      confidence: 0.88,
      evidenceRefs: [EvidenceRef],
    },
  ],
  primaryCategory: 'school',
  riskSignals: [],
  redactionNotes: [],
  confidence: 0.88,
  uncertaintyReason: null,
  sourceEvidenceRefs: [EvidenceRef],
  imageDigest: EvidenceRef.digest,
  rawImageRetained: false,
  imageDeletionState: 'deleted',
  custodyState: 'child-device-query-store',
  policyEligible: true,
  localOnly: true,
  remoteAiUsed: false,
});

const Handoff = screenVlmQueueHandoffFromJob({
  job: VlmJob,
  handoffId: 'screen-vlm-journal-read-model-handoff',
  queuedAt: '2026-06-06T00:50:01.000Z',
  acceptedAt: '2026-06-06T00:50:02.000Z',
  statusReason: 'The encrypted selected-window capture is ready for local VLM status projection.',
});

const QueuedStatus = screenVlmQueuedStatusFromHandoff({
  handoff: Handoff,
  statusId: 'screen-vlm-journal-read-model-queued',
  updatedAt: '2026-06-06T00:50:02.000Z',
});

const CompletedStatus = screenVlmCompletedStatusFromResult({
  result: VlmResult,
  statusId: 'screen-vlm-journal-read-model-completed',
});

function journalLine() {
  return screenVlmJournalLineFromCompletedStatus({
    statusRow: CompletedStatus,
    entryId: 'screen-vlm-journal-read-model-entry',
    segmentId: 'screen-vlm-journal-read-model-segment',
    writtenAt: '2026-06-06T00:50:07.000Z',
    nonce: 'screen-vlm-journal-read-model-nonce',
    ciphertext: 'encrypted-screen-vlm-journal-read-model-status-row',
  });
}

function readModel() {
  return screenVlmReadModelFromCompletedStatus({
    statusRow: CompletedStatus,
    request: ActivityRequest,
    generatedAt: '2026-06-06T00:50:08.000Z',
    rowId: 'screen-vlm-journal-read-model-row',
    label: 'School window classified by local VLM',
    deviceId: 'child-device-vlm-journal',
    journalEntryId: 'screen-vlm-journal-read-model-entry',
    policyDecisionRef: 'screen-vlm-journal-read-model-policy-dry-run',
    policyAction: 'allow',
    policyReasonCodes: ['school-content-allowed'],
    parentRuleRefs: ['parent-rule-school-hours'],
    parentExplanationRefs: ['parent-explanation-school-window'],
    explanationReasons: ['journal-entry-cited', 'deleted-image-custody-cited'],
  });
}

describe('screen VLM journal read-model contracts', () => {
  it('projects a completed VLM status row into encrypted journal and Activity Screen rows', () => {
    const journal = journalLine();
    const model = readModel();
    const projection = screenVlmJournalReadModelProjection({
      projectionId: 'screen-vlm-journal-read-model-projection',
      statusRow: CompletedStatus,
      journalLine: journal,
      readModel: model,
    });

    expect(projection.journalLine.eventId).toBe(CompletedStatus.statusId);
    expect(projection.readModel.rows[0]?.queueJobId).toBe(CompletedStatus.queueJobId);
    expect(projection.readModel.rows[0]?.modelRuntimeRef).toBe(ScreenVlmWorkerRuntimeRef);
    expect(projection.readModel.rows[0]?.modelId).toBe(ScreenVlmWorkerModelId);
    expect(projection.readModel.rows[0]?.promptOrTemplateVersion).toBe(ScreenVlmWorkerTemplateVersion);
    expect(projection.readModel.rows[0]?.rawImageRetained).toBe(false);
    expect(projection.readModel.rows[0]?.imageDeletionState).toBe('deleted');
  });
});

describe('screen VLM journal read-model contract guards', () => {
  it('rejects queued rows before analysis and completed rows before deletion', () => {
    expect(() =>
      screenVlmJournalLineFromCompletedStatus({
        statusRow: QueuedStatus,
        entryId: 'screen-vlm-journal-read-model-entry',
        segmentId: 'screen-vlm-journal-read-model-segment',
        writtenAt: '2026-06-06T00:50:07.000Z',
        nonce: 'screen-vlm-journal-read-model-nonce',
        ciphertext: 'encrypted-screen-vlm-journal-read-model-status-row',
      })
    ).toThrow();

    const undeleted = {
      ...CompletedStatus,
      statusId: 'screen-vlm-journal-read-model-undeleted',
      custodyState: 'child-device-temp-queue',
      result: {
        ...VlmResult,
        imageDeletionState: 'deletePending',
        custodyState: 'child-device-temp-queue',
      },
    } as unknown as typeof CompletedStatus;

    expect(() =>
      screenVlmJournalReadModelProjection({
        projectionId: 'screen-vlm-journal-read-model-undeleted-projection',
        statusRow: undeleted,
        journalLine: journalLine(),
        readModel: readModel(),
      })
    ).toThrow();
  });

  it('rejects parent-visible rows that retain raw images or omit journal evidence', () => {
    const journal = journalLine();
    const model = readModel();

    expect(() =>
      screenVlmJournalReadModelProjection({
        projectionId: 'screen-vlm-journal-read-model-raw-retained',
        statusRow: CompletedStatus,
        journalLine: journal,
        readModel: {
          ...model,
          rows: [
            {
              ...model.rows[0],
              rawImageRetained: true,
            },
          ],
        } as unknown as typeof model,
      })
    ).toThrow();

    expect(() =>
      screenVlmJournalReadModelProjection({
        projectionId: 'screen-vlm-journal-read-model-missing-journal-ref',
        statusRow: CompletedStatus,
        journalLine: journal,
        readModel: {
          ...model,
          rows: [
            {
              ...model.rows[0],
              evidence: [],
            },
          ],
        },
      })
    ).toThrow();
  });
});

describe('screen VLM journal read-model proof', () => {
  it('proves VLM status journal/read-model projections without portal or enforcement claims', () => {
    const proof = ScreenVlmJournalReadModelProofSchema.parse({
      schemaVersion: ScreenVlmJournalReadModelSchemaVersion,
      proofId: 'screen-vlm-journal-read-model-proof',
      proofTier: ScreenVlmJournalReadModelProofTier,
      projections: [
        screenVlmJournalReadModelProjection({
          projectionId: 'screen-vlm-journal-read-model-projection',
          statusRow: CompletedStatus,
          journalLine: journalLine(),
          readModel: readModel(),
        }),
      ],
      localOnly: true,
      remoteAiUsed: false,
      rawImageRetained: false,
      portalRuntimeClaimed: false,
      enforcementClaimed: false,
    });

    expect(proof.projections).toHaveLength(1);
    expect(proof.projections[0]?.nonClaims.portalRuntimeClaimed).toBe(false);
    expect(proof.projections[0]?.readModel.rows[0]?.policyDecisionRef).toBe(
      'screen-vlm-journal-read-model-policy-dry-run'
    );
  });
});
