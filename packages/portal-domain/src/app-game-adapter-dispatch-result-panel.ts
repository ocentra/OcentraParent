import {
  AgentAppGameAdapterDispatchAdapterExecutionDecision,
  AgentAppGameAdapterDispatchAdapterExecutionState,
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchExecutionAuditDecision,
  AgentAppGameAdapterDispatchExecutionAuditState,
  type AgentAppGameAdapterDispatchExecute,
  type AgentAppGameAdapterDispatchExecuteResult,
  type AgentAppGameAdapterDispatchResult,
  type AgentAppGameAdapterDispatchResultReadModel,
  type AgentAppGameAdapterDispatchResultRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-result';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  Accepted: decodeDisplayText('Command accepted'),
  Blocked: decodeDisplayText('Blocked before command'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
} as const;

const CommandResultLabels = {
  [AgentAppGameAdapterDispatchCommandResultState.CommandAccepted]: Readable.Accepted,
  [AgentAppGameAdapterDispatchCommandResultState.BlockedBeforeCommand]: Readable.Blocked,
  [AgentAppGameAdapterDispatchCommandResultState.ManualRequired]: decodeDisplayText('Manual proof required'),
  [AgentAppGameAdapterDispatchCommandResultState.Unavailable]: decodeDisplayText('Adapter unavailable'),
  [AgentAppGameAdapterDispatchCommandResultState.Unsupported]: decodeDisplayText('Platform unsupported'),
  [AgentAppGameAdapterDispatchCommandResultState.Degraded]: decodeDisplayText('Adapter degraded'),
} as const;

const ProductClaim = decodeDisplayText(
  'Adapter dispatch execution is reported only for the scoped Windows owned-process app/game timer row when real enforcement audit evidence is attached. Broad installed-app blocking, platform enforcement, provider delivery, child delivery, and private diagnostics remain unclaimed.'
);
const DispatchCommand = decodeDisplayText('Dispatch command');
const DispatchEvent = decodeDisplayText('Dispatch event');
const DispatchResult = decodeDisplayText('Dispatch result');
const DispatchAction = decodeDisplayText('Dispatch action');
const AuditReferences = decodeDisplayText('Audit references');
const TimerReferences = decodeDisplayText('Timer references');
const ExecutionAudit = decodeDisplayText('Execution audit');
const ExecutionAuditReferences = decodeDisplayText('Execution audit refs');
const AdapterExecution = decodeDisplayText('Adapter execution');
const AdapterExecutionResult = decodeDisplayText('Adapter execution result');
const AdapterExecutionStatus = decodeDisplayText('Adapter execution status');
const AdapterExecutionRefs = decodeDisplayText('Adapter execution refs');
const ExecuteCommand = decodeDisplayText('Execute command');
const ExecuteStatus = decodeDisplayText('Execute status');
const ExecuteResult = decodeDisplayText('Execute result');
const ExecuteAudit = decodeDisplayText('Execute audit');
const ExecuteReadback = decodeDisplayText('Execute readback');

export type AppGameAdapterDispatchResultPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGameAdapterDispatchResultPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGameAdapterDispatchResultPanelDetail[];
};

export type AppGameAdapterDispatchResultPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGameAdapterDispatchResultPanelDetail[];
  readonly rows: readonly AppGameAdapterDispatchResultPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createAppGameAdapterDispatchResultPanelIntent(
  readModelResult: AgentAppGameAdapterDispatchResult | null,
  executeResult: AgentAppGameAdapterDispatchExecute | null = null
): AppGameAdapterDispatchResultPanelIntent {
  const base = baseIntent();

  if (readModelResult === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
        ...executeSummary(executeResult),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      rows: [],
    };
  }

  if (!readModelResult.ok) {
    return {
      ...base,
      loadState: Readable.Review,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Review),
        detail(PortalDetails.Reason, displayText(readModelResult.reason)),
        ...executeSummary(executeResult),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState: dispatchResultLoadState(readModelResult.value),
    summaryDetails: [...readModelSummary(readModelResult.value), ...executeSummary(executeResult)],
    rows: readModelResult.value.rows.map(dispatchResultRow),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: decodeDisplayText('App/game adapter dispatch result'),
    body: decodeDisplayText('Service-backed command-result handoff for scoped app/game adapter dispatch.'),
    emptyMessage: decodeDisplayText('No app/game adapter dispatch result read model has been reported yet.'),
    productClaim: ProductClaim,
  };
}

