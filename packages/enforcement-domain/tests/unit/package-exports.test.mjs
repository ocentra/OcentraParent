import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const packageJson = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'));

describe('enforcement domain package exports', () => {
  it('removes the wildcard and keeps the surviving enforcement surface explicit', () => {
    expect(Object.keys(packageJson.exports).sort()).toEqual([
      './package-info',
      './tamper-uninstall-artifact-status-read-model',
      './v0-8-browser-enforcement-timer-recovery-proof-values',
    ]);
    expect(packageJson.exports['./*']).toBeUndefined();
    expect(packageJson.exports['./tamper-uninstall-artifact-status-read-model']).toEqual({
      import: './dist/src/tamper-uninstall-artifact-status-read-model.js',
      types: './dist/src/tamper-uninstall-artifact-status-read-model.d.ts',
    });
    expect(packageJson.exports['./v0-8-browser-enforcement-timer-recovery-proof-values']).toEqual({
      import: './dist/src/v0-8-browser-enforcement-timer-recovery-proof-values.js',
      types: './dist/src/v0-8-browser-enforcement-timer-recovery-proof-values.d.ts',
    });
    expect(packageJson.exports['./package-info']).toEqual({
      import: './dist/src/package-info.js',
      types: './dist/src/package-info.d.ts',
    });
    expect(packageJson.exports['./enforcement']).toBeUndefined();
    expect(packageJson.exports['./enforcement-host-adapter-preflight']).toBeUndefined();
    expect(packageJson.exports['./enforcement-os-adapter-product-proof']).toBeUndefined();
    expect(packageJson.exports['./enforcement-policy-dispatch']).toBeUndefined();
    expect(packageJson.exports['./enforcement-process-package-identity']).toBeUndefined();
    expect(packageJson.exports['./enforcement-proof-shape']).toBeUndefined();
    expect(packageJson.exports['./enforcement-readiness']).toBeUndefined();
    expect(packageJson.exports['./tamper-uninstall-artifact-status']).toBeUndefined();
    expect(packageJson.exports['./v0-8-broad-os-adapter-proof']).toBeUndefined();
    expect(packageJson.exports['./v0-8-broad-os-adapter-runtime-proof']).toBeUndefined();
    expect(packageJson.exports['./v0-8-browser-enforcement-timer-recovery-proof']).toBeUndefined();
    expect(packageJson.exports['./v0-8-cross-platform-enforcement-capability-proof']).toBeUndefined();
    expect(packageJson.exports['./v0-8-enforcement-integrity-runtime-audit']).toBeUndefined();
    expect(packageJson.exports['./v0-8-enforcement-product-control-spine']).toBeUndefined();
    expect(packageJson.exports['./v0-8-integrity-alert-status-bridge']).toBeUndefined();
    expect(packageJson.exports['./v0-8-os-adapter-manual-artifact-gates']).toBeUndefined();
    expect(packageJson.exports['./v0-8-supported-adapter-runtime-proof']).toBeUndefined();
  });
});
