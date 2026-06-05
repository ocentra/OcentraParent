import { describe, expect, it } from 'vitest';
import {
  screenAiRemoteBoundaryProofCoversRequiredRows,
  ScreenAiRemoteBoundaryProofSchema,
  ScreenAiRemoteBoundaryRowSchema,
  summarizeScreenAiRemoteBoundaryProof,
} from '../src/screen-ai-remote-boundary-proof';

describe('screen AI remote boundary proof', () => {
  it('keeps child-safety screen AI local-only while parent remote states are non-authoritative', () => {
    const proof = parseProof();

    expect(screenAiRemoteBoundaryProofCoversRequiredRows(proof)).toBe(true);
    expect(summarizeScreenAiRemoteBoundaryProof(proof)).toEqual({
      rowCount: 3,
      childSafetyLocalOnlyRows: 1,
      parentOnlyRemoteStateRows: 2,
      childSafetyRemoteClaimRows: 0,
      remotePolicyAuthorityRows: 0,
      remoteEnforcementRows: 0,
      rawImageRetainedRows: 0,
    });
  });

  it('rejects child-safety rows that authorize or execute remote API AI', () => {
    const row = proofRows()[0];

    expect(() =>
      ScreenAiRemoteBoundaryRowSchema.parse({
        ...row,
        rowId: 'invalid-child-safety-remote-api',
        remoteApiCredentialState: 'authorized-unavailable',
      })
    ).toThrow(/child safety local-only/u);

    expect(() =>
      ScreenAiRemoteBoundaryRowSchema.parse({
        ...row,
        rowId: 'invalid-child-safety-remote-executed',
        remoteApiExecutionState: 'unavailable',
      })
    ).toThrow(/child safety local-only/u);
  });

  it('rejects remote parent rows that are allowed into child-safety policy input', () => {
    const row = proofRows()[1];

    expect(() =>
      ScreenAiRemoteBoundaryRowSchema.parse({
        ...row,
        rowId: 'invalid-parent-assistant-child-safety-input',
        childSafetyInputAllowed: true,
      })
    ).toThrow(/non-authoritative/u);
  });

  it('rejects any remote policy authority enforcement or raw-image retention claim', () => {
    const row = proofRows()[2];

    expect(() =>
      ScreenAiRemoteBoundaryRowSchema.parse({
        ...row,
        rowId: 'invalid-remote-policy-authority',
        claimFlags: {
          ...row.claimFlags,
          remoteResultCanSetPolicy: true,
        },
      })
    ).toThrow(/remote child-safety, policy, enforcement, and raw-image claims/u);

    expect(() =>
      ScreenAiRemoteBoundaryRowSchema.parse({
        ...row,
        rowId: 'invalid-remote-raw-image-retained',
        claimFlags: {
          ...row.claimFlags,
          rawScreenImageRetained: true,
        },
      })
    ).toThrow(/remote child-safety, policy, enforcement, and raw-image claims/u);
  });
});

function parseProof() {
  return ScreenAiRemoteBoundaryProofSchema.parse({
    schemaVersion: 'v0.6',
    proofId: 'screen-ai-remote-boundary-proof',
    generatedAt: '2026-06-05T21:26:00.000Z',
    rows: proofRows(),
  });
}

function proofRows() {
  return [
    baseRow({
      rowId: 'screen-ai-child-safety-local-only',
      purpose: 'child-safety-screen-analysis',
      evidenceKind: 'screen-summary',
      boundaryState: 'child-safety-local-only',
      decision: 'route-child-local',
      childSafetyInputAllowed: true,
      parentOnlySurfaceAllowed: false,
      localRuntimeRequired: true,
      remoteApiCredentialState: 'not-used',
      remoteApiExecutionState: 'not-executed',
    }),
    baseRow({
      rowId: 'screen-ai-parent-assistant-api-unavailable',
      purpose: 'parent-assistant',
      evidenceKind: 'parent-assistant-context',
      boundaryState: 'parent-assistant-api-authorized-unavailable',
      decision: 'surface-parent-unavailable',
      childSafetyInputAllowed: false,
      parentOnlySurfaceAllowed: true,
      localRuntimeRequired: false,
      remoteApiCredentialState: 'authorized-unavailable',
      remoteApiExecutionState: 'unavailable',
    }),
    baseRow({
      rowId: 'screen-ai-parent-report-api-degraded',
      purpose: 'parent-report',
      evidenceKind: 'parent-report-context',
      boundaryState: 'parent-report-api-authorized-degraded',
      decision: 'surface-parent-degraded',
      childSafetyInputAllowed: false,
      parentOnlySurfaceAllowed: true,
      localRuntimeRequired: false,
      remoteApiCredentialState: 'authorized-degraded',
      remoteApiExecutionState: 'degraded',
    }),
  ];
}

function baseRow(overrides: Record<string, unknown>) {
  return {
    schemaVersion: 'v0.6',
    rowId: 'screen-ai-remote-boundary-row',
    purpose: 'child-safety-screen-analysis',
    evidenceKind: 'screen-summary',
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: 'screen-summary-evidence-wikipedia-school',
        kind: 'activity-event',
        observedAt: '2026-06-05T21:26:00.000Z',
      },
    ],
    sourceArtifacts: ['output/ai-plan-proof/screen-summary-ai-context/proof-summary.json'],
    boundaryState: 'child-safety-local-only',
    decision: 'route-child-local',
    childSafetyInputAllowed: true,
    parentOnlySurfaceAllowed: false,
    localRuntimeRequired: true,
    remoteApiCredentialState: 'not-used',
    remoteApiExecutionState: 'not-executed',
    rawImageState: 'deleted',
    claimFlags: {
      remoteAiUsedForChildSafety: false,
      remoteApiAllowedForChildSafety: false,
      remoteResultCanSetPolicy: false,
      remoteResultCanEnforce: false,
      rawScreenImageRetained: false,
    },
    ...overrides,
  };
}
