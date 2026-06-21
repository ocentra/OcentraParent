import {
  Schema,
  withParser,
  type Infer,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  TrackingAiProviderRouteSchema,
  TrackingEvidenceTraceSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy';
import { TrackingPolicyAuditRefSchema, TrackingPolicyReasonCodeSchema } from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const CustodyRefSchema = brandedNonEmptyStringSchema('TrackingAiCustodyRef');
const CustodyRefParsedSchema = withParser(CustodyRefSchema);
const ParentEvidenceReferenceIdParsedSchema = withParser(ParentEvidenceReferenceIdSchema);
const RoutePurposeSchema = withParser(
  Schema.Literal(
    'child-safety-default',
    'parent-device-context',
    'family-hub-local',
    'parent-approved-remote-report',
    'metadata-only-fallback',
    'no-ai-unavailable'
  )
);

export const TrackingAiProviderRoutingProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    purpose: RoutePurposeSchema,
    route: TrackingAiProviderRouteSchema,
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema).pipe(Schema.minItems(1)),
    custodyRefs: Schema.Array(CustodyRefSchema).pipe(Schema.minItems(1)),
    defaultChildSafetyPath: Schema.Boolean,
    explicitParentApprovalRequired: Schema.Boolean,
    parentApprovalRecorded: Schema.Boolean,
    assistantPreviewOnly: Schema.Literal(true),
    assistantCanWritePolicyDirectly: Schema.Literal(false),
    assistantCanEnforceDirectly: Schema.Literal(false),
    childAgentValidationRequired: Schema.Literal(true),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (row) =>
        !row.route.remoteDataAllowed ||
        (row.explicitParentApprovalRequired &&
          row.parentApprovalRecorded &&
          row.purpose === 'parent-approved-remote-report') ||
        'Tracking remote AI routes need explicit parent approval and must stay outside child safety decisions'
    )
  )
);

export type TrackingAiProviderRoutingProofRow = Infer<typeof TrackingAiProviderRoutingProofRowSchema>;

export function buildTrackingAiProviderRoutingProofRows(): readonly TrackingAiProviderRoutingProofRow[] {
  return [
    row({
      purpose: 'child-safety-default',
      mode: 'child-local',
      capabilityState: 'available',
      remoteDataAllowed: false,
      unavailableReason: null,
      defaultChildSafetyPath: true,
      explicitParentApprovalRequired: false,
      parentApprovalRecorded: false,
      reasonCodes: ['child-local-ai-default-safety-path'],
    }),
    row({
      purpose: 'parent-device-context',
      mode: 'parent-local',
      capabilityState: 'available',
      remoteDataAllowed: false,
      unavailableReason: null,
      defaultChildSafetyPath: false,
      explicitParentApprovalRequired: false,
      parentApprovalRecorded: false,
      reasonCodes: ['parent-local-context-route'],
    }),
    row({
      purpose: 'family-hub-local',
      mode: 'family-ai-hub',
      capabilityState: 'degraded',
      remoteDataAllowed: false,
      unavailableReason: 'family-ai-hub-degraded',
      defaultChildSafetyPath: false,
      explicitParentApprovalRequired: false,
      parentApprovalRecorded: false,
      reasonCodes: ['family-ai-hub-degraded'],
    }),
    row({
      purpose: 'parent-approved-remote-report',
      mode: 'parent-approved-remote',
      capabilityState: 'manual-required',
      remoteDataAllowed: true,
      unavailableReason: null,
      defaultChildSafetyPath: false,
      explicitParentApprovalRequired: true,
      parentApprovalRecorded: true,
      reasonCodes: ['remote-ai-parent-approved-report-only'],
    }),
    row({
      purpose: 'metadata-only-fallback',
      mode: 'metadata-only',
      capabilityState: 'disabled-by-default',
      remoteDataAllowed: false,
      unavailableReason: 'remote-ai-disabled-by-default',
      defaultChildSafetyPath: false,
      explicitParentApprovalRequired: false,
      parentApprovalRecorded: false,
      reasonCodes: ['metadata-only-ai-fallback'],
    }),
    row({
      purpose: 'no-ai-unavailable',
      mode: 'no-ai',
      capabilityState: 'unavailable',
      remoteDataAllowed: false,
      unavailableReason: 'ai-provider-unavailable',
      defaultChildSafetyPath: false,
      explicitParentApprovalRequired: false,
      parentApprovalRecorded: false,
      reasonCodes: ['ai-provider-unavailable'],
    }),
  ];
}

