import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameLinuxForegroundCaptureReadinessSchema,
  type AppGameLinuxForegroundCaptureReadiness,
} from './app-game-linux-foreground-capture-readiness';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameLinuxForegroundSourcePreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-linux-foreground-source-preflight')
);

export const AppGameLinuxForegroundSourcePreflightStateSchema = withParser(
  Schema.Literal('foreground-source-preflight-ready', 'foreground-tool-install-required', 'display-source-not-ready')
);

export const AppGameLinuxForegroundSourcePreflightCustodySchema = withParser(
  Schema.Literal('no-window-title-captured')
);

export const AppGameLinuxForegroundSourcePreflightRefSchema = withParser(
  Schema.Literal(
    'linux-foreground-source-preflight-ref',
    'linux-foreground-capture-readiness-ref',
    'linux-wslg-display-ref',
    'linux-wslg-x11-socket-ref',
    'linux-wslg-wayland-socket-ref'
  )
);

export const AppGameLinuxForegroundSourcePreflightGapSchema = withParser(
  Schema.Literal(
    'linux-active-window-tool-not-available',
    'linux-active-window-title-not-captured',
    'linux-foreground-capture-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved'
  )
);

const LinuxForegroundSourceLabelSchema = brandedNonEmptyStringSchema('AppGameLinuxForegroundSourcePreflightLabel');

const AppGameLinuxForegroundSourcePreflightBaseSchema = Schema.Struct({
  schemaVersion: AppGameLinuxForegroundSourcePreflightSchemaVersionSchema,
  preflightId: LinuxForegroundSourceLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceReadinessId: LinuxForegroundSourceLabelSchema,
  preflightState: AppGameLinuxForegroundSourcePreflightStateSchema,
  custodyState: AppGameLinuxForegroundSourcePreflightCustodySchema,
  displayProofAttached: Schema.Boolean,
  foregroundToolAvailable: Schema.Boolean,
  foregroundSourcePreflightReady: Schema.Boolean,
  rawWindowTitleCaptured: Schema.Literal(false),
  foregroundCaptureClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameLinuxForegroundSourcePreflightRefSchema),
  openGaps: Schema.Array(AppGameLinuxForegroundSourcePreflightGapSchema),
  parentVisibleSummary: LinuxForegroundSourceLabelSchema,
});

type AppGameLinuxForegroundSourcePreflightCandidate = Infer<typeof AppGameLinuxForegroundSourcePreflightBaseSchema>;

export const AppGameLinuxForegroundSourcePreflightReadModelSchema = withParser(
  AppGameLinuxForegroundSourcePreflightBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        linuxForegroundSourcePreflightIsHonest(readModel) ||
        'Expected Linux foreground source preflight to track display and tool readiness without raw window title custody or enforcement claims'
    )
  )
);

export type AppGameLinuxForegroundSourcePreflightReadModel = Infer<
  typeof AppGameLinuxForegroundSourcePreflightReadModelSchema
>;

export const decodeAppGameLinuxForegroundSourcePreflightReadModel = Schema.decodeUnknownSync(
  AppGameLinuxForegroundSourcePreflightReadModelSchema
);

export function createAppGameLinuxForegroundSourcePreflightReadModel(input: {
  readonly readiness: AppGameLinuxForegroundCaptureReadiness;
  readonly generatedAt: AppGameLinuxForegroundSourcePreflightReadModel['generatedAt'];
}): AppGameLinuxForegroundSourcePreflightReadModel {
  const source = AppGameLinuxForegroundCaptureReadinessSchema.parse(input.readiness);
  const displayReady = source.displayProofAttached && source.x11SocketObserved && source.waylandSocketObserved;
  const preflightReady = displayReady && source.foregroundToolAvailable;

  return decodeAppGameLinuxForegroundSourcePreflightReadModel({
    schemaVersion: 'app-game-linux-foreground-source-preflight',
    preflightId: 'linux-foreground-source-preflight-ref',
    generatedAt: input.generatedAt,
    sourceReadinessId: source.readModelId,
    preflightState: linuxForegroundSourcePreflightState(displayReady, source.foregroundToolAvailable),
    custodyState: 'no-window-title-captured',
    displayProofAttached: displayReady,
    foregroundToolAvailable: source.foregroundToolAvailable,
    foregroundSourcePreflightReady: preflightReady,
    rawWindowTitleCaptured: false,
    foregroundCaptureClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: linuxForegroundSourcePreflightRefs(displayReady),
    openGaps: source.openGaps,
    parentVisibleSummary: preflightReady
      ? 'Linux foreground source preflight is ready for a real active-window capture command, but no raw title or enforcement is claimed.'
      : 'Linux foreground source preflight still requires a display source and active-window tool before capture can be proved.',
  });
}

export function summarizeAppGameLinuxForegroundSourcePreflightReadModel(
  readModel: AppGameLinuxForegroundSourcePreflightReadModel
) {
  return {
    preflightState: readModel.preflightState,
    foregroundSourcePreflightReady: readModel.foregroundSourcePreflightReady,
    displayProofAttached: readModel.displayProofAttached,
    foregroundToolAvailable: readModel.foregroundToolAvailable,
    openGapCount: readModel.openGaps.length,
  } as const;
}

function linuxForegroundSourcePreflightState(displayReady: boolean, foregroundToolAvailable: boolean) {
  if (!displayReady) {
    return 'display-source-not-ready';
  }
  if (!foregroundToolAvailable) {
    return 'foreground-tool-install-required';
  }
  return 'foreground-source-preflight-ready';
}

function linuxForegroundSourcePreflightRefs(displayReady: boolean) {
  const refs = ['linux-foreground-source-preflight-ref', 'linux-foreground-capture-readiness-ref'];
  if (displayReady) {
    refs.push('linux-wslg-display-ref', 'linux-wslg-x11-socket-ref', 'linux-wslg-wayland-socket-ref');
  }
  return refs;
}

function linuxForegroundSourcePreflightIsHonest(readModel: AppGameLinuxForegroundSourcePreflightCandidate): boolean {
  return (
    linuxForegroundSourceStateIsConsistent(readModel) &&
    readModel.custodyState === 'no-window-title-captured' &&
    !readModel.rawWindowTitleCaptured &&
    !readModel.foregroundCaptureClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed &&
    readModel.openGaps.includes('linux-foreground-capture-not-proved') &&
    readModel.openGaps.includes('linux-child-device-delivery-not-proved')
  );
}

function linuxForegroundSourceStateIsConsistent(readModel: AppGameLinuxForegroundSourcePreflightCandidate): boolean {
  if (readModel.preflightState === 'display-source-not-ready') {
    return !readModel.displayProofAttached && !readModel.foregroundSourcePreflightReady;
  }
  if (readModel.preflightState === 'foreground-tool-install-required') {
    return (
      readModel.displayProofAttached &&
      !readModel.foregroundToolAvailable &&
      !readModel.foregroundSourcePreflightReady &&
      readModel.openGaps.includes('linux-active-window-tool-not-available')
    );
  }
  return (
    readModel.displayProofAttached &&
    readModel.foregroundToolAvailable &&
    readModel.foregroundSourcePreflightReady &&
    readModel.proofRefs.includes('linux-foreground-source-preflight-ref')
  );
}
