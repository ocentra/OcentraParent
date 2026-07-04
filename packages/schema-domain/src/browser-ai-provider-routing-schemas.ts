import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserCustodyLabelSchema } from './browser-schemas';
import {
  BrowserAiModelRuntimePreferenceSchema,
  BrowserAiModelRuntimeRefSchema,
  BrowserAiRequestedTaskSchema,
  BrowserUrlAiAnalysisInputSchema,
  BrowserUrlAiAnalysisRequestIdSchema,
} from './browser-ai-analysis-schemas';
import {
  browserAiProviderCapabilityIsConsistent,
} from './browser-ai-provider-routing-capability';
import {
  browserAiProviderRouteIsConsistent,
  planBrowserAiLocalProviderRoute as planBrowserAiLocalProviderRouteLogic,
} from './browser-ai-provider-routing-planning';
const OptionalProviderTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalModelRuntimeRefSchema = Schema.Union(BrowserAiModelRuntimeRefSchema, Schema.Null);

export const BrowserAiProviderRouteSchemaVersion = 1;

export const BrowserAiProviderRouteIdSchema = withParser(brandedNonEmptyStringSchema('BrowserAiProviderRouteId'));
export const BrowserAiProviderIdSchema = withParser(brandedNonEmptyStringSchema('BrowserAiProviderId'));

export const BrowserAiProviderKindSchema = withParser(
  Schema.Literal('child-device-local-ai', 'family-ai-hub', 'parent-approved-remote-ai', 'metadata-only', 'no-ai')
);
export const BrowserAiProviderRouteModeSchema = withParser(
  Schema.Literal(
    'local-only',
    'local-then-family-hub',
    'local-then-parent-approved-remote',
    'metadata-only',
    'parent-review-when-unavailable'
  )
);
export const BrowserAiProviderCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabled-by-parent', 'model-missing', 'provider-unavailable', 'resource-exhausted')
);
export const BrowserAiProviderDegradedStateSchema = withParser(
  Schema.Literal(
    'none',
    'disabled-by-parent',
    'model-missing',
    'provider-unavailable',
    'resource-exhausted',
    'unsupported-task',
    'custody-unsafe',
    'manual-required'
  )
);
export const BrowserAiProviderExecutionStateSchema = withParser(
  Schema.Literal('selected', 'degraded', 'manual-required', 'unavailable')
);

const SupportedTasksSchema = Schema.Array(BrowserAiRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI supported task')
);
const AuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one browser AI provider audit evidence id')
);
const DegradedStatesSchema = Schema.Array(BrowserAiProviderDegradedStateSchema);

const BrowserAiProviderCapabilityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiProviderRouteSchemaVersion),
  providerId: BrowserAiProviderIdSchema,
  checkedAt: ActivityTimestampSchema,
  providerKind: BrowserAiProviderKindSchema,
  capabilityState: BrowserAiProviderCapabilityStateSchema,
  supportedTasks: SupportedTasksSchema,
  modelRuntimeRef: OptionalModelRuntimeRefSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  noRetention: Schema.Boolean,
  localOnly: Schema.Boolean,
  parentApprovedRemoteEnabled: Schema.Boolean,
  canRunOnChildDevice: Schema.Boolean,
  degradedStates: DegradedStatesSchema,
  unavailableReason: OptionalProviderTextSchema,
});
export const BrowserAiProviderCapabilitySchema = withParser(
  BrowserAiProviderCapabilityBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiProviderCapabilityIsConsistent(value) ||
        'Expected browser AI provider capability to preserve local custody, retention, and availability boundaries'
    )
  )
);

const BrowserAiProviderRouteBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiProviderRouteSchemaVersion),
  routeId: BrowserAiProviderRouteIdSchema,
  requestId: BrowserUrlAiAnalysisRequestIdSchema,
  routedAt: ActivityTimestampSchema,
  routeMode: BrowserAiProviderRouteModeSchema,
  modelRuntimePreference: BrowserAiModelRuntimePreferenceSchema,
  providerKind: BrowserAiProviderKindSchema,
  capability: BrowserAiProviderCapabilitySchema,
  executionState: BrowserAiProviderExecutionStateSchema,
  selectedRuntimeRef: OptionalModelRuntimeRefSchema,
  degradedStates: DegradedStatesSchema,
  auditEvidenceIds: AuditEvidenceIdsSchema,
  dataScopeVisible: Schema.Boolean,
  retentionVisible: Schema.Boolean,
  custodyVisible: Schema.Boolean,
  providerVisible: Schema.Boolean,
  noRetentionVisible: Schema.Boolean,
  parentExplicitRemoteApproval: Schema.Boolean,
  remoteDefaultForBlocking: Schema.Boolean,
  remoteCanOverrideStricterLocalRules: Schema.Boolean,
  remoteOutageDisablesLocalSafety: Schema.Boolean,
});
export const BrowserAiProviderRouteSchema = withParser(
  BrowserAiProviderRouteBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiProviderRouteIsConsistent(value) ||
        'Expected browser AI provider route to be auditable, visible, and non-remote-default'
    )
  )
);

const BrowserAiLocalProviderRouteRequestSchema = withParser(
  Schema.Struct({
    routeId: BrowserAiProviderRouteIdSchema,
    routedAt: ActivityTimestampSchema,
    input: BrowserUrlAiAnalysisInputSchema,
    capability: BrowserAiProviderCapabilitySchema,
    auditEvidenceIds: AuditEvidenceIdsSchema,
  })
);

export const decodeBrowserAiProviderCapability = Schema.decodeUnknownSync(BrowserAiProviderCapabilitySchema);
export const decodeBrowserAiProviderRoute = Schema.decodeUnknownSync(BrowserAiProviderRouteSchema);

export function planBrowserAiLocalProviderRoute(
  request: Infer<typeof BrowserAiLocalProviderRouteRequestSchema>
): BrowserAiProviderRoute {
  return BrowserAiProviderRouteSchema.parse(planBrowserAiLocalProviderRouteLogic(BrowserAiLocalProviderRouteRequestSchema.parse(request)));
}

export type BrowserAiProviderKind = Infer<typeof BrowserAiProviderKindSchema>;
export type BrowserAiProviderCapability = Infer<typeof BrowserAiProviderCapabilitySchema>;
export type BrowserAiProviderDegradedState = Infer<typeof BrowserAiProviderDegradedStateSchema>;
export type BrowserAiProviderRoute = Infer<typeof BrowserAiProviderRouteSchema>;
