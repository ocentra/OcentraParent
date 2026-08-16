/* generated from crates/network-core/src/network_control_catalog.ts.txt */

import {
  NetworkControlCapabilitySeeds,
  NetworkControlCatalogSettingSeeds,
  NetworkControlCatalogSourceDocuments,
} from './network-control-catalog-data';
import {
  capabilityStateFromSourceState,
  explicitOptionLabels as networkExplicitOptionLabels,
  policyLaneFor as networkPolicyLaneFor,
  questionFromSourceText as networkQuestionFromSourceText,
} from './network-control-catalog-metadata';
import {
  NetworkControlCapabilityIdSchema,
  NetworkControlCapabilitySchema,
  NetworkControlCatalogIdSchema,
  NetworkControlCatalogSchema,
  NetworkControlEffectivePolicySchema,
  NetworkControlSettingIdSchema,
  NetworkControlPolicyValueSchema,
  NetworkControlUpdateCommandSchema,
  type NetworkControlCapability,
  type NetworkControlCatalog,
  type NetworkControlEffectivePolicy,
  type NetworkControlPolicyValue,
} from './network-control-catalog-schema';
import { ParentContractSchemaVersion } from './family-reference-primitives';
import {
  NetworkControlEffectModeOptions,
  NetworkControlTargetScopeOptions,
  buildTabs,
  networkControlCatalogAcceptedOptionCountFromSeeds,
} from './network-control-catalog-builders';

export const NetworkControlProposalSettingCount = NetworkControlCatalogSettingSeeds.filter(
  (seed) => seed[0] === 'docs/network-control-schema-proposal.md'
).length;

export const NetworkControlGuideSettingCount = NetworkControlCatalogSettingSeeds.filter(
  (seed) => seed[0] === 'docs/network-control-capability-guide.md'
).length;

export const NetworkControlSourceOptionCount = NetworkControlCatalogSettingSeeds.reduce(
  (count, seed) => count + seed[15].length,
  0
);

export const policyLaneFor = networkPolicyLaneFor;
export const questionFromSourceText = networkQuestionFromSourceText;
export const explicitOptionLabels = networkExplicitOptionLabels;

export const NetworkControlCapabilities: readonly NetworkControlCapability[] = NetworkControlCapabilitySeeds.map(
  (seed) =>
    NetworkControlCapabilitySchema.parse({
      capabilityId: NetworkControlCapabilityIdSchema.parse(seed[0]),
      state: capabilityStateFromSourceState(seed[1]),
      sourceState: seed[1],
      proof: seed[2],
      affectsSettings: seed[3].map((settingId) => NetworkControlSettingIdSchema.parse(settingId)),
    })
);

export const BaselineNetworkControlCatalog: NetworkControlCatalog = NetworkControlCatalogSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  catalogId: NetworkControlCatalogIdSchema.parse('network-control-catalog-v1'),
  sidePanelCategory: 'network',
  sourceDocuments: [...NetworkControlCatalogSourceDocuments],
  settingCount: NetworkControlCatalogSettingSeeds.length,
  acceptedOptionCount: networkControlCatalogAcceptedOptionCountFromSeeds(NetworkControlCatalogSettingSeeds),
  targetScopeOptions: NetworkControlTargetScopeOptions,
  effectModeOptions: NetworkControlEffectModeOptions,
  tabs: buildTabs(NetworkControlCatalogSettingSeeds),
});

export function networkControlCatalogSettings(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}

export function networkControlCatalogSettingCount(catalog = BaselineNetworkControlCatalog) {
  return networkControlCatalogSettings(catalog).length;
}

export function networkControlCatalogSectionCount(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.reduce((count, tab) => count + tab.sections.length, 0);
}

export function networkControlCatalogGroupCount(catalog = BaselineNetworkControlCatalog) {
  return catalog.tabs.reduce(
    (count, tab) => count + tab.sections.reduce((sectionCount, section) => sectionCount + section.groups.length, 0),
    0
  );
}

export function networkControlCatalogAcceptedOptionCount(catalog = BaselineNetworkControlCatalog) {
  return networkControlCatalogSettings(catalog).reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

export function networkControlCatalogSourceOptionCount() {
  return NetworkControlSourceOptionCount;
}

export function networkControlCatalogCanRender(catalog = BaselineNetworkControlCatalog) {
  if (catalog.sidePanelCategory !== 'network' || catalog.tabs.length === 0) {
    return false;
  }
  return catalog.tabs.every((tab) =>
    tab.sections.every((section) =>
      section.groups.every((group) =>
        group.settings.every(
          (setting) =>
            setting.policyLane.length > 0 &&
            setting.controlKind.length > 0 &&
            setting.cardKind.length > 0 &&
            setting.layoutHints.preferredColumnSpan > 0 &&
            setting.targetScopeOptions.length > 0 &&
            setting.effectModeOptions.length > 0 &&
            setting.capabilityRequirement.length > 0 &&
            setting.unsafeOrUnsupportedFallback.length > 0
        )
      )
    )
  );
}

export function decodeNetworkControlCatalog(input: unknown) {
  return NetworkControlCatalogSchema.parse(input);
}

export function decodeNetworkControlCapability(input: unknown) {
  return NetworkControlCapabilitySchema.parse(input);
}

export function decodeNetworkControlPolicyValue(input: unknown) {
  return NetworkControlPolicyValueSchema.parse(input);
}

export function decodeNetworkControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): NetworkControlPolicyValue {
  const parsed = decodeNetworkControlPolicyValue(input);
  const knownSettingIds = new Set(networkControlCatalogSettings(catalog).map((setting) => String(setting.settingId)));
  const seenSettingIds = new Set<string>();
  for (const setting of parsed.settings) {
    const settingId = String(setting.settingId);
    if (!knownSettingIds.has(settingId)) {
      throw new Error(`Unknown network control setting id: ${settingId}`);
    }
    if (seenSettingIds.has(settingId)) {
      throw new Error(`Duplicate network control setting id: ${settingId}`);
    }
    seenSettingIds.add(settingId);
  }
  return parsed;
}

export function decodeNetworkControlEffectivePolicy(input: unknown) {
  return NetworkControlEffectivePolicySchema.parse(input);
}

export function decodeNetworkControlUpdateCommand(input: unknown) {
  return NetworkControlUpdateCommandSchema.parse(input);
}

export function decodeNetworkControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): ReturnType<typeof decodeNetworkControlUpdateCommand> {
  const parsed = decodeNetworkControlUpdateCommand(input);
  const writesToPaths = new Set(networkControlCatalogSettings(catalog).map((setting) => String(setting.writesTo)));
  for (const patch of parsed.patch) {
    const path = String(patch.path);
    if (!writesToPaths.has(path)) {
      throw new Error(`Unknown network control writesTo path: ${path}`);
    }
  }
  return parsed;
}

export function buildNetworkControlEffectivePolicyPlan(
  policy: NetworkControlPolicyValue,
  catalog = BaselineNetworkControlCatalog
): NetworkControlEffectivePolicy['plans'] {
  const settingMetadata = new Map(
    networkControlCatalogSettings(catalog).map((setting) => [String(setting.settingId), setting] as const)
  );
  return policy.settings.map((policySetting) => {
    const setting = settingMetadata.get(String(policySetting.settingId));
    if (setting === undefined) {
      throw new Error(`Unknown network control setting id: ${String(policySetting.settingId)}`);
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
