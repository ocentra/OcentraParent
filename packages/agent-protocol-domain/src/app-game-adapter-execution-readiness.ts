import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const AdapterReadinessText = Schema.String.pipe(Schema.minLength(1));
const AdapterReadinessCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameAdapterExecutionReadinessPayloadField = 'appGameAdapterExecutionReadinessReadModel' as const;

export const AgentAppGameAdapterProductMeaning = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AgentAppGameAdapterExecutionState = {
  ProvedScopedExecution: 'proved-scoped-execution',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
  Unsupported: 'unsupported',
  Degraded: 'degraded',
  NotClaimed: 'not-claimed',
} as const;

export const AgentAppGameAdapterExecutionDecision = {
  ExecutionAllowed: 'execution-allowed',
  BlockedBeforeExecution: 'blocked-before-execution',
} as const;

export const AgentAppGameAdapterHostCapabilityState = {
  Available: 'available',
  NotDetected: 'not-detected',
  NotApplicable: 'not-applicable',
} as const;

const AgentAppGameAdapterExecutionReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  rowId: AdapterReadinessText,
  sourceProofEntryId: AdapterReadinessText,
  platform: AdapterReadinessText,
  productMeanings: Schema.Array(
    Schema.Literal(AgentAppGameAdapterProductMeaning.NativeApp, AgentAppGameAdapterProductMeaning.NativeGame)
  ),
  adapterCapability: AdapterReadinessText,
  adapterExecutionState: Schema.Literal(
    AgentAppGameAdapterExecutionState.ProvedScopedExecution,
    AgentAppGameAdapterExecutionState.ManualRequired,
    AgentAppGameAdapterExecutionState.Unavailable,
    AgentAppGameAdapterExecutionState.Unsupported,
    AgentAppGameAdapterExecutionState.Degraded,
    AgentAppGameAdapterExecutionState.NotClaimed
  ),
  executionDecision: Schema.Literal(
    AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
    AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution
  ),
  runtimeBoundary: AdapterReadinessText,
  targetIdentityState: AdapterReadinessText,
  rollbackReferenceState: AdapterReadinessText,
  auditReferenceState: AdapterReadinessText,
  evidenceRefs: Schema.Array(AdapterReadinessText),
  hostCapabilityState: Schema.Literal(
    AgentAppGameAdapterHostCapabilityState.Available,
    AgentAppGameAdapterHostCapabilityState.NotDetected,
    AgentAppGameAdapterHostCapabilityState.NotApplicable
  ),
  hostCapabilityEvidenceRefs: Schema.Array(AdapterReadinessText),
  hostCapabilityProbeRefs: Schema.Array(AdapterReadinessText),
  linkedProofArtifacts: Schema.Array(AdapterReadinessText),
  manualProofRequirements: Schema.Array(AdapterReadinessText),
  claimBoundary: AdapterReadinessText,
  fallbackBehavior: AdapterReadinessText,
  adapterExecutionClaimed: Schema.Boolean,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  lastCheckedAt: AdapterReadinessText,
});

type AgentAppGameAdapterExecutionReadinessRowCandidate = Infer<
  typeof AgentAppGameAdapterExecutionReadinessRowBaseSchema
>;

export const AgentAppGameAdapterExecutionReadinessRowSchema = withParser(
  AgentAppGameAdapterExecutionReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGameAdapterExecutionReadinessRowCandidate) =>
        adapterExecutionRowIsHonest(row) ||
        'Expected only the Windows owned-process time-limit app/game row to allow adapter execution'
    )
  )
);

const AgentAppGameAdapterExecutionReadinessReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  readModelId: AdapterReadinessText,
  generatedAt: AdapterReadinessText,
  sourceReadModelIds: Schema.Array(AdapterReadinessText),
  custodyLabel: AdapterReadinessText,
  capabilityStatus: AdapterReadinessText,
  returned: AdapterReadinessCount,
  executionAllowedCount: AdapterReadinessCount,
  blockedBeforeExecutionCount: AdapterReadinessCount,
  adapterExecutionClaimedCount: AdapterReadinessCount,
  hostCapabilityAvailableCount: AdapterReadinessCount,
  hostCapabilityNotDetectedCount: AdapterReadinessCount,
  hostCapabilityNotApplicableCount: AdapterReadinessCount,
  hostCapabilityProbeRefCount: AdapterReadinessCount,
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  rows: Schema.Array(AgentAppGameAdapterExecutionReadinessRowSchema),
});

