import {
  BrowserAiChildUxSnapshotIdSchema,
  BrowserAiChildUxStateSchema,
  BrowserAiChildUxTextTokenSchema,
} from '@ocentra-parent/activity-domain/browser-ai-child-ux-schemas';
import {
  BrowserAiParentExplanationAuditRefSchema,
  BrowserAiParentExplanationIdSchema,
  BrowserAiParentExplanationStateSchema,
  BrowserAiParentExplanationTextTokenSchema,
} from '@ocentra-parent/activity-domain/browser-ai-parent-explanation-schemas';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const BrowserAiUxReadModelText = Schema.String.pipe(Schema.minLength(1));
const BrowserAiUxReadModelCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableBrowserAiUxReadModelText = Schema.Union(BrowserAiUxReadModelText, Schema.Null);

const BrowserAiUxReadModelEvidenceIdsSchema = Schema.Array(BrowserAiUxReadModelText).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser AI UX evidence ids')
);
const BrowserAiUxReadModelAuditRefsSchema = Schema.Array(BrowserAiParentExplanationAuditRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser AI UX audit refs')
);

export const BrowserAiUxReadModelSchemaVersion = 1;

const BrowserAiUxReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiUxReadModelSchemaVersion),
  rowId: BrowserAiUxReadModelText,
  sourceEvidenceIds: BrowserAiUxReadModelEvidenceIdsSchema,
  childSnapshotId: BrowserAiChildUxSnapshotIdSchema,
  childState: BrowserAiChildUxStateSchema,
  childPrimaryTextToken: BrowserAiChildUxTextTokenSchema,
  childDeliveryState: BrowserAiUxReadModelText,
  adapterProofRef: NullableBrowserAiUxReadModelText,
  parentExplanationId: BrowserAiParentExplanationIdSchema,
  parentExplanationState: BrowserAiParentExplanationStateSchema,
  parentTitleTextToken: BrowserAiParentExplanationTextTokenSchema,
  explanationAuditRefs: BrowserAiUxReadModelAuditRefsSchema,
  modelRuntimeVisible: Schema.Boolean,
  policyRuleVisible: Schema.Boolean,
  actionVisible: Schema.Boolean,
  childExperienceVisible: Schema.Boolean,
  degradedStateVisible: Schema.Boolean,
  manualFallbackVisible: Schema.Boolean,
  runtimeDeliveryClaimed: Schema.Boolean,
  renderedUiClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});

export const AgentBrowserAiUxReadModelRowSchema = withParser(
  BrowserAiUxReadModelRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiUxReadModelRowIsHonest(value) ||
        'Expected browser AI UX row to preserve proof, fallback, UI, and enforcement boundaries'
    )
  )
);

export const AgentBrowserAiUxReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserAiUxReadModelSchemaVersion),
    generatedAt: BrowserAiUxReadModelText,
    custodyLabel: BrowserAiUxReadModelText,
    capabilityStatus: BrowserAiUxReadModelText,
    returned: BrowserAiUxReadModelCount,
    latestEventId: NullableBrowserAiUxReadModelText,
    rows: Schema.Array(AgentBrowserAiUxReadModelRowSchema),
  }).pipe(
    Schema.filter(
      (value) =>
        value.returned === value.rows.length || 'Expected browser AI UX returned count to match row count'
    )
  )
);

export type AgentBrowserAiUxReadModelRow = Infer<typeof AgentBrowserAiUxReadModelRowSchema>;
export type AgentBrowserAiUxReadModel = Infer<typeof AgentBrowserAiUxReadModelSchema>;

export type AgentBrowserAiUxReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentBrowserAiUxReadModelResult =
  | {
      readonly ok: true;
      readonly value: AgentBrowserAiUxReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentBrowserAiUxReadModelFailureReason;
    };

export function parseAgentBrowserAiUxReadModelEvent(event: AgentEventEnvelope): AgentBrowserAiUxReadModelResult {
  if (event.event !== AgentEvent.BrowserAiUxReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserAiUxReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentBrowserAiUxReadModelSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function browserAiUxReadModelRowIsHonest(value: Infer<typeof BrowserAiUxReadModelRowBaseSchema>) {
  if (value.runtimeDeliveryClaimed || value.renderedUiClaimed || value.directEnforcementClaimed) {
    return false;
  }
  if (!requiredVisibilityIsPresent(value)) {
    return false;
  }
  if (!manualFallbackVisibilityMatchesState(value)) {
    return false;
  }
  return renderedChildStateHasAdapterProof(value);
}

function requiredVisibilityIsPresent(value: Infer<typeof BrowserAiUxReadModelRowBaseSchema>) {
  return value.modelRuntimeVisible && value.policyRuleVisible && value.actionVisible && value.childExperienceVisible;
}

function manualFallbackVisibilityMatchesState(value: Infer<typeof BrowserAiUxReadModelRowBaseSchema>) {
  const requiresManualVisibility =
    value.childState === 'manual_required' ||
    value.childState === 'unavailable' ||
    value.parentExplanationState === 'manual_required' ||
    value.parentExplanationState === 'unavailable';

  if (requiresManualVisibility) {
    return value.manualFallbackVisible && value.degradedStateVisible;
  }
  return true;
}

function renderedChildStateHasAdapterProof(value: Infer<typeof BrowserAiUxReadModelRowBaseSchema>) {
  if (
    value.childDeliveryState === 'checking-hold-rendered' ||
    value.childDeliveryState === 'warn-page-rendered' ||
    value.childDeliveryState === 'block-page-rendered' ||
    value.childDeliveryState === 'approval-hold-rendered'
  ) {
    return value.adapterProofRef !== null;
  }
  return true;
}

function adapterFailure(reason: AgentBrowserAiUxReadModelFailureReason): AgentBrowserAiUxReadModelResult {
  return {
    ok: false,
    reason,
  };
}
