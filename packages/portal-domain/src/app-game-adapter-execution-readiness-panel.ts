import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
  type AgentAppGameAdapterExecutionReadinessReadModel,
  type AgentAppGameAdapterExecutionReadinessResult,
  type AgentAppGameAdapterExecutionReadinessRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-adapter-execution-readiness';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  Allowed: decodeDisplayText('Execution allowed'),
  Blocked: decodeDisplayText('Blocked before execution'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
} as const;

const AdapterReadinessLabels = {
  [AgentAppGameAdapterExecutionState.ProvedScopedExecution]: decodeDisplayText('Scoped adapter execution proved'),
  [AgentAppGameAdapterExecutionState.ManualRequired]: decodeDisplayText('Manual proof required'),
  [AgentAppGameAdapterExecutionState.Unavailable]: decodeDisplayText('Adapter unavailable'),
  [AgentAppGameAdapterExecutionState.Unsupported]: decodeDisplayText('Platform unsupported'),
  [AgentAppGameAdapterExecutionState.Degraded]: decodeDisplayText('Adapter degraded'),
  [AgentAppGameAdapterExecutionState.NotClaimed]: Readable.NotClaimed,
} as const;

const ProductClaim = decodeDisplayText(
  'Adapter execution is surfaced only for the scoped Windows owned-process app/game timer boundary. Broad installed-app blocking, platform enforcement, provider delivery, child delivery, and private diagnostics remain unclaimed.'
);
const ProofReferences = decodeDisplayText('Proof references');

export type AppGameAdapterExecutionReadinessPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGameAdapterExecutionReadinessPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGameAdapterExecutionReadinessPanelDetail[];
};

export type AppGameAdapterExecutionReadinessPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGameAdapterExecutionReadinessPanelDetail[];
  readonly rows: readonly AppGameAdapterExecutionReadinessPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createAppGameAdapterExecutionReadinessPanelIntent(
  readModelResult: AgentAppGameAdapterExecutionReadinessResult | null
): AppGameAdapterExecutionReadinessPanelIntent {
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
    loadState: adapterReadinessLoadState(readModelResult.value),
    summaryDetails: readModelSummary(readModelResult.value),
    rows: readModelResult.value.rows.map(adapterReadinessRow),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: decodeDisplayText('App/game adapter execution readiness'),
    body: decodeDisplayText(
      'Service-backed adapter execution readiness derived from the supported-adapter runtime proof.'
    ),
    emptyMessage: decodeDisplayText('No app/game adapter execution readiness read model has been reported yet.'),
    productClaim: ProductClaim,
  };
}

function readModelSummary(
  readModel: AgentAppGameAdapterExecutionReadinessReadModel
): readonly AppGameAdapterExecutionReadinessPanelDetail[] {
  return [
    detail(PortalDetails.Status, adapterReadinessLoadState(readModel)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.Custody, displayText(readModel.custodyLabel)),
    detail(PortalDetails.Capability, displayText(readModel.capabilityStatus)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
    detail(PortalDetails.ReadModelRows, countText(readModel.executionAllowedCount)),
    detail(PortalDetails.ManualReview, countText(readModel.blockedBeforeExecutionCount)),
    detail(PortalDetails.AdapterDispatch, claimedValue(readModel.adapterExecutionClaimedCount > 0)),
    detail(PortalDetails.PlatformState, claimedValue(readModel.platformEnforcementClaimed)),
    detail(PortalDetails.ChildDelivery, claimedValue(readModel.childDeviceDeliveryClaimed)),
    detail(PortalDetails.ProductClaim, ProductClaim),
  ];
}

function adapterReadinessRow(row: AgentAppGameAdapterExecutionReadinessRow): AppGameAdapterExecutionReadinessPanelRow {
  return {
    title: displayText(row.sourceProofEntryId),
    details: [
      detail(PortalDetails.Platform, displayText(row.platform)),
      detail(PortalDetails.Capability, displayText(row.adapterCapability)),
      detail(PortalDetails.Status, AdapterReadinessLabels[row.adapterExecutionState]),
      detail(PortalDetails.AdapterBoundary, displayText(row.runtimeBoundary)),
      detail(PortalDetails.PreviewStatus, decisionLabel(row.executionDecision)),
      detail(PortalDetails.EvidenceReferences, joinedOrNotReported(row.evidenceRefs)),
      detail(ProofReferences, joinedOrNotReported(row.linkedProofArtifacts)),
      detail(PortalDetails.ManualReview, joinedOrNotReported(row.manualProofRequirements)),
      detail(PortalDetails.AdapterDispatch, claimedValue(row.adapterExecutionClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(row.platformEnforcementClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(row.childDeviceDeliveryClaimed)),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
  };
}

function adapterReadinessLoadState(readModel: AgentAppGameAdapterExecutionReadinessReadModel): DisplayText {
  if (readModel.returned === 0) {
    return Readable.Unavailable;
  }
  if (readModel.executionAllowedCount > 0 && readModel.blockedBeforeExecutionCount > 0) {
    return Readable.Review;
  }
  return Readable.Ready;
}

function decisionLabel(decision: string): DisplayText {
  if (decision === AgentAppGameAdapterExecutionDecision.ExecutionAllowed) {
    return Readable.Allowed;
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

function detail(label: DisplayText, value: DisplayText): AppGameAdapterExecutionReadinessPanelDetail {
  return {
    label,
    value,
  };
}
