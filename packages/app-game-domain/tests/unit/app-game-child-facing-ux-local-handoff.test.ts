import { describe, expect, it } from 'vitest';
import { AppGameChildUxCardSchema } from '../../src/app-game-child-facing-ux';
import {
  AppGameChildUxCapabilityState,
  AppGameChildUxClaimState,
  AppGameChildUxCopyToken,
  AppGameChildUxExplanationSource,
  AppGameChildUxPrimaryAction,
  AppGameChildUxSurfaceState,
  AppGameChildUxTargetKind,
} from '../../src/app-game-child-facing-ux-rules';
import { buildAppGameChildUxHandoffReadModel } from '../../src/app-game-child-facing-ux-handoff';
import {
  AppGameChildUxLocalHandoffArtifactReadModelSchema,
  AppGameChildUxLocalHandoffArtifactRecordSchema,
  buildAppGameChildUxLocalHandoffArtifactReadModel,
  parseAppGameChildUxLocalHandoffJsonl,
  serializeAppGameChildUxLocalHandoffJsonl,
} from '../../src/app-game-child-facing-ux-local-handoff';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/family-domain/reference-primitives';

const Timestamp = '2026-06-07T20:13:00Z';

const Device = {
  deviceId: 'device-child-ux-local-handoff',
  childProfileId: 'child-profile-child-ux-local-handoff',
  label: 'Study PC',
  platform: ParentPlatform.Windows,
} as const;

const EvidenceReference = {
  evidenceReferenceId: 'evidence-child-ux-local-handoff',
  kind: ParentEvidenceReferenceKind.PolicyDecision,
  observedAt: Timestamp,
} as const;

const ApprovalRequestRef = {
  actionReferenceId: 'approval-request-child-ux-local-handoff',
  actor: {
    actorId: 'child-device-local-agent',
    role: ParentActorRole.System,
  },
  policyVersion: 'policy-child-ux-local-handoff-v1',
  createdAt: Timestamp,
} as const;

const BaseChildCard = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  childUxStateId: 'child-ux-limit-reached-local-handoff',
  device: Device,
  target: {
    targetKind: AppGameChildUxTargetKind.NativeGame,
    targetRef: 'target-native-game-child-ux-local-handoff',
    childSafeDisplayLabelToken: AppGameChildUxCopyToken.LimitReachedTitle,
  },
  surfaceState: AppGameChildUxSurfaceState.TimeLimitReached,
  capabilityState: AppGameChildUxCapabilityState.Supported,
  claimState: AppGameChildUxClaimState.LimitReached,
  explanationSource: AppGameChildUxExplanationSource.ParentRule,
  titleToken: AppGameChildUxCopyToken.LimitReachedTitle,
  bodyToken: AppGameChildUxCopyToken.LimitReachedBody,
  primaryAction: AppGameChildUxPrimaryAction.RequestMoreTime,
  primaryActionToken: AppGameChildUxCopyToken.RequestMoreTimeAction,
  evidenceReferences: [EvidenceReference],
  childReasonReferences: ['child-reason-limit-reached-local-handoff'],
  childStatusReferences: ['child-status-limit-reached-local-handoff'],
  approvalRequestRef: ApprovalRequestRef,
  privateDiagnosticReferences: [],
  adapterActionRef: null,
} as const;

const HandoffOptions = {
  generatedAt: Timestamp,
  handoffId: 'app-game-child-ux-local-handoff-readiness',
  localHandoffRootRef: 'child-device-local-ux-handoff-root',
} as const;

const ArtifactOptions = {
  generatedAt: Timestamp,
  localArtifactRootRef: 'child-device-local-ux-artifact-root',
  localArtifactFileRef: 'child-device-local-ux-artifact-jsonl',
} as const;

