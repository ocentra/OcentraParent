import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
  type AgentAppGameTimerParentSurfaceReadModel,
  type AgentAppGameTimerParentSurfaceResult,
  type AgentAppGameTimerParentSurfaceRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-timer-parent-surface-read-model';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  ManualRequired: requiredReadableValue('manual-required'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
} as const;

const TimerParentSurfaceStateLabels = {
  [AgentAppGameTimerParentSurfaceState.ReadyForParentSurface]: decodeDisplayText('Ready for parent surface'),
  [AgentAppGameTimerParentSurfaceState.BlockedBySourceFreshness]: decodeDisplayText('Blocked by source freshness'),
  [AgentAppGameTimerParentSurfaceState.BlockedByCompilerDecision]: decodeDisplayText('Blocked by compiler decision'),
  [AgentAppGameTimerParentSurfaceState.RuntimeManualRequired]: decodeDisplayText('Runtime manual required'),
} satisfies Readonly<Record<AgentAppGameTimerParentSurfaceState, DisplayText>>;

const TimerParentSurfaceTargetLabels = {
  [AgentAppGameTimerParentSurfaceTargetDomain.NativeApp]: decodeDisplayText('Native app'),
  [AgentAppGameTimerParentSurfaceTargetDomain.NativeGame]: decodeDisplayText('Native game'),
} satisfies Readonly<Record<AgentAppGameTimerParentSurfaceTargetDomain, DisplayText>>;

const TimerParentSurfaceDetails = {
  AuditRuntime: decodeDisplayText('Audit runtime'),
  ChildFacingReasonRefs: decodeDisplayText('Child-facing reason refs'),
  ChildFacingStatusRefs: decodeDisplayText('Child-facing status refs'),
  ChildUxLocalArtifactRecords: decodeDisplayText('Child UX local artifact records'),
  ChildUxLocalArtifactRefs: decodeDisplayText('Child UX local artifact refs'),
  ChildUxLocalArtifactSkipped: decodeDisplayText('Child UX local artifact skipped'),
  ChildUxHandoffBlocked: decodeDisplayText('Child UX handoff blocked'),
  ChildUxHandoffReady: decodeDisplayText('Child UX handoff ready'),
  ChildUxHandoffRefs: decodeDisplayText('Child UX handoff refs'),
  ControlActionCapabilities: decodeDisplayText('Control action capabilities'),
  ControlActionEnforcementStatuses: decodeDisplayText('Control action enforcement statuses'),
  ControlActionResultRefs: decodeDisplayText('Control action result refs'),
  ControlActionResultStatuses: decodeDisplayText('Control action result statuses'),
  ControlActionResults: decodeDisplayText('Control action results'),
  DurableSchedulerStorage: decodeDisplayText('Durable scheduler storage'),
  RollbackRuntime: decodeDisplayText('Rollback runtime'),
  SchedulerPersistence: decodeDisplayText('Scheduler persistence'),
  TimerRuntime: decodeDisplayText('Timer runtime'),
} as const;

const TimerParentSurfaceProductClaims = {
  ActiveStateAndControlResults: decodeDisplayText(
    'Active timer state-store and control action-result rows are visible; live scheduling automation, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.'
  ),
  ActiveStateStore: decodeDisplayText(
    'Active timer state-store is visible; live scheduling execution, durable audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.'
  ),
  ControlActionResults: decodeDisplayText(
    'Control action-result rows are visible from app/game SQLite replay; live scheduling automation, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.'
  ),
} as const;

export type AppGameTimerParentSurfacePanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGameTimerParentSurfacePanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGameTimerParentSurfacePanelDetail[];
};

export type AppGameTimerParentSurfacePanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGameTimerParentSurfacePanelDetail[];
  readonly rows: readonly AppGameTimerParentSurfacePanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createAppGameTimerParentSurfacePanelIntent(
  readModelResult: AgentAppGameTimerParentSurfaceResult | null
): AppGameTimerParentSurfacePanelIntent {
  const base = baseIntent();

  if (readModelResult === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
        detail(PortalDetails.ProductClaim, base.productClaim),
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
        detail(PortalDetails.ProductClaim, base.productClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState: timerSurfaceLoadState(readModelResult.value),
    summaryDetails: readModelSummary(readModelResult.value, productClaim(readModelResult.value, base.productClaim)),
    rows: readModelResult.value.rows.map((row) =>
      timerSurfaceRow(row, productClaim(readModelResult.value, base.productClaim))
    ),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: resolvePortalDevText(PortalDevTextToken.AppGameTimerParentSurface),
    body: resolvePortalDevText(PortalDevTextToken.AppGameTimerParentSurfaceBody),
    emptyMessage: resolvePortalDevText(PortalDevTextToken.AppGameTimerParentSurfaceNoData),
    productClaim: resolvePortalDevText(PortalDevTextToken.AppGameTimerParentSurfaceNoRuntimeClaim),
  };
}

function readModelSummary(
  readModel: AgentAppGameTimerParentSurfaceReadModel,
  productClaim: DisplayText
): readonly AppGameTimerParentSurfacePanelDetail[] {
  return [
    detail(PortalDetails.Status, timerSurfaceLoadState(readModel)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.Custody, readableValue(readModel.custodyLabel)),
    detail(PortalDetails.Capability, readableValue(readModel.capabilityStatus)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
    detail(PortalDetails.ReadModelRows, displayText(String(readModel.readyForParentSurfaceCount))),
    detail(PortalDetails.ManualReview, displayText(String(readModel.runtimeManualRequiredCount))),
    detail(TimerParentSurfaceDetails.ControlActionResults, countText(readModel.controlActionResultCount)),
    detail(TimerParentSurfaceDetails.ControlActionResultRefs, actionResultReferences(readModel)),
    detail(
      TimerParentSurfaceDetails.ControlActionResultStatuses,
      joinedOrNotReported(readModel.controlActionResultStatuses)
    ),
    detail(
      TimerParentSurfaceDetails.ControlActionCapabilities,
      joinedOrNotReported(readModel.controlActionResultCapabilityStates)
    ),
    detail(
      TimerParentSurfaceDetails.ControlActionEnforcementStatuses,
      joinedOrNotReported(readModel.controlActionResultEnforcementStatuses)
    ),
    detail(
      TimerParentSurfaceDetails.ChildFacingReasonRefs,
      joinedOrNotReported(readModel.childFacingReasonReferenceIds)
    ),
    detail(
      TimerParentSurfaceDetails.ChildFacingStatusRefs,
      joinedOrNotReported(readModel.childFacingStatusReferenceIds)
    ),
    detail(TimerParentSurfaceDetails.ChildUxHandoffReady, countText(readModel.childUxHandoffReadyCount)),
    detail(TimerParentSurfaceDetails.ChildUxHandoffBlocked, countText(readModel.childUxHandoffBlockedCount)),
    detail(TimerParentSurfaceDetails.ChildUxHandoffRefs, joinedOrNotReported(readModel.childUxHandoffReferenceIds)),
    detail(
      TimerParentSurfaceDetails.ChildUxLocalArtifactRecords,
      countText(readModel.childUxLocalHandoffArtifactRecordCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxLocalArtifactSkipped,
      countText(readModel.childUxLocalHandoffArtifactSkippedCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxLocalArtifactRefs,
      joinedOrNotReported(readModel.childUxLocalHandoffArtifactReferenceIds)
    ),
    detail(TimerParentSurfaceDetails.TimerRuntime, claimedValue(readModel.timerRuntimeClaimed)),
    detail(TimerParentSurfaceDetails.SchedulerPersistence, claimedValue(readModel.schedulerPersistenceClaimed)),
    detail(TimerParentSurfaceDetails.DurableSchedulerStorage, claimedValue(readModel.durableSchedulerStorageClaimed)),
    detail(TimerParentSurfaceDetails.AuditRuntime, claimedValue(readModel.auditRuntimeClaimed)),
    detail(TimerParentSurfaceDetails.RollbackRuntime, claimedValue(readModel.rollbackRuntimeClaimed)),
    detail(PortalDetails.AdapterDispatch, Readable.NotClaimed),
    detail(PortalDetails.ChildDelivery, Readable.NotClaimed),
    detail(PortalDetails.PlatformState, Readable.NotClaimed),
    detail(PortalDetails.ProductClaim, productClaim),
  ];
}

function timerSurfaceRow(
  row: AgentAppGameTimerParentSurfaceRow,
  productClaim: DisplayText
): AppGameTimerParentSurfacePanelRow {
  return {
    title: displayText(row.rowId),
    details: [
      detail(PortalDetails.TargetType, TimerParentSurfaceTargetLabels[row.targetDomain]),
      detail(PortalDetails.Status, TimerParentSurfaceStateLabels[row.timerSurfaceState]),
      detail(PortalDetails.RowCount, countText(row.rowCount)),
      detail(PortalDetails.EvidenceReferences, evidenceReferences(row)),
      detail(PortalDetails.ProductClaim, productClaim),
    ],
  };
}

function timerSurfaceLoadState(readModel: AgentAppGameTimerParentSurfaceReadModel): DisplayText {
  if (readModel.returned === 0) {
    return Readable.Unavailable;
  }
  if (readModel.readyForParentSurfaceCount === readModel.returned) {
    return Readable.Ready;
  }
  return Readable.Review;
}

function productClaim(readModel: AgentAppGameTimerParentSurfaceReadModel, fallback: DisplayText): DisplayText {
  const hasActiveState =
    readModel.timerRuntimeClaimed || readModel.schedulerPersistenceClaimed || readModel.durableSchedulerStorageClaimed;
  const hasControlActionResults = readModel.controlActionResultCount > 0;

  if (hasActiveState && hasControlActionResults) {
    return TimerParentSurfaceProductClaims.ActiveStateAndControlResults;
  }
  if (hasActiveState) {
    return TimerParentSurfaceProductClaims.ActiveStateStore;
  }
  if (hasControlActionResults) {
    return TimerParentSurfaceProductClaims.ControlActionResults;
  }
  return fallback;
}

function claimedValue(value: boolean): DisplayText {
  return value ? Readable.Ready : Readable.NotClaimed;
}

function evidenceReferences(row: AgentAppGameTimerParentSurfaceRow): DisplayText {
  const references = [
    ...row.evidenceReferenceIds.map((reference) => String(reference)),
    ...row.evidence.map((evidence) => String(evidence.evidenceId)),
  ];
  const uniqueReferences = [...new Set(references)].filter(Boolean);

  if (uniqueReferences.length === 0) {
    return resolvePortalDevText(PortalDevTextToken.NotReported);
  }
  return displayText(uniqueReferences.join(DetailSeparator));
}

function actionResultReferences(readModel: AgentAppGameTimerParentSurfaceReadModel): DisplayText {
  return joinedOrNotReported(readModel.controlActionResultReferenceIds);
}

function joinedOrNotReported(values: readonly string[]): DisplayText {
  if (values.length === 0) {
    return resolvePortalDevText(PortalDevTextToken.NotReported);
  }
  return displayText(values.join(DetailSeparator));
}

function readableValue(value: unknown): DisplayText {
  const key = String(value);
  return PortalReadableValues[key] ?? displayText(key);
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

function detail(label: DisplayText, value: DisplayText): AppGameTimerParentSurfacePanelDetail {
  return {
    label,
    value,
  };
}
