import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const PlatformProofStatusText = Schema.String.pipe(Schema.minLength(1));
const PlatformProofStatusCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGamePlatformProofStatusPayloadField = 'appGamePlatformProofStatusReadModel' as const;

export const AgentAppGamePlatformProofStatusPlatform = {
  Windows: 'windows',
  Android: 'android',
  Linux: 'linux',
  Macos: 'macos',
  Ios: 'ios',
} as const;

export const AgentAppGamePlatformProofStatusState = {
  ScopedWindowsExecutionProved: 'scoped-windows-execution-proved',
  AndroidHostVisible: 'android-host-visible',
  AndroidHostNotDetected: 'android-host-not-detected',
  LinuxHostVisible: 'linux-host-visible',
  LinuxHostNotDetected: 'linux-host-not-detected',
  LocalRuntimeNotApplicable: 'local-runtime-not-applicable',
} as const;

export const AgentAppGamePlatformProofStatusAuthority = {
  ScopedExecutionOnly: 'scoped-execution-only',
  VisibilityOnly: 'visibility-only',
  NotLocallyProvable: 'not-locally-provable',
} as const;

export const AgentAppGamePlatformProofStatusHostCapability = {
  Available: 'available',
  NotDetected: 'not-detected',
  NotApplicable: 'not-applicable',
} as const;

const AgentAppGamePlatformProofStatusRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  rowId: PlatformProofStatusText,
  platform: Schema.Literal(
    AgentAppGamePlatformProofStatusPlatform.Windows,
    AgentAppGamePlatformProofStatusPlatform.Android,
    AgentAppGamePlatformProofStatusPlatform.Linux,
    AgentAppGamePlatformProofStatusPlatform.Macos,
    AgentAppGamePlatformProofStatusPlatform.Ios
  ),
  proofState: Schema.Literal(
    AgentAppGamePlatformProofStatusState.ScopedWindowsExecutionProved,
    AgentAppGamePlatformProofStatusState.AndroidHostVisible,
    AgentAppGamePlatformProofStatusState.AndroidHostNotDetected,
    AgentAppGamePlatformProofStatusState.LinuxHostVisible,
    AgentAppGamePlatformProofStatusState.LinuxHostNotDetected,
    AgentAppGamePlatformProofStatusState.LocalRuntimeNotApplicable
  ),
  authorityState: Schema.Literal(
    AgentAppGamePlatformProofStatusAuthority.ScopedExecutionOnly,
    AgentAppGamePlatformProofStatusAuthority.VisibilityOnly,
    AgentAppGamePlatformProofStatusAuthority.NotLocallyProvable
  ),
  hostCapabilityState: Schema.Literal(
    AgentAppGamePlatformProofStatusHostCapability.Available,
    AgentAppGamePlatformProofStatusHostCapability.NotDetected,
    AgentAppGamePlatformProofStatusHostCapability.NotApplicable
  ),
  hostCapabilityEvidenceRefs: Schema.Array(PlatformProofStatusText),
  hostCapabilityProbeRefs: Schema.Array(PlatformProofStatusText),
  productMeanings: Schema.Array(Schema.Literal('native-app', 'native-game')),
  proofRefs: Schema.Array(PlatformProofStatusText),
  openGaps: Schema.Array(PlatformProofStatusText),
  adapterDispatchClaimed: Schema.Literal(false),
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  lastCheckedAt: PlatformProofStatusText,
});

type AgentAppGamePlatformProofStatusRowCandidate = Infer<
  typeof AgentAppGamePlatformProofStatusRowBaseSchema
>;

export const AgentAppGamePlatformProofStatusRowSchema = withParser(
  AgentAppGamePlatformProofStatusRowBaseSchema.pipe(
    Schema.filter(
      (row: AgentAppGamePlatformProofStatusRowCandidate) =>
        platformProofStatusRowIsHonest(row) ||
        'Expected app/game platform proof status rows to keep platform control claims unproved'
    )
  )
);

const AgentAppGamePlatformProofStatusReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  readModelId: PlatformProofStatusText,
  generatedAt: PlatformProofStatusText,
  sourceReadModelIds: Schema.Array(PlatformProofStatusText),
  custodyLabel: PlatformProofStatusText,
  capabilityStatus: PlatformProofStatusText,
  returned: PlatformProofStatusCount,
  hostVisibleCount: PlatformProofStatusCount,
  hostNotDetectedCount: PlatformProofStatusCount,
  localRuntimeNotApplicableCount: PlatformProofStatusCount,
  enforcementReadyCount: PlatformProofStatusCount,
  openGapCount: PlatformProofStatusCount,
  adapterDispatchClaimed: Schema.Literal(false),
  broadInstalledAppBlockingClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  privateDiagnosticsClaimed: Schema.Literal(false),
  rows: Schema.Array(AgentAppGamePlatformProofStatusRowSchema),
});

