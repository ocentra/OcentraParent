import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppGameBroadBlockingGateMatrix } from './app-game-broad-blocking-proof-gate-data';
import { type AppGameBroadBlockingGate } from './app-game-broad-blocking-proof-gates';
import {
  V08OsAdapterManualArtifactGateReadModel,
  type V08OsAdapterManualArtifactGateEntry,
} from '@ocentra-parent/schema-domain/v0-8-os-adapter-manual-artifact-gates';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameWindowsBroadBlockingAuthorityPreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-windows-broad-blocking-authority-preflight')
);

export const AppGameWindowsBroadBlockingAuthorityPreflightStateSchema = withParser(
  Schema.Literal('host-visible-policy-proof-missing', 'authority-ready')
);

export const AppGameWindowsBroadBlockingAuthorityPreflightActionSchema = withParser(
  Schema.Literal(
    'block-launch-applocker',
    'block-launch-app-control',
    'system-app-allowlist',
    'policy-rollback',
    'audit-custody'
  )
);

export const AppGameWindowsBroadBlockingAuthorityPreflightProofRefSchema = withParser(
  Schema.Literal(
    'windows-host-local-probe-ref',
    'windows-broad-blocking-gate-ref',
    'windows-applocker-proof',
    'windows-applocker-audit-proof',
    'windows-app-control-proof',
    'windows-system-app-allowlist-proof',
    'windows-rollback-proof',
    'windows-audit-custody-proof'
  )
);

export const AppGameWindowsBroadBlockingAuthorityPreflightBlockerSchema = withParser(
  Schema.Literal(
    'windows-applocker-enforce-not-proved',
    'windows-app-control-not-proved',
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-adapter-dispatch-blocked-before-authority'
  )
);

const WindowsBroadBlockingLabelSchema = brandedNonEmptyStringSchema(
  'AppGameWindowsBroadBlockingAuthorityPreflightLabel'
);

const WindowsBroadBlockingPreflightRowBaseSchema = Schema.Struct({
  action: AppGameWindowsBroadBlockingAuthorityPreflightActionSchema,
  authorityState: AppGameWindowsBroadBlockingAuthorityPreflightStateSchema,
  sourceGateIds: Schema.Array(WindowsBroadBlockingLabelSchema),
  requiredProofRefs: Schema.Array(AppGameWindowsBroadBlockingAuthorityPreflightProofRefSchema),
  blockerRefs: Schema.Array(AppGameWindowsBroadBlockingAuthorityPreflightBlockerSchema),
  canDispatchAdapter: Schema.Boolean,
  broadBlockingClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
});

const WindowsBroadBlockingPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameWindowsBroadBlockingAuthorityPreflightSchemaVersionSchema,
  preflightId: WindowsBroadBlockingLabelSchema,
  generatedAt: ParentTimestampSchema,
  authorityState: AppGameWindowsBroadBlockingAuthorityPreflightStateSchema,
  windowsHostProbeAttached: Schema.Boolean,
  appLockerProofAttached: Schema.Boolean,
  appControlProofAttached: Schema.Boolean,
  systemAppAllowlistProofAttached: Schema.Boolean,
  rollbackProofAttached: Schema.Boolean,
  auditCustodyProofAttached: Schema.Boolean,
  rows: Schema.Array(WindowsBroadBlockingPreflightRowBaseSchema),
  dispatchableActionCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  blockedActionCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  rawExecutablePathsClaimed: Schema.Literal(false),
  rawPolicyXmlClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  broadBlockingClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameWindowsBroadBlockingAuthorityPreflightProofRefSchema),
  openBlockers: Schema.Array(AppGameWindowsBroadBlockingAuthorityPreflightBlockerSchema),
  parentVisibleSummary: WindowsBroadBlockingLabelSchema,
});

type WindowsBroadBlockingPreflightRowCandidate = Infer<typeof WindowsBroadBlockingPreflightRowBaseSchema>;
type WindowsBroadBlockingPreflightReadModelCandidate = Infer<typeof WindowsBroadBlockingPreflightReadModelBaseSchema>;

export const AppGameWindowsBroadBlockingAuthorityPreflightRowSchema = withParser(
  WindowsBroadBlockingPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        windowsBroadBlockingPreflightRowIsHonest(row) ||
        'Expected Windows broad blocking authority rows to remain blocked until AppLocker/App Control, allowlist, rollback, and audit proof are attached'
    )
  )
);

export const AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema = withParser(
  WindowsBroadBlockingPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        windowsBroadBlockingPreflightReadModelIsHonest(readModel) ||
        'Expected Windows broad blocking authority preflight to keep adapter dispatch blocked before complete platform proof'
    )
  )
);

