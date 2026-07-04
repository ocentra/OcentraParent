import {
  NetworkControlCapabilitySeeds,
  NetworkControlCatalogSettingSeeds,
  NetworkControlCatalogSourceDocuments,
} from './network-control-catalog-data';
import { capabilityStateFromSourceState } from './network-control-catalog-metadata';
import {
  NetworkControlCapabilityIdSchema,
  NetworkControlCapabilitySchema,
  NetworkControlCatalogIdSchema,
  NetworkControlCatalogSchema,
  NetworkControlSettingIdSchema,
  type NetworkControlCapability,
  type NetworkControlCatalog,
  type NetworkControlEffectivePolicy,
  type NetworkControlPolicyValue,
} from './network-control-catalog-schema';
import { ParentContractSchemaVersion } from './family-reference-primitives';
import {
  NetworkControlEffectModeOptions as NetworkControlEffectModeOptionsLogic,
  NetworkControlTargetScopeOptions as NetworkControlTargetScopeOptionsLogic,
  buildTabs,
  networkControlCatalogAcceptedOptionCountFromSeeds,
} from './network-control-catalog-builders';
import {
  buildNetworkControlEffectivePolicyPlan as buildNetworkControlEffectivePolicyPlanLogic,
  decodeNetworkControlCatalog as decodeNetworkControlCatalogLogic,
  decodeNetworkControlCapability as decodeNetworkControlCapabilityLogic,
  decodeNetworkControlPolicyValue as decodeNetworkControlPolicyValueLogic,
  decodeNetworkControlPolicyValueForCatalog as decodeNetworkControlPolicyValueForCatalogLogic,
  decodeNetworkControlEffectivePolicy as decodeNetworkControlEffectivePolicyLogic,
  decodeNetworkControlUpdateCommand as decodeNetworkControlUpdateCommandLogic,
  decodeNetworkControlUpdateCommandForCatalog as decodeNetworkControlUpdateCommandForCatalogLogic,
  networkControlCatalogCanRender as networkControlCatalogCanRenderLogic,
} from './network-control-catalog-validations';

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

export const NetworkControlTargetScopeOptions = NetworkControlTargetScopeOptionsLogic;
export const NetworkControlEffectModeOptions = NetworkControlEffectModeOptionsLogic;

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
  return networkControlCatalogCanRenderLogic(catalog);
}

export function decodeNetworkControlCatalog(input: unknown) {
  return decodeNetworkControlCatalogLogic(input);
}

export function decodeNetworkControlCapability(input: unknown) {
  return decodeNetworkControlCapabilityLogic(input);
}

export function decodeNetworkControlPolicyValue(input: unknown) {
  return decodeNetworkControlPolicyValueLogic(input);
}

export function decodeNetworkControlPolicyValueForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): NetworkControlPolicyValue {
  return decodeNetworkControlPolicyValueForCatalogLogic(input, catalog);
}

export function decodeNetworkControlEffectivePolicy(input: unknown) {
  return decodeNetworkControlEffectivePolicyLogic(input);
}

export function decodeNetworkControlUpdateCommand(input: unknown) {
  return decodeNetworkControlUpdateCommandLogic(input);
}

export function decodeNetworkControlUpdateCommandForCatalog(
  input: unknown,
  catalog = BaselineNetworkControlCatalog
): ReturnType<typeof decodeNetworkControlUpdateCommandLogic> {
  return decodeNetworkControlUpdateCommandForCatalogLogic(input, catalog);
}

export function buildNetworkControlEffectivePolicyPlan(
  policy: NetworkControlPolicyValue,
  catalog = BaselineNetworkControlCatalog
): NetworkControlEffectivePolicy['plans'] {
  return buildNetworkControlEffectivePolicyPlanLogic(policy, catalog);
}
