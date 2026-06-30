const V08_BROWSER_ENFORCEMENT_TIMER_RECOVERY_PROOF_VALUES_TYPESCRIPT: &str = r#"/* generated from crates/child-enforcement-core/src/v08_browser_enforcement_timer_recovery_proof_values.rs */

import {
  ParentControlCapabilityName,
  ParentControlCapabilityStatus,
  type ParentControlCapabilityStatus as ParentControlCapabilityStatusType,
} from '@ocentra-parent/schema-domain/capabilities';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V08BrowserEnforcementTimerRecoveryProofReadModelSchema,
  V08TimerRecoveryProofEntrySchema,
  V08TimerRecoverySurface,
  V08UnmanagedBrowserFallbackProofEntrySchema,
  V08UnmanagedBrowserFallbackSurface,
  type V08BrowserEnforcementTimerRecoveryProofReadModel as V08BrowserEnforcementTimerRecoveryProofReadModelType,
  type V08TimerRecoveryEventKind,
  type V08TimerRecoveryLifecycleState,
  type V08TimerRecoveryParentVisibleState,
  type V08TimerRecoveryProofEntry,
  type V08TimerRecoveryResultStatus,
  type V08TimerRecoveryRollbackState,
  type V08TimerRecoveryStatePersistence,
  type V08TimerRecoverySurface as V08TimerRecoverySurfaceType,
  type V08UnmanagedBrowserFallbackExecutionState,
  type V08UnmanagedBrowserFallbackProofEntry,
  type V08UnmanagedBrowserFallbackState,
  type V08UnmanagedBrowserFallbackSurface as V08UnmanagedBrowserFallbackSurfaceType,
  type V08UnmanagedBrowserProcessIdentityState,
} from '@ocentra-parent/schema-domain/v0-8-browser-enforcement-timer-recovery-proof';

