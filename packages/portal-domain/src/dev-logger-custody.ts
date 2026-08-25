import type { GeneratedLogFields as LogFields } from '@ocentra-parent/logging-domain/generated/logging-contracts';
import { redactStructuredLogValue, redactUnstructuredLogText } from '@ocentra-parent/logging-domain/core/log-redaction';
import type { PortalLoggerRuntime } from './dev-logger-contracts';

const MaximumPortalMessageBytes = 16 * 1024;
const MaximumPortalCompatibilityBodyBytes = 256 * 1024;
const portalQueueStorageAdapters = new WeakMap<object, PortalQueueStorage>();

export interface PortalQueueStorage {
  readonly durability: 'persistent';
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

interface PortalHostStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export function resolvePortalQueueStorage(runtime: PortalLoggerRuntime): PortalQueueStorage | null {
  try {
    const storage = runtime['localStorage'] as Partial<PortalHostStorage> | null | undefined;
    if (!isPortalHostStorage(storage)) {
      return null;
    }
    const hostStorage = storage as PortalHostStorage & object;
    const existing = portalQueueStorageAdapters.get(hostStorage);
    if (existing != null) {
      return existing;
    }
    const adapter: PortalQueueStorage = {
      durability: 'persistent',
      getItem: (key) => hostStorage.getItem(key),
      setItem: (key, value) => hostStorage.setItem(key, value),
      removeItem: (key) => hostStorage.removeItem(key),
    };
    portalQueueStorageAdapters.set(hostStorage, adapter);
    return adapter;
  } catch {
    return null;
  }
}

export function custodiedPortalLogFields(fields: LogFields): LogFields | null {
  const sanitized = redactStructuredLogValue(fields);
  return typeof sanitized === 'object' && sanitized != null && !Array.isArray(sanitized)
    ? (sanitized as LogFields)
    : null;
}

export function custodiedPortalLogMessage(message: string): string {
  if (message.length > MaximumPortalMessageBytes || utf8BytesExceed(message, MaximumPortalMessageBytes)) {
    throw new Error('portal log message exceeds its custody limit');
  }
  return redactUnstructuredLogText(message);
}

export function custodiedPortalCompatibilityBody(entry: object): string | null {
  const body = JSON.stringify(entry);
  return body.length > MaximumPortalCompatibilityBodyBytes || utf8BytesExceed(body, MaximumPortalCompatibilityBodyBytes)
    ? null
    : body;
}

function isPortalHostStorage(storage: Partial<PortalHostStorage> | null | undefined): storage is PortalHostStorage {
  return (
    storage != null &&
    typeof storage.getItem === 'function' &&
    typeof storage.setItem === 'function' &&
    typeof storage.removeItem === 'function'
  );
}

function utf8BytesExceed(value: string, maximumBytes: number): boolean {
  let bytes = 0;
  for (const character of value) {
    const codePoint = character.codePointAt(0) ?? 0;
    bytes += codePoint <= 0x7f ? 1 : codePoint <= 0x7ff ? 2 : codePoint <= 0xffff ? 3 : 4;
    if (bytes > maximumBytes) {
      return true;
    }
  }
  return false;
}
