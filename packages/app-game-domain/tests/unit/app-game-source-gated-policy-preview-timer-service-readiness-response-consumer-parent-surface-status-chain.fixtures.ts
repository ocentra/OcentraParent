import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNonClaims,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-rules';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';

const SharedGeneratedAt = '2026-06-06T10:15:00Z';
const SharedServiceReadinessProofRefs = ['future-app-game-source-gated-policy-preview-timer-proof'] as const;
const SharedProtocolProofRefs = ['future-app-game-timer-service-readiness-protocol-proof'] as const;
const SharedProtocolCommandRefs = ['future-app-game-timer-service-readiness-protocol-command'] as const;
const SharedProtocolEventRefs = ['future-app-game-timer-service-readiness-protocol-event'] as const;
const SharedServiceHandlerRefs = ['future-app-game-timer-service-readiness-service-handler'] as const;
const SharedServiceReadApiProofRefs = ['future-app-game-timer-service-readiness-read-api-proof'] as const;
const SharedReadApiResponseProofRefs = ['future-app-game-timer-service-readiness-read-api-response-proof'] as const;
const SharedReadApiResponseConsumerProofRefs = [
  'future-app-game-timer-service-readiness-read-api-response-consumer-proof',
] as const;
const SharedParentSurfaceProofRefs = [
  'future-app-game-timer-service-readiness-response-consumer-parent-surface-proof',
] as const;
const SharedParentSurfaceReadModelProofRefs = [
  'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
] as const;
const SharedParentSurfaceStatusProofRefs = [
  'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof',
] as const;
const SharedServiceReadApiRef = 'future-service-readiness-read-api-contract';
const SharedParentSurfaceReadModelRef =
  'future-service-readiness-response-consumer-parent-surface-read-model-proof';
const SharedParentSurfaceStatusRef =
  'future-service-readiness-response-consumer-parent-surface-status-proof';

const StatusReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof',
  generatedAt: '2026-06-06T10:30:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
  ],
  parentSurfaceStatusReadModelRef: 'future-service-readiness-response-consumer-parent-surface-status-read-model-proof',
} as const;

const ParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof',
  generatedAt: '2026-06-06T10:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
  ],
  parentSurfaceRef: 'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof',
} as const;

const ParentSurfaceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T11:05:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  ],
  parentSurfaceReadModelRef:
    'future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
} as const;

type SeedStatusRowInput = {
  readonly sourceRowId: string;
  readonly targetDomain: 'native-app' | 'native-game';
  readonly state: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState;
  readonly sourceEvidenceRef: string;
};

export function buildParentSurfaceStatusReadModelHandoffFixture() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff(
    StatusReadModelHandoffOptions,
    buildParentSurfaceStatusHandoffFixture()
  );
}

export function buildParentSurfaceHandoffFixture() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff(
    ParentSurfaceHandoffOptions,
    buildParentSurfaceStatusReadModelHandoffFixture()
  );
}

export function buildParentSurfaceReadModelHandoffFixture() {
  return buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff(
    ParentSurfaceReadModelHandoffOptions,
    buildParentSurfaceHandoffFixture()
  );
}

function buildParentSurfaceStatusHandoffFixture() {
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.parse(
    {
      schemaVersion: ParentContractSchemaVersion.V0_6,
      responseConsumerParentSurfaceStatusHandoffId:
        'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-proof',
      sourceResponseConsumerParentSurfaceReadModelHandoffId:
        'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-proof',
      generatedAt: SharedGeneratedAt,
      sourceContractRefs: [
        'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff',
        'docs/expectations/app-game-evidence.md',
        'docs/expectations/enforcement.md',
      ],
      parentSurfaceStatusRef: SharedParentSurfaceStatusRef,
      rows: [
        buildStatusRow({
          sourceRowId: 'seed-parent-surface-status-row-1',
          targetDomain: 'native-app',
          state:
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired,
          sourceEvidenceRef: 'app-game-source-freshness-evidence/native-app-ready',
        }),
        buildStatusRow({
          sourceRowId: 'seed-parent-surface-status-row-2',
          targetDomain: 'native-app',
          state:
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness,
          sourceEvidenceRef: 'app-game-source-freshness-evidence/native-app-stale',
        }),
        buildStatusRow({
          sourceRowId: 'seed-parent-surface-status-row-3',
          targetDomain: 'native-game',
          state:
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision,
          sourceEvidenceRef: 'app-game-source-freshness-evidence/native-game-compiler-blocked',
        }),
      ],
      nativeAppRowCount: 2,
      nativeGameRowCount: 1,
      parentSurfaceStatusProofRequiredCount: 1,
      blockedBySourceFreshnessCount: 1,
      blockedByCompilerDecisionCount: 1,
      responseConsumerParentSurfaceStatusHandoffNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
    }
  );
}

function buildStatusRow({ sourceRowId, targetDomain, state, sourceEvidenceRef }: SeedStatusRowInput) {
  const isProofRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired;

  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: sourceRowId,
    sourceResponseConsumerParentSurfaceReadModelHandoffRowId: `${sourceRowId}:source-parent-surface-read-model`,
    targetDomain,
    responseConsumerParentSurfaceStatusHandoffState: state,
    inheritedProtocolProofRefs: isProofRequired ? SharedProtocolProofRefs : [],
    inheritedAgentProtocolCommandRefs: isProofRequired ? SharedProtocolCommandRefs : [],
    inheritedAgentProtocolEventRefs: isProofRequired ? SharedProtocolEventRefs : [],
    inheritedServiceHandlerRefs: isProofRequired ? SharedServiceHandlerRefs : [],
    inheritedServiceReadApiProofRefs: isProofRequired ? SharedServiceReadApiProofRefs : [],
    inheritedReadApiResponseProofRefs: isProofRequired ? SharedReadApiResponseProofRefs : [],
    inheritedReadApiResponseConsumerProofRefs: isProofRequired ? SharedReadApiResponseConsumerProofRefs : [],
    inheritedParentSurfaceProofRefs: isProofRequired ? SharedParentSurfaceProofRefs : [],
    inheritedParentSurfaceReadModelProofRefs: isProofRequired ? SharedParentSurfaceReadModelProofRefs : [],
    requiredParentSurfaceStatusProofRefs: isProofRequired ? SharedParentSurfaceStatusProofRefs : [],
    inheritedServiceReadinessProofRefs: isProofRequired ? SharedServiceReadinessProofRefs : [],
    sourceEvidenceRefs: [sourceEvidenceRef],
    serviceReadApiRef: SharedServiceReadApiRef,
    parentSurfaceReadModelRef: SharedParentSurfaceReadModelRef,
    parentSurfaceStatusRef: SharedParentSurfaceStatusRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
    generatedAt: SharedGeneratedAt,
  };
}
