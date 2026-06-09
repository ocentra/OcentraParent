import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
  type AgentAppGameAdapterDispatchPreflightReadModel,
  type AgentAppGameAdapterDispatchPreflightResult,
  type AgentAppGameAdapterDispatchPreflightRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-dispatch-preflight';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  Blocked: decodeDisplayText('Blocked before dispatch'),
  Eligible: decodeDisplayText('Dispatch eligible'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
} as const;

const DispatchPreflightLabels = {
  [AgentAppGameAdapterDispatchPreflightState.DispatchEligible]: Readable.Eligible,
  [AgentAppGameAdapterDispatchPreflightState.BlockedBeforeDispatch]: Readable.Blocked,
  [AgentAppGameAdapterDispatchPreflightState.ManualRequired]: decodeDisplayText('Manual proof required'),
  [AgentAppGameAdapterDispatchPreflightState.Unavailable]: decodeDisplayText('Adapter unavailable'),
  [AgentAppGameAdapterDispatchPreflightState.Unsupported]: decodeDisplayText('Platform unsupported'),
  [AgentAppGameAdapterDispatchPreflightState.Degraded]: decodeDisplayText('Adapter degraded'),
} as const;

const ProductClaim = decodeDisplayText(
  'Adapter dispatch preflight only marks the scoped Windows owned-process app/game timer row as dispatch eligible. Android and Linux host capability refs remain visibility-only and do not make dispatch eligible. Adapter execution, broad installed-app blocking, platform enforcement, provider delivery, child delivery, and private diagnostics remain unclaimed.'
);
const AuditReferences = decodeDisplayText('Audit references');
const DispatchOutcome = decodeDisplayText('Dispatch outcome');
const DispatchIntent = decodeDisplayText('Dispatch intent');
const HostAvailableRows = decodeDisplayText('Host available rows');
const HostNotDetectedRows = decodeDisplayText('Host not-detected rows');
const HostNotApplicableRows = decodeDisplayText('Host not-applicable rows');
const HostProbeRefs = decodeDisplayText('Host probe refs');
const TimerReferences = decodeDisplayText('Timer references');

export type AppGameAdapterDispatchPreflightPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGameAdapterDispatchPreflightPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGameAdapterDispatchPreflightPanelDetail[];
};

export type AppGameAdapterDispatchPreflightPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGameAdapterDispatchPreflightPanelDetail[];
  readonly rows: readonly AppGameAdapterDispatchPreflightPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createAppGameAdapterDispatchPreflightPanelIntent(
  readModelResult: AgentAppGameAdapterDispatchPreflightResult | null
): AppGameAdapterDispatchPreflightPanelIntent {
  const base = baseIntent();

  if (readModelResult === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
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
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState: dispatchPreflightLoadState(readModelResult.value),
    summaryDetails: readModelSummary(readModelResult.value),
    rows: readModelResult.value.rows.map(dispatchPreflightRow),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: decodeDisplayText('App/game adapter dispatch preflight'),
    body: decodeDisplayText(
      'Service-backed dispatch preflight derived from adapter execution readiness and policy dispatch.'
    ),
    emptyMessage: decodeDisplayText('No app/game adapter dispatch preflight read model has been reported yet.'),
    productClaim: ProductClaim,
  };
}

function readModelSummary(
  readModel: AgentAppGameAdapterDispatchPreflightReadModel
): readonly AppGameAdapterDispatchPreflightPanelDetail[] {
  return [
    detail(PortalDetails.Status, dispatchPreflightLoadState(readModel)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.Custody, displayText(readModel.custodyLabel)),
    detail(PortalDetails.Capability, displayText(readModel.capabilityStatus)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
    detail(PortalDetails.ReadModelRows, countText(readModel.dispatchEligibleCount)),
    detail(PortalDetails.ManualReview, countText(readModel.blockedBeforeDispatchCount)),
    detail(PortalDetails.AdapterDispatch, claimedValue(readModel.adapterDispatchEligibleCount > 0)),
    detail(PortalDetails.ExecutionState, claimedValue(readModel.adapterDispatchExecutedClaimedCount > 0)),
    detail(HostAvailableRows, countText(readModel.hostCapabilityAvailableCount)),
    detail(HostNotDetectedRows, countText(readModel.hostCapabilityNotDetectedCount)),
    detail(HostNotApplicableRows, countText(readModel.hostCapabilityNotApplicableCount)),
    detail(HostProbeRefs, countText(readModel.hostCapabilityProbeRefCount)),
    detail(PortalDetails.PlatformState, claimedValue(readModel.platformEnforcementClaimed)),
    detail(PortalDetails.ChildDelivery, claimedValue(readModel.childDeviceDeliveryClaimed)),
    detail(PortalDetails.ProductClaim, ProductClaim),
  ];
}

function dispatchPreflightRow(row: AgentAppGameAdapterDispatchPreflightRow): AppGameAdapterDispatchPreflightPanelRow {
  return {
    title: displayText(row.sourceProofEntryId),
    details: [
      detail(PortalDetails.Platform, displayText(row.platform)),
      detail(PortalDetails.Capability, displayText(row.adapterCapability)),
      detail(PortalDetails.Status, DispatchPreflightLabels[row.dispatchPreflightState]),
      detail(PortalDetails.AdapterBoundary, displayText(row.sourceExecutionReadinessRowId)),
      detail(PortalDetails.PreviewStatus, decisionLabel(row.dispatchDecision)),
      detail(DispatchIntent, optionalText(row.dispatchIntentId)),
      detail(DispatchOutcome, outcomeLabel(row.dispatchOutcomeState)),
      detail(PortalDetails.EvidenceReferences, joinedOrNotReported(row.dispatchEvidenceRefs)),
      detail(PortalDetails.HostCapabilityState, displayText(row.hostCapabilityState)),
      detail(PortalDetails.HostCapabilityEvidence, joinedOrNotReported(row.hostCapabilityEvidenceRefs)),
      detail(PortalDetails.HostCapabilityProbe, joinedOrNotReported(row.hostCapabilityProbeRefs)),
      detail(AuditReferences, joinedOrNotReported(row.dispatchAuditRefs)),
      detail(TimerReferences, joinedOrNotReported(row.dispatchTimerRefs)),
      detail(PortalDetails.ManualReview, joinedOrNotReported(row.manualProofRequirements)),
      detail(PortalDetails.AdapterDispatch, claimedValue(row.adapterDispatchEligible)),
      detail(PortalDetails.ExecutionState, claimedValue(row.adapterDispatchExecutedClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(row.platformEnforcementClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(row.childDeviceDeliveryClaimed)),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
  };
}

function dispatchPreflightLoadState(readModel: AgentAppGameAdapterDispatchPreflightReadModel): DisplayText {
  if (readModel.returned === 0) {
    return Readable.Unavailable;
  }
  if (readModel.dispatchEligibleCount > 0 && readModel.blockedBeforeDispatchCount > 0) {
    return Readable.Review;
  }
  return Readable.Ready;
}

function decisionLabel(decision: string): DisplayText {
  if (decision === AgentAppGameAdapterDispatchDecision.DispatchEligible) {
    return Readable.Eligible;
  }
  return Readable.Blocked;
}

function outcomeLabel(outcome: string): DisplayText {
  if (outcome === AgentAppGameAdapterDispatchOutcomeState.DispatchReady) {
    return decodeDisplayText('Dispatch ready');
  }
  return displayText(outcome);
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

function detail(label: DisplayText, value: DisplayText): AppGameAdapterDispatchPreflightPanelDetail {
  return {
    label,
    value,
  };
}
