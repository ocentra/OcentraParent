import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const LinuxActiveWindowText = Schema.String.pipe(Schema.minLength(1));

export const AppGameLinuxActiveWindowToolProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-linux-active-window-tool-proof')
);

export const AppGameLinuxActiveWindowToolStateSchema = withParser(
  Schema.Literal('xprop-available', 'xdotool-available', 'active-window-tool-missing')
);

export const AppGameLinuxActiveWindowRefStateSchema = withParser(
  Schema.Literal('active-window-ref-observed', 'no-active-window-ref', 'active-window-query-unavailable')
);

export const AppGameLinuxActiveWindowToolProofRefSchema = withParser(
  Schema.Literal(
    'linux-wsl-runtime-proof-ref',
    'linux-wslg-display-ref',
    'linux-active-window-tool-ref',
    'linux-active-window-ref-proof'
  )
);

export const AppGameLinuxActiveWindowToolGapSchema = withParser(
  Schema.Literal(
    'linux-active-window-ref-not-observed',
    'linux-active-window-title-not-captured',
    'linux-foreground-capture-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved'
  )
);

const LinuxActiveWindowLabelSchema = LinuxActiveWindowText.pipe(Schema.brand('AppGameLinuxActiveWindowToolProofLabel'));

const AppGameLinuxActiveWindowToolProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameLinuxActiveWindowToolProofSchemaVersionSchema,
  proofId: LinuxActiveWindowLabelSchema,
  toolState: AppGameLinuxActiveWindowToolStateSchema,
  activeWindowRefState: AppGameLinuxActiveWindowRefStateSchema,
  displaySourceObserved: Schema.Boolean,
  toolAvailable: Schema.Boolean,
  activeWindowRefObserved: Schema.Boolean,
  rawWindowTitleStored: Schema.Literal(false),
  rawProcessNameStored: Schema.Literal(false),
  foregroundCaptureClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameLinuxActiveWindowToolProofRefSchema),
  openGaps: Schema.Array(AppGameLinuxActiveWindowToolGapSchema),
  parentVisibleSummary: LinuxActiveWindowLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type LinuxActiveWindowCandidate = Infer<typeof AppGameLinuxActiveWindowToolProofBaseSchema>;

export const AppGameLinuxActiveWindowToolProofSchema = withParser(
  AppGameLinuxActiveWindowToolProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        linuxActiveWindowToolProofIsHonest(proof) ||
        'Expected Linux active-window tool proof to keep raw title/process custody, foreground capture, adapter dispatch, enforcement, and child delivery unclaimed'
    )
  )
);

export type AppGameLinuxActiveWindowToolProof = Infer<typeof AppGameLinuxActiveWindowToolProofSchema>;

export const decodeAppGameLinuxActiveWindowToolProof = Schema.decodeUnknownSync(
  AppGameLinuxActiveWindowToolProofSchema
);

export function createAppGameLinuxActiveWindowToolProof(input: {
  readonly toolState: AppGameLinuxActiveWindowToolProof['toolState'];
  readonly activeWindowRefState: AppGameLinuxActiveWindowToolProof['activeWindowRefState'];
  readonly displaySourceObserved: AppGameLinuxActiveWindowToolProof['displaySourceObserved'];
  readonly checkedAt: AppGameLinuxActiveWindowToolProof['checkedAt'];
}): AppGameLinuxActiveWindowToolProof {
  const toolAvailable = input.toolState !== 'active-window-tool-missing';
  const activeWindowRefObserved = input.activeWindowRefState === 'active-window-ref-observed';

  return decodeAppGameLinuxActiveWindowToolProof({
    schemaVersion: 'app-game-linux-active-window-tool-proof',
    proofId: 'linux-active-window-tool-proof-ref',
    toolState: input.toolState,
    activeWindowRefState: input.activeWindowRefState,
    displaySourceObserved: input.displaySourceObserved,
    toolAvailable,
    activeWindowRefObserved,
    rawWindowTitleStored: false,
    rawProcessNameStored: false,
    foregroundCaptureClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: linuxActiveWindowToolProofRefs(input.displaySourceObserved, toolAvailable, activeWindowRefObserved),
    openGaps: linuxActiveWindowToolOpenGaps(activeWindowRefObserved),
    parentVisibleSummary: linuxActiveWindowSummary(toolAvailable, activeWindowRefObserved),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameLinuxActiveWindowToolProof(proof: AppGameLinuxActiveWindowToolProof) {
  return {
    toolState: proof.toolState,
    activeWindowRefState: proof.activeWindowRefState,
    displaySourceObserved: proof.displaySourceObserved,
    toolAvailable: proof.toolAvailable,
    activeWindowRefObserved: proof.activeWindowRefObserved,
    openGapCount: proof.openGaps.length,
  } as const;
}

function linuxActiveWindowToolProofRefs(
  displaySourceObserved: boolean,
  toolAvailable: boolean,
  activeWindowRefObserved: boolean
) {
  const refs = ['linux-wsl-runtime-proof-ref'];
  if (displaySourceObserved) {
    refs.push('linux-wslg-display-ref');
  }
  if (toolAvailable) {
    refs.push('linux-active-window-tool-ref');
  }
  if (activeWindowRefObserved) {
    refs.push('linux-active-window-ref-proof');
  }
  return refs;
}

function linuxActiveWindowToolOpenGaps(activeWindowRefObserved: boolean) {
  const gaps = [
    'linux-active-window-title-not-captured',
    'linux-foreground-capture-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved',
  ];
  if (!activeWindowRefObserved) {
    gaps.unshift('linux-active-window-ref-not-observed');
  }
  return gaps;
}

function linuxActiveWindowSummary(toolAvailable: boolean, activeWindowRefObserved: boolean) {
  if (toolAvailable && activeWindowRefObserved) {
    return 'Linux active-window tool and opaque active-window ref are visible, but raw title custody, foreground capture, enforcement, and child delivery remain unclaimed.';
  }
  if (toolAvailable) {
    return 'Linux active-window tool is visible, but no active-window ref is observed yet, so foreground capture remains unproved.';
  }
  return 'Linux active-window tool is missing, so foreground capture remains blocked before runtime proof.';
}

function linuxActiveWindowToolProofIsHonest(proof: LinuxActiveWindowCandidate): boolean {
  return (
    proof.toolAvailable === (proof.toolState !== 'active-window-tool-missing') &&
    proof.activeWindowRefObserved === (proof.activeWindowRefState === 'active-window-ref-observed') &&
    proof.openGaps.includes('linux-active-window-title-not-captured') &&
    proof.openGaps.includes('linux-foreground-capture-not-proved') &&
    proof.openGaps.includes('linux-child-device-delivery-not-proved') &&
    !proof.rawWindowTitleStored &&
    !proof.rawProcessNameStored &&
    !proof.foregroundCaptureClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.childDeviceDeliveryClaimed
  );
}