type TimerEntryInput = {
  proofEntryId: string;
  surface: V08TimerRecoverySurfaceType;
  lifecycleState: V08TimerRecoveryLifecycleState;
  timerEventKind: V08TimerRecoveryEventKind;
  resultStatus: V08TimerRecoveryResultStatus;
  rollbackState: V08TimerRecoveryRollbackState;
  statePersistence: V08TimerRecoveryStatePersistence;
  capabilityStatus: ParentControlCapabilityStatusType;
  parentVisibleStates: readonly V08TimerRecoveryParentVisibleState[];
  nextCheckAtVisible: boolean;
  failureStateVisible: boolean;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

type FallbackEntryInput = {
  proofEntryId: string;
  surface: V08UnmanagedBrowserFallbackSurfaceType;
  capabilityStatus: ParentControlCapabilityStatusType;
  fallbackState: V08UnmanagedBrowserFallbackState;
  adapterExecutionState: V08UnmanagedBrowserFallbackExecutionState;
  processIdentityRequired: boolean;
  processIdentityState: V08UnmanagedBrowserProcessIdentityState;
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

const generatedAt = '2026-06-03T21:39:51.644Z';

export const V08BrowserEnforcementTimerRecoveryProofReadModel: V08BrowserEnforcementTimerRecoveryProofReadModelType =
  V08BrowserEnforcementTimerRecoveryProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'v0-8-browser-enforcement-timer-recovery-proof',
    generatedAt,
    sourceReadModelIds: [
      'v0-8-supported-adapter-runtime-proof',
      'v0-8-browser-domain-adapter-proof',
      'v0-8-enforcement-product-control-spine',
      'v0-8-enforcement-timer-recovery-mvp',
    ],
    timerEntries: [
      timerEntry({
        proofEntryId: 'v0-8-timer-created-active',
        surface: V08TimerRecoverySurface.CreatedActive,
        lifecycleState: 'created',
        timerEventKind: 'created',
        resultStatus: 'no-op',
        rollbackState: 'available',
        statePersistence: 'active-state-persisted',
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        parentVisibleStates: ['next-check-visible'],
        nextCheckAtVisible: true,
        failureStateVisible: false,
        linkedProofCommands: ['node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs'],
        linkedProofArtifacts: ['test-results/v0-8-enforcement-timer-recovery-mvp'],
        manualProofRequirements: [],
        claimBoundary:
          'Timer create proof is limited to persisted active timer state, audit refs, and parent-visible next-check state.',
        fallbackBehavior:
          'Reject or report unavailable when the service cannot persist the active timer state for recovery.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-extended-active',
        surface: V08TimerRecoverySurface.ExtendedActive,
        lifecycleState: 'extended',
        timerEventKind: 'extended',
        resultStatus: 'no-op',
        rollbackState: 'available',
        statePersistence: 'active-state-persisted',
        capabilityStatus: ParentControlCapabilityStatus.Supported,
        parentVisibleStates: ['next-check-visible'],
        nextCheckAtVisible: true,
        failureStateVisible: false,
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['service command for active timer extension', 'proof artifact for updated expiry'],
        claimBoundary:
          'The contract supports extension as an active timer event, but the current service proof does not expose a dedicated extend command.',
        fallbackBehavior:
          'Keep existing active timer next-check visible and require a new service proof before claiming extension execution.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-expired-cleared',
        surface: V08TimerRecoverySurface.ExpiredCleared,
        lifecycleState: 'expired',
        timerEventKind: 'expired',
        resultStatus: 'expired',
        rollbackState: 'not-required',
        statePersistence: 'active-state-cleared',
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        parentVisibleStates: ['terminal-state-visible'],
        nextCheckAtVisible: false,
        failureStateVisible: false,
        linkedProofCommands: [
          'node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs',
          'cargo test -p ocentra-parent-agent-service enforcement_timer_expiry',
        ],
        linkedProofArtifacts: [
          'test-results/v0-8-enforcement-timer-recovery-mvp',
          'crates/agent-service/tests/unit.rs',
        ],
        manualProofRequirements: [],
        claimBoundary:
          'Timer expiry proof is limited to app time-limit active state expiry and clearing the persisted timer state.',
        fallbackBehavior: 'Report recovery-needed when expiry is requested without compatible active timer state.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-cancelled-cleared',
        surface: V08TimerRecoverySurface.CancelledCleared,
        lifecycleState: 'cancelled',
        timerEventKind: 'cancelled',
        resultStatus: 'superseded',
        rollbackState: 'not-required',
        statePersistence: 'active-state-cleared',
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        parentVisibleStates: ['terminal-state-visible'],
        nextCheckAtVisible: false,
        failureStateVisible: false,
        linkedProofCommands: [
          'node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs',
          'cargo test -p ocentra-parent-agent-service enforcement_timer',
        ],
        linkedProofArtifacts: [
          'test-results/v0-8-enforcement-timer-recovery-mvp',
          'crates/agent-service/tests/unit.rs',
        ],
        manualProofRequirements: [],
        claimBoundary: 'Cancel proof is parent-override scoped and clears only the matching active timer state.',
        fallbackBehavior: 'Do not treat parent cancel as broad rollback, browser control, or host policy removal.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-restart-recovered-active',
        surface: V08TimerRecoverySurface.RestartRecoveredActive,
        lifecycleState: 'restart-recovered',
        timerEventKind: 'restart-recovered',
        resultStatus: 'no-op',
        rollbackState: 'not-required',
        statePersistence: 'active-state-persisted',
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        parentVisibleStates: ['next-check-visible'],
        nextCheckAtVisible: true,
        failureStateVisible: false,
        linkedProofCommands: [
          'node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs',
          'cargo test -p ocentra-parent-agent-service enforcement_timer',
        ],
        linkedProofArtifacts: [
          'test-results/v0-8-enforcement-timer-recovery-mvp',
          'crates/agent-service/tests/unit.rs',
        ],
        manualProofRequirements: [],
        claimBoundary: 'Restart recovery proof preserves timer identity and next-check state after service restart.',
        fallbackBehavior:
          'Return recovery-needed instead of inventing timer state when the active state file is absent.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-recovery-needed-missing-state',
        surface: V08TimerRecoverySurface.RecoveryNeededMissingState,
        lifecycleState: 'recovery-needed',
        timerEventKind: 'recovery-needed',
        resultStatus: 'unavailable',
        rollbackState: 'unavailable',
        statePersistence: 'active-state-missing',
        capabilityStatus: ParentControlCapabilityStatus.Unavailable,
        parentVisibleStates: ['failure-state-visible'],
        nextCheckAtVisible: false,
        failureStateVisible: true,
        linkedProofCommands: [
          'node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs',
          'cargo test -p ocentra-parent-agent-service timer_recovery_reports_unavailable_when_active_state_is_missing',
        ],
        linkedProofArtifacts: [
          'test-results/v0-8-enforcement-timer-recovery-mvp',
          'crates/agent-service/tests/unit.rs',
        ],
        manualProofRequirements: ['operator-visible retry or manual review state'],
        claimBoundary:
          'Recovery-needed proof records missing active timer state as a failure state, not as a recovered or expired timer.',
        fallbackBehavior:
          'Show unavailable recovery state with reason rather than silently clearing or recreating a timer.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-rollback-completed-cleared',
        surface: V08TimerRecoverySurface.RollbackCompletedCleared,
        lifecycleState: 'rollback-completed',
        timerEventKind: 'rollback-completed',
        resultStatus: 'rolled-back',
        rollbackState: 'completed',
        statePersistence: 'active-state-cleared',
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        parentVisibleStates: ['terminal-state-visible', 'rollback-state-visible'],
        nextCheckAtVisible: false,
        failureStateVisible: false,
        linkedProofCommands: ['cargo test -p ocentra-parent-agent-core enforcement_timer_state'],
        linkedProofArtifacts: ['crates/agent-core/tests/unit.rs'],
        manualProofRequirements: [],
        claimBoundary:
          'Rollback-completed proof is a timer transition result boundary and does not prove host browser/domain rollback.',
        fallbackBehavior:
          'Expose rollback completed as terminal timer state only when adapter outcome reports rolled-back.',
      }),
      timerEntry({
        proofEntryId: 'v0-8-timer-rollback-unavailable-visible',
        surface: V08TimerRecoverySurface.RollbackUnavailableVisible,
        lifecycleState: 'rollback-unavailable',
        timerEventKind: 'unavailable',
        resultStatus: 'unavailable',
        rollbackState: 'unavailable',
        statePersistence: 'active-state-cleared',
        capabilityStatus: ParentControlCapabilityStatus.Unavailable,
        parentVisibleStates: ['failure-state-visible', 'rollback-state-visible'],
        nextCheckAtVisible: false,
        failureStateVisible: true,
        linkedProofCommands: ['cargo test -p ocentra-parent-agent-core enforcement_timer_state'],
        linkedProofArtifacts: ['crates/agent-core/tests/unit.rs'],
        manualProofRequirements: ['operator-visible rollback unavailable state'],
        claimBoundary:
          'Rollback-unavailable proof records rollback failure visibility and does not claim browser policy restoration.',
        fallbackBehavior:
          'Expose unavailable rollback state with failure reason and require manual review for host policy rollback.',
      }),
    ],
    unmanagedFallbackEntries: [
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-process-identity-required',
        surface: V08UnmanagedBrowserFallbackSurface.ProcessIdentityRequired,
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        fallbackState: 'process-identity-required',
        adapterExecutionState: 'process-identity-rejected',
        processIdentityRequired: true,
        processIdentityState: 'missing-process-identity-rejected',
        linkedProofCommands: ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        linkedProofArtifacts: ['test-results/windows-managed-unmanaged-browser-enforcement-proof'],
        manualProofRequirements: [],
        claimBoundary:
          'Unmanaged browser process actions require process id and expected process name; URL strings are not accepted as identity.',
        fallbackBehavior: 'Reject missing process identity before attempting terminate or relaunch fallback actions.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-report-only',
        surface: V08UnmanagedBrowserFallbackSurface.ReportOnly,
        capabilityStatus: ParentControlCapabilityStatus.Supported,
        fallbackState: 'report-only',
        adapterExecutionState: 'returns-report-only',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['browser integration or managed-session proof before escalating beyond report-only'],
        claimBoundary:
          'Report-only unmanaged browser fallback records process suspicion without claiming warning delivery or blocking.',
        fallbackBehavior:
          'Keep report-only separate from warn, review, terminate, relaunch, degraded, and unavailable states.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-warn-child',
        surface: V08UnmanagedBrowserFallbackSurface.WarnChild,
        capabilityStatus: ParentControlCapabilityStatus.Supported,
        fallbackState: 'warn-child',
        adapterExecutionState: 'returns-degraded-noop',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        linkedProofArtifacts: ['test-results/windows-managed-unmanaged-browser-enforcement-proof'],
        manualProofRequirements: ['notification delivery artifact', 'browser warning surface artifact'],
        claimBoundary:
          'Warn-child unmanaged fallback is a degraded no-op until notification and browser warning delivery are proved.',
        fallbackBehavior:
          'Return degraded warning state instead of claiming delivered warning or content-aware control.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-parent-review',
        surface: V08UnmanagedBrowserFallbackSurface.ParentReview,
        capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
        fallbackState: 'parent-review',
        adapterExecutionState: 'returns-parent-review',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['parent approval workflow artifact', 'review queue proof'],
        claimBoundary:
          'Parent-review fallback is a manual review state and is not a browser block, warning delivery, or exact URL claim.',
        fallbackBehavior: 'Route unknown unmanaged browser cases to review when enforcement identity is incomplete.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-terminate-process',
        surface: V08UnmanagedBrowserFallbackSurface.TerminateProcess,
        capabilityStatus: ParentControlCapabilityStatus.Implemented,
        fallbackState: 'terminate-process',
        adapterExecutionState: 'executes-real-service',
        processIdentityRequired: true,
        processIdentityState: 'pid-name-validated',
        linkedProofCommands: ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        linkedProofArtifacts: ['test-results/windows-managed-unmanaged-browser-enforcement-proof'],
        manualProofRequirements: [],
        claimBoundary: 'Terminate fallback is limited to a validated unmanaged browser process id and process name.',
        fallbackBehavior: 'Fail closed on process name mismatch and do not infer tab, title, URL, content, or intent.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-relaunch-managed-browser',
        surface: V08UnmanagedBrowserFallbackSurface.RelaunchManagedBrowser,
        capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
        fallbackState: 'relaunch-managed-browser',
        adapterExecutionState: 'returns-manual-required',
        processIdentityRequired: true,
        processIdentityState: 'pid-name-required',
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['managed browser launch proof', 'handoff custody artifact', 'rollback artifact'],
        claimBoundary:
          'Relaunch-managed fallback remains manual-required until the service proves managed launch and custody.',
        fallbackBehavior:
          'Do not kill and relaunch unmanaged browser sessions without process identity and managed-session proof.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-manual-required',
        surface: V08UnmanagedBrowserFallbackSurface.ManualRequired,
        capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
        fallbackState: 'manual-required',
        adapterExecutionState: 'returns-manual-required',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['operator-visible manual browser control state', 'host browser policy artifact'],
        claimBoundary:
          'Manual-required fallback covers browser controls that need host policy, managed session, or operator setup.',
        fallbackBehavior: 'Report manual-required instead of upgrading process evidence into browser/domain blocking.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-degraded',
        surface: V08UnmanagedBrowserFallbackSurface.Degraded,
        capabilityStatus: ParentControlCapabilityStatus.Supported,
        fallbackState: 'degraded',
        adapterExecutionState: 'returns-degraded-noop',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        linkedProofArtifacts: ['test-results/windows-managed-unmanaged-browser-enforcement-proof'],
        manualProofRequirements: ['delivery dependency proof', 'permission recovery proof'],
        claimBoundary:
          'Degraded unmanaged fallback is visible as a no-op or partial capability state, not enforcement success.',
        fallbackBehavior: 'Keep degraded separate from report-only, manual-required, and unavailable states.',
      }),
      fallbackEntry({
        proofEntryId: 'v0-8-unmanaged-unavailable',
        surface: V08UnmanagedBrowserFallbackSurface.Unavailable,
        capabilityStatus: ParentControlCapabilityStatus.Unavailable,
        fallbackState: 'unavailable',
        adapterExecutionState: 'returns-unavailable',
        processIdentityRequired: false,
        processIdentityState: 'not-applicable',
        linkedProofCommands: [],
        linkedProofArtifacts: [],
        manualProofRequirements: ['supported browser executable proof', 'target platform adapter proof'],
        claimBoundary: 'Unavailable fallback records when unmanaged browser handling cannot run on the target host.',
        fallbackBehavior: 'Report unavailable instead of falling through to broad app or domain blocking.',
      }),
    ],
  });

