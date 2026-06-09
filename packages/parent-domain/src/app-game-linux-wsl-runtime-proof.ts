import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const LinuxWslProofText = Schema.String.pipe(Schema.minLength(1));

export const AppGameLinuxWslRuntimeProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-linux-wsl-runtime-proof')
);

export const AppGameLinuxWslRuntimeTargetKindSchema = withParser(Schema.Literal('wsl2-distro'));

export const AppGameLinuxWslRuntimeStateSchema = withParser(Schema.Literal('runtime-observed'));

export const AppGameLinuxWslDockerStateSchema = withParser(Schema.Literal('docker-cli-unavailable', 'docker-visible'));

export const AppGameLinuxWslSessionStateSchema = withParser(
  Schema.Literal('systemd-session-observed', 'session-not-proved')
);

export const AppGameLinuxWslDisplayStateSchema = withParser(
  Schema.Literal('wslg-display-observed', 'display-not-proved')
);

export const AppGameLinuxWslSocketStateSchema = withParser(Schema.Literal('socket-observed', 'socket-not-proved'));

export const AppGameLinuxWslForegroundProbeStateSchema = withParser(
  Schema.Literal('active-window-tool-missing', 'active-window-not-proved')
);

export const AppGameLinuxWslProofRefSchema = withParser(
  Schema.Literal(
    'linux-wsl-distro-ref',
    'linux-wsl-kernel-ref',
    'linux-wsl-package-manager-ref',
    'linux-wsl-process-ref',
    'linux-wsl-session-ref',
    'linux-wslg-display-ref',
    'linux-wslg-x11-socket-ref',
    'linux-wslg-wayland-socket-ref',
    'linux-docker-cli-ref'
  )
);

const LinuxWslProofLabelSchema = LinuxWslProofText.pipe(Schema.brand('AppGameLinuxWslRuntimeProofLabel'));
const LinuxWslPositiveCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(1));

const AppGameLinuxWslRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameLinuxWslRuntimeProofSchemaVersionSchema,
  proofId: LinuxWslProofLabelSchema,
  targetKind: AppGameLinuxWslRuntimeTargetKindSchema,
  runtimeState: AppGameLinuxWslRuntimeStateSchema,
  distroRef: LinuxWslProofLabelSchema,
  distroId: LinuxWslProofLabelSchema,
  distroVersion: LinuxWslProofLabelSchema,
  kernelRelease: LinuxWslProofLabelSchema,
  architecture: LinuxWslProofLabelSchema,
  packageManagerVisibleCount: LinuxWslPositiveCountSchema,
  processSnapshotCount: LinuxWslPositiveCountSchema,
  systemdSessionState: AppGameLinuxWslSessionStateSchema,
  displayState: AppGameLinuxWslDisplayStateSchema,
  x11SocketState: AppGameLinuxWslSocketStateSchema,
  waylandSocketState: AppGameLinuxWslSocketStateSchema,
  foregroundProbeState: AppGameLinuxWslForegroundProbeStateSchema,
  dockerState: AppGameLinuxWslDockerStateSchema,
  proofRefs: Schema.Array(AppGameLinuxWslProofRefSchema),
  packageNamesRedacted: Schema.Boolean,
  processNamesRedacted: Schema.Boolean,
  rawDistroNameRedacted: Schema.Boolean,
  mechanismProofAttached: Schema.Boolean,
  distroProofAttached: Schema.Boolean,
  sessionProofAttached: Schema.Boolean,
  displayProofAttached: Schema.Boolean,
  rollbackProofAttached: Schema.Boolean,
  auditProofAttached: Schema.Boolean,
  foregroundCaptureClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  parentVisibleSummary: LinuxWslProofLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type AppGameLinuxWslRuntimeProofCandidate = Infer<typeof AppGameLinuxWslRuntimeProofBaseSchema>;

export const AppGameLinuxWslRuntimeProofSchema = withParser(
  AppGameLinuxWslRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        linuxWslRuntimeProofIsHonest(proof) ||
        'Expected Linux WSL runtime proof to redact raw distro/package/process details and keep broad blocking/enforcement false without mechanism, distro, session, rollback, and audit proof'
    )
  )
);

export type AppGameLinuxWslRuntimeProof = Infer<typeof AppGameLinuxWslRuntimeProofSchema>;
export type AppGameLinuxWslProofRef = Infer<typeof AppGameLinuxWslProofRefSchema>;

export const decodeAppGameLinuxWslRuntimeProof = Schema.decodeUnknownSync(AppGameLinuxWslRuntimeProofSchema);

export function summarizeAppGameLinuxWslRuntimeProof(proof: AppGameLinuxWslRuntimeProof) {
  return {
    targetKind: proof.targetKind,
    distroId: proof.distroId,
    distroVersion: proof.distroVersion,
    packageManagerVisibleCount: proof.packageManagerVisibleCount,
    processSnapshotCount: proof.processSnapshotCount,
    dockerState: proof.dockerState,
    displayState: proof.displayState,
    foregroundProbeState: proof.foregroundProbeState,
    displayProofAttached: proof.displayProofAttached,
    foregroundCaptureClaimed: proof.foregroundCaptureClaimed,
    proofComplete:
      proof.mechanismProofAttached &&
      proof.distroProofAttached &&
      proof.sessionProofAttached &&
      proof.displayProofAttached &&
      proof.rollbackProofAttached &&
      proof.auditProofAttached,
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
  } as const;
}

function linuxWslRuntimeProofIsHonest(proof: AppGameLinuxWslRuntimeProofCandidate): boolean {
  return (
    proof.targetKind === 'wsl2-distro' &&
    proof.runtimeState === 'runtime-observed' &&
    proof.distroRef === 'linux-wsl-distro-ref' &&
    proof.proofRefs.includes('linux-wsl-distro-ref') &&
    proof.proofRefs.includes('linux-wsl-kernel-ref') &&
    proof.proofRefs.includes('linux-wsl-package-manager-ref') &&
    proof.proofRefs.includes('linux-wsl-process-ref') &&
    proof.proofRefs.includes('linux-wsl-session-ref') &&
    proof.proofRefs.includes('linux-docker-cli-ref') &&
    proof.packageNamesRedacted &&
    proof.processNamesRedacted &&
    proof.rawDistroNameRedacted &&
    linuxWslDisplayProofIsHonest(proof) &&
    !proof.foregroundCaptureClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.broadBlockingClaimed &&
    !proof.platformEnforcementClaimed &&
    (!proof.mechanismProofAttached ||
      !proof.distroProofAttached ||
      !proof.sessionProofAttached ||
      !proof.displayProofAttached ||
      !proof.rollbackProofAttached ||
      !proof.auditProofAttached)
  );
}

function linuxWslDisplayProofIsHonest(proof: AppGameLinuxWslRuntimeProofCandidate): boolean {
  if (proof.displayState === 'display-not-proved') {
    return (
      proof.x11SocketState === 'socket-not-proved' &&
      proof.waylandSocketState === 'socket-not-proved' &&
      proof.foregroundProbeState === 'active-window-not-proved' &&
      !proof.displayProofAttached
    );
  }

  return (
    proof.x11SocketState === 'socket-observed' &&
    proof.waylandSocketState === 'socket-observed' &&
    proof.foregroundProbeState === 'active-window-tool-missing' &&
    proof.proofRefs.includes('linux-wslg-display-ref') &&
    proof.proofRefs.includes('linux-wslg-x11-socket-ref') &&
    proof.proofRefs.includes('linux-wslg-wayland-socket-ref') &&
    proof.displayProofAttached
  );
}
