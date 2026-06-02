import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ChildProfileIdSchema, ParentActorIdSchema } from './reference-primitives';
import { HouseholdCanonicalDeviceIdSchema } from './household-device-spine';
import { LanPairingSchemaVersionSchema, LanPairingTimestampSchema } from './lan-pairing-values';

const NonEmptyLanDeviceActionText = Schema.String.pipe(Schema.minLength(1));

export const LanHouseholdDeviceActionIdSchema = NonEmptyLanDeviceActionText.pipe(
  Schema.brand('LanHouseholdDeviceActionId')
);

export const LanHouseholdDeviceActionKindSchema = withParser(
  Schema.Literal('assign', 'rename', 'ignore', 'restore', 'trust')
);

export const LanHouseholdDeviceDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    actionId: LanHouseholdDeviceActionIdSchema,
    actionKind: LanHouseholdDeviceActionKindSchema,
    canonicalDeviceId: HouseholdCanonicalDeviceIdSchema,
    childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
    displayName: Schema.Union(NonEmptyLanDeviceActionText, Schema.Null),
    parentActorId: ParentActorIdSchema,
    decidedAt: LanPairingTimestampSchema,
    revokedAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export type LanHouseholdDeviceActionId = typeof LanHouseholdDeviceActionIdSchema.Type;
export type LanHouseholdDeviceActionKind = Infer<typeof LanHouseholdDeviceActionKindSchema>;
export type LanHouseholdDeviceDecision = Infer<typeof LanHouseholdDeviceDecisionSchema>;
