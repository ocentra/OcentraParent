import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const LinuxNftablesCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const LinuxNftablesRefs = AgentProtocolDefaults.NetworkLinuxNftablesLabStatus;

export const AgentNetworkLinuxNftablesLabStateSchema = withParser(
  Schema.Literal('executed-and-rolled-back', 'manual-required', 'unavailable')
);

export const AgentNetworkLinuxNftablesLabCommandKindSchema = withParser(
  Schema.Literal(
    'create-table',
    'create-chain',
    'add-rule',
    'verify-rule-present',
    'delete-table',
    'verify-table-removed'
  )
);

const AgentNetworkLinuxNftablesLabCommandRowSchema = Schema.Struct({
  kind: AgentNetworkLinuxNftablesLabCommandKindSchema,
  commandRef: NonEmptyStringSchema,
  exitStatus: Schema.Number.pipe(Schema.int()),
  outputSha256: NonEmptyStringSchema,
  tablePresentAfterCommand: Schema.Boolean,
  chainPresentAfterCommand: Schema.Boolean,
  rulePresentAfterCommand: Schema.Boolean,
});

const AgentNetworkLinuxNftablesLabStatusFields = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  labRef: NonEmptyStringSchema,
  linuxAdapterGateRef: NonEmptyStringSchema,
  policyDecisionRef: NonEmptyStringSchema,
  parentRuleRef: NonEmptyStringSchema,
  evidenceRefs: Schema.Array(NonEmptyStringSchema),
  distroRef: NonEmptyStringSchema,
  kernelRef: NonEmptyStringSchema,
  tableName: NonEmptyStringSchema,
  chainName: NonEmptyStringSchema,
  targetRemoteAddress: NonEmptyStringSchema,
  state: AgentNetworkLinuxNftablesLabStateSchema,
  wslHostObserved: Schema.Boolean,
  rootPermissionObserved: Schema.Boolean,
  nftToolObserved: Schema.Boolean,
  commandCount: LinuxNftablesCount,
  requiredCommandCount: LinuxNftablesCount,
  tableCreateObserved: Schema.Boolean,
  chainCreateObserved: Schema.Boolean,
  ruleAddObserved: Schema.Boolean,
  verifyPresentObserved: Schema.Boolean,
  rollbackObserved: Schema.Boolean,
  verifyRemovedObserved: Schema.Boolean,
  labPacketFilterRuleExecuted: Schema.Boolean,
  rollbackVerified: Schema.Boolean,
  productionEnforcementClaimed: Schema.Literal(false),
  persistentRuleClaimed: Schema.Literal(false),
  genericLinuxSupportClaimed: Schema.Literal(false),
  serviceManagerInstallClaimed: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
  policyEngineExecutionClaimed: Schema.Literal(false),
  enforcementCommandPublished: Schema.Literal(false),
  commandEvidence: Schema.Array(AgentNetworkLinuxNftablesLabCommandRowSchema),
});

export type AgentNetworkLinuxNftablesLabCommandRow = Infer<typeof AgentNetworkLinuxNftablesLabCommandRowSchema>;
export type AgentNetworkLinuxNftablesLabStatus = Infer<typeof AgentNetworkLinuxNftablesLabStatusFields>;

export const AgentNetworkLinuxNftablesLabStatusSchema = withParser(
  AgentNetworkLinuxNftablesLabStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkLinuxNftablesLabStatus) =>
        (refsMatch(status) &&
          executionShapeMatches(status) &&
          commandCountsMatch(status) &&
          commandEvidenceMatches(status)) ||
        'Network Linux nftables lab status must preserve the bounded lab refs, six command rows, ' +
          'rollback verification, and no production/content/enforcement claims'
    )
  )
);

export type AgentNetworkLinuxNftablesLabStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLinuxNftablesLabStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-linux-nftables-lab-status'
        | 'invalid-linux-nftables-lab-status-json'
        | 'invalid-linux-nftables-lab-status';
    };

export function parseAgentNetworkLinuxNftablesLabStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLinuxNftablesLabStatusParseResult {
  if (event.event !== AgentEvent.NetworkLinuxNftablesLabStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-linux-nftables-lab-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-linux-nftables-lab-status-json' };
  }

  const parsed = AgentNetworkLinuxNftablesLabStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-linux-nftables-lab-status' };
  }

  return { ok: true, status: parsed.data };
}

