import { type LogFields } from '@ocentra-parent/schema-domain/logging-contracts';
import { type SafeParseResult } from '@ocentra-parent/schema-domain/effect';
import {
  AgentBrowserRuntimePhase,
  AgentBrowserRuntimeEventChainEntrySchema,
  AgentBrowserRuntimeEventChainStreamSchema,
  type AgentBrowserRuntimeActionIntentCandidate,
  type AgentBrowserRuntimeEventChainEntry,
  type AgentBrowserRuntimeEventChainStream,
} from '@ocentra-parent/schema-domain/agent-browser-runtime-events';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type AgentBrowserRuntimeActionIntentStatus = {
  readonly candidateCount: number;
  readonly handoffCandidateCount: number;
  readonly handoffOutboxRefs: readonly string[];
  readonly handoffRefs: readonly string[];
  readonly childAcceptedRows: number;
  readonly childCommandRefs: readonly string[];
  readonly childAcceptedEventRefs: readonly string[];
  readonly parentReadModelRefs: readonly string[];
  readonly dispatchAttemptCount: 0;
  readonly adapterExecutionCount: 0;
  readonly childInterventionExecutionCount: 0;
  readonly enforcementExecutionCount: 0;
  readonly dryRunOnly: true;
  readonly policyAuthorityOnly: true;
  readonly candidates: readonly AgentBrowserRuntimeActionIntentCandidate[];
};

export type AgentBrowserRuntimeSocialProviderReceiptStatus = {
  readonly receiptBoundaryRows: number;
  readonly providerDispatchRequiredRows: number;
  readonly manualReceiptRequiredRows: number;
  readonly providerAttemptRefs: readonly string[];
  readonly providerReceiptProofRefs: readonly string[];
  readonly durableRows: number;
  readonly durableResultRefs: readonly string[];
  readonly durableStoreRefs: readonly string[];
  readonly readModelRefs: readonly string[];
  readonly supportStatusRefs: readonly string[];
  readonly providerDeliveryClaimed: false;
  readonly receiptIngestionClaimed: false;
  readonly parentNotificationDeliveryClaimed: false;
  readonly reportDeliveryClaimed: false;
  readonly finalPolicyExecutionClaimed: false;
  readonly connectorNativeRuntimeClaimed: false;
  readonly enforcementClaimed: false;
};

export type AgentBrowserRuntimeEventChainStreamFailureReason =
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-entry'
  | 'invalid-stream';

export type AgentBrowserRuntimeEventChainStreamResult =
  | {
      readonly ok: true;
      readonly value: AgentBrowserRuntimeEventChainStream;
    }
  | {
      readonly ok: false;
      readonly reason: AgentBrowserRuntimeEventChainStreamFailureReason;
    };

export function parseAgentBrowserRuntimeEventChainStreamFields(
  fields: LogFields
): AgentBrowserRuntimeEventChainStreamResult {
  const raw = fields[AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream];
  if (typeof raw !== 'string') {
    return parserFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return parserFailure('invalid-json');
  }

  if (!Array.isArray(decoded)) {
    return parserFailure('invalid-entry');
  }

  const entries: AgentBrowserRuntimeEventChainEntry[] = [];
  for (const entry of decoded) {
    const parsed = AgentBrowserRuntimeEventChainEntrySchema.safeParse(entry);
    if (!parsed.success || parsed.data === undefined) {
      return parserFailure('invalid-entry');
    }
    entries.push(parsed.data);
  }

  return streamResult(AgentBrowserRuntimeEventChainStreamSchema.safeParse(streamFieldsCandidate(fields, entries)));
}

export function deriveAgentBrowserRuntimeActionIntentStatus(
  stream: AgentBrowserRuntimeEventChainStream
): AgentBrowserRuntimeActionIntentStatus {
  const candidates = actionIntentCandidatesFromEntries(stream.entries);
  return {
    candidateCount: stream.actionIntentCandidates,
    handoffCandidateCount: stream.actionIntentHandoffCandidates,
    handoffOutboxRefs: stream.actionIntentHandoffOutboxRefs,
    handoffRefs: stream.actionIntentHandoffRefs,
    childAcceptedRows: stream.actionIntentChildAcceptedRows,
    childCommandRefs: stream.actionIntentChildCommandRefs,
    childAcceptedEventRefs: stream.actionIntentChildAcceptedEventRefs,
    parentReadModelRefs: stream.actionIntentParentReadModelRefs,
    dispatchAttemptCount: 0,
    adapterExecutionCount: 0,
    childInterventionExecutionCount: 0,
    enforcementExecutionCount: 0,
    dryRunOnly: true,
    policyAuthorityOnly: true,
    candidates,
  };
}