type AgentAppGamePlatformProofStatusReadModelCandidate = Infer<
  typeof AgentAppGamePlatformProofStatusReadModelBaseSchema
>;

export const AgentAppGamePlatformProofStatusReadModelSchema = withParser(
  AgentAppGamePlatformProofStatusReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel: AgentAppGamePlatformProofStatusReadModelCandidate) =>
        platformProofStatusCountsMatch(readModel) ||
        'Expected app/game platform proof status counts to match status rows'
    )
  )
);

export type AgentAppGamePlatformProofStatusRow = Infer<typeof AgentAppGamePlatformProofStatusRowSchema>;
export type AgentAppGamePlatformProofStatusReadModel = Infer<
  typeof AgentAppGamePlatformProofStatusReadModelSchema
>;

export type AgentAppGamePlatformProofStatusFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePlatformProofStatusResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGamePlatformProofStatusReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePlatformProofStatusFailureReason;
    };

export function parseAgentAppGamePlatformProofStatusEvent(
  event: AgentEventEnvelope
): AgentAppGamePlatformProofStatusResult {
  if (event.event !== AgentEvent.ActivityAppGamePlatformProofStatusReadModelReported) {
    return platformStatusFailure('wrong-event');
  }

  const raw = event.payload[AgentAppGamePlatformProofStatusPayloadField];
  if (!isAgentProtocolLogText(raw)) {
    return platformStatusFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return platformStatusFailure('invalid-json');
  }

  const parsed = AgentAppGamePlatformProofStatusReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return platformStatusFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function platformProofStatusRowIsHonest(row: AgentAppGamePlatformProofStatusRowCandidate): boolean {
  return (
    noRuntimeClaimUpgrade(row) &&
    row.productMeanings.includes('native-app') &&
    row.productMeanings.includes('native-game') &&
    row.proofRefs.length > 0 &&
    row.openGaps.length > 0 &&
    hostCapabilityRefsMatchState(row)
  );
}

function noRuntimeClaimUpgrade(row: AgentAppGamePlatformProofStatusRowCandidate): boolean {
  return (
    !row.adapterDispatchClaimed &&
    !row.broadInstalledAppBlockingClaimed &&
    !row.platformEnforcementClaimed &&
    !row.providerDeliveryClaimed &&
    !row.childDeviceDeliveryClaimed &&
    !row.privateDiagnosticsClaimed
  );
}

function hostCapabilityRefsMatchState(row: AgentAppGamePlatformProofStatusRowCandidate): boolean {
  if (row.hostCapabilityState === AgentAppGamePlatformProofStatusHostCapability.Available) {
    return row.hostCapabilityEvidenceRefs.length > 0 && row.hostCapabilityProbeRefs.length > 0;
  }
  if (row.hostCapabilityState === AgentAppGamePlatformProofStatusHostCapability.NotApplicable) {
    return row.hostCapabilityEvidenceRefs.length === 0 && row.hostCapabilityProbeRefs.length === 0;
  }
  return row.hostCapabilityEvidenceRefs.length === 0;
}

function platformProofStatusCountsMatch(readModel: AgentAppGamePlatformProofStatusReadModelCandidate): boolean {
  return (
    readModel.returned === readModel.rows.length &&
    readModel.hostVisibleCount ===
      readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGamePlatformProofStatusHostCapability.Available
      ).length &&
    readModel.hostNotDetectedCount ===
      readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGamePlatformProofStatusHostCapability.NotDetected
      ).length &&
    readModel.localRuntimeNotApplicableCount ===
      readModel.rows.filter(
        (row) => row.hostCapabilityState === AgentAppGamePlatformProofStatusHostCapability.NotApplicable
      ).length &&
    readModel.enforcementReadyCount === 0 &&
    readModel.openGapCount === readModel.rows.reduce((count, row) => count + row.openGaps.length, 0)
  );
}

function platformStatusFailure(
  reason: AgentAppGamePlatformProofStatusFailureReason
): AgentAppGamePlatformProofStatusResult {
  return {
    ok: false,
    reason,
  };
}
