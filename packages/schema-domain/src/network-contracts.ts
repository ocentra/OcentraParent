import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  ActivityCaptureCapabilityStatusSchema,
  ActivityDomainAttributionStatusSchema,
  ActivityProcessAttributionStatusSchema,
} from './activity-capture';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './evidence-primitives';

const NetworkConfidenceScore = Schema.Number.pipe(Schema.between(0, 1));
const NonEmptyNetworkEvidenceRefs = Schema.Array(ActivityEvidenceRefSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected at least one network evidence ref')
);

export const ActivityNetworkContractSchemaVersion = 1;

export const ActivityNetworkClaimScopeSchema = withParser(
  Schema.Literal(
    'destination-ip',
    'destination-domain',
    'protocol',
    'port',
    'process-attribution',
    'bytes-counts',
    'timing',
    'interface-state',
    'vpn-proxy-tunnel-indicator',
    'activity-category-candidate'
  )
);
export const ActivityNetworkUnsupportedClaimSchema = withParser(
  Schema.Literal('exact-url', 'exact-video', 'private-message', 'search-query', 'page-content', 'screen-activity', 'decrypted-payload')
);
export const ActivityNetworkEvidenceGradeSchema = withParser(Schema.Literal('A', 'B', 'C', 'D'));
export const ActivityNetworkDomainEvidenceSourceSchema = withParser(
  Schema.Literal('dns-query', 'dns-response', 'tls-sni', 'http-host', 'reverse-lookup', 'ip-only', 'unavailable')
);
export const ActivityNetworkActivityClassificationKindSchema = withParser(
  Schema.Literal('social', 'video', 'game', 'cloud-gaming', 'vpn-proxy-tunnel', 'tor', 'remote-desktop', 'torrent', 'download', 'update', 'school-productivity', 'unknown')
);
export const ActivityNetworkAdapterCapabilityStateSchema = withParser(
  Schema.Literal('proved-available', 'manual-required', 'unavailable', 'dry-run-only')
);
export const ActivityNetworkPolicyActionModeSchema = withParser(
  Schema.Literal('observe-only', 'dry-run', 'manual-required', 'adapter-unavailable', 'apply-ready')
);
export const ActivityNetworkPolicyActionKindSchema = withParser(
  Schema.Literal('none', 'ask-parent', 'warn-child', 'monitor', 'limit', 'block')
);

export const ActivityNetworkFlowEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityNetworkContractSchemaVersion),
    flowEvidenceId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    capabilityStatus: ActivityCaptureCapabilityStatusSchema,
    domainAttributionStatus: ActivityDomainAttributionStatusSchema,
    processAttributionStatus: ActivityProcessAttributionStatusSchema,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    confidence: NetworkConfidenceScore,
    claimScopes: Schema.Array(ActivityNetworkClaimScopeSchema),
    unsupportedClaimAttempts: Schema.Array(ActivityNetworkUnsupportedClaimSchema),
    evidence: NonEmptyNetworkEvidenceRefs,
  }).pipe(
    Schema.filter((flow) => flow.claimScopes.length > 0 || 'Expected network flow evidence to declare at least one supported claim scope'),
    Schema.filter((flow) => flow.unsupportedClaimAttempts.length === 0 || 'Network-only evidence cannot claim private content')
  )
);

export const ActivityNetworkDomainEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityNetworkContractSchemaVersion),
    domainEvidenceId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    source: ActivityNetworkDomainEvidenceSourceSchema,
    attributionStatus: ActivityDomainAttributionStatusSchema,
    domainName: Schema.Union(NonEmptyStringSchema, Schema.Null),
    destinationIp: Schema.Union(NonEmptyStringSchema, Schema.Null),
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    confidence: NetworkConfidenceScore,
    evidence: NonEmptyNetworkEvidenceRefs,
  }).pipe(
    Schema.filter((entry) => {
      const sourceHasDomain =
        entry.source === 'dns-query' ||
        entry.source === 'dns-response' ||
        entry.source === 'tls-sni' ||
        entry.source === 'http-host' ||
        entry.source === 'reverse-lookup';
      return !sourceHasDomain || entry.domainName !== null || 'Expected domain evidence source to include domainName';
    }),
    Schema.filter(
      (entry) =>
        entry.source !== 'ip-only' ||
        (entry.domainName === null && entry.destinationIp !== null && entry.attributionStatus === 'ip-only') ||
        'Expected ip-only domain evidence to omit domainName, include destinationIp, and use ip-only attribution'
    ),
    Schema.filter(
      (entry) =>
        entry.source !== 'unavailable' ||
        (entry.domainName === null && entry.evidenceGrade === 'D' && entry.attributionStatus === 'unavailable') ||
        'Expected unavailable domain evidence to be grade D with unavailable attribution and no domainName'
    )
  )
);

export const ActivityNetworkActivityClassificationSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityNetworkContractSchemaVersion),
    classificationId: ActivityEvidenceIdSchema,
    classifiedAt: ActivityTimestampSchema,
    kind: ActivityNetworkActivityClassificationKindSchema,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    confidence: NetworkConfidenceScore,
    uncertaintyReason: Schema.Union(NonEmptyStringSchema, Schema.Null),
    evidenceIds: Schema.Array(ActivityEvidenceIdSchema),
    evidence: NonEmptyNetworkEvidenceRefs,
  }).pipe(
    Schema.filter((entry) => entry.evidenceIds.length > 0 || 'Expected network activity classification to cite source evidence ids'),
    Schema.filter(
      (entry) =>
        entry.kind !== 'unknown' ||
        (entry.uncertaintyReason !== null && (entry.evidenceGrade === 'C' || entry.evidenceGrade === 'D')) ||
        'Expected unknown network classification to carry uncertainty and grade C/D'
    )
  )
);

export const ActivityNetworkAdapterCapabilitySchema = withParser(
  Schema.Struct({
    capabilityId: NonEmptyStringSchema,
    state: ActivityNetworkAdapterCapabilityStateSchema,
    proofRefs: Schema.Array(ActivityEvidenceRefSchema),
    manualRequiredReason: Schema.Union(NonEmptyStringSchema, Schema.Null),
  }).pipe(
    Schema.filter((entry) => entry.state !== 'proved-available' || entry.proofRefs.length > 0 || 'Expected proved network adapter capability to cite proof refs'),
    Schema.filter((entry) => (entry.state !== 'manual-required' && entry.state !== 'unavailable') || entry.manualRequiredReason !== null || 'Expected manual-required or unavailable network capability to include a reason')
  )
);

export const ActivityNetworkPolicyActionSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityNetworkContractSchemaVersion),
    actionId: ActivityEvidenceIdSchema,
    decidedAt: ActivityTimestampSchema,
    mode: ActivityNetworkPolicyActionModeSchema,
    action: ActivityNetworkPolicyActionKindSchema,
    evidenceGrade: ActivityNetworkEvidenceGradeSchema,
    policyDecisionRef: Schema.Union(NonEmptyStringSchema, Schema.Null),
    adapterCapability: ActivityNetworkAdapterCapabilitySchema,
    adapterCallAuthorized: Schema.Boolean,
    evidence: NonEmptyNetworkEvidenceRefs,
  }).pipe(
    Schema.filter((entry) => {
      const canApply =
        entry.mode === 'apply-ready' &&
        entry.evidenceGrade === 'A' &&
        entry.adapterCapability.state === 'proved-available' &&
        entry.policyDecisionRef !== null &&
        (entry.action === 'monitor' || entry.action === 'limit' || entry.action === 'block');
      return entry.adapterCallAuthorized === canApply || 'Expected adapter authorization to match policy and proof state';
    }),
    Schema.filter((entry) => entry.mode !== 'observe-only' || (entry.action === 'none' && entry.adapterCallAuthorized === false) || 'Expected observe-only network policy action to avoid adapter calls')
  )
);

export type ActivityNetworkClaimScope = Infer<typeof ActivityNetworkClaimScopeSchema>;
export type ActivityNetworkUnsupportedClaim = Infer<typeof ActivityNetworkUnsupportedClaimSchema>;
export type ActivityNetworkEvidenceGrade = Infer<typeof ActivityNetworkEvidenceGradeSchema>;
export type ActivityNetworkDomainEvidenceSource = Infer<typeof ActivityNetworkDomainEvidenceSourceSchema>;
export type ActivityNetworkActivityClassificationKind = Infer<typeof ActivityNetworkActivityClassificationKindSchema>;
export type ActivityNetworkAdapterCapabilityState = Infer<typeof ActivityNetworkAdapterCapabilityStateSchema>;
export type ActivityNetworkPolicyActionMode = Infer<typeof ActivityNetworkPolicyActionModeSchema>;
export type ActivityNetworkPolicyActionKind = Infer<typeof ActivityNetworkPolicyActionKindSchema>;
export type ActivityNetworkFlowEvidence = Infer<typeof ActivityNetworkFlowEvidenceSchema>;
export type ActivityNetworkDomainEvidence = Infer<typeof ActivityNetworkDomainEvidenceSchema>;
export type ActivityNetworkActivityClassification = Infer<typeof ActivityNetworkActivityClassificationSchema>;
export type ActivityNetworkAdapterCapability = Infer<typeof ActivityNetworkAdapterCapabilitySchema>;
export type ActivityNetworkPolicyAction = Infer<typeof ActivityNetworkPolicyActionSchema>;
