import { ChildDomainRuntimeEventTypeLiteral } from './child-runtime-events';
import { EventingEventTypeSchema } from './eventing';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const NullableBrowserRuntimeText = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const AgentBrowserRuntimeEventType = {
  EvidenceObserved: 'browser.evidence.observed',
  EvidenceJournaled: 'browser.evidence.journaled',
  AiAnalysisRequested: ChildDomainRuntimeEventTypeLiteral.BrowserAiAnalysisRequested,
  AiAnalysisCompleted: 'browser.ai.analysis.completed',
  PolicyEvaluationRequested: ChildDomainRuntimeEventTypeLiteral.BrowserPolicyEvaluationRequested,
  PolicyDecisionCompleted: 'browser.policy.decision.completed',
  InterventionCommandIssued: 'browser.intervention.command.issued',
  InterventionResultObserved: 'browser.intervention.result.observed',
  AuditEntryCommitted: 'browser.audit.entry.committed',
  ReadModelProjected: 'browser.read-model.projected',
} as const;

export const AgentBrowserRuntimePhase = {
  EvidenceObserved: 'EvidenceObserved',
  EvidenceJournaled: 'EvidenceJournaled',
  AiAnalysisRequested: 'AiAnalysisRequested',
  AiAnalysisCompleted: 'AiAnalysisCompleted',
  PolicyEvaluationRequested: 'PolicyEvaluationRequested',
  PolicyDecisionCompleted: 'PolicyDecisionCompleted',
  InterventionCommandIssued: 'InterventionCommandIssued',
  InterventionResultObserved: 'InterventionResultObserved',
  AuditEntryCommitted: 'AuditEntryCommitted',
  ReadModelProjected: 'ReadModelProjected',
} as const;

export const AgentBrowserRuntimeCapabilityStatus = {
  Available: 'available',
  TabListOnly: 'tab-list-only',
  UnsupportedBrowser: 'unsupported-browser',
  UnmanagedBrowser: 'unmanaged-browser',
  ManagedProfileMissing: 'managed-profile-missing',
  BridgeMissing: 'bridge-missing',
  PermissionLimited: 'permission-limited',
  Stale: 'stale',
  AdapterError: 'adapter-error',
  DisabledByParent: 'disabled-by-parent',
} as const;

export const AgentBrowserRuntimeCustodyLabel = {
  ChildDeviceLocal: 'child-device-local',
  LocalNetworkChildAgent: 'local-network-child-agent',
  ParentCache: 'parent-cache',
  ParentOwnedExport: 'parent-owned-export',
  Unavailable: 'unavailable',
} as const;

export const AgentBrowserRuntimeQueryVisibility = {
  LiveLocal: 'live-local',
  LiveLan: 'live-lan',
  ParentCache: 'parent-cache',
  ParentOwnedExport: 'parent-owned-export',
  Unavailable: 'unavailable',
} as const;

export const AgentBrowserRuntimeEventTypeSchema = withParser(
  Schema.Literal(
    AgentBrowserRuntimeEventType.EvidenceObserved,
    AgentBrowserRuntimeEventType.EvidenceJournaled,
    AgentBrowserRuntimeEventType.AiAnalysisRequested,
    AgentBrowserRuntimeEventType.AiAnalysisCompleted,
    AgentBrowserRuntimeEventType.PolicyEvaluationRequested,
    AgentBrowserRuntimeEventType.PolicyDecisionCompleted,
    AgentBrowserRuntimeEventType.InterventionCommandIssued,
    AgentBrowserRuntimeEventType.InterventionResultObserved,
    AgentBrowserRuntimeEventType.AuditEntryCommitted,
    AgentBrowserRuntimeEventType.ReadModelProjected
  ).pipe(
    Schema.filter(
      (eventType) =>
        EventingEventTypeSchema.safeParse(eventType).success ||
        'Expected browser runtime event type to satisfy the shared eventing taxonomy'
    )
  )
);

export const AgentBrowserRuntimePhaseSchema = withParser(
  Schema.Literal(
    AgentBrowserRuntimePhase.EvidenceObserved,
    AgentBrowserRuntimePhase.EvidenceJournaled,
    AgentBrowserRuntimePhase.AiAnalysisRequested,
    AgentBrowserRuntimePhase.AiAnalysisCompleted,
    AgentBrowserRuntimePhase.PolicyEvaluationRequested,
    AgentBrowserRuntimePhase.PolicyDecisionCompleted,
    AgentBrowserRuntimePhase.InterventionCommandIssued,
    AgentBrowserRuntimePhase.InterventionResultObserved,
    AgentBrowserRuntimePhase.AuditEntryCommitted,
    AgentBrowserRuntimePhase.ReadModelProjected
  )
);