describe('app/game child-facing UX local handoff artifact', () => {
  it('writes and rereads local child UX artifact rows only for ready app/game handoff rows', () => {
    const handoff = buildAppGameChildUxHandoffReadModel(HandoffOptions, [
      AppGameChildUxCardSchema.parse(BaseChildCard),
      AppGameChildUxCardSchema.parse(nativeAppSubmittedCard()),
    ]);
    const artifact = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
    const jsonl = serializeAppGameChildUxLocalHandoffJsonl(artifact);
    const records = parseAppGameChildUxLocalHandoffJsonl(jsonl);

    expect(artifact.writtenRecordCount).toBe(2);
    expect(artifact.skippedBlockedRowCount).toBe(0);
    expect(records.map((record) => record.recordId)).toEqual([
      'app-game-child-ux-local-handoff-child-ux-limit-reached-local-handoff',
      'app-game-child-ux-local-handoff-child-ux-native-app-request-submitted',
    ]);
    expect(records.map((record) => record.card.target.targetKind)).toEqual([
      AppGameChildUxTargetKind.NativeGame,
      AppGameChildUxTargetKind.NativeApp,
    ]);
    expect(records.map((record) => record.childDeliveryRuntimeClaimed)).toEqual([false, false]);
    expect(records.map((record) => record.privateDiagnosticsIncluded)).toEqual([false, false]);
  });

  it('keeps blocked missing-ref rows out of the local child UX JSONL artifact', () => {
    const handoff = buildAppGameChildUxHandoffReadModel(HandoffOptions, [
      AppGameChildUxCardSchema.parse(BaseChildCard),
      AppGameChildUxCardSchema.parse(blockedWarningCard()),
    ]);
    const artifact = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
    const records = parseAppGameChildUxLocalHandoffJsonl(serializeAppGameChildUxLocalHandoffJsonl(artifact));

    expect(artifact.writtenRecordCount).toBe(1);
    expect(artifact.skippedBlockedRowCount).toBe(1);
    expect(records.map((record) => record.card.childUxStateId)).toEqual(['child-ux-limit-reached-local-handoff']);
  });

  it('rejects delivery adapter platform and diagnostics overclaims at the artifact boundary', () => {
    const handoff = buildAppGameChildUxHandoffReadModel(HandoffOptions, [
      AppGameChildUxCardSchema.parse(BaseChildCard),
    ]);
    const artifact = buildAppGameChildUxLocalHandoffArtifactReadModel(ArtifactOptions, handoff);
    const record = artifact.records[0];

    expect(
      AppGameChildUxLocalHandoffArtifactReadModelSchema.safeParse({
        ...artifact,
        childDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameChildUxLocalHandoffArtifactRecordSchema.safeParse({
        ...record,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(() =>
      parseAppGameChildUxLocalHandoffJsonl(`${JSON.stringify({ ...record, privateDiagnosticsIncluded: true })}\n`)
    ).toThrow();
  });
});

function nativeAppSubmittedCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-native-app-request-submitted',
    target: {
      targetKind: AppGameChildUxTargetKind.NativeApp,
      targetRef: 'target-native-app-child-ux-local-handoff',
      childSafeDisplayLabelToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    },
    surfaceState: AppGameChildUxSurfaceState.RequestSubmitted,
    claimState: AppGameChildUxClaimState.RequestSubmitted,
    titleToken: AppGameChildUxCopyToken.RequestSubmittedTitle,
    bodyToken: AppGameChildUxCopyToken.RequestSubmittedBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: ['child-reason-request-submitted-local-handoff'],
    childStatusReferences: ['child-status-request-submitted-local-handoff'],
    approvalRequestRef: null,
  } as const;
}

function blockedWarningCard() {
  return {
    ...BaseChildCard,
    childUxStateId: 'child-ux-warning-blocked-local-handoff',
    surfaceState: AppGameChildUxSurfaceState.FamilyRuleWarning,
    claimState: AppGameChildUxClaimState.WarningOnly,
    titleToken: AppGameChildUxCopyToken.FamilyRuleTitle,
    bodyToken: AppGameChildUxCopyToken.FamilyRuleBody,
    primaryAction: AppGameChildUxPrimaryAction.Dismiss,
    primaryActionToken: AppGameChildUxCopyToken.DismissAction,
    childReasonReferences: [],
    approvalRequestRef: null,
  } as const;
}
