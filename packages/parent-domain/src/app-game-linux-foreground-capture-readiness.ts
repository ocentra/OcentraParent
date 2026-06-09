import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { type AppGameLinuxWslRuntimeProof } from './app-game-linux-wsl-runtime-proof';
import { ParentTimestampSchema } from './reference-primitives';

const LinuxForegroundCaptureText = Schema.String.pipe(Schema.minLength(1));

export const AppGameLinuxForegroundCaptureReadinessSchemaVersionSchema = withParser(
  Schema.Literal('app-game-linux-foreground-capture-readiness')
);

export const AppGameLinuxForegroundCaptureReadinessStateSchema = withParser(
  Schema.Literal('display-ready-capture-tool-missing', 'display-not-ready', 'foreground-capture-not-proved')
);

export const AppGameLinuxForegroundCaptureSourceStateSchema = withParser(
  Schema.Literal('wslg-display-sockets-observed', 'source-not-available')
);

export const AppGameLinuxForegroundCaptureCustodyStateSchema = withParser(
  Schema.Literal('no-window-title-custody')
);

export const AppGameLinuxForegroundCaptureReadinessRefSchema = withParser(
  Schema.Literal(
    'linux-foreground-capture-readiness-ref',
    'linux-wslg-display-ref',
    'linux-wslg-x11-socket-ref',
    'linux-wslg-wayland-socket-ref'
  )
);

export const AppGameLinuxForegroundCaptureReadinessGapSchema = withParser(
  Schema.Literal(
    'linux-active-window-tool-not-available',
    'linux-active-window-title-not-captured',
    'linux-foreground-capture-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved'
  )
);

const LinuxForegroundCaptureLabelSchema = LinuxForegroundCaptureText.pipe(
  Schema.brand('AppGameLinuxForegroundCaptureReadinessLabel')
);

const AppGameLinuxForegroundCaptureReadinessBaseSchema = Schema.Struct({
  schemaVersion: AppGameLinuxForegroundCaptureReadinessSchemaVersionSchema,
  readModelId: LinuxForegroundCaptureLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofId: LinuxForegroundCaptureLabelSchema,
  readinessState: AppGameLinuxForegroundCaptureReadinessStateSchema,
  sourceState: AppGameLinuxForegroundCaptureSourceStateSchema,
  custodyState: AppGameLinuxForegroundCaptureCustodyStateSchema,
  displayProofAttached: Schema.Boolean,
  x11SocketObserved: Schema.Boolean,
  waylandSocketObserved: Schema.Boolean,
  foregroundToolAvailable: Schema.Boolean,
  foregroundCaptureReady: Schema.Boolean,
  rawWindowTitleClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  childDeviceDeliveryClaimed: Schema.Boolean,
  proofRefs: Schema.Array(AppGameLinuxForegroundCaptureReadinessRefSchema),
  openGaps: Schema.Array(AppGameLinuxForegroundCaptureReadinessGapSchema),
  parentVisibleSummary: LinuxForegroundCaptureLabelSchema,
});

type AppGameLinuxForegroundCaptureReadinessCandidate = Infer<
  typeof AppGameLinuxForegroundCaptureReadinessBaseSchema
>;

export const AppGameLinuxForegroundCaptureReadinessSchema = withParser(
  AppGameLinuxForegroundCaptureReadinessBaseSchema.pipe(
    Schema.filter(
      (readiness) =>
        linuxForegroundCaptureReadinessIsHonest(readiness) ||
        'Expected Linux foreground capture readiness to report WSLg display/socket readiness without raw window title custody or enforcement claims'
    )
  )
);

export type AppGameLinuxForegroundCaptureReadiness = Infer<
  typeof AppGameLinuxForegroundCaptureReadinessSchema
>;

export const decodeAppGameLinuxForegroundCaptureReadiness = Schema.decodeUnknownSync(
  AppGameLinuxForegroundCaptureReadinessSchema
);

