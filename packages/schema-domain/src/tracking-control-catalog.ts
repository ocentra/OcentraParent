/* generated from crates/tracking-core/src/tracking_control_catalog.ts.txt */

import {
  buildTrackingControlCatalogTabs,
  buildTrackingControlOptionLabels,
  countTrackingControlAcceptedOptionsFromSeeds,
} from './tracking-control-catalog-build';
import {
  TrackingControlCapabilitySeeds,
  TrackingControlCatalogEffectModeLabels,
  TrackingControlCatalogSettingSeeds,
  TrackingControlCatalogSourceDocuments,
  TrackingControlCatalogTargetScopeLabels,
} from './tracking-control-catalog-data';
import { capabilityStateFromSourceState } from './tracking-control-catalog-metadata';
import {
  TrackingControlCapabilityIdSchema,
  TrackingControlCapabilitySchema,
  TrackingControlCatalogIdSchema,
  TrackingControlCatalogSchema,
  TrackingControlEffectivePolicySchema,
  TrackingControlPolicyValueSchema,
  TrackingControlSettingIdSchema,
  TrackingControlUpdateCommandSchema,
  type TrackingControlCapability,
  type TrackingControlCatalog,
  type TrackingControlCatalogGroup,
  type TrackingControlCatalogSection,
  type TrackingControlCatalogSetting,
  type TrackingControlCatalogTab,
  type TrackingControlEffectivePolicy,
  type TrackingControlPolicyValue,
  type TrackingControlUpdateCommand,
} from './tracking-control-catalog-schema';
import { ParentContractSchemaVersion } from './family-reference-primitives';

export {
  TrackingControlCapabilitySchema,
  TrackingControlCatalogSchema,
  TrackingControlEffectivePolicySchema,
  TrackingControlPolicyValueSchema,
  TrackingControlUpdateCommandSchema,
};
export type {
  TrackingControlCapability,
  TrackingControlCatalog,
  TrackingControlCatalogGroup,
  TrackingControlCatalogSection,
  TrackingControlCatalogSetting,
  TrackingControlCatalogTab,
  TrackingControlEffectivePolicy,
  TrackingControlPolicyValue,
  TrackingControlUpdateCommand,
};

const TrackingControlDefaultPostureSettingId = 'location.defaultPosture';
const TrackingControlTemporaryLiveCompanionSettings = [
  { purpose: 'duration', settingId: 'live.maxSessionMinutes' },
  { purpose: 'fallback', settingId: 'permissions.whenPermissionMissing' },
  {
    purpose: 'custody',
    settingId: 'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196',
  },
  {
    purpose: 'audit',
    settingId: 'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191',
  },
] as const;

export const TrackingControlCapabilities: readonly TrackingControlCapability[] = TrackingControlCapabilitySeeds.map(
  (seed) =>
    TrackingControlCapabilitySchema.parse({
      capabilityId: TrackingControlCapabilityIdSchema.parse(seed[0]),
      state: capabilityStateFromSourceState(seed[1]),
      sourceState: seed[1],
      proof: seed[2],
      affectsSettings: seed[3].map((settingId) => TrackingControlSettingIdSchema.parse(settingId)),
    })
);

const TrackingControlTargetScopeOptions = optionLabels(
  'tracking-control.target-scope',
  TrackingControlCatalogTargetScopeLabels
);

const TrackingControlEffectModeOptions = optionLabels(
  'tracking-control.effect-mode',
  TrackingControlCatalogEffectModeLabels
);

export const BaselineTrackingControlCatalog: TrackingControlCatalog = TrackingControlCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: TrackingControlCatalogIdSchema.parse('tracking-control-full-catalog-v1'),
  sidePanelCategory: 'tracking',
  sourceDocuments: [...TrackingControlCatalogSourceDocuments],
  settingCount: TrackingControlCatalogSettingSeeds.length,
  acceptedOptionCount: countTrackingControlAcceptedOptionsFromSeeds(
    TrackingControlCatalogSettingSeeds,
    TrackingControlTargetScopeOptions,
    TrackingControlEffectModeOptions
  ),
  targetScopeOptions: TrackingControlTargetScopeOptions,
  effectModeOptions: TrackingControlEffectModeOptions,
  tabs: buildTrackingControlCatalogTabs(
    TrackingControlCatalogSettingSeeds,
    TrackingControlTargetScopeOptions,
    TrackingControlEffectModeOptions
  ),
});

