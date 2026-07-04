import { HouseholdAuthorityInputSchema, SessionFreshnessStateSchema } from './family-household-authority';
import { ParentContractSchemaVersionSchema } from './family-reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import { RecoveryOperationSchema } from './family-restore-lifecycle';
import { SetupInviteSchema } from './family-setup-invite';
import { type Infer, Schema, withParser } from './effect';
import { SetupPairingIntentIdSchema } from './setup-pairing-intent';

export const RegistrationEntryRouteLiteral = {
  Register: 'register',
  Login: 'login',
  Logout: 'logout',
  InviteAccept: 'invite-accept',
  ResumeSetup: 'resume-setup',
  Recovery: 'recovery',
} as const;

export const RegistrationRecoveryMethodLiteral = {
  Password: 'password',
  Passkey: 'passkey',
  EmailLink: 'email-link',
} as const;

export const RegistrationIdentityProviderStateLiteral = {
  Available: 'available',
  ProviderUnavailable: 'provider-unavailable',
} as const;

export const RegistrationSetupStateLiteral = {
  Unauthenticated: 'unauthenticated',
  AuthenticatedNoHousehold: 'authenticated-no-household',
  HouseholdNoChild: 'household-no-child',
  HouseholdChildNoDevice: 'household-child-no-device',
  Paired: 'paired',
  Degraded: 'degraded',
} as const;

export const RegistrationEntryRejectionReasonLiteral = {
  ExpiredInvite: 'expired-invite',
  RevokedInvite: 'revoked-invite',
  CrossFamily: 'cross-family',
  WrongRole: 'wrong-role',
} as const;

export const RegistrationEntryFailureStateLiteral = {
  SessionExpired: 'session-expired',
  ProviderUnavailable: 'provider-unavailable',
} as const;

export const RegistrationHandoffFieldLiteral = {
  ProviderState: 'provider-state',
  SessionFreshness: 'session-freshness',
  ParentAccount: 'parent-account',
  Family: 'family',
  ChildProfile: 'child-profile',
  ChildDevice: 'child-device',
  SetupInvite: 'setup-invite',
  RecoveryMethod: 'recovery-method',
  RecoveryOperation: 'recovery-operation',
  PairingIntentId: 'pairing-intent-id',
} as const;

export const RegistrationForbiddenCollectionLiteral = {
  ChildActivityData: 'child-activity-data',
  ChildEvidenceArtifacts: 'child-evidence-artifacts',
  ChildContentPayloads: 'child-content-payloads',
} as const;

export const RegistrationEntryRouteSchema = withParser(
  Schema.Literal(
    RegistrationEntryRouteLiteral.Register,
    RegistrationEntryRouteLiteral.Login,
    RegistrationEntryRouteLiteral.Logout,
    RegistrationEntryRouteLiteral.InviteAccept,
    RegistrationEntryRouteLiteral.ResumeSetup,
    RegistrationEntryRouteLiteral.Recovery
  )
);

export const RegistrationRecoveryMethodSchema = withParser(
  Schema.Literal(
    RegistrationRecoveryMethodLiteral.Password,
    RegistrationRecoveryMethodLiteral.Passkey,
    RegistrationRecoveryMethodLiteral.EmailLink
  )
);

export const RegistrationIdentityProviderStateSchema = withParser(
  Schema.Literal(
    RegistrationIdentityProviderStateLiteral.Available,
    RegistrationIdentityProviderStateLiteral.ProviderUnavailable
  )
);

export const RegistrationSetupStateSchema = withParser(
  Schema.Literal(
    RegistrationSetupStateLiteral.Unauthenticated,
    RegistrationSetupStateLiteral.AuthenticatedNoHousehold,
    RegistrationSetupStateLiteral.HouseholdNoChild,
    RegistrationSetupStateLiteral.HouseholdChildNoDevice,
    RegistrationSetupStateLiteral.Paired,
    RegistrationSetupStateLiteral.Degraded
  )
);

export const RegistrationEntryRejectionReasonSchema = withParser(
  Schema.Literal(
    RegistrationEntryRejectionReasonLiteral.ExpiredInvite,
    RegistrationEntryRejectionReasonLiteral.RevokedInvite,
    RegistrationEntryRejectionReasonLiteral.CrossFamily,
    RegistrationEntryRejectionReasonLiteral.WrongRole
  )
);