export type AppGameWindowsBroadBlockingAuthorityPreflightRow = Infer<
  typeof AppGameWindowsBroadBlockingAuthorityPreflightRowSchema
>;
export type AppGameWindowsBroadBlockingAuthorityPreflightReadModel = Infer<
  typeof AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema
>;
export type AppGameWindowsBroadBlockingAuthorityPreflightBlocker = Infer<
  typeof AppGameWindowsBroadBlockingAuthorityPreflightBlockerSchema
>;

export const decodeAppGameWindowsBroadBlockingAuthorityPreflightReadModel = Schema.decodeUnknownSync(
  AppGameWindowsBroadBlockingAuthorityPreflightReadModelSchema
);

export function createAppGameWindowsBroadBlockingAuthorityPreflightReadModel(input: {
  readonly generatedAt: AppGameWindowsBroadBlockingAuthorityPreflightReadModel['generatedAt'];
  readonly broadBlockingGates?: readonly AppGameBroadBlockingGate[];
  readonly manualArtifactGates?: readonly V08OsAdapterManualArtifactGateEntry[];
}): AppGameWindowsBroadBlockingAuthorityPreflightReadModel {
  const broadBlockingGates = input.broadBlockingGates ?? AppGameBroadBlockingGateMatrix.gates;
  const manualArtifactGates = input.manualArtifactGates ?? V08OsAdapterManualArtifactGateReadModel.entries;
  const windowsHostProbeAttached = manualArtifactGates.some(
    (entry) =>
      entry.platform === 'windows' &&
      entry.hostCapabilityProbeRefs.some(
        (probeRef: V08OsAdapterManualArtifactGateEntry['hostCapabilityProbeRefs'][number]) =>
          String(probeRef) === 'windows-host-local-probe-ref'
      )
  );
  const proofRefs = windowsHostProbeAttached
    ? (['windows-host-local-probe-ref', 'windows-broad-blocking-gate-ref'] as const)
    : (['windows-broad-blocking-gate-ref'] as const);
  const openBlockers = windowsBroadBlockingOpenBlockers(false);
  const rows = windowsBroadBlockingActions().map((action) =>
    windowsBroadBlockingPreflightRow(action, windowsGateIdsForAction(broadBlockingGates, action), openBlockers)
  );

  return decodeAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
    schemaVersion: 'app-game-windows-broad-blocking-authority-preflight',
    preflightId: 'windows-broad-blocking-authority-preflight-ref',
    generatedAt: input.generatedAt,
    authorityState: 'host-visible-policy-proof-missing',
    windowsHostProbeAttached,
    appLockerProofAttached: false,
    appControlProofAttached: false,
    systemAppAllowlistProofAttached: false,
    rollbackProofAttached: false,
    auditCustodyProofAttached: false,
    rows,
    dispatchableActionCount: rows.filter((row) => row.canDispatchAdapter).length,
    blockedActionCount: rows.filter((row) => !row.canDispatchAdapter).length,
    rawExecutablePathsClaimed: false,
    rawPolicyXmlClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs,
    openBlockers,
    parentVisibleSummary:
      'Windows host visibility exists for app/game broad-blocking preflight, but AppLocker/App Control enforce proof, system-app allowlist proof, rollback proof, and audit custody proof are missing, so broad launch blocking remains blocked before adapter dispatch.',
  });
}

export function summarizeAppGameWindowsBroadBlockingAuthorityPreflightReadModel(
  readModel: AppGameWindowsBroadBlockingAuthorityPreflightReadModel
) {
  return {
    authorityState: readModel.authorityState,
    windowsHostProbeAttached: readModel.windowsHostProbeAttached,
    dispatchableActionCount: readModel.dispatchableActionCount,
    blockedActionCount: readModel.blockedActionCount,
    openBlockerCount: readModel.openBlockers.length,
  } as const;
}

function windowsBroadBlockingActions() {
  return [
    'block-launch-applocker',
    'block-launch-app-control',
    'system-app-allowlist',
    'policy-rollback',
    'audit-custody',
  ] as const;
}

