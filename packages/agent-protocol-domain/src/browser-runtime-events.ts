import { type LogFields } from '@ocentra-parent/logging-domain/contracts';
import { type Infer, type SafeParseResult, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentProtocolDefaults } from './defaults';

const BrowserRuntimeText = Schema.String.pipe(Schema.minLength(1));
const NullableBrowserRuntimeText = Schema.Union(BrowserRuntimeText, Schema.Null);

export const AgentBrowserRuntimeEventType = {
  EvidenceObserved: 'browser.evidence.observed',
  EvidenceJournaled: 'browser.evidence.journaled',
  AiAnalysisRequested: 'browser.ai-analysis.requested',
  AiAnalysisCompleted: 'browser.ai-analysis.completed',
  PolicyEvaluationRequested: 'browser.policy-evaluation.requested',
  PolicyDecisionCompleted: 'browser.policy-decision.completed',
  InterventionCommandIssued: 'browser.intervention-command.issued',
  InterventionResultObserved: 'browser.intervention-result.observed',
  AuditEntryCommitted: 'browser.audit-entry.committed',
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
    sourceRef: BrowserRuntimeText,
    evidenceRef: BrowserRuntimeText,
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
    observedAt: BrowserRuntimeText,
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
    eventRef: BrowserRuntimeText,
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
    actionIntentHandoffOutboxRefs: Schema.Array(BrowserRuntimeText),
    actionIntentHandoffRefs: Schema.Array(BrowserRuntimeText),
    actionIntentDispatchAttempts: Schema.Literal(0),
    actionIntentAdapterExecutions: Schema.Literal(0),
    actionIntentChildInterventionExecutions: Schema.Literal(0),
    actionIntentEnforcementExecutions: Schema.Literal(0),
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

export type AgentBrowserRuntimeActionIntentStatus = {
  readonly candidateCount: number;
  readonly handoffCandidateCount: number;
  readonly handoffOutboxRefs: readonly string[];
  readonly handoffRefs: readonly string[];
  readonly dispatchAttemptCount: 0;
  readonly adapterExecutionCount: 0;
  readonly childInterventionExecutionCount: 0;
  readonly enforcementExecutionCount: 0;
  readonly dryRunOnly: true;
  readonly policyAuthorityOnly: true;
  readonly candidates: readonly AgentBrowserRuntimeActionIntentCandidate[];
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

  return streamResult(
    AgentBrowserRuntimeEventChainStreamSchema.safeParse({
      observedRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeObservedRows),
      streamedEvents: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents),
      failedRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeFailedRows),
      exactUrlRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows),
      manualRequiredRows: numberField(fields, AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows),
      interventionCommandEvents: numberField(
        fields,
        AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents
      ),
      readModelProjectionEvents: numberField(
        fields,
        AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents
      ),
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
      entries,
    })
  );
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
    dispatchAttemptCount: 0,
    adapterExecutionCount: 0,
    childInterventionExecutionCount: 0,
    enforcementExecutionCount: 0,
    dryRunOnly: true,
    policyAuthorityOnly: true,
    candidates,
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