export const RegistrationEntryFailureStateSchema = withParser(
  Schema.Literal(
    RegistrationEntryFailureStateLiteral.SessionExpired,
    RegistrationEntryFailureStateLiteral.ProviderUnavailable
  )
);

export const RegistrationHandoffFieldSchema = withParser(
  Schema.Literal(
    RegistrationHandoffFieldLiteral.ProviderState,
    RegistrationHandoffFieldLiteral.SessionFreshness,
    RegistrationHandoffFieldLiteral.ParentAccount,
    RegistrationHandoffFieldLiteral.Family,
    RegistrationHandoffFieldLiteral.ChildProfile,
    RegistrationHandoffFieldLiteral.ChildDevice,
    RegistrationHandoffFieldLiteral.SetupInvite,
    RegistrationHandoffFieldLiteral.RecoveryMethod,
    RegistrationHandoffFieldLiteral.RecoveryOperation,
    RegistrationHandoffFieldLiteral.PairingIntentId
  )
);

export const RegistrationForbiddenCollectionSchema = withParser(
  Schema.Literal(
    RegistrationForbiddenCollectionLiteral.ChildActivityData,
    RegistrationForbiddenCollectionLiteral.ChildEvidenceArtifacts,
    RegistrationForbiddenCollectionLiteral.ChildContentPayloads
  )
);

export const RegistrationEntryRouteContractSchema = withParser(
  Schema.Struct({
    routeId: RegistrationEntryRouteSchema,
    requiresAuthenticatedParent: Schema.Boolean,
    requiresSetupInvite: Schema.Boolean,
    requiresRecoveryMethod: Schema.Boolean,
    recoveryMethods: Schema.Array(RegistrationRecoveryMethodSchema),
  })
);

export const RegistrationStateMatrixRowSchema = withParser(
  Schema.Struct({
    setupState: RegistrationSetupStateSchema,
    allowedRoutes: Schema.Array(RegistrationEntryRouteSchema),
    allowedHandoffFields: Schema.Array(RegistrationHandoffFieldSchema),
    forbiddenCollections: Schema.Array(RegistrationForbiddenCollectionSchema),
  })
);

export const RegistrationIdentityHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    routeId: RegistrationEntryRouteSchema,
    recoveryMethod: Schema.Union(RegistrationRecoveryMethodSchema, Schema.Null),
    providerState: Schema.Union(RegistrationIdentityProviderStateSchema, Schema.Null),
    sessionFreshnessState: Schema.Union(SessionFreshnessStateSchema, Schema.Null),
    parentAccount: Schema.Union(ParentAccountReferenceSchema, Schema.Null),
    family: Schema.Union(FamilyReferenceSchema, Schema.Null),
    childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    childDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    setupInvite: Schema.Union(SetupInviteSchema, Schema.Null),
    recoveryOperation: Schema.Union(RecoveryOperationSchema, Schema.Null),
    pairingIntentId: Schema.Union(SetupPairingIntentIdSchema, Schema.Null),
    householdAuthorityInput: Schema.Union(HouseholdAuthorityInputSchema, Schema.Null),
  })
);

export const RegistrationEntryDecisionSchema = withParser(
  Schema.Struct({
    setupState: RegistrationSetupStateSchema,
    allowedRoutes: Schema.Array(RegistrationEntryRouteSchema),
    allowedHandoffFields: Schema.Array(RegistrationHandoffFieldSchema),
    forbiddenCollections: Schema.Array(RegistrationForbiddenCollectionSchema),
    rejectedReason: Schema.Union(RegistrationEntryRejectionReasonSchema, Schema.Null),
    failureState: Schema.Union(RegistrationEntryFailureStateSchema, Schema.Null),
  })
);

export type RegistrationEntryRoute = Infer<typeof RegistrationEntryRouteSchema>;
export type RegistrationRecoveryMethod = Infer<typeof RegistrationRecoveryMethodSchema>;
export type RegistrationIdentityProviderState = Infer<typeof RegistrationIdentityProviderStateSchema>;
export type RegistrationSetupState = Infer<typeof RegistrationSetupStateSchema>;
export type RegistrationEntryRejectionReason = Infer<typeof RegistrationEntryRejectionReasonSchema>;
export type RegistrationEntryFailureState = Infer<typeof RegistrationEntryFailureStateSchema>;
export type RegistrationHandoffField = Infer<typeof RegistrationHandoffFieldSchema>;
export type RegistrationForbiddenCollection = Infer<typeof RegistrationForbiddenCollectionSchema>;
export type RegistrationEntryRouteContract = Infer<typeof RegistrationEntryRouteContractSchema>;
export type RegistrationStateMatrixRow = Infer<typeof RegistrationStateMatrixRowSchema>;
export type RegistrationIdentityHandoff = Infer<typeof RegistrationIdentityHandoffSchema>;
export type RegistrationEntryDecision = Infer<typeof RegistrationEntryDecisionSchema>;

