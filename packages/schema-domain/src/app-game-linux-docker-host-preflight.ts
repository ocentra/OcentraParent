import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameLinuxDockerHostPreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-linux-docker-host-preflight')
);

export const AppGameLinuxDockerHostPreflightStateSchema = withParser(
  Schema.Literal('docker-daemon-visible', 'docker-cli-visible-daemon-unavailable', 'docker-cli-unavailable')
);

export const AppGameLinuxDockerInventoryStateSchema = withParser(
  Schema.Literal('inventory-visible', 'inventory-empty', 'inventory-unavailable')
);

export const AppGameLinuxDockerHostPreflightRefSchema = withParser(
  Schema.Literal(
    'linux-docker-host-preflight-ref',
    'linux-docker-cli-ref',
    'linux-docker-daemon-ref',
    'linux-docker-context-ref',
    'linux-docker-image-inventory-ref',
    'linux-docker-container-inventory-ref'
  )
);

export const AppGameLinuxDockerHostPreflightGapSchema = withParser(
  Schema.Literal(
    'linux-docker-cli-not-proved',
    'linux-docker-daemon-not-proved',
    'linux-docker-image-inventory-not-proved',
    'linux-docker-container-inventory-not-proved',
    'linux-container-policy-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved'
  )
);

const LinuxDockerPreflightLabelSchema = brandedNonEmptyStringSchema('AppGameLinuxDockerHostPreflightLabel');
const LinuxDockerPreflightCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGameLinuxDockerHostPreflightBaseSchema = Schema.Struct({
  schemaVersion: AppGameLinuxDockerHostPreflightSchemaVersionSchema,
  readModelId: LinuxDockerPreflightLabelSchema,
  generatedAt: ParentTimestampSchema,
  preflightState: AppGameLinuxDockerHostPreflightStateSchema,
  contextInventoryState: AppGameLinuxDockerInventoryStateSchema,
  imageInventoryState: AppGameLinuxDockerInventoryStateSchema,
  containerInventoryState: AppGameLinuxDockerInventoryStateSchema,
  contextCount: LinuxDockerPreflightCountSchema,
  imageCount: LinuxDockerPreflightCountSchema,
  containerCount: LinuxDockerPreflightCountSchema,
  dockerCliObserved: Schema.Boolean,
  dockerDaemonObserved: Schema.Boolean,
  contextNamesRedacted: Schema.Literal(true),
  imageNamesRedacted: Schema.Literal(true),
  containerIdsRedacted: Schema.Literal(true),
  adapterDispatchClaimed: Schema.Literal(false),
  containerPolicyClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameLinuxDockerHostPreflightRefSchema),
  openGaps: Schema.Array(AppGameLinuxDockerHostPreflightGapSchema),
  parentVisibleSummary: LinuxDockerPreflightLabelSchema,
});

type AppGameLinuxDockerHostPreflightCandidate = Infer<typeof AppGameLinuxDockerHostPreflightBaseSchema>;

export const AppGameLinuxDockerHostPreflightReadModelSchema = withParser(
  AppGameLinuxDockerHostPreflightBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        linuxDockerHostPreflightIsHonest(readModel) ||
        'Expected Linux Docker host preflight to expose only redacted Docker host readiness counts without container policy or enforcement claims'
    )
  )
);

export type AppGameLinuxDockerHostPreflightReadModel = Infer<typeof AppGameLinuxDockerHostPreflightReadModelSchema>;
export type AppGameLinuxDockerHostPreflightGap = Infer<typeof AppGameLinuxDockerHostPreflightGapSchema>;

export const decodeAppGameLinuxDockerHostPreflightReadModel = Schema.decodeUnknownSync(
  AppGameLinuxDockerHostPreflightReadModelSchema
);

export function createAppGameLinuxDockerHostPreflightReadModel(input: {
  readonly generatedAt: AppGameLinuxDockerHostPreflightReadModel['generatedAt'];
  readonly dockerCliObserved: boolean;
  readonly dockerDaemonObserved: boolean;
  readonly contextCount: number;
  readonly imageCount: number;
  readonly containerCount: number;
}): AppGameLinuxDockerHostPreflightReadModel {
  return decodeAppGameLinuxDockerHostPreflightReadModel({
    schemaVersion: 'app-game-linux-docker-host-preflight',
    readModelId: 'linux-docker-host-preflight-ref',
    generatedAt: input.generatedAt,
    preflightState: linuxDockerPreflightState(input.dockerCliObserved, input.dockerDaemonObserved),
    contextInventoryState: linuxDockerInventoryState(input.dockerCliObserved, input.contextCount),
    imageInventoryState: linuxDockerInventoryState(input.dockerDaemonObserved, input.imageCount),
    containerInventoryState: linuxDockerInventoryState(input.dockerDaemonObserved, input.containerCount),
    contextCount: input.contextCount,
    imageCount: input.imageCount,
    containerCount: input.containerCount,
    dockerCliObserved: input.dockerCliObserved,
    dockerDaemonObserved: input.dockerDaemonObserved,
    contextNamesRedacted: true,
    imageNamesRedacted: true,
    containerIdsRedacted: true,
    adapterDispatchClaimed: false,
    containerPolicyClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: linuxDockerPreflightRefs(input.dockerCliObserved, input.dockerDaemonObserved),
    openGaps: linuxDockerPreflightGaps(input.dockerCliObserved, input.dockerDaemonObserved, input),
    parentVisibleSummary:
      'Linux Docker host preflight records redacted CLI, daemon, context, image, and container counts only; container policy and enforcement remain unclaimed.',
  });
}