export function summarizeTrackingAiProviderRoutingProof(rows: readonly TrackingAiProviderRoutingProofRow[]) {
  const parsedRows = rows.map((entry) => TrackingAiProviderRoutingProofRowSchema.parse(entry));

  return {
    routeModes: parsedRows.map((entry) => entry.route.mode),
    capabilityStates: parsedRows.map((entry) => entry.route.capabilityState),
    defaultChildSafetyRouteCount: parsedRows.filter((entry) => entry.defaultChildSafetyPath).length,
    remoteAllowedRouteCount: parsedRows.filter((entry) => entry.route.remoteDataAllowed).length,
    remoteAllowedRoutesRequireParentApproval: parsedRows
      .filter((entry) => entry.route.remoteDataAllowed)
      .every((entry) => entry.explicitParentApprovalRequired && entry.parentApprovalRecorded),
    degradedOrUnavailableRouteCount: parsedRows.filter((entry) =>
      ['degraded', 'unavailable', 'disabled-by-default', 'manual-required'].includes(entry.route.capabilityState)
    ).length,
    assistantCanWritePolicyDirectly: parsedRows.some((entry) => entry.assistantCanWritePolicyDirectly),
    assistantCanEnforceDirectly: parsedRows.some((entry) => entry.assistantCanEnforceDirectly),
    allRowsHaveEvidenceAndCustody: parsedRows.every(
      (entry) => entry.evidenceReferences.length > 0 && entry.custodyRefs.length > 0
    ),
  };
}

function row(input: {
  readonly purpose: Infer<typeof RoutePurposeSchema>;
  readonly mode: Infer<typeof TrackingAiProviderRouteSchema>['mode'];
  readonly capabilityState: Infer<typeof TrackingAiProviderRouteSchema>['capabilityState'];
  readonly remoteDataAllowed: boolean;
  readonly unavailableReason: string | null;
  readonly defaultChildSafetyPath: boolean;
  readonly explicitParentApprovalRequired: boolean;
  readonly parentApprovalRecorded: boolean;
  readonly reasonCodes: readonly string[];
}): TrackingAiProviderRoutingProofRow {
  return TrackingAiProviderRoutingProofRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    purpose: input.purpose,
    route: {
      schemaVersion: TrackingPolicySchemaVersion,
      providerRouteId: `${input.mode}-tracking-ai-route`,
      mode: input.mode,
      capabilityState: input.capabilityState,
      remoteDataAllowed: input.remoteDataAllowed,
      unavailableReason: input.unavailableReason === null ? null : reasonCode(input.unavailableReason),
      auditRefs: ['tracking-ai-provider-route-proof'],
    },
    evidenceReferences: [
      {
        evidenceReferenceId: evidenceRef(`${input.mode}-evidence-ref`),
        kind: 'journal-event',
        observedAt: '2026-06-06T18:32:00.000Z',
      },
    ],
    custodyRefs: [custodyRef(`${input.mode}-local-custody-ref`)],
    defaultChildSafetyPath: input.defaultChildSafetyPath,
    explicitParentApprovalRequired: input.explicitParentApprovalRequired,
    parentApprovalRecorded: input.parentApprovalRecorded,
    assistantPreviewOnly: true,
    assistantCanWritePolicyDirectly: false,
    assistantCanEnforceDirectly: false,
    childAgentValidationRequired: true,
    reasonCodes: input.reasonCodes.map(reasonCode),
    auditRefs: ['tracking-ai-provider-routing-proof'],
  });
}

function evidenceRef(value: string): Infer<typeof ParentEvidenceReferenceIdSchema> {
  return ParentEvidenceReferenceIdParsedSchema.parse(value);
}

function reasonCode(value: string): Infer<typeof TrackingPolicyReasonCodeSchema> {
  return TrackingPolicyReasonCodeSchema.parse(value);
}

function custodyRef(value: string): Infer<typeof CustodyRefSchema> {
  return CustodyRefParsedSchema.parse(value);
}

