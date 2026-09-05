import { isTauri } from '@tauri-apps/api/core';
import { ParentHostBridgeRuntime } from '../generated/parent-ui-bridge';

export function isParentTauriRuntime(): boolean {
  if (isTauri()) {
    return true;
  }
  return (
    typeof window !== ParentHostBridgeRuntime.TypeofUndefined &&
    ParentHostBridgeRuntime.TauriInternalWindowKey in window
  );
}
