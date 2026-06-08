import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
  type AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord,
  type AgentAppGameTimerParentSurfaceReadModel,
  type AgentAppGameTimerParentSurfaceResult,
  type AgentAppGameTimerParentSurfaceRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-timer-parent-surface-read-model';
import { AppGameChildUxParentPreferenceSetupDraftStatus } from '@ocentra-parent/parent-domain/app-game-child-facing-ux-parent-preference-setup-draft';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  HistoryRowVisible: decodeDisplayText('History row visible'),
  ManualRequired: requiredReadableValue('manual-required'),
  ManualActionRequired: decodeDisplayText('Manual action required'),
  NotClaimed: requiredReadableValue('not-claimed'),
  PreferenceSetupRequired: decodeDisplayText('Preference setup required'),
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
  ChildUxLocalArtifactRecordSources: decodeDisplayText('Child UX local artifact sources'),
  ChildUxLocalArtifactRecordTargets: decodeDisplayText('Child UX local artifact targets'),
  ChildUxLocalArtifactRefs: decodeDisplayText('Child UX local artifact refs'),
  ChildUxLocalArtifactSkipped: decodeDisplayText('Child UX local artifact skipped'),
  ChildUxParentSurfaceIntentHistoryVisible: decodeDisplayText('Child UX parent-surface history visible'),
  ChildUxParentSurfaceIntentArtifactRefs: decodeDisplayText('Child UX parent-surface artifact refs'),
  ChildUxParentSurfaceIntentDrillInRefs: decodeDisplayText('Child UX parent-surface drill-in refs'),
  ChildUxParentSurfaceIntentManualRequired: decodeDisplayText('Child UX parent-surface manual required'),
  ChildUxParentSurfaceIntentManualProofRefs: decodeDisplayText('Child UX parent-surface manual proof refs'),
  ChildUxParentSurfaceIntentPreferenceSetup: decodeDisplayText('Child UX parent-surface preference setup'),
  ChildUxParentSurfaceIntentRefs: decodeDisplayText('Child UX parent-surface refs'),
  ChildUxParentSurfaceIntentSources: decodeDisplayText('Child UX parent-surface sources'),
  ChildUxParentSurfaceIntentTargets: decodeDisplayText('Child UX parent-surface targets'),
  ChildUxParentSurfaceIntentUnavailable: decodeDisplayText('Child UX parent-surface unavailable'),
  ParentPreferenceSetupDraftRefs: decodeDisplayText('Parent preference setup draft refs'),
  ParentPreferenceSetupDraftStatus: decodeDisplayText('Parent preference setup draft status'),
  ParentPreferenceSetupMutation: decodeDisplayText('Parent preference setup mutation'),
  ParentPreferenceSetupRequestRefs: decodeDisplayText('Parent preference setup request refs'),
  ParentPreferenceSetupRequestStatus: decodeDisplayText('Parent preference setup request status'),
  ParentPreferenceSetupRequestUnavailable: decodeDisplayText('Parent preference setup request unavailable'),
  ParentPreferenceSetupRuleMutation: decodeDisplayText('Notification rule mutation'),
  ParentPreferenceSetupUi: decodeDisplayText('Parent preference setup UI'),
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
  readonly parentActionRows: readonly AppGameTimerParentSurfacePanelRow[];
  readonly parentPreferenceSetupRows: readonly AppGameTimerParentSurfacePanelRow[];
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
      parentActionRows: [],
      parentPreferenceSetupRows: [],
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
      parentActionRows: [],
      parentPreferenceSetupRows: [],
      rows: [],
    };
  }

  const resolvedProductClaim = productClaim(readModelResult.value, base.productClaim);

  return {
    ...base,
    loadState: timerSurfaceLoadState(readModelResult.value),
    summaryDetails: readModelSummary(readModelResult.value, resolvedProductClaim),
    parentActionRows: parentSurfaceIntentRows(readModelResult.value, resolvedProductClaim),
    parentPreferenceSetupRows: parentPreferenceSetupRows(readModelResult.value, resolvedProductClaim),
    rows: readModelResult.value.rows.map((row) => timerSurfaceRow(row, resolvedProductClaim)),
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
    detail(
      TimerParentSurfaceDetails.ChildUxLocalArtifactRecordSources,
      joinedOrNotReported(readModel.childUxLocalHandoffArtifactRecords.map((record) => record.sourceResultId))
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxLocalArtifactRecordTargets,
      joinedOrNotReported(readModel.childUxLocalHandoffArtifactRecords.map((record) => record.targetDomain))
    ),
    ...parentSurfaceIntentDetails(readModel),
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

function parentSurfaceIntentDetails(
  readModel: AgentAppGameTimerParentSurfaceReadModel
): readonly AppGameTimerParentSurfacePanelDetail[] {
  return [
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentManualRequired,
      countText(readModel.childUxParentSurfaceIntentManualActionRequiredCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentUnavailable,
      countText(readModel.childUxParentSurfaceIntentUnavailableVisibleCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentHistoryVisible,
      countText(readModel.childUxParentSurfaceIntentHistoryVisibleCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentPreferenceSetup,
      countText(readModel.childUxParentSurfaceIntentPreferenceSetupRequiredCount)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentRefs,
      joinedOrNotReported(readModel.childUxParentSurfaceIntentReferenceIds)
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentSources,
      joinedOrNotReported(readModel.childUxParentSurfaceIntentRecords.map((record) => record.sourceResultId))
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentArtifactRefs,
      joinedOrNotReported(readModel.childUxParentSurfaceIntentRecords.map((record) => record.sourceArtifactReferenceId))
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentTargets,
      joinedOrNotReported(readModel.childUxParentSurfaceIntentRecords.map((record) => record.targetDomain))
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentDrillInRefs,
      joinedOrNotReported(readModel.childUxParentSurfaceIntentRecords.flatMap((record) => record.drillInReferenceIds))
    ),
    detail(
      TimerParentSurfaceDetails.ChildUxParentSurfaceIntentManualProofRefs,
      joinedOrNotReported(
        readModel.childUxParentSurfaceIntentRecords.flatMap((record) => record.manualProofReferenceIds)
      )
    ),
    detail(
      TimerParentSurfaceDetails.ParentPreferenceSetupRequestStatus,
      countText(readModel.childUxParentPreferenceSetupRequestReadyCount)
    ),
    detail(
      TimerParentSurfaceDetails.ParentPreferenceSetupRequestUnavailable,
      countText(readModel.childUxParentPreferenceSetupRequestUnavailableVisibleCount)
    ),
    detail(
      TimerParentSurfaceDetails.ParentPreferenceSetupRequestRefs,
      joinedOrNotReported(readModel.childUxParentPreferenceSetupRequestReferenceIds)
    ),
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

function parentSurfaceIntentRows(
  readModel: AgentAppGameTimerParentSurfaceReadModel,
  productClaim: DisplayText
): readonly AppGameTimerParentSurfacePanelRow[] {
  return readModel.childUxParentSurfaceIntentRecords.map((record) => ({
    title: displayText(record.parentSurfaceIntentReferenceId),
    details: [
      detail(PortalDetails.TargetType, TimerParentSurfaceTargetLabels[record.targetDomain]),
      detail(PortalDetails.Status, parentSurfaceRecordReadableValue(record.parentSurfaceStatus)),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentArtifactRefs,
        displayText(record.sourceArtifactReferenceId)
      ),
      detail(TimerParentSurfaceDetails.ChildUxParentSurfaceIntentSources, displayText(record.sourceResultId)),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentHistoryVisible,
        parentSurfaceRecordReadableValue(record.historyVisibility)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentPreferenceSetup,
        parentSurfaceRecordReadableValue(record.preferenceVisibility)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentDrillInRefs,
        joinedOrNotReported(record.drillInReferenceIds)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentManualProofRefs,
        joinedOrNotReported(record.manualProofReferenceIds)
      ),
      detail(PortalDetails.AdapterDispatch, claimedValue(record.adapterDispatchClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(record.childDeliveryClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(record.platformEnforcementClaimed)),
      detail(PortalDetails.ProductClaim, productClaim),
    ],
  }));
}

function parentPreferenceSetupRows(
  readModel: AgentAppGameTimerParentSurfaceReadModel,
  productClaim: DisplayText
): readonly AppGameTimerParentSurfacePanelRow[] {
  return readModel.childUxParentPreferenceSetupRecords.map((record) => parentPreferenceSetupRow(record, productClaim));
}

function parentPreferenceSetupRow(
  record: AgentAppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord,
  productClaim: DisplayText
): AppGameTimerParentSurfacePanelRow {
  return {
    title: displayText(record.parentPreferenceSetupReferenceId),
    details: [
      detail(PortalDetails.TargetType, TimerParentSurfaceTargetLabels[record.targetDomain]),
      detail(
        TimerParentSurfaceDetails.ParentPreferenceSetupDraftStatus,
        parentPreferenceSetupDraftReadableValue(record.draftStatus)
      ),
      detail(
        TimerParentSurfaceDetails.ParentPreferenceSetupDraftRefs,
        displayText(record.parentPreferenceSetupReferenceId)
      ),
      detail(
        TimerParentSurfaceDetails.ParentPreferenceSetupRequestStatus,
        parentPreferenceSetupRequestReadableValue(record.parentPreferenceSetupRequestStatus)
      ),
      detail(
        TimerParentSurfaceDetails.ParentPreferenceSetupRequestRefs,
        joinedOrNotReported(record.parentPreferenceSetupRequestReferenceIds)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentRefs,
        displayText(record.sourceParentSurfaceIntentReferenceId)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentArtifactRefs,
        displayText(record.sourceArtifactReferenceId)
      ),
      detail(TimerParentSurfaceDetails.ChildUxParentSurfaceIntentSources, displayText(record.sourceResultId)),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentDrillInRefs,
        joinedOrNotReported(record.drillInReferenceIds)
      ),
      detail(
        TimerParentSurfaceDetails.ChildUxParentSurfaceIntentManualProofRefs,
        joinedOrNotReported(record.manualProofReferenceIds)
      ),
      detail(
        TimerParentSurfaceDetails.ParentPreferenceSetupUi,
        parentPreferenceSetupRequestUiReadableValue(record.parentPreferenceSetupRequestStatus)
      ),
      detail(TimerParentSurfaceDetails.ParentPreferenceSetupMutation, Readable.NotClaimed),
      detail(TimerParentSurfaceDetails.ParentPreferenceSetupRuleMutation, Readable.NotClaimed),
      detail(PortalDetails.AdapterDispatch, claimedValue(record.adapterDispatchClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(record.childDeliveryClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(record.platformEnforcementClaimed)),
      detail(PortalDetails.ProductClaim, productClaim),
    ],
  };
}

function parentPreferenceSetupRequestReadableValue(status: string): DisplayText {
  if (status === 'request-ready') {
    return Readable.Ready;
  }
  return Readable.Unavailable;
}

function parentPreferenceSetupRequestUiReadableValue(status: string): DisplayText {
  if (status === 'request-ready') {
    return Readable.Ready;
  }
  return Readable.NotClaimed;
}

function parentPreferenceSetupDraftReadableValue(
  status: (typeof AppGameChildUxParentPreferenceSetupDraftStatus)[keyof typeof AppGameChildUxParentPreferenceSetupDraftStatus]
): DisplayText {
  if (status === AppGameChildUxParentPreferenceSetupDraftStatus.DraftReady) {
    return Readable.PreferenceSetupRequired;
  }
  return Readable.Unavailable;
}

function parentSurfaceRecordReadableValue(value: string): DisplayText {
  if (value === 'history-row-visible') {
    return Readable.HistoryRowVisible;
  }
  if (value === 'manual-action-required') {
    return Readable.ManualActionRequired;
  }
  if (value === 'preference-setup-required') {
    return Readable.PreferenceSetupRequired;
  }
  return readableValue(value);
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
