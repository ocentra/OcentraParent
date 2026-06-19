import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ChildProfileIdSchema, ParentActorIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { HouseholdCanonicalDeviceIdSchema } from './household-device-spine';
import { LanPairingSchemaVersionSchema, LanPairingTimestampSchema } from './lan-pairing-values';

export const LAN_HOUSEHOLD_DEVICE_KIND_VALUES = ['mobile', 'desktop', 'laptop', 'tablet', 'router', 'unknown'] as const;
export const LAN_HOUSEHOLD_ACTION_DEVICE_KIND_FIELD = 'deviceKind';

export const LanHouseholdDeviceActionIdSchema = brandedNonEmptyStringSchema('LanHouseholdDeviceActionId');

export const LanHouseholdDeviceActionKindSchema = withParser(
  Schema.Literal('assign', 'rename', 'ignore', 'restore', 'trust')
);

export const LanHouseholdDeviceKindSchema = withParser(Schema.Literal(...LAN_HOUSEHOLD_DEVICE_KIND_VALUES));

export const LanHouseholdDeviceDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    actionId: LanHouseholdDeviceActionIdSchema,
    actionKind: LanHouseholdDeviceActionKindSchema,
    canonicalDeviceId: HouseholdCanonicalDeviceIdSchema,
    childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
    displayName: Schema.Union(NonEmptyStringSchema, Schema.Null),
    deviceKind: Schema.optionalWith(Schema.Union(LanHouseholdDeviceKindSchema, Schema.Null), { default: () => null }),
    parentActorId: ParentActorIdSchema,
    decidedAt: LanPairingTimestampSchema,
    revokedAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export type LanHouseholdDeviceActionId = typeof LanHouseholdDeviceActionIdSchema.Type;
export type LanHouseholdDeviceActionKind = Infer<typeof LanHouseholdDeviceActionKindSchema>;
export type LanHouseholdDeviceKind = Infer<typeof LanHouseholdDeviceKindSchema>;
export type LanHouseholdDeviceDecision = Infer<typeof LanHouseholdDeviceDecisionSchema>;

