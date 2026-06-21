import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  type BrowserSocialAccountFlowEvidence,
  BrowserSocialAccountFlowEvidenceIdSchema,
  BrowserSocialAccountFlowEvidenceSchema,
  type BrowserSocialAccountFlowKind,
} from './browser-social-account-flow-schemas';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
} from './browser-social-platform-route-schemas';
const SocialFormShapeSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social form-shape source evidence ids')
);

export const BrowserSocialFormShapeSchemaVersion = 1;

export const BrowserSocialFormShapeEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialFormShapeEvidenceId')
);

export const BrowserSocialFormShapeKindSchema = withParser(
  Schema.Literal('signup-form', 'login-form', 'account-switch-form', 'unknown-form')
);

export const BrowserSocialFormShapeDetectionStateSchema = withParser(
  Schema.Literal('detected', 'not-detected', 'manual-required')
);

export const BrowserSocialFormControlKindSchema = withParser(
  Schema.Literal(
    'email-input',
    'username-input',
    'password-input',
    'display-name-input',
    'birthdate-input',
    'phone-input',
    'submit-button',
    'account-switch-link',
    'unknown-control'
  )
);

const BrowserSocialFormControlHintSchema = Schema.Struct({
  controlKind: BrowserSocialFormControlKindSchema,
  valueCaptured: Schema.Boolean,
});

const BrowserSocialFormShapeDetectorInputBaseSchema = Schema.Struct({
  formShapeEvidenceId: BrowserSocialFormShapeEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFormShapeSourceEvidenceIdsSchema,
  accountFlowEvidence: BrowserSocialAccountFlowEvidenceSchema,
  controls: Schema.Array(BrowserSocialFormControlHintSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one sanitized form control hint')
  ),
});

const BrowserSocialFormShapeDetectorInputSchema = withParser(
  BrowserSocialFormShapeDetectorInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialFormShapeDetectorInputIsConsistent(value) ||
        'Expected route-only social account-flow evidence and sanitized form controls'
    )
  )
);

const BrowserSocialFormShapeEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialFormShapeSchemaVersion),
  formShapeEvidenceId: BrowserSocialFormShapeEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFormShapeSourceEvidenceIdsSchema,
  accountFlowEvidenceId: BrowserSocialAccountFlowEvidenceIdSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  accountFlowKind: Schema.Union(
    Schema.Literal('signup-route'),
    Schema.Literal('login-route'),
    Schema.Literal('account-switch-route')
  ),
  formShapeKind: BrowserSocialFormShapeKindSchema,
  detectionState: BrowserSocialFormShapeDetectionStateSchema,
  matchedControlKinds: Schema.Array(BrowserSocialFormControlKindSchema),
  missingControlKinds: Schema.Array(BrowserSocialFormControlKindSchema),
  manualRequired: Schema.Boolean,
  rawDomCaptured: Schema.Boolean,
  fieldValuesCaptured: Schema.Boolean,
  credentialCaptured: Schema.Boolean,
  formSubmissionClaimed: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  parentApprovalDecisionClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});

export const BrowserSocialFormShapeEvidenceSchema = withParser(
  BrowserSocialFormShapeEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialFormShapeEvidenceIsConsistent(value) ||
        'Expected social form-shape evidence to preserve sanitized detector boundaries'
    )
  )
);

export const decodeBrowserSocialFormShapeEvidence = Schema.decodeUnknownSync(BrowserSocialFormShapeEvidenceSchema);

export type BrowserSocialFormControlHint = Infer<typeof BrowserSocialFormControlHintSchema>;
export type BrowserSocialFormControlKind = Infer<typeof BrowserSocialFormControlKindSchema>;
export type BrowserSocialFormShapeDetectorInput = Infer<typeof BrowserSocialFormShapeDetectorInputSchema>;
export type BrowserSocialFormShapeDetectionState = Infer<typeof BrowserSocialFormShapeDetectionStateSchema>;
export type BrowserSocialFormShapeEvidence = Infer<typeof BrowserSocialFormShapeEvidenceSchema>;
export type BrowserSocialFormShapeEvidenceId = Infer<typeof BrowserSocialFormShapeEvidenceIdSchema>;
export type BrowserSocialFormShapeKind = Infer<typeof BrowserSocialFormShapeKindSchema>;

