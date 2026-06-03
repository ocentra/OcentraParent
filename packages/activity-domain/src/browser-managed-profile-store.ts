import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserDegradedReasonSchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamilySchema,
  BrowserProfileIdSchema,
  BrowserProfilePathRefSchema,
} from './browser-schemas';

const NonEmptyProfileStoreText = Schema.String.pipe(Schema.minLength(1));
const RedactedProfileRefText = NonEmptyProfileStoreText.pipe(
  Schema.filter((value) => profileRefIsRedacted(value) || 'Expected a redacted managed profile ref')
);

export const BrowserManagedProfileLifecycleStateSchema = withParser(
  Schema.Literal(
    'ready',
    'missing',
    'repair-required',
    'deleted',
    'unsafe-default-profile',
    'unowned-profile',
    'unavailable'
  )
);

export const BrowserProfileRootRefSchema = withParser(
  RedactedProfileRefText.pipe(Schema.brand('BrowserProfileRootRef'))
);
export const BrowserProfileScopeIdSchema = withParser(
  NonEmptyProfileStoreText.pipe(Schema.brand('BrowserProfileScopeId'))
);
export const BrowserPolicyRevisionSchema = withParser(
  NonEmptyProfileStoreText.pipe(Schema.brand('BrowserPolicyRevision'))
);

const BrowserManagedProfileStoreEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  profileId: BrowserProfileIdSchema,
  profilePathRef: BrowserProfilePathRefSchema,
  profileRootRef: BrowserProfileRootRefSchema,
  profileScopeId: BrowserProfileScopeIdSchema,
  deviceId: ActivityDeviceIdSchema,
  browserFamily: BrowserFamilySchema,
  browserChannel: BrowserChannelSchema,
  lifecycleState: BrowserManagedProfileLifecycleStateSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  policyRevision: BrowserPolicyRevisionSchema,
  createdAt: ActivityTimestampSchema,
  updatedAt: ActivityTimestampSchema,
  missingSince: Schema.Union(ActivityTimestampSchema, Schema.Null),
  repairedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  deletedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  repairReason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
});

export const BrowserManagedProfileStoreEntrySchema = withParser(
  BrowserManagedProfileStoreEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserManagedProfileStoreEntryIsConsistent(entry) || 'Inconsistent managed profile store lifecycle state'
    )
  )
);

export type BrowserManagedProfileLifecycleState = Infer<typeof BrowserManagedProfileLifecycleStateSchema>;
export type BrowserProfileRootRef = Infer<typeof BrowserProfileRootRefSchema>;
export type BrowserProfileScopeId = Infer<typeof BrowserProfileScopeIdSchema>;
export type BrowserPolicyRevision = Infer<typeof BrowserPolicyRevisionSchema>;
export type BrowserManagedProfileStoreEntry = Infer<typeof BrowserManagedProfileStoreEntrySchema>;

export const BrowserManagedProfileLifecycleState = {
  Ready: BrowserManagedProfileLifecycleStateSchema.parse('ready'),
  Missing: BrowserManagedProfileLifecycleStateSchema.parse('missing'),
  RepairRequired: BrowserManagedProfileLifecycleStateSchema.parse('repair-required'),
  Deleted: BrowserManagedProfileLifecycleStateSchema.parse('deleted'),
  UnsafeDefaultProfile: BrowserManagedProfileLifecycleStateSchema.parse('unsafe-default-profile'),
  UnownedProfile: BrowserManagedProfileLifecycleStateSchema.parse('unowned-profile'),
  Unavailable: BrowserManagedProfileLifecycleStateSchema.parse('unavailable'),
} as const;

function browserManagedProfileStoreEntryIsConsistent(
  entry: Infer<typeof BrowserManagedProfileStoreEntryBaseSchema>
): boolean {
  if (entry.lifecycleState === 'ready') {
    return entry.missingSince === null && entry.deletedAt === null;
  }

  if (entry.lifecycleState === 'missing') {
    return entry.missingSince !== null && entry.deletedAt === null && entry.repairReason !== null;
  }

  if (entry.lifecycleState === 'repair-required') {
    return entry.repairReason !== null && entry.deletedAt === null;
  }

  if (entry.lifecycleState === 'deleted') {
    return entry.deletedAt !== null && entry.repairReason !== null;
  }

  if (entry.lifecycleState === 'unsafe-default-profile' || entry.lifecycleState === 'unowned-profile') {
    return entry.repairReason !== null;
  }

  return entry.lifecycleState === 'unavailable' && entry.repairReason !== null;
}

function profileRefIsRedacted(value: string): boolean {
  return !value.includes('/') && !value.includes('\\') && !value.includes(':');
}
