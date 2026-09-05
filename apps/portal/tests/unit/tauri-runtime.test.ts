import { afterEach, describe, expect, it } from 'vitest';
import { isParentTauriRuntime } from '../../src/tauri-runtime';

const TauriRuntimeMarker = 'isTauri';
const originalTauriRuntimeMarker = Object.getOwnPropertyDescriptor(globalThis, TauriRuntimeMarker);

afterEach(() => {
  if (originalTauriRuntimeMarker === undefined) {
    Reflect.deleteProperty(globalThis, TauriRuntimeMarker);
    return;
  }
  Object.defineProperty(globalThis, TauriRuntimeMarker, originalTauriRuntimeMarker);
});

describe('Tauri runtime detection', () => {
  it('recognizes the official Tauri v2 runtime marker without requiring the legacy internals key', () => {
    Object.defineProperty(globalThis, TauriRuntimeMarker, {
      configurable: true,
      value: true,
    });

    expect(isParentTauriRuntime()).toBe(true);
  });
});
