import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentActorIdSchema, ParentDeviceIdSchema } from './reference-primitives';
import { ParentEvidenceReferenceSchema } from './references';
import {
  LanPairingAuditEventIdSchema,
  LanPairingAuditEventTypeSchema,
  LanPairingControllerLeaseIdSchema,
  LanPairingIdSchema,
  LanPairingIntentIdSchema,
  LanPairingIntentKindSchema,
  LanPairingOriginSchema,
  LanPairingParentAuthoritySchema,
  LanPairingProofDigestSchema,
  LanPairingRejectionReasonSchema,
  LanPairingResponseStateSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';

export const LanPairingParentIntentEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    intentId: LanPairingIntentIdSchema,
    intentKind: LanPairingIntentKindSchema,
    targetChildDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    pairingId: LanPairingIdSchema,
    proofDigest: LanPairingProofDigestSchema,
    origin: LanPairingOriginSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
    controllerLeaseId: LanPairingControllerLeaseIdSchema,
    controllerDeviceId: ParentDeviceIdSchema,
    parentActorId: ParentActorIdSchema,
    parentAuthority: LanPairingParentAuthoritySchema,
    controllerLeaseIssuedAt: LanPairingTimestampSchema,
    controllerLeaseExpiresAt: LanPairingTimestampSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  })
);

export const LanChildAgentResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    intentId: LanPairingIntentIdSchema,
    targetChildDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    state: LanPairingResponseStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    auditEventId: LanPairingAuditEventIdSchema,
    respondedAt: LanPairingTimestampSchema,
  })
);

export const LanPairingAuditEventSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    auditEventId: LanPairingAuditEventIdSchema,
    eventType: LanPairingAuditEventTypeSchema,
    pairingId: Schema.Union(LanPairingIdSchema, Schema.Null),
    intentId: Schema.Union(LanPairingIntentIdSchema, Schema.Null),
    childDeviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    parentDeviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    controllerLeaseId: Schema.Union(LanPairingControllerLeaseIdSchema, Schema.Null),
    controllerDeviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    parentActorId: Schema.Union(ParentActorIdSchema, Schema.Null),
    routeId: LanPairingRouteIdSchema,
    origin: Schema.Union(LanPairingOriginSchema, Schema.Null),
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    observedAt: LanPairingTimestampSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  })
);

export type LanPairingParentIntentEnvelope = Infer<typeof LanPairingParentIntentEnvelopeSchema>;
export type LanChildAgentResponse = Infer<typeof LanChildAgentResponseSchema>;
export type LanPairingAuditEvent = Infer<typeof LanPairingAuditEventSchema>;
