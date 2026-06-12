import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '../../src/primitives';
import {
  AgentAppGameBoundaryReadModelKind,
  parseAgentAppGameBoundaryReadModelEvent,
} from '../../src/app-game-boundary-read-model';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const BoundaryReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-04T01:55:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'notClaimed',
  returned: 2,
  evidenceClaimRowCount: 1,
  identityRowCount: 0,
  approvalAuthorityRowCount: 0,
  approvalActionResultRowCount: 0,
  platformAuthorityMatrixCount: 0,
  platformAuthorityRowCount: 0,
  aiClassifierResultRowCount: 1,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGameBoundaryReadModelKind.EvidenceClaim,
      boundaryKind: AgentAppGameBoundaryReadModelKind.EvidenceClaim,
      rowCount: 1,
      evidenceReferenceIds: ['claim-1'],
      evidence: [
        {
          evidenceId: 'claim-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGameBoundaryReadModelKind.AiClassifierResult,
      boundaryKind: AgentAppGameBoundaryReadModelKind.AiClassifierResult,
      rowCount: 1,
      evidenceReferenceIds: ['classifier-run-1', 'source-evidence-1'],
      evidence: [
        {
          evidenceId: 'classifier-run-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
  ],
} as const;

describe('agent app-game boundary read-model parser', () => {
  it('parses the dedicated boundary read-model event payload', () => {
    const parsed = parseAgentAppGameBoundaryReadModelEvent(boundaryEvent(JSON.stringify(BoundaryReadModel)));

    expect(parsed).toEqual({
      ok: true,
      value: BoundaryReadModel,
    });
  });

  it('rejects wrong events and invalid payloads without inventing boundary rows', () => {
    expect(
      parseAgentAppGameBoundaryReadModelEvent({
        ...boundaryEvent(JSON.stringify(BoundaryReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameBoundaryReadModelEvent(boundaryEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameBoundaryReadModelEvent(
        boundaryEvent(JSON.stringify({ ...BoundaryReadModel, rows: [{ rowCount: -1 }] }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function boundaryEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-boundary-read-model-event',
    correlationId: 'app-game-boundary-read-model-command',
    sentAt: '2026-06-04T01:55:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameBoundaryReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameBoundaryReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