export const AgentBrowserRuntimeCapabilityStatusSchema = withParser(
  Schema.Literal(
    AgentBrowserRuntimeCapabilityStatus.Available,
    AgentBrowserRuntimeCapabilityStatus.TabListOnly,
    AgentBrowserRuntimeCapabilityStatus.UnsupportedBrowser,
    AgentBrowserRuntimeCapabilityStatus.UnmanagedBrowser,
    AgentBrowserRuntimeCapabilityStatus.ManagedProfileMissing,
    AgentBrowserRuntimeCapabilityStatus.BridgeMissing,
    AgentBrowserRuntimeCapabilityStatus.PermissionLimited,
    AgentBrowserRuntimeCapabilityStatus.Stale,
    AgentBrowserRuntimeCapabilityStatus.AdapterError,
    AgentBrowserRuntimeCapabilityStatus.DisabledByParent
  )
);

export const AgentBrowserRuntimeCustodyLabelSchema = withParser(
  Schema.Literal(
    AgentBrowserRuntimeCustodyLabel.ChildDeviceLocal,
    AgentBrowserRuntimeCustodyLabel.LocalNetworkChildAgent,
    AgentBrowserRuntimeCustodyLabel.ParentCache,
    AgentBrowserRuntimeCustodyLabel.ParentOwnedExport,
    AgentBrowserRuntimeCustodyLabel.Unavailable
  )
);

export const AgentBrowserRuntimeQueryVisibilitySchema = withParser(
  Schema.Literal(
    AgentBrowserRuntimeQueryVisibility.LiveLocal,
    AgentBrowserRuntimeQueryVisibility.LiveLan,
    AgentBrowserRuntimeQueryVisibility.ParentCache,
    AgentBrowserRuntimeQueryVisibility.ParentOwnedExport,
    AgentBrowserRuntimeQueryVisibility.Unavailable
  )
);

export const AgentBrowserRuntimeEventPayloadSchema = withParser(
  Schema.Struct({
    phase: AgentBrowserRuntimePhaseSchema,
    sourceRef: NonEmptyStringSchema,
    evidenceRef: NonEmptyStringSchema,
    capabilityStatus: AgentBrowserRuntimeCapabilityStatusSchema,
    custodyLabel: AgentBrowserRuntimeCustodyLabelSchema,
    queryVisibility: AgentBrowserRuntimeQueryVisibilitySchema,
    degradedReason: NullableBrowserRuntimeText,
    journalRef: NullableBrowserRuntimeText,
    aiRequestRef: NullableBrowserRuntimeText,
    aiAnalysisRef: NullableBrowserRuntimeText,
    policyEvaluationRef: NullableBrowserRuntimeText,
    policyDecisionRef: NullableBrowserRuntimeText,
    policyPreviewId: NullableBrowserRuntimeText,
    assistantActionIntentId: NullableBrowserRuntimeText,
    interventionCommandRef: NullableBrowserRuntimeText,
    interventionResultRef: NullableBrowserRuntimeText,
    auditEntryRef: NullableBrowserRuntimeText,
    readModelRef: NullableBrowserRuntimeText,
    previousPhaseRef: NullableBrowserRuntimeText,
    exactUrlClaimed: Schema.Boolean,
    aiAuthority: Schema.Literal(false),
    policyAuthority: Schema.Boolean,
    dryRun: Schema.Boolean,
    adapterDispatchClaimed: Schema.Boolean,
    interventionCommandAllowed: Schema.Boolean,
    observedAt: NonEmptyStringSchema,
  }).pipe(
    Schema.filter(
      (payload) =>
        browserRuntimePayloadIsHonest(payload) ||
        'Expected browser runtime payload to preserve evidence-only AI and intervention boundaries'
    )
  )
);

export const AgentBrowserRuntimeEventChainEntrySchema = withParser(
  Schema.Struct({
    eventType: AgentBrowserRuntimeEventTypeSchema,
    eventRef: NonEmptyStringSchema,
    payload: AgentBrowserRuntimeEventPayloadSchema,
  }).pipe(
    Schema.filter(
      (entry) =>
        phaseMatchesEventType(entry.payload.phase, entry.eventType) ||
        'Expected browser runtime event type to match the payload phase'
    )
  )
);