export const RegistrationEntryRoute = {
  Register: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.Register),
  Login: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.Login),
  Logout: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.Logout),
  InviteAccept: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.InviteAccept),
  ResumeSetup: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.ResumeSetup),
  Recovery: RegistrationEntryRouteSchema.parse(RegistrationEntryRouteLiteral.Recovery),
} as const;

export const RegistrationRecoveryMethod = {
  Password: RegistrationRecoveryMethodSchema.parse(RegistrationRecoveryMethodLiteral.Password),
  Passkey: RegistrationRecoveryMethodSchema.parse(RegistrationRecoveryMethodLiteral.Passkey),
  EmailLink: RegistrationRecoveryMethodSchema.parse(RegistrationRecoveryMethodLiteral.EmailLink),
} as const;

export const RegistrationIdentityProviderState = {
  Available: RegistrationIdentityProviderStateSchema.parse(RegistrationIdentityProviderStateLiteral.Available),
  ProviderUnavailable: RegistrationIdentityProviderStateSchema.parse(
    RegistrationIdentityProviderStateLiteral.ProviderUnavailable
  ),
} as const;

export const RegistrationSetupState = {
  Unauthenticated: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.Unauthenticated),
  AuthenticatedNoHousehold: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.AuthenticatedNoHousehold),
  HouseholdNoChild: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.HouseholdNoChild),
  HouseholdChildNoDevice: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.HouseholdChildNoDevice),
  Paired: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.Paired),
  Degraded: RegistrationSetupStateSchema.parse(RegistrationSetupStateLiteral.Degraded),
} as const;

export const RegistrationEntryRejectionReason = {
  ExpiredInvite: RegistrationEntryRejectionReasonSchema.parse(RegistrationEntryRejectionReasonLiteral.ExpiredInvite),
  RevokedInvite: RegistrationEntryRejectionReasonSchema.parse(RegistrationEntryRejectionReasonLiteral.RevokedInvite),
  CrossFamily: RegistrationEntryRejectionReasonSchema.parse(RegistrationEntryRejectionReasonLiteral.CrossFamily),
  WrongRole: RegistrationEntryRejectionReasonSchema.parse(RegistrationEntryRejectionReasonLiteral.WrongRole),
} as const;

export const RegistrationEntryFailureState = {
  SessionExpired: RegistrationEntryFailureStateSchema.parse(RegistrationEntryFailureStateLiteral.SessionExpired),
  ProviderUnavailable: RegistrationEntryFailureStateSchema.parse(
    RegistrationEntryFailureStateLiteral.ProviderUnavailable
  ),
} as const;

const SharedForbiddenCollections = RegistrationStateMatrixRowSchema.parse({
  setupState: RegistrationSetupState.Unauthenticated,
  allowedRoutes: [],
  allowedHandoffFields: [],
  forbiddenCollections: [
    RegistrationForbiddenCollectionLiteral.ChildActivityData,
    RegistrationForbiddenCollectionLiteral.ChildEvidenceArtifacts,
    RegistrationForbiddenCollectionLiteral.ChildContentPayloads,
  ],
}).forbiddenCollections;

function registrationEntryRouteContractShape(
  routeId: RegistrationEntryRoute,
  requiresAuthenticatedParent: boolean,
  requiresSetupInvite: boolean,
  requiresRecoveryMethod: boolean,
  recoveryMethods: readonly RegistrationRecoveryMethod[]
): RegistrationEntryRouteContract {
  return RegistrationEntryRouteContractSchema.parse({
    routeId,
    requiresAuthenticatedParent,
    requiresSetupInvite,
    requiresRecoveryMethod,
    recoveryMethods,
  });
}

function registrationStateMatrixRowShape(
  setupState: RegistrationSetupState,
  allowedRoutes: readonly RegistrationEntryRoute[],
  allowedHandoffFields: readonly RegistrationHandoffField[]
): RegistrationStateMatrixRow {
  return RegistrationStateMatrixRowSchema.parse({
    setupState,
    allowedRoutes,
    allowedHandoffFields,
    forbiddenCollections: SharedForbiddenCollections,
  });
}