type AgentAppGameAdapterExecutionReadinessReadModelCandidate = Infer<
  typeof AgentAppGameAdapterExecutionReadinessReadModelBaseSchema
>;

export const AgentAppGameAdapterExecutionReadinessReadModelSchema = withParser(
  AgentAppGameAdapterExecutionReadinessReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGameAdapterExecutionReadinessReadModelCandidate) =>
        (readModel.returned === readModel.rows.length &&
          readModel.executionAllowedCount ===
            readModel.rows.filter(
              (row) => row.executionDecision === AgentAppGameAdapterExecutionDecision.ExecutionAllowed
            ).length &&
          readModel.blockedBeforeExecutionCount ===
            readModel.rows.filter(
              (row) => row.executionDecision === AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution
            ).length &&
          readModel.adapterExecutionClaimedCount ===
            readModel.rows.filter((row) => row.adapterExecutionClaimed).length &&
          readModel.hostCapabilityAvailableCount ===
            readModel.rows.filter((row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available)
              .length &&
          readModel.hostCapabilityNotDetectedCount ===
            readModel.rows.filter(
              (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotDetected
            ).length &&
          readModel.hostCapabilityNotApplicableCount ===
            readModel.rows.filter(
              (row) => row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable
            ).length &&
          readModel.hostCapabilityProbeRefCount ===
            readModel.rows.reduce((count, row) => count + row.hostCapabilityProbeRefs.length, 0) &&
          new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length) ||
        'Expected app/game adapter execution readiness counts and row ids to match the rows'
    )
  )
);

export type AgentAppGameAdapterExecutionReadinessRow = Infer<typeof AgentAppGameAdapterExecutionReadinessRowSchema>;
export type AgentAppGameAdapterExecutionReadinessReadModel = Infer<
  typeof AgentAppGameAdapterExecutionReadinessReadModelSchema
>;

export type AgentAppGameAdapterExecutionReadinessFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameAdapterExecutionReadinessResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameAdapterExecutionReadinessReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameAdapterExecutionReadinessFailureReason;
    };

export function parseAgentAppGameAdapterExecutionReadinessEvent(
  event: AgentEventEnvelope
): AgentAppGameAdapterExecutionReadinessResult {
  if (event.event !== AgentEvent.ActivityAppGameAdapterExecutionReadinessReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGameAdapterExecutionReadinessPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGameAdapterExecutionReadinessReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function adapterExecutionRowIsHonest(row: AgentAppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (row.adapterExecutionState === AgentAppGameAdapterExecutionState.ProvedScopedExecution) {
    return (
      row.executionDecision === AgentAppGameAdapterExecutionDecision.ExecutionAllowed &&
      row.platform === 'windows' &&
      row.runtimeBoundary === 'windows-app-game-owned-process-time-limit' &&
      row.adapterExecutionClaimed &&
      row.manualProofRequirements.length === 0 &&
      row.evidenceRefs.length > 0 &&
      row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available &&
      row.hostCapabilityEvidenceRefs.length > 0 &&
      row.hostCapabilityProbeRefs.length > 0 &&
      row.linkedProofArtifacts.length > 0
    );
  }

  return (
    row.executionDecision === AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution &&
    !row.adapterExecutionClaimed &&
    hostCapabilityStateMatchesEvidence(row) &&
    hostCapabilityProbeRefsAreParentSafe(row) &&
    row.manualProofRequirements.length > 0
  );
}

function hostCapabilityStateMatchesEvidence(row: AgentAppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.Available) {
    return row.hostCapabilityEvidenceRefs.length > 0;
  }
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable) {
    return row.hostCapabilityEvidenceRefs.length === 0;
  }
  return true;
}

function hostCapabilityProbeRefsAreParentSafe(row: AgentAppGameAdapterExecutionReadinessRowCandidate): boolean {
  if (row.hostCapabilityState === AgentAppGameAdapterHostCapabilityState.NotApplicable) {
    return row.hostCapabilityProbeRefs.length === 0;
  }
  return row.hostCapabilityProbeRefs.every((ref) => ref.endsWith('-probe-ref'));
}

function adapterFailure(
  reason: AgentAppGameAdapterExecutionReadinessFailureReason
): AgentAppGameAdapterExecutionReadinessResult {
  return {
    ok: false,
    reason,
  };
}