function refsMatch(status: AgentNetworkLinuxNftablesLabStatus): boolean {
  return (
    status.statusRef === LinuxNftablesRefs.StatusRef &&
    status.labRef === LinuxNftablesRefs.LabRef &&
    status.linuxAdapterGateRef === LinuxNftablesRefs.LinuxAdapterGateRef &&
    status.policyDecisionRef === LinuxNftablesRefs.PolicyDecisionRef &&
    status.parentRuleRef === LinuxNftablesRefs.ParentRuleRef &&
    status.evidenceRefs.length === 1 &&
    status.evidenceRefs[0] === LinuxNftablesRefs.EvidenceRef &&
    status.distroRef === LinuxNftablesRefs.DistroRef &&
    status.kernelRef === LinuxNftablesRefs.KernelRef &&
    status.tableName === LinuxNftablesRefs.TableName &&
    status.chainName === LinuxNftablesRefs.ChainName &&
    status.targetRemoteAddress === LinuxNftablesRefs.TargetRemoteAddress
  );
}

function executionShapeMatches(status: AgentNetworkLinuxNftablesLabStatus): boolean {
  return (
    status.state === 'executed-and-rolled-back' &&
    status.wslHostObserved &&
    status.rootPermissionObserved &&
    status.nftToolObserved &&
    status.tableCreateObserved &&
    status.chainCreateObserved &&
    status.ruleAddObserved &&
    status.verifyPresentObserved &&
    status.rollbackObserved &&
    status.verifyRemovedObserved &&
    status.labPacketFilterRuleExecuted &&
    status.rollbackVerified
  );
}

function commandCountsMatch(status: AgentNetworkLinuxNftablesLabStatus): boolean {
  return (
    status.requiredCommandCount === 6 &&
    status.commandCount === 6 &&
    status.commandEvidence.length === status.commandCount
  );
}

function commandEvidenceMatches(status: AgentNetworkLinuxNftablesLabStatus): boolean {
  return (
    commandMatches(
      status.commandEvidence[0],
      'create-table',
      LinuxNftablesRefs.CreateTableCommandRef,
      LinuxNftablesRefs.CreateTableOutputSha256,
      true,
      false,
      false
    ) &&
    commandMatches(
      status.commandEvidence[1],
      'create-chain',
      LinuxNftablesRefs.CreateChainCommandRef,
      LinuxNftablesRefs.CreateChainOutputSha256,
      true,
      true,
      false
    ) &&
    commandMatches(
      status.commandEvidence[2],
      'add-rule',
      LinuxNftablesRefs.AddRuleCommandRef,
      LinuxNftablesRefs.AddRuleOutputSha256,
      true,
      true,
      true
    ) &&
    commandMatches(
      status.commandEvidence[3],
      'verify-rule-present',
      LinuxNftablesRefs.VerifyRuleCommandRef,
      LinuxNftablesRefs.VerifyRuleOutputSha256,
      true,
      true,
      true
    ) &&
    commandMatches(
      status.commandEvidence[4],
      'delete-table',
      LinuxNftablesRefs.DeleteTableCommandRef,
      LinuxNftablesRefs.DeleteTableOutputSha256,
      false,
      false,
      false
    ) &&
    commandMatches(
      status.commandEvidence[5],
      'verify-table-removed',
      LinuxNftablesRefs.VerifyRemovedCommandRef,
      LinuxNftablesRefs.VerifyRemovedOutputSha256,
      false,
      false,
      false
    )
  );
}

function commandMatches(
  command: AgentNetworkLinuxNftablesLabCommandRow | undefined,
  kind: AgentNetworkLinuxNftablesLabCommandRow['kind'],
  commandRef: string,
  outputSha256: string,
  tablePresentAfterCommand: boolean,
  chainPresentAfterCommand: boolean,
  rulePresentAfterCommand: boolean
): boolean {
  return (
    command?.kind === kind &&
    command.commandRef === commandRef &&
    command.exitStatus === 0 &&
    command.outputSha256 === outputSha256 &&
    command.tablePresentAfterCommand === tablePresentAfterCommand &&
    command.chainPresentAfterCommand === chainPresentAfterCommand &&
    command.rulePresentAfterCommand === rulePresentAfterCommand
  );
}