export const AgentBrowserRuntimeEventChainStreamSchema = withParser(
  Schema.Struct({
    observedRows: Schema.Number,
    streamedEvents: Schema.Number,
    failedRows: Schema.Number,
    exactUrlRows: Schema.Number,
    manualRequiredRows: Schema.Number,
    interventionCommandEvents: Schema.Number,
    readModelProjectionEvents: Schema.Number,
    actionIntentCandidates: Schema.Number,
    actionIntentHandoffCandidates: Schema.Number,
    actionIntentHandoffOutboxRefs: Schema.Array(NonEmptyStringSchema),
    actionIntentHandoffRefs: Schema.Array(NonEmptyStringSchema),
    actionIntentChildAcceptedRows: Schema.Number,
    actionIntentChildCommandRefs: Schema.Array(NonEmptyStringSchema),
    actionIntentChildAcceptedEventRefs: Schema.Array(NonEmptyStringSchema),
    actionIntentParentReadModelRefs: Schema.Array(NonEmptyStringSchema),
    actionIntentDispatchAttempts: Schema.Literal(0),
    actionIntentAdapterExecutions: Schema.Literal(0),
    actionIntentChildInterventionExecutions: Schema.Literal(0),
    actionIntentEnforcementExecutions: Schema.Literal(0),
    socialProviderReceiptBoundaryRows: Schema.Number,
    socialProviderDispatchRequiredRows: Schema.Number,
    socialProviderManualReceiptRequiredRows: Schema.Number,
    socialProviderAttemptRefs: Schema.Array(NonEmptyStringSchema),
    socialProviderReceiptProofRefs: Schema.Array(NonEmptyStringSchema),
    socialProviderDurableRows: Schema.Number,
    socialProviderDurableResultRefs: Schema.Array(NonEmptyStringSchema),
    socialProviderDurableStoreRefs: Schema.Array(NonEmptyStringSchema),
    socialProviderReadModelRefs: Schema.Array(NonEmptyStringSchema),
    socialProviderSupportStatusRefs: Schema.Array(NonEmptyStringSchema),
    entries: Schema.Array(AgentBrowserRuntimeEventChainEntrySchema),
  }).pipe(
    Schema.filter(
      (stream) =>
        stream.streamedEvents === stream.entries.length ||
        'Expected browser runtime streamedEvents to match event-chain entries'
    ),
    Schema.filter(
      (stream) =>
        stream.actionIntentCandidates >= actionIntentCandidatesFromEntries(stream.entries).length ||
        'Expected browser runtime action-intent candidate count to cover stream candidates'
    ),
    Schema.filter(
      (stream) =>
        (stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffOutboxRefs.length &&
          stream.actionIntentHandoffCandidates >= stream.actionIntentHandoffRefs.length &&
          stream.actionIntentHandoffOutboxRefs.length === stream.actionIntentHandoffRefs.length) ||
        'Expected browser runtime handoff refs to be paired with prepared handoff candidates'
    ),
    Schema.filter(
      (stream) =>
        browserRuntimeActionIntentChildStatusIsHonest(stream) ||
        'Expected browser runtime child status refs to match observed child accepted rows'
    ),
    Schema.filter(
      (stream) =>
        browserRuntimeSocialProviderReceiptStateIsHonest(stream) ||
        'Expected browser runtime social provider receipt refs to remain receipt-boundary proof only'
    )
  )
);

export type AgentBrowserRuntimeEventType = Infer<typeof AgentBrowserRuntimeEventTypeSchema>;
export type AgentBrowserRuntimePhase = Infer<typeof AgentBrowserRuntimePhaseSchema>;
export type AgentBrowserRuntimeCapabilityStatus = Infer<typeof AgentBrowserRuntimeCapabilityStatusSchema>;
export type AgentBrowserRuntimeCustodyLabel = Infer<typeof AgentBrowserRuntimeCustodyLabelSchema>;
export type AgentBrowserRuntimeQueryVisibility = Infer<typeof AgentBrowserRuntimeQueryVisibilitySchema>;
export type AgentBrowserRuntimeEventPayload = Infer<typeof AgentBrowserRuntimeEventPayloadSchema>;
export type AgentBrowserRuntimeEventChainEntry = Infer<typeof AgentBrowserRuntimeEventChainEntrySchema>;
export type AgentBrowserRuntimeEventChainStream = Infer<typeof AgentBrowserRuntimeEventChainStreamSchema>;

