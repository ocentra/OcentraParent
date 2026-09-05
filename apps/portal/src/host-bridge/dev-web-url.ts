import { ParentHostBridgeRuntime, type ParentDevBridgeUrl } from '../../generated/parent-ui-bridge';

export function resolveParentDevBridgeUrl(): ParentDevBridgeUrl | null {
  const value = import.meta.env[ParentHostBridgeRuntime.DevBridgeUrlEnvKey];
  return typeof value === ParentHostBridgeRuntime.StringType && value.trim().length > 0 ? value.trim() : null;
}

export function trimTrailingSlash(value: ParentDevBridgeUrl): ParentDevBridgeUrl {
  return value.endsWith(ParentHostBridgeRuntime.UrlPathSeparator) ? value.slice(0, -1) : value;
}