export function detectBrowserSocialFormShape(
  input: BrowserSocialFormShapeDetectorInput
): BrowserSocialFormShapeEvidence {
  const parsed = BrowserSocialFormShapeDetectorInputSchema.parse(input);
  const matchedControlKinds = uniqueControlKinds(parsed.controls.map((control) => control.controlKind));
  const formShapeKind = formShapeKindForAccountFlow(parsed.accountFlowEvidence.accountFlowKind);
  const missingControlKinds = missingRequiredControls(formShapeKind, matchedControlKinds);

  return BrowserSocialFormShapeEvidenceSchema.parse({
    schemaVersion: BrowserSocialFormShapeSchemaVersion,
    formShapeEvidenceId: parsed.formShapeEvidenceId,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    accountFlowEvidenceId: parsed.accountFlowEvidence.accountFlowEvidenceId,
    socialRouteEvidenceId: parsed.accountFlowEvidence.socialRouteEvidenceId,
    platform: parsed.accountFlowEvidence.platform,
    accountFlowKind: parsed.accountFlowEvidence.accountFlowKind,
    formShapeKind,
    detectionState: 'detected',
    matchedControlKinds,
    missingControlKinds,
    manualRequired: false,
    rawDomCaptured: false,
    fieldValuesCaptured: false,
    credentialCaptured: false,
    formSubmissionClaimed: false,
    accountIdentityClaimed: false,
    parentApprovalDecisionClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function socialFormShapeDetectorInputIsConsistent(value: Infer<typeof BrowserSocialFormShapeDetectorInputBaseSchema>) {
  return (
    accountFlowEvidenceCanDetectForm(value.accountFlowEvidence) &&
    value.controls.every((control) => !control.valueCaptured) &&
    missingRequiredControls(
      formShapeKindForAccountFlow(value.accountFlowEvidence.accountFlowKind),
      uniqueControlKinds(value.controls.map((control) => control.controlKind))
    ).length === 0
  );
}

function browserSocialFormShapeEvidenceIsConsistent(value: Infer<typeof BrowserSocialFormShapeEvidenceBaseSchema>) {
  if (socialFormShapeEvidenceClaimsAuthority(value)) {
    return false;
  }
  if (value.detectionState !== 'detected') {
    return value.formShapeKind === 'unknown-form' && value.manualRequired;
  }
  return (
    !value.manualRequired &&
    value.formShapeKind === formShapeKindForAccountFlow(value.accountFlowKind) &&
    value.missingControlKinds.length === 0 &&
    value.matchedControlKinds.length > 0 &&
    !value.matchedControlKinds.includes('unknown-control')
  );
}

function socialFormShapeEvidenceClaimsAuthority(value: Infer<typeof BrowserSocialFormShapeEvidenceBaseSchema>) {
  return (
    value.rawDomCaptured ||
    value.fieldValuesCaptured ||
    value.credentialCaptured ||
    value.formSubmissionClaimed ||
    value.accountIdentityClaimed ||
    value.parentApprovalDecisionClaimed ||
    value.aiDecisionClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}

function accountFlowEvidenceCanDetectForm(value: BrowserSocialAccountFlowEvidence) {
  return value.evidenceState === 'route-only' && value.exactManagedBrowserRouteEvidence && !value.manualRequired;
}

function formShapeKindForAccountFlow(value: BrowserSocialAccountFlowKind) {
  if (value === 'signup-route') {
    return 'signup-form' as const;
  }
  if (value === 'login-route') {
    return 'login-form' as const;
  }
  return 'account-switch-form' as const;
}

function missingRequiredControls(
  formShapeKind: BrowserSocialFormShapeKind,
  controls: readonly BrowserSocialFormControlKind[]
) {
  if (formShapeKind === 'account-switch-form') {
    return controls.includes('account-switch-link') ? [] : missingControls(controls, ['submit-button']);
  }
  return missingControls(controls, ['password-input', 'submit-button', 'email-input']);
}

function missingControls(
  controls: readonly BrowserSocialFormControlKind[],
  required: readonly BrowserSocialFormControlKind[]
) {
  return required.filter((control) => !controls.includes(control));
}

function uniqueControlKinds(controls: readonly BrowserSocialFormControlKind[]) {
  return [...new Set(controls)].filter((control) => control !== 'unknown-control');
}