function windowsBroadBlockingPreflightRow(
  action: ReturnType<typeof windowsBroadBlockingActions>[number],
  sourceGateIds: readonly string[],
  openBlockers: readonly AppGameWindowsBroadBlockingAuthorityPreflightBlocker[]
): AppGameWindowsBroadBlockingAuthorityPreflightRow {
  return AppGameWindowsBroadBlockingAuthorityPreflightRowSchema.parse({
    action,
    authorityState: 'host-visible-policy-proof-missing',
    sourceGateIds,
    requiredProofRefs: windowsRequiredProofRefs(action),
    blockerRefs: openBlockers,
    canDispatchAdapter: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function windowsRequiredProofRefs(action: ReturnType<typeof windowsBroadBlockingActions>[number]) {
  switch (action) {
    case 'block-launch-applocker':
      return ['windows-applocker-proof', 'windows-rollback-proof', 'windows-audit-custody-proof'] as const;
    case 'block-launch-app-control':
      return ['windows-app-control-proof', 'windows-rollback-proof', 'windows-audit-custody-proof'] as const;
    case 'system-app-allowlist':
      return ['windows-system-app-allowlist-proof'] as const;
    case 'policy-rollback':
      return ['windows-rollback-proof'] as const;
    case 'audit-custody':
      return ['windows-applocker-audit-proof', 'windows-audit-custody-proof'] as const;
  }
}

function windowsGateIdsForAction(
  broadBlockingGates: readonly AppGameBroadBlockingGate[],
  action: ReturnType<typeof windowsBroadBlockingActions>[number]
) {
  const windowsBlockLaunchGates = broadBlockingGates
    .filter((gate) => gate.platform === 'windows' && gate.action === 'block-launch')
    .map((gate) => gate.gateId);
  if (action === 'block-launch-app-control' || action === 'system-app-allowlist') {
    return windowsBlockLaunchGates.filter((gateId) => gateId.includes('app-control'));
  }
  if (action === 'audit-custody') {
    return windowsBlockLaunchGates.filter((gateId) => gateId.includes('audit'));
  }
  return windowsBlockLaunchGates;
}

function windowsBroadBlockingOpenBlockers(
  authorityReady: boolean
): readonly AppGameWindowsBroadBlockingAuthorityPreflightBlocker[] {
  if (authorityReady) {
    return [];
  }
  return [
    'windows-applocker-enforce-not-proved',
    'windows-app-control-not-proved',
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-adapter-dispatch-blocked-before-authority',
  ] as const;
}

function windowsBroadBlockingPreflightRowIsHonest(row: WindowsBroadBlockingPreflightRowCandidate): boolean {
  if (row.authorityState === 'authority-ready') {
    return row.canDispatchAdapter && row.blockerRefs.length === 0 && row.requiredProofRefs.length > 0;
  }
  return (
    !row.canDispatchAdapter &&
    row.sourceGateIds.length > 0 &&
    row.blockerRefs.includes('windows-adapter-dispatch-blocked-before-authority') &&
    !row.broadBlockingClaimed &&
    !row.platformEnforcementClaimed
  );
}

function windowsBroadBlockingPreflightReadModelIsHonest(
  readModel: WindowsBroadBlockingPreflightReadModelCandidate
): boolean {
  return (
    windowsBroadBlockingCountsAreHonest(readModel) &&
    windowsBroadBlockingAuthorityReadinessIsHonest(readModel) &&
    windowsBroadBlockingOpenBlockersAreHonest(readModel) &&
    windowsBroadBlockingClaimsRemainScoped(readModel)
  );
}

function windowsBroadBlockingCountsAreHonest(readModel: WindowsBroadBlockingPreflightReadModelCandidate): boolean {
  return (
    readModel.dispatchableActionCount === readModel.rows.filter((row) => row.canDispatchAdapter).length &&
    readModel.blockedActionCount === readModel.rows.filter((row) => !row.canDispatchAdapter).length
  );
}

function windowsBroadBlockingAuthorityReadinessIsHonest(
  readModel: WindowsBroadBlockingPreflightReadModelCandidate
): boolean {
  return (
    readModel.authorityState === 'host-visible-policy-proof-missing' &&
    readModel.windowsHostProbeAttached &&
    !readModel.appLockerProofAttached &&
    !readModel.appControlProofAttached &&
    !readModel.systemAppAllowlistProofAttached &&
    !readModel.rollbackProofAttached &&
    !readModel.auditCustodyProofAttached
  );
}

function windowsBroadBlockingOpenBlockersAreHonest(
  readModel: WindowsBroadBlockingPreflightReadModelCandidate
): boolean {
  return readModel.openBlockers.includes('windows-adapter-dispatch-blocked-before-authority');
}

function windowsBroadBlockingClaimsRemainScoped(readModel: WindowsBroadBlockingPreflightReadModelCandidate): boolean {
  return (
    !readModel.rawExecutablePathsClaimed &&
    !readModel.rawPolicyXmlClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.broadBlockingClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed
  );
}
