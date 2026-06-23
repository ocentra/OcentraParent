import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameWindowsLocalPolicyEvidenceProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-windows-local-policy-evidence-proof')
);

export const AppGameWindowsLocalPolicyEvidenceServiceStateSchema = withParser(
  Schema.Literal('appidsvc-running', 'appidsvc-stopped', 'appidsvc-unavailable')
);

export const AppGameWindowsLocalPolicyEvidencePolicyStateSchema = withParser(
  Schema.Literal('policy-readable', 'policy-empty', 'policy-unavailable')
);

export const AppGameWindowsLocalPolicyEvidenceAppControlStateSchema = withParser(
  Schema.Literal('app-control-present', 'app-control-not-present', 'app-control-unavailable')
);

export const AppGameWindowsLocalPolicyEvidenceProofRefSchema = withParser(
  Schema.Literal(
    'windows-applocker-service-state-ref',
    'windows-applocker-local-policy-state-ref',
    'windows-device-guard-policy-state-ref'
  )
);

export const AppGameWindowsLocalPolicyEvidenceGapSchema = withParser(
  Schema.Literal(
    'windows-applocker-service-not-running',
    'windows-applocker-enforce-policy-not-observed',
    'windows-app-control-enforcement-not-observed',
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-broad-blocking-adapter-dispatch-not-proved'
  )
);

const WindowsPolicyLabelSchema = brandedNonEmptyStringSchema('AppGameWindowsLocalPolicyEvidenceProofLabel');

const WindowsPolicyCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGameWindowsLocalPolicyEvidenceProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameWindowsLocalPolicyEvidenceProofSchemaVersionSchema,
  proofId: WindowsPolicyLabelSchema,
  serviceState: AppGameWindowsLocalPolicyEvidenceServiceStateSchema,
  appLockerPolicyState: AppGameWindowsLocalPolicyEvidencePolicyStateSchema,
  appControlPolicyState: AppGameWindowsLocalPolicyEvidenceAppControlStateSchema,
  appLockerRuleCount: WindowsPolicyCountSchema,
  appControlPolicyCount: WindowsPolicyCountSchema,
  policyReadable: Schema.Boolean,
  enforceModeObserved: Schema.Boolean,
  auditModeObserved: Schema.Boolean,
  appControlEnforcementObserved: Schema.Boolean,
  proofRefs: Schema.Array(AppGameWindowsLocalPolicyEvidenceProofRefSchema),
  openGaps: Schema.Array(AppGameWindowsLocalPolicyEvidenceGapSchema),
  rawExecutablePathsStored: Schema.Literal(false),
  rawPolicyXmlStored: Schema.Literal(false),
  rawPublisherRulesStored: Schema.Literal(false),
  broadBlockingClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: WindowsPolicyLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type WindowsPolicyCandidate = Infer<typeof AppGameWindowsLocalPolicyEvidenceProofBaseSchema>;

export const AppGameWindowsLocalPolicyEvidenceProofSchema = withParser(
  AppGameWindowsLocalPolicyEvidenceProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        windowsLocalPolicyEvidenceProofIsHonest(proof) ||
        'Expected Windows local policy evidence proof to keep raw policy data, broad blocking, adapter dispatch, enforcement, and child delivery unclaimed'
    )
  )
);

export type AppGameWindowsLocalPolicyEvidenceProof = Infer<typeof AppGameWindowsLocalPolicyEvidenceProofSchema>;

export const decodeAppGameWindowsLocalPolicyEvidenceProof = Schema.decodeUnknownSync(
  AppGameWindowsLocalPolicyEvidenceProofSchema
);

