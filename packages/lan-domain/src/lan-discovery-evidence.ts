import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { LanPairingSchemaVersionSchema, LanPairingTimestampSchema } from './lan-pairing-values';

export const LanDiscoveryEvidenceIdSchema = brandedNonEmptyStringSchema('LanDiscoveryEvidenceId');

export const LanDiscoveryEvidenceSourceSchema = withParser(
  Schema.Literal(
    'local-service',
    'windows-neighbor-table',
    'dns-cache',
    'netbios',
    'trusted-registry',
    'parent-assignment',
    'child-agent-hello',
    'child-agent-heartbeat'
  )
);

export const LanDiscoveryEvidenceKindSchema = withParser(
  Schema.Literal(
    'interface',
    'ip-address',
    'mac-address',
    'hostname',
    'vendor',
    'router-classification',
    'child-agent-presence',
    'trusted-registry',
    'parent-decision',
    'route'
  )
);

export const LanDiscoveryEvidenceConfidenceSchema = withParser(
  Schema.Literal('confirmed', 'strong', 'weak', 'manual-required', 'rejected')
);

export const LanDiscoveryEvidenceRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    evidenceId: LanDiscoveryEvidenceIdSchema,
    source: LanDiscoveryEvidenceSourceSchema,
    evidenceKind: LanDiscoveryEvidenceKindSchema,
    deviceId: ParentDeviceIdSchema,
    value: NonEmptyStringSchema,
    normalizedValue: NonEmptyStringSchema,
    firstSeenAt: LanPairingTimestampSchema,
    lastSeenAt: LanPairingTimestampSchema,
    expiresAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    confidence: LanDiscoveryEvidenceConfidenceSchema,
    mergeKey: NonEmptyStringSchema,
    note: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export type LanDiscoveryEvidenceId = typeof LanDiscoveryEvidenceIdSchema.Type;
export type LanDiscoveryEvidenceSource = Infer<typeof LanDiscoveryEvidenceSourceSchema>;
export type LanDiscoveryEvidenceKind = Infer<typeof LanDiscoveryEvidenceKindSchema>;
export type LanDiscoveryEvidenceConfidence = Infer<typeof LanDiscoveryEvidenceConfidenceSchema>;
export type LanDiscoveryEvidenceRecord = Infer<typeof LanDiscoveryEvidenceRecordSchema>;