export function deriveAgentBrowserRuntimeSocialProviderReceiptStatus(
  stream: AgentBrowserRuntimeEventChainStream
): AgentBrowserRuntimeSocialProviderReceiptStatus {
  return {
    receiptBoundaryRows: stream.socialProviderReceiptBoundaryRows,
    providerDispatchRequiredRows: stream.socialProviderDispatchRequiredRows,
    manualReceiptRequiredRows: stream.socialProviderManualReceiptRequiredRows,
    providerAttemptRefs: stream.socialProviderAttemptRefs,
    providerReceiptProofRefs: stream.socialProviderReceiptProofRefs,
    durableRows: stream.socialProviderDurableRows,
    durableResultRefs: stream.socialProviderDurableResultRefs,
    durableStoreRefs: stream.socialProviderDurableStoreRefs,
    readModelRefs: stream.socialProviderReadModelRefs,
    supportStatusRefs: stream.socialProviderSupportStatusRefs,
    providerDeliveryClaimed: false,
    receiptIngestionClaimed: false,
    parentNotificationDeliveryClaimed: false,
    reportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  };
}

function actionIntentCandidatesFromEntries(
  entries: readonly AgentBrowserRuntimeEventChainEntry[]
): AgentBrowserRuntimeActionIntentCandidate[] {
  return entries.flatMap((entry) => actionIntentCandidateFromEntry(entry));
}

function streamResult(
  parsed: SafeParseResult<AgentBrowserRuntimeEventChainStream>
): AgentBrowserRuntimeEventChainStreamResult {
  if (!parsed.success || parsed.data === undefined) {
    return parserFailure('invalid-stream');
  }
  return {
    ok: true,
    value: parsed.data,
  };
}

function parserFailure(
  reason: AgentBrowserRuntimeEventChainStreamFailureReason
): AgentBrowserRuntimeEventChainStreamResult {
  return {
    ok: false,
    reason,
  };
}

function streamFieldsCandidate(fields: LogFields, entries: readonly AgentBrowserRuntimeEventChainEntry[]) {
  return {
    ...streamCountFields(fields),
    ...streamActionIntentFields(fields),
    ...streamSocialProviderReceiptFields(fields),
    entries,
  };
}

function streamCountFields(fields: LogFields) {
  return {
    observedRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeObservedRows),
    streamedEvents: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents),
    failedRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeFailedRows),
    exactUrlRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows),
    manualRequiredRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows),
    interventionCommandEvents: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents),
    readModelProjectionEvents: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents),
  };
}

function streamActionIntentFields(fields: LogFields) {
  return {
    actionIntentCandidates: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeActionIntentCandidates),
    actionIntentHandoffCandidates: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffCandidates
    ),
    actionIntentHandoffOutboxRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffOutboxRefs
    ),
    actionIntentHandoffRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentHandoffRefs
    ),
    actionIntentChildAcceptedRows: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedRows
    ),
    actionIntentChildCommandRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildCommandRefs
    ),
    actionIntentChildAcceptedEventRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildAcceptedEventRefs
    ),
    actionIntentParentReadModelRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentParentReadModelRefs
    ),
    actionIntentDispatchAttempts: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentDispatchAttempts
    ),
    actionIntentAdapterExecutions: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentAdapterExecutions
    ),
    actionIntentChildInterventionExecutions: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentChildInterventionExecutions
    ),
    actionIntentEnforcementExecutions: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeActionIntentEnforcementExecutions
    ),
  };
}

function streamSocialProviderReceiptFields(fields: LogFields) {
  return {
    socialProviderReceiptBoundaryRows: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptBoundaryRows
    ),
    socialProviderDispatchRequiredRows: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDispatchRequiredRows
    ),
    socialProviderManualReceiptRequiredRows: numberField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderManualReceiptRequiredRows
    ),
    socialProviderAttemptRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderAttemptRefs
    ),
    socialProviderReceiptProofRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReceiptProofRefs
    ),
    socialProviderDurableRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableRows),
    socialProviderDurableResultRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableResultRefs
    ),
    socialProviderDurableStoreRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderDurableStoreRefs
    ),
    socialProviderReadModelRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderReadModelRefs
    ),
    socialProviderSupportStatusRefs: stringArrayField(
      fields,
      AgentProtocolDefaults.Field.BrowserRuntimeSocialProviderSupportStatusRefs
    ),
  };
}

function numberField(fields: LogFields, key: string): number | null {
  const value = fields[key];
  return typeof value === 'number' ? value : null;
}

function stringArrayField(fields: LogFields, key: string): readonly string[] | null {
  const value = fields[key];
  if (typeof value !== 'string') {
    return null;
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(value);
  } catch {
    return null;
  }
  if (!Array.isArray(decoded) || decoded.some((entry) => typeof entry !== 'string' || entry.length === 0)) {
    return null;
  }
  return decoded;
}

function actionIntentCandidateFromEntry(
  entry: AgentBrowserRuntimeEventChainEntry
): AgentBrowserRuntimeActionIntentCandidate[] {
  const payload = entry.payload;
  if (
    payload.phase !== AgentBrowserRuntimePhase.PolicyDecisionCompleted ||
    !payload.dryRun ||
    !payload.policyAuthority ||
    payload.policyPreviewId === null ||
    payload.assistantActionIntentId === null
  ) {
    return [];
  }
  return [
    {
      eventRef: entry.eventRef,
      policyPreviewId: payload.policyPreviewId,
      assistantActionIntentId: payload.assistantActionIntentId,
      sourceRef: payload.sourceRef,
      evidenceRef: payload.evidenceRef,
      observedAt: payload.observedAt,
    },
  ];
}
