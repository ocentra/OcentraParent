/* generated from crates/logging-core/src/parent_log_runtime.rs */

export function parseGeneratedBoolean(value: string | undefined, fallback: boolean): boolean {
  if (value == null) {
    return fallback;
  }
  const normalized = value.trim().toLowerCase();
  if (normalized === 'true' || normalized === '1' || normalized === 'yes' || normalized === 'on') {
    return true;
  }
  if (normalized === 'false' || normalized === '0' || normalized === 'no' || normalized === 'off') {
    return false;
  }
  return fallback;
}

export function parseGeneratedList(value: string | undefined): string[] {
  if (value == null || value.trim().length === 0) {
    return [];
  }
  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

export function parseGeneratedLevel(value: string | undefined, fallback = 'info'): string {
  if (value == null || value.trim().length === 0) {
    return fallback;
  }
  const normalized = value.trim().toLowerCase();
  return ['trace', 'debug', 'info', 'warn', 'error'].includes(normalized) ? normalized : fallback;
}

export function parseGeneratedBridgeMode(value: string | undefined): 'local' | 'tunnel' | 'disabled' {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'tunnel') {
    return 'tunnel';
  }
  if (normalized === 'disabled') {
    return 'disabled';
  }
  return 'local';
}

export function normalizeGeneratedDebugPath(value: string): string {
  return value.replace(/\\/g, '/').toLowerCase();
}