function readModelSummary(
  readModel: AgentAppGameAdapterDispatchResultReadModel
): readonly AppGameAdapterDispatchResultPanelDetail[] {
  return [
    detail(PortalDetails.Status, dispatchResultLoadState(readModel)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.Custody, displayText(readModel.custodyLabel)),
    detail(PortalDetails.Capability, displayText(readModel.capabilityStatus)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
    detail(PortalDetails.ReadModelRows, countText(readModel.commandAcceptedCount)),
    detail(PortalDetails.ManualReview, countText(readModel.blockedBeforeCommandCount)),
    detail(PortalDetails.AdapterDispatch, claimedValue(readModel.adapterDispatchCommandResultClaimedCount > 0)),
    detail(ExecutionAudit, claimedValue(readModel.serviceLocalExecutionAuditClaimedCount > 0)),
    detail(PortalDetails.ExecutionState, claimedValue(readModel.adapterDispatchExecutedClaimedCount > 0)),
    detail(AdapterExecution, countText(readModel.adapterExecutionReportedCount)),
    detail(PortalDetails.PlatformState, claimedValue(readModel.platformEnforcementClaimed)),
    detail(PortalDetails.ChildDelivery, claimedValue(readModel.childDeviceDeliveryClaimed)),
    detail(PortalDetails.ProductClaim, ProductClaim),
  ];
}

function executeSummary(
  executeResult: AgentAppGameAdapterDispatchExecute | null
): readonly AppGameAdapterDispatchResultPanelDetail[] {
  if (executeResult === null) {
    return [];
  }
  if (!executeResult.ok) {
    return [detail(ExecuteStatus, Readable.Review), detail(PortalDetails.Reason, displayText(executeResult.reason))];
  }
  return executeResultSummary(executeResult.value);
}

function executeResultSummary(
  executeResult: AgentAppGameAdapterDispatchExecuteResult
): readonly AppGameAdapterDispatchResultPanelDetail[] {
  return [
    detail(ExecuteCommand, displayText(executeResult.commandId)),
    detail(ExecuteStatus, displayText(executeResult.executionStatus)),
    detail(ExecuteResult, displayText(executeResult.executionResultId)),
    detail(AdapterExecutionStatus, displayText(executeResult.executionAdapterResultCode)),
    detail(ExecuteAudit, displayText(executeResult.executionAuditEventId)),
    detail(ExecuteReadback, displayText(executeResult.readbackCommandName)),
    detail(PortalDetails.AdapterDispatch, claimedValue(executeResult.adapterDispatchExecutedClaimed)),
    detail(PortalDetails.PlatformState, claimedValue(executeResult.platformEnforcementClaimed)),
    detail(PortalDetails.ChildDelivery, claimedValue(executeResult.childDeviceDeliveryClaimed)),
  ];
}

function dispatchResultRow(row: AgentAppGameAdapterDispatchResultRow): AppGameAdapterDispatchResultPanelRow {
  return {
    title: displayText(row.sourceProofEntryId),
    details: [
      detail(PortalDetails.Platform, displayText(row.platform)),
      detail(PortalDetails.Capability, displayText(row.adapterCapability)),
      detail(PortalDetails.Status, CommandResultLabels[row.dispatchCommandResultState]),
      detail(PortalDetails.AdapterBoundary, displayText(row.sourceDispatchPreflightRowId)),
      detail(PortalDetails.PreviewStatus, decisionLabel(row.dispatchCommandResultDecision)),
      detail(DispatchCommand, optionalText(row.enforcementCommandName)),
      detail(DispatchEvent, optionalText(row.enforcementEventName)),
      detail(DispatchAction, optionalText(row.enforcementActionMode)),
      detail(DispatchResult, optionalText(row.dispatchCommandResultId)),
      detail(AuditReferences, joinedOrNotReported(row.dispatchCommandAuditRefs)),
      detail(TimerReferences, joinedOrNotReported(row.dispatchCommandTimerRefs)),
      detail(ExecutionAudit, executionAuditLabel(row.dispatchExecutionAuditState)),
      detail(ExecutionAuditReferences, joinedOrNotReported(row.dispatchExecutionAuditRefs)),
      detail(AdapterExecution, adapterExecutionLabel(row.dispatchAdapterExecutionState)),
      detail(AdapterExecutionResult, optionalText(row.dispatchAdapterExecutionResultId)),
      detail(AdapterExecutionStatus, optionalText(row.dispatchAdapterExecutionStatus)),
      detail(AdapterExecutionRefs, joinedOrNotReported(row.dispatchAdapterExecutionRefs)),
      detail(PortalDetails.ManualReview, joinedOrNotReported(row.manualProofRequirements)),
      detail(PortalDetails.AdapterDispatch, claimedValue(row.adapterDispatchCommandResultClaimed)),
      detail(ExecutionAudit, claimedValue(row.serviceLocalExecutionAuditClaimed)),
      detail(PortalDetails.ExecutionState, claimedValue(row.adapterDispatchExecutedClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(row.platformEnforcementClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(row.childDeviceDeliveryClaimed)),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
  };
}

function dispatchResultLoadState(readModel: AgentAppGameAdapterDispatchResultReadModel): DisplayText {
  if (readModel.returned === 0) {
    return Readable.Unavailable;
  }
  if (readModel.commandAcceptedCount > 0 && readModel.blockedBeforeCommandCount > 0) {
    return Readable.Review;
  }
  return Readable.Ready;
}

function executionAuditLabel(state: string): DisplayText {
  if (state === AgentAppGameAdapterDispatchExecutionAuditState.ServiceLocalAuditRecorded) {
    return decodeDisplayText('Service-local audit recorded');
  }
  if (state === AgentAppGameAdapterDispatchExecutionAuditDecision.BlockedBeforeExecutionAudit) {
    return decodeDisplayText('Blocked before execution audit');
  }
  return displayText(state);
}

function adapterExecutionLabel(state: string): DisplayText {
  if (state === AgentAppGameAdapterDispatchAdapterExecutionState.AdapterExecutionReported) {
    return decodeDisplayText('Adapter execution reported');
  }
  if (state === AgentAppGameAdapterDispatchAdapterExecutionDecision.AdapterExecutionEvidenceMissing) {
    return decodeDisplayText('Execution evidence missing');
  }
  if (state === AgentAppGameAdapterDispatchAdapterExecutionDecision.BlockedBeforeAdapterExecution) {
    return decodeDisplayText('Blocked before adapter execution');
  }
  return displayText(state);
}

function decisionLabel(decision: string): DisplayText {
  if (decision === AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted) {
    return Readable.Accepted;
  }
  return Readable.Blocked;
}

function claimedValue(value: boolean): DisplayText {
  return value ? Readable.Ready : Readable.NotClaimed;
}

function joinedOrNotReported(values: readonly string[]): DisplayText {
  if (values.length === 0) {
    return decodeDisplayText('Not reported');
  }
  return displayText(values.join(DetailSeparator));
}

function optionalText(value: string | null): DisplayText {
  if (value === null) {
    return decodeDisplayText('Not reported');
  }
  return displayText(value);
}

function requiredReadableValue(key: string): DisplayText {
  const value = PortalReadableValues[key];
  if (value === undefined) {
    throw new Error(`Missing portal readable value: ${key}`);
  }
  return value;
}

function countText(value: number): DisplayText {
  return displayText(String(value));
}

function displayText(value: string): DisplayText {
  return decodeDisplayText(value);
}

function detail(label: DisplayText, value: DisplayText): AppGameAdapterDispatchResultPanelDetail {
  return {
    label,
    value,
  };
}