function timerEntry(input: TimerEntryInput): V08TimerRecoveryProofEntry {
  return V08TimerRecoveryProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: 'windows',
    capability: ParentControlCapabilityName.AppTimeLimit,
    lastCheckedAt: generatedAt,
    ...input,
    parentVisibleStates: [...input.parentVisibleStates],
    linkedProofCommands: [...input.linkedProofCommands],
    linkedProofArtifacts: [...input.linkedProofArtifacts],
    manualProofRequirements: [...input.manualProofRequirements],
  });
}

function fallbackEntry(input: FallbackEntryInput): V08UnmanagedBrowserFallbackProofEntry {
  return V08UnmanagedBrowserFallbackProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    platform: 'windows',
    capability: ParentControlCapabilityName.UnmanagedBrowserDetection,
    exactUrlClaimed: false,
    activeTabClaimed: false,
    titleClaimed: false,
    contentClaimed: false,
    notificationDeliveryClaimed: false,
    broadBrowserBlockingClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
    linkedProofCommands: [...input.linkedProofCommands],
    linkedProofArtifacts: [...input.linkedProofArtifacts],
    manualProofRequirements: [...input.manualProofRequirements],
  });
}
"#;

pub fn v08_browser_enforcement_timer_recovery_proof_values_typescript() -> String {
    V08_BROWSER_ENFORCEMENT_TIMER_RECOVERY_PROOF_VALUES_TYPESCRIPT.to_string()
}
