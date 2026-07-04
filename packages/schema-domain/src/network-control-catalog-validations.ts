import {
  NetworkControlCapabilitySchema,
  NetworkControlCatalogSchema,
  NetworkControlEffectivePolicySchema,
  NetworkControlPolicyValueSchema,
  NetworkControlUpdateCommandSchema,
  type NetworkControlCatalog,
  type NetworkControlEffectivePolicy,
  type NetworkControlPolicyValue,
  type NetworkControlUpdateCommand,
} from './network-control-catalog-schema';

export function networkControlCatalogCanRender(catalog: NetworkControlCatalog) {
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
  catalog: NetworkControlCatalog
): NetworkControlPolicyValue {
  const parsed = decodeNetworkControlPolicyValue(input);
  const knownSettingIds = new Set(catalogSettings(catalog).map((setting) => String(setting.settingId)));
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
  catalog: NetworkControlCatalog
): NetworkControlUpdateCommand {
  const parsed = decodeNetworkControlUpdateCommand(input);
  const writesToPaths = new Set(catalogSettings(catalog).map((setting) => String(setting.writesTo)));
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
  catalog: NetworkControlCatalog
): NetworkControlEffectivePolicy['plans'] {
  const settingMetadata = new Map(
    catalogSettings(catalog).map((setting) => [String(setting.settingId), setting] as const)
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

function catalogSettings(catalog: NetworkControlCatalog) {
  return catalog.tabs.flatMap((tab) =>
    tab.sections.flatMap((section) => section.groups.flatMap((group) => group.settings))
  );
}