export function summarizeAppGameLinuxDockerHostPreflightReadModel(readModel: AppGameLinuxDockerHostPreflightReadModel) {
  return {
    preflightState: readModel.preflightState,
    dockerCliObserved: readModel.dockerCliObserved,
    dockerDaemonObserved: readModel.dockerDaemonObserved,
    contextCount: readModel.contextCount,
    imageCount: readModel.imageCount,
    containerCount: readModel.containerCount,
    openGapCount: readModel.openGaps.length,
    adapterDispatchClaimed: readModel.adapterDispatchClaimed,
    platformEnforcementClaimed: readModel.platformEnforcementClaimed,
  } as const;
}

function linuxDockerPreflightState(dockerCliObserved: boolean, dockerDaemonObserved: boolean) {
  if (!dockerCliObserved) {
    return 'docker-cli-unavailable';
  }
  return dockerDaemonObserved ? 'docker-daemon-visible' : 'docker-cli-visible-daemon-unavailable';
}

function linuxDockerInventoryState(sourceAvailable: boolean, count: number) {
  if (!sourceAvailable) {
    return 'inventory-unavailable';
  }
  return count > 0 ? 'inventory-visible' : 'inventory-empty';
}

function linuxDockerPreflightRefs(dockerCliObserved: boolean, dockerDaemonObserved: boolean) {
  const refs = ['linux-docker-host-preflight-ref'];
  if (dockerCliObserved) {
    refs.push('linux-docker-cli-ref', 'linux-docker-context-ref');
  }
  if (dockerDaemonObserved) {
    refs.push('linux-docker-daemon-ref', 'linux-docker-image-inventory-ref', 'linux-docker-container-inventory-ref');
  }
  return refs;
}

function linuxDockerPreflightGaps(
  dockerCliObserved: boolean,
  dockerDaemonObserved: boolean,
  input: { readonly imageCount: number; readonly containerCount: number }
) {
  const gaps: AppGameLinuxDockerHostPreflightGap[] = [
    'linux-container-policy-not-proved',
    'linux-platform-enforcement-not-proved',
    'linux-child-device-delivery-not-proved',
  ];
  if (!dockerCliObserved) {
    gaps.push('linux-docker-cli-not-proved');
  }
  if (!dockerDaemonObserved) {
    gaps.push('linux-docker-daemon-not-proved');
  }
  if (!dockerDaemonObserved || input.imageCount === 0) {
    gaps.push('linux-docker-image-inventory-not-proved');
  }
  if (!dockerDaemonObserved || input.containerCount === 0) {
    gaps.push('linux-docker-container-inventory-not-proved');
  }
  return gaps;
}

function linuxDockerHostPreflightIsHonest(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  return (
    linuxDockerStateIsConsistent(readModel) &&
    linuxDockerInventoriesAreConsistent(readModel) &&
    linuxDockerRedactionIsHonest(readModel) &&
    linuxDockerClaimsRemainScoped(readModel) &&
    linuxDockerRequiredProofsArePresent(readModel)
  );
}

function linuxDockerRedactionIsHonest(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  return readModel.contextNamesRedacted && readModel.imageNamesRedacted && readModel.containerIdsRedacted;
}

function linuxDockerClaimsRemainScoped(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  return (
    !readModel.adapterDispatchClaimed &&
    !readModel.containerPolicyClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed
  );
}

function linuxDockerRequiredProofsArePresent(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  return (
    readModel.proofRefs.includes('linux-docker-host-preflight-ref') &&
    readModel.openGaps.includes('linux-container-policy-not-proved') &&
    readModel.openGaps.includes('linux-platform-enforcement-not-proved') &&
    readModel.openGaps.includes('linux-child-device-delivery-not-proved')
  );
}

function linuxDockerStateIsConsistent(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  if (readModel.preflightState === 'docker-cli-unavailable') {
    return !readModel.dockerCliObserved && !readModel.dockerDaemonObserved;
  }
  if (readModel.preflightState === 'docker-cli-visible-daemon-unavailable') {
    return readModel.dockerCliObserved && !readModel.dockerDaemonObserved;
  }
  return readModel.dockerCliObserved && readModel.dockerDaemonObserved;
}

function linuxDockerInventoriesAreConsistent(readModel: AppGameLinuxDockerHostPreflightCandidate): boolean {
  return (
    readModel.contextInventoryState ===
      linuxDockerInventoryState(readModel.dockerCliObserved, readModel.contextCount) &&
    readModel.imageInventoryState === linuxDockerInventoryState(readModel.dockerDaemonObserved, readModel.imageCount) &&
    readModel.containerInventoryState ===
      linuxDockerInventoryState(readModel.dockerDaemonObserved, readModel.containerCount)
  );
}
