import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const TestDirectory = resolve(dirname(fileURLToPath(import.meta.url)), '..');

describe('browser managed and intervention evidence source shape', () => {
  it('keeps managed browser evidence separate from exact URL and unmanaged browser claims', () => {
    const browserStatusSource = readFileSync(resolve(TestDirectory, '..', 'src', 'browser-status-panel.ts'), 'utf8');
    const browserInterventionSource = readFileSync(
      resolve(TestDirectory, '..', 'src', 'browser-intervention-panel.ts'),
      'utf8'
    );

    expect(browserStatusSource).toContain('PortalDetails.ManagedSession');
    expect(browserStatusSource).toContain('PortalDetails.Profile');
    expect(browserStatusSource).toContain('PortalDetails.Bridge');
    expect(browserStatusSource).toContain('PortalDetails.LastObserved');
    expect(browserStatusSource).not.toContain('PortalDetails.ExactUrlClaim');
    expect(browserStatusSource).not.toContain('PortalDetails.Url');

    expect(browserInterventionSource).toContain('PortalDetails.ExactUrlClaim');
    expect(browserInterventionSource).toContain('PortalDetails.UnmanagedBrowserEnforcement');
    expect(browserInterventionSource).toContain('PortalDetails.UnmanagedDetection');
    expect(browserInterventionSource).toContain('PortalDetails.UnmanagedFallbackAction');
    expect(browserInterventionSource).toContain('PortalDetails.Url');
    expect(browserInterventionSource).toContain('PortalDetails.ProcessId');
  });

  it('keeps the browser intervention empty state explicit when the read model is absent', () => {
    const browserInterventionSource = readFileSync(
      resolve(TestDirectory, '..', 'src', 'browser-intervention-panel.ts'),
      'utf8'
    );
    const browserStatusSource = readFileSync(resolve(TestDirectory, '..', 'src', 'browser-status-panel.ts'), 'utf8');

    expect(browserInterventionSource).toContain('NoBrowserIntervention');
    expect(browserStatusSource).toContain('NoBrowserManagedStatus');
  });
});