export const RegistrationEntryRouteContracts = [
  registrationEntryRouteContractShape(RegistrationEntryRoute.Register, false, false, false, []),
  registrationEntryRouteContractShape(RegistrationEntryRoute.Login, false, false, false, []),
  registrationEntryRouteContractShape(RegistrationEntryRoute.Logout, true, false, false, []),
  registrationEntryRouteContractShape(RegistrationEntryRoute.InviteAccept, false, true, false, []),
  registrationEntryRouteContractShape(RegistrationEntryRoute.ResumeSetup, true, false, false, []),
  registrationEntryRouteContractShape(RegistrationEntryRoute.Recovery, false, false, true, [
    RegistrationRecoveryMethod.Password,
    RegistrationRecoveryMethod.Passkey,
    RegistrationRecoveryMethod.EmailLink,
  ]),
] as const;

export const RegistrationStateMatrix = [
  registrationStateMatrixRowShape(
    RegistrationSetupState.Unauthenticated,
    [
      RegistrationEntryRoute.Register,
      RegistrationEntryRoute.Login,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.Recovery,
    ],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SetupInvite,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
    ]
  ),
  registrationStateMatrixRowShape(
    RegistrationSetupState.AuthenticatedNoHousehold,
    [
      RegistrationEntryRoute.Logout,
      RegistrationEntryRoute.ResumeSetup,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.Recovery,
    ],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SessionFreshness,
      RegistrationHandoffFieldLiteral.ParentAccount,
      RegistrationHandoffFieldLiteral.SetupInvite,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
    ]
  ),
  registrationStateMatrixRowShape(
    RegistrationSetupState.HouseholdNoChild,
    [
      RegistrationEntryRoute.Logout,
      RegistrationEntryRoute.ResumeSetup,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.Recovery,
    ],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SessionFreshness,
      RegistrationHandoffFieldLiteral.ParentAccount,
      RegistrationHandoffFieldLiteral.Family,
      RegistrationHandoffFieldLiteral.SetupInvite,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
    ]
  ),
  registrationStateMatrixRowShape(
    RegistrationSetupState.HouseholdChildNoDevice,
    [
      RegistrationEntryRoute.Logout,
      RegistrationEntryRoute.ResumeSetup,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.Recovery,
    ],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SessionFreshness,
      RegistrationHandoffFieldLiteral.ParentAccount,
      RegistrationHandoffFieldLiteral.Family,
      RegistrationHandoffFieldLiteral.ChildProfile,
      RegistrationHandoffFieldLiteral.SetupInvite,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
      RegistrationHandoffFieldLiteral.PairingIntentId,
    ]
  ),
  registrationStateMatrixRowShape(
    RegistrationSetupState.Paired,
    [RegistrationEntryRoute.Logout, RegistrationEntryRoute.ResumeSetup, RegistrationEntryRoute.Recovery],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SessionFreshness,
      RegistrationHandoffFieldLiteral.ParentAccount,
      RegistrationHandoffFieldLiteral.Family,
      RegistrationHandoffFieldLiteral.ChildProfile,
      RegistrationHandoffFieldLiteral.ChildDevice,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
      RegistrationHandoffFieldLiteral.PairingIntentId,
    ]
  ),
  registrationStateMatrixRowShape(
    RegistrationSetupState.Degraded,
    [
      RegistrationEntryRoute.Login,
      RegistrationEntryRoute.Logout,
      RegistrationEntryRoute.InviteAccept,
      RegistrationEntryRoute.ResumeSetup,
      RegistrationEntryRoute.Recovery,
    ],
    [
      RegistrationHandoffFieldLiteral.ProviderState,
      RegistrationHandoffFieldLiteral.SessionFreshness,
      RegistrationHandoffFieldLiteral.ParentAccount,
      RegistrationHandoffFieldLiteral.Family,
      RegistrationHandoffFieldLiteral.ChildProfile,
      RegistrationHandoffFieldLiteral.ChildDevice,
      RegistrationHandoffFieldLiteral.SetupInvite,
      RegistrationHandoffFieldLiteral.RecoveryMethod,
      RegistrationHandoffFieldLiteral.RecoveryOperation,
      RegistrationHandoffFieldLiteral.PairingIntentId,
    ]
  ),
] as const;