export function createAppGameWindowsLocalPolicyEvidenceProof(input: {
  readonly serviceState: AppGameWindowsLocalPolicyEvidenceProof['serviceState'];
  readonly appLockerPolicyState: AppGameWindowsLocalPolicyEvidenceProof['appLockerPolicyState'];
  readonly appControlPolicyState: AppGameWindowsLocalPolicyEvidenceProof['appControlPolicyState'];
  readonly appLockerRuleCount: AppGameWindowsLocalPolicyEvidenceProof['appLockerRuleCount'];
  readonly appControlPolicyCount: AppGameWindowsLocalPolicyEvidenceProof['appControlPolicyCount'];
  readonly policyReadable: AppGameWindowsLocalPolicyEvidenceProof['policyReadable'];
  readonly enforceModeObserved: AppGameWindowsLocalPolicyEvidenceProof['enforceModeObserved'];
  readonly auditModeObserved: AppGameWindowsLocalPolicyEvidenceProof['auditModeObserved'];
  readonly appControlEnforcementObserved: AppGameWindowsLocalPolicyEvidenceProof['appControlEnforcementObserved'];
  readonly checkedAt: AppGameWindowsLocalPolicyEvidenceProof['checkedAt'];
}): AppGameWindowsLocalPolicyEvidenceProof {
  return decodeAppGameWindowsLocalPolicyEvidenceProof({
    schemaVersion: 'app-game-windows-local-policy-evidence-proof',
    proofId: 'windows-local-policy-evidence-proof-ref',
    serviceState: input.serviceState,
    appLockerPolicyState: input.appLockerPolicyState,
    appControlPolicyState: input.appControlPolicyState,
    appLockerRuleCount: input.appLockerRuleCount,
    appControlPolicyCount: input.appControlPolicyCount,
    policyReadable: input.policyReadable,
    enforceModeObserved: input.enforceModeObserved,
    auditModeObserved: input.auditModeObserved,
    appControlEnforcementObserved: input.appControlEnforcementObserved,
    proofRefs: windowsLocalPolicyProofRefs(input),
    openGaps: windowsLocalPolicyOpenGaps(input),
    rawExecutablePathsStored: false,
    rawPolicyXmlStored: false,
    rawPublisherRulesStored: false,
    broadBlockingClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary: windowsLocalPolicySummary(input),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameWindowsLocalPolicyEvidenceProof(proof: AppGameWindowsLocalPolicyEvidenceProof) {
  return {
    serviceState: proof.serviceState,
    appLockerPolicyState: proof.appLockerPolicyState,
    appControlPolicyState: proof.appControlPolicyState,
    appLockerRuleCount: proof.appLockerRuleCount,
    appControlPolicyCount: proof.appControlPolicyCount,
    enforceModeObserved: proof.enforceModeObserved,
    appControlEnforcementObserved: proof.appControlEnforcementObserved,
    openGapCount: proof.openGaps.length,
  } as const;
}

function windowsLocalPolicyProofRefs(input: {
  readonly policyReadable: boolean;
  readonly appControlPolicyState: WindowsPolicyCandidate['appControlPolicyState'];
}) {
  const refs = ['windows-applocker-service-state-ref'];
  if (input.policyReadable) {
    refs.push('windows-applocker-local-policy-state-ref');
  }
  if (input.appControlPolicyState !== 'app-control-unavailable') {
    refs.push('windows-device-guard-policy-state-ref');
  }
  return refs;
}

function windowsLocalPolicyOpenGaps(input: {
  readonly serviceState: WindowsPolicyCandidate['serviceState'];
  readonly enforceModeObserved: boolean;
  readonly appControlEnforcementObserved: boolean;
}) {
  const gaps = [
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-broad-blocking-adapter-dispatch-not-proved',
  ];
  if (input.serviceState !== 'appidsvc-running') {
    gaps.unshift('windows-applocker-service-not-running');
  }
  if (!input.enforceModeObserved) {
    gaps.unshift('windows-applocker-enforce-policy-not-observed');
  }
  if (!input.appControlEnforcementObserved) {
    gaps.unshift('windows-app-control-enforcement-not-observed');
  }
  return gaps;
}

function windowsLocalPolicySummary(input: {
  readonly serviceState: WindowsPolicyCandidate['serviceState'];
  readonly enforceModeObserved: boolean;
  readonly appControlEnforcementObserved: boolean;
}) {
  if (input.serviceState === 'appidsvc-running' && (input.enforceModeObserved || input.appControlEnforcementObserved)) {
    return 'Windows local policy evidence is visible, but allowlist, rollback, audit custody, adapter dispatch, and child delivery proof are still required before broad app/game blocking can be claimed.';
  }

  return 'Windows local policy evidence is visible only as preflight; AppLocker/App Control enforcement, allowlist, rollback, audit custody, adapter dispatch, and child delivery remain unproved.';
}

function windowsLocalPolicyEvidenceProofIsHonest(proof: WindowsPolicyCandidate): boolean {
  return (
    proof.proofRefs.includes('windows-applocker-service-state-ref') &&
    proof.openGaps.includes('windows-system-app-allowlist-not-proved') &&
    proof.openGaps.includes('windows-rollback-not-proved') &&
    proof.openGaps.includes('windows-audit-custody-not-proved') &&
    proof.openGaps.includes('windows-broad-blocking-adapter-dispatch-not-proved') &&
    !proof.rawExecutablePathsStored &&
    !proof.rawPolicyXmlStored &&
    !proof.rawPublisherRulesStored &&
    !proof.broadBlockingClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.childDeviceDeliveryClaimed
  );
}