export function createAppGameLinuxForegroundCaptureReadiness(input: {
  readonly linuxProof: AppGameLinuxWslRuntimeProof;
  readonly generatedAt: AppGameLinuxForegroundCaptureReadiness['generatedAt'];
}): AppGameLinuxForegroundCaptureReadiness {
  const displayReady =
    input.linuxProof.displayProofAttached &&
    input.linuxProof.x11SocketState === 'socket-observed' &&
    input.linuxProof.waylandSocketState === 'socket-observed';
  const foregroundToolAvailable = input.linuxProof.foregroundProbeState !== 'active-window-tool-missing';
  const readModel = {
    schemaVersion: 'app-game-linux-foreground-capture-readiness',
    readModelId: 'linux-foreground-capture-readiness-ref',
    generatedAt: input.generatedAt,
    sourceProofId: input.linuxProof.proofId,
    readinessState: linuxForegroundCaptureReadinessState(displayReady, foregroundToolAvailable),
    sourceState: displayReady ? 'wslg-display-sockets-observed' : 'source-not-available',
    custodyState: 'no-window-title-custody',
    displayProofAttached: input.linuxProof.displayProofAttached,
    x11SocketObserved: input.linuxProof.x11SocketState === 'socket-observed',
    waylandSocketObserved: input.linuxProof.waylandSocketState === 'socket-observed',
    foregroundToolAvailable,
    foregroundCaptureReady: false,
    rawWindowTitleClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: linuxForegroundCaptureProofRefs(displayReady),
    openGaps: linuxForegroundCaptureOpenGaps(foregroundToolAvailable),
    parentVisibleSummary:
      'Linux WSLg display and socket readiness is visible, but active foreground capture remains unproved until a real foreground source is attached without raw title custody.',
  };

  return decodeAppGameLinuxForegroundCaptureReadiness(readModel);
}

export function summarizeAppGameLinuxForegroundCaptureReadiness(
  readiness: AppGameLinuxForegroundCaptureReadiness
) {
  return {
    readinessState: readiness.readinessState,
    sourceState: readiness.sourceState,
    displayProofAttached: readiness.displayProofAttached,
    foregroundToolAvailable: readiness.foregroundToolAvailable,
    foregroundCaptureReady: readiness.foregroundCaptureReady,
    openGapCount: readiness.openGaps.length,
  } as const;
}

function linuxForegroundCaptureReadinessState(displayReady: boolean, foregroundToolAvailable: boolean) {
  if (!displayReady) {
    return 'display-not-ready';
  }
  if (!foregroundToolAvailable) {
    return 'display-ready-capture-tool-missing';
  }
  return 'foreground-capture-not-proved';
}

function linuxForegroundCaptureProofRefs(displayReady: boolean) {
  const refs = ['linux-foreground-capture-readiness-ref'];
  if (displayReady) {
    refs.push('linux-wslg-display-ref', 'linux-wslg-x11-socket-ref', 'linux-wslg-wayland-socket-ref');
  }
  return refs;
}

function linuxForegroundCaptureOpenGaps(foregroundToolAvailable: boolean) {
  const gaps = [
    'linux-active-window-title-not-captured',
    'linux-foreground-capture-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved',
  ];
  if (!foregroundToolAvailable) {
    gaps.unshift('linux-active-window-tool-not-available');
  }
  return gaps;
}

function linuxForegroundCaptureReadinessIsHonest(
  readiness: AppGameLinuxForegroundCaptureReadinessCandidate
): boolean {
  return (
    linuxForegroundCaptureStateIsConsistent(readiness) &&
    readiness.custodyState === 'no-window-title-custody' &&
    !readiness.foregroundCaptureReady &&
    !readiness.rawWindowTitleClaimed &&
    !readiness.adapterDispatchClaimed &&
    !readiness.platformEnforcementClaimed &&
    !readiness.childDeviceDeliveryClaimed &&
    readiness.openGaps.includes('linux-foreground-capture-not-proved') &&
    readiness.openGaps.includes('linux-child-device-delivery-not-proved')
  );
}

function linuxForegroundCaptureStateIsConsistent(
  readiness: AppGameLinuxForegroundCaptureReadinessCandidate
): boolean {
  if (readiness.readinessState === 'display-not-ready') {
    return !readiness.displayProofAttached && readiness.sourceState === 'source-not-available';
  }
  if (readiness.readinessState === 'display-ready-capture-tool-missing') {
    return (
      readiness.displayProofAttached &&
      readiness.x11SocketObserved &&
      readiness.waylandSocketObserved &&
      !readiness.foregroundToolAvailable &&
      readiness.proofRefs.includes('linux-wslg-display-ref')
    );
  }
  return (
    readiness.displayProofAttached &&
    readiness.foregroundToolAvailable &&
    readiness.sourceState === 'wslg-display-sockets-observed'
  );
}
