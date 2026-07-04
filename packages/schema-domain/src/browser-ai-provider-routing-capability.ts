export interface BrowserAiProviderCapability {
  readonly providerKind: string;
  readonly parentApprovedRemoteEnabled: boolean;
  readonly canRunOnChildDevice: boolean;
  readonly localOnly: boolean;
  readonly noRetention: boolean;
  readonly capabilityState: string;
  readonly modelRuntimeRef: string | null;
  readonly degradedStates: readonly string[];
  readonly unavailableReason: string | null;
}

export function browserAiProviderCapabilityIsConsistent(value: BrowserAiProviderCapability) {
  if (value.providerKind !== 'child-device-local-ai') {
    return value.parentApprovedRemoteEnabled && value.canRunOnChildDevice === false;
  }
  if (!value.localOnly || value.parentApprovedRemoteEnabled || !value.canRunOnChildDevice || !value.noRetention) {
    return false;
  }
  if (value.capabilityState === 'available') {
    return value.modelRuntimeRef !== null && value.degradedStates.length === 0 && value.unavailableReason === null;
  }
  return value.modelRuntimeRef === null && value.degradedStates.length > 0 && value.unavailableReason !== null;
}