export function trackingControlCatalogSettings(catalog = BaselineTrackingControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function decodeTrackingControlCatalog(input: unknown) {
  return TrackingControlCatalogSchema.parse(input);
}

export function decodeTrackingControlPolicyValue(input: unknown) {
  return TrackingControlPolicyValueSchema.parse(input);
}

export function decodeTrackingControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineTrackingControlCatalog
): TrackingControlPolicyValue {
  const parsed = decodeTrackingControlPolicyValue(input);
  const knownSettingIds = new Set(trackingControlCatalogSettings(catalog).map((setting) => String(setting.settingId)));
  const seenSettingIds = new Set<string>();

  for (const setting of parsed.settings) {
    const settingId = String(setting.settingId);
    if (!knownSettingIds.has(settingId)) {
      throw new Error(`Unknown tracking control setting id: ${settingId}`);
    }
    if (seenSettingIds.has(settingId)) {
      throw new Error(`Duplicate tracking control setting id: ${settingId}`);
    }
    seenSettingIds.add(settingId);
  }

  assertTrackingControlTemporaryLiveCompanionSettings(parsed.settings);
  return parsed;
}

export function decodeTrackingControlEffectivePolicy(input: unknown) {
  return TrackingControlEffectivePolicySchema.parse(input);
}

export function decodeTrackingControlUpdateCommand(input: unknown) {
  return TrackingControlUpdateCommandSchema.parse(input);
}

export function decodeTrackingControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineTrackingControlCatalog
): TrackingControlUpdateCommand {
  const parsed = decodeTrackingControlUpdateCommand(input);
  const writesToPaths = new Set(trackingControlCatalogSettings(catalog).map((setting) => String(setting.writesTo)));

  for (const patch of parsed.patch) {
    const path = String(patch.path);
    if (!writesToPaths.has(path)) {
      throw new Error(`Unknown tracking control writesTo path: ${path}`);
    }
  }

  return parsed;
}

export function buildTrackingControlEffectivePolicyPlan(
  policy: TrackingControlPolicyValue,
  catalog = BaselineTrackingControlCatalog
): TrackingControlEffectivePolicy['plans'] {
  assertTrackingControlTemporaryLiveCompanionSettings(policy.settings);
  const settingMetadata = new Map(
    trackingControlCatalogSettings(catalog).map((setting) => [String(setting.settingId), setting])
  );

  return policy.settings.map((policySetting) => {
    const setting = settingMetadata.get(String(policySetting.settingId));
    if (setting === undefined) {
      throw new Error(`Unknown tracking control setting id: ${String(policySetting.settingId)}`);
    }

    return {
      settingId: setting.settingId,
      writesTo: setting.writesTo,
      effectStatus: setting.effectStatus,
      runtimeOwner: setting.runtimeOwner,
      capabilityState: setting.capabilityState,
      fallback: setting.unsafeOrUnsupportedFallback,
    };
  });
}

function optionLabels(settingId: string, labels: readonly string[]) {
  return buildTrackingControlOptionLabels(settingId, labels);
}

function assertTrackingControlTemporaryLiveCompanionSettings(settings: TrackingControlPolicyValue['settings']) {
  const settingsById = new Map(settings.map((setting) => [String(setting.settingId), setting.value]));
  if (settingsById.get(TrackingControlDefaultPostureSettingId) !== 'temporary-live') {
    return;
  }

  const missing = TrackingControlTemporaryLiveCompanionSettings.filter(
    ({ settingId }) => !settingsById.has(settingId)
  ).map(({ purpose, settingId }) => `${purpose}=${settingId}`);

  if (missing.length > 0) {
    throw new Error(`Temporary live posture requires companion tracking control settings: ${missing.join(', ')}`);
  }
}