export type AgentBrowserRuntimeActionIntentCandidate = {
  readonly eventRef: string;
  readonly policyPreviewId: string;
  readonly assistantActionIntentId: string;
  readonly sourceRef: string;
  readonly evidenceRef: string;
  readonly observedAt: string;
};

function actionIntentCandidatesFromEntries(
  entries: readonly AgentBrowserRuntimeEventChainEntry[]
): AgentBrowserRuntimeActionIntentCandidate[] {
  return entries.flatMap((entry) => actionIntentCandidateFromEntry(entry));
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

function browserRuntimeSocialProviderReceiptStateIsHonest(stream: {
  readonly socialProviderReceiptBoundaryRows: number;
  readonly socialProviderDispatchRequiredRows: number;
  readonly socialProviderManualReceiptRequiredRows: number;
  readonly socialProviderAttemptRefs: readonly string[];
  readonly socialProviderReceiptProofRefs: readonly string[];
  readonly socialProviderDurableRows: number;
  readonly socialProviderDurableResultRefs: readonly string[];
  readonly socialProviderDurableStoreRefs: readonly string[];
  readonly socialProviderReadModelRefs: readonly string[];
  readonly socialProviderSupportStatusRefs: readonly string[];
}): boolean {
  if (
    stream.socialProviderReceiptBoundaryRows !==
    stream.socialProviderDispatchRequiredRows + stream.socialProviderManualReceiptRequiredRows
  ) {
    return false;
  }
  if (stream.socialProviderDispatchRequiredRows === 0) {
    return socialProviderReceiptRefsAreEmpty(stream);
  }
  return (
    stream.socialProviderAttemptRefs.length === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderReceiptProofRefs.length === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderDurableRows === stream.socialProviderDispatchRequiredRows &&
    stream.socialProviderDurableResultRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderDurableStoreRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderReadModelRefs.length === stream.socialProviderDurableRows &&
    stream.socialProviderSupportStatusRefs.length === stream.socialProviderDurableRows
  );
}

function socialProviderReceiptRefsAreEmpty(stream: {
  readonly socialProviderAttemptRefs: readonly string[];
  readonly socialProviderReceiptProofRefs: readonly string[];
  readonly socialProviderDurableRows: number;
  readonly socialProviderDurableResultRefs: readonly string[];
  readonly socialProviderDurableStoreRefs: readonly string[];
  readonly socialProviderReadModelRefs: readonly string[];
  readonly socialProviderSupportStatusRefs: readonly string[];
}): boolean {
  return (
    stream.socialProviderAttemptRefs.length === 0 &&
    stream.socialProviderReceiptProofRefs.length === 0 &&
    stream.socialProviderDurableRows === 0 &&
    stream.socialProviderDurableResultRefs.length === 0 &&
    stream.socialProviderDurableStoreRefs.length === 0 &&
    stream.socialProviderReadModelRefs.length === 0 &&
    stream.socialProviderSupportStatusRefs.length === 0
  );
}

function browserRuntimeActionIntentChildStatusIsHonest(stream: {
  readonly actionIntentChildAcceptedRows: number;
  readonly actionIntentChildCommandRefs: readonly string[];
  readonly actionIntentChildAcceptedEventRefs: readonly string[];
  readonly actionIntentParentReadModelRefs: readonly string[];
}): boolean {
  return (
    stream.actionIntentChildCommandRefs.length === stream.actionIntentChildAcceptedRows &&
    stream.actionIntentChildAcceptedEventRefs.length === stream.actionIntentChildAcceptedRows &&
    stream.actionIntentParentReadModelRefs.length === stream.actionIntentChildAcceptedRows
  );
}

function browserRuntimePayloadIsHonest(payload: {
  readonly exactUrlClaimed: boolean;
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
  readonly degradedReason: string | null;
  readonly dryRun: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly interventionCommandAllowed: boolean;
  readonly interventionCommandRef: string | null;
  readonly interventionResultRef: string | null;
}): boolean {
  if (!browserRuntimeContextSupportsExactUrl(payload) && payload.exactUrlClaimed) {
    return false;
  }
  if (!browserRuntimeUnavailableContextHasReason(payload)) {
    return false;
  }
  if (!payload.exactUrlClaimed && payload.interventionCommandAllowed) {
    return false;
  }
  if (!browserRuntimeDryRunHasNoDispatch(payload)) {
    return false;
  }
  if (payload.adapterDispatchClaimed && !payload.interventionCommandAllowed) {
    return false;
  }
  if (!payload.interventionCommandAllowed) {
    return payload.interventionCommandRef === null && payload.interventionResultRef === null;
  }
  return payload.interventionCommandRef !== null && payload.adapterDispatchClaimed;
}

function browserRuntimeContextSupportsExactUrl(payload: {
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly custodyLabel: AgentBrowserRuntimeCustodyLabel;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
}): boolean {
  const capabilityAllowsExactUrl =
    payload.capabilityStatus === AgentBrowserRuntimeCapabilityStatus.Available ||
    payload.capabilityStatus === AgentBrowserRuntimeCapabilityStatus.TabListOnly;
  const queryAllowsExactUrl =
    payload.queryVisibility === AgentBrowserRuntimeQueryVisibility.LiveLocal ||
    payload.queryVisibility === AgentBrowserRuntimeQueryVisibility.LiveLan;
  return (
    capabilityAllowsExactUrl &&
    queryAllowsExactUrl &&
    payload.custodyLabel !== AgentBrowserRuntimeCustodyLabel.Unavailable
  );
}

function browserRuntimeUnavailableContextHasReason(payload: {
  readonly capabilityStatus: AgentBrowserRuntimeCapabilityStatus;
  readonly queryVisibility: AgentBrowserRuntimeQueryVisibility;
  readonly degradedReason: string | null;
}): boolean {
  if (
    payload.queryVisibility !== AgentBrowserRuntimeQueryVisibility.Unavailable &&
    payload.capabilityStatus !== AgentBrowserRuntimeCapabilityStatus.BridgeMissing &&
    payload.capabilityStatus !== AgentBrowserRuntimeCapabilityStatus.Stale &&
    payload.capabilityStatus !== AgentBrowserRuntimeCapabilityStatus.AdapterError
  ) {
    return true;
  }
  return payload.degradedReason !== null;
}

function browserRuntimeDryRunHasNoDispatch(payload: {
  readonly dryRun: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly interventionCommandAllowed: boolean;
  readonly interventionCommandRef: string | null;
  readonly interventionResultRef: string | null;
}): boolean {
  if (!payload.dryRun) {
    return true;
  }
  return (
    !payload.adapterDispatchClaimed &&
    !payload.interventionCommandAllowed &&
    payload.interventionCommandRef === null &&
    payload.interventionResultRef === null
  );
}

function phaseMatchesEventType(phase: AgentBrowserRuntimePhase, eventType: AgentBrowserRuntimeEventType): boolean {
  return eventType === phaseToEventType(phase);
}

function phaseToEventType(phase: AgentBrowserRuntimePhase): AgentBrowserRuntimeEventType {
  switch (phase) {
    case AgentBrowserRuntimePhase.EvidenceObserved:
      return AgentBrowserRuntimeEventType.EvidenceObserved;
    case AgentBrowserRuntimePhase.EvidenceJournaled:
      return AgentBrowserRuntimeEventType.EvidenceJournaled;
    case AgentBrowserRuntimePhase.AiAnalysisRequested:
      return AgentBrowserRuntimeEventType.AiAnalysisRequested;
    case AgentBrowserRuntimePhase.AiAnalysisCompleted:
      return AgentBrowserRuntimeEventType.AiAnalysisCompleted;
    case AgentBrowserRuntimePhase.PolicyEvaluationRequested:
      return AgentBrowserRuntimeEventType.PolicyEvaluationRequested;
    case AgentBrowserRuntimePhase.PolicyDecisionCompleted:
      return AgentBrowserRuntimeEventType.PolicyDecisionCompleted;
    case AgentBrowserRuntimePhase.InterventionCommandIssued:
      return AgentBrowserRuntimeEventType.InterventionCommandIssued;
    case AgentBrowserRuntimePhase.InterventionResultObserved:
      return AgentBrowserRuntimeEventType.InterventionResultObserved;
    case AgentBrowserRuntimePhase.AuditEntryCommitted:
      return AgentBrowserRuntimeEventType.AuditEntryCommitted;
    case AgentBrowserRuntimePhase.ReadModelProjected:
      return AgentBrowserRuntimeEventType.ReadModelProjected;
  }
}
