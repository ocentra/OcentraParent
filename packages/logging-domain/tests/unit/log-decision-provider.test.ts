import { describe, expect, it } from 'vitest';
import { createParentLogConfig } from '../../src/core/logConfig';
import { createParentLogDecisionProvider } from '../../src/core/logDecisionProvider';

describe('log decision provider', () => {
  it('always stores error and warn even when low-level storage is disabled', () => {
    const provider = createParentLogDecisionProvider({
      NODE_ENV: 'production',
      OCENTRA_PARENT_LOG_ENABLED: 'false',
      OCENTRA_PARENT_LOG_STORE: 'false',
    });

    expect(provider.shouldStoreLog('portal', 'error')).toBe(true);
    expect(provider.shouldStoreLog('portal', 'warn')).toBe(true);
    expect(provider.shouldStoreLog('portal', 'info')).toBe(false);
  });

  it('keeps console and storage decisions separate', () => {
    const provider = createParentLogDecisionProvider({
      NODE_ENV: 'production',
      OCENTRA_PARENT_LOG_CONSOLE: 'false',
      OCENTRA_PARENT_LOG_STORE: 'true',
      OCENTRA_PARENT_DEBUG_SOURCES: 'portal',
    });

    expect(provider.shouldStoreLog('portal', 'debug')).toBe(true);
    expect(provider.shouldLogToConsole('portal', 'debug')).toBe(false);
  });

  it('supports source file and run level debug selection for low-level logs', () => {
    const provider = createParentLogDecisionProvider({
      NODE_ENV: 'production',
      OCENTRA_PARENT_LOG_LEVEL: 'error',
      OCENTRA_PARENT_DEBUG_SOURCES: 'agent-service',
      OCENTRA_PARENT_DEBUG_FILES: 'apps/portal/src/dev-logger.ts',
      OCENTRA_PARENT_DEBUG_RUNS: 'run-42',
    });

    expect(provider.shouldStoreLog('agent-service', 'debug')).toBe(true);
    expect(
      provider.shouldStoreLog('portal', 'debug', {
        filePath: 'apps/portal/src/dev-logger.ts',
      })
    ).toBe(true);
    expect(
      provider.shouldStoreLog('portal', 'debug', {
        runId: 'run-42',
      })
    ).toBe(true);
    expect(provider.shouldStoreLog('portal', 'debug')).toBe(false);
  });

  it('uses local bridge by default and only switches to tunnel when requested', () => {
    const localConfig = createParentLogConfig({
      NODE_ENV: 'development',
    });
    const tunnelConfig = createParentLogConfig({
      NODE_ENV: 'development',
      OCENTRA_PARENT_LOG_BRIDGE_MODE: 'tunnel',
      OCENTRA_PARENT_LOG_BRIDGE_URL: 'https://bridge.example.test',
      OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH: 'true',
    });

    expect(localConfig.bridgeMode).toBe('local');
    expect(localConfig.bridgeUrl).toBe('http://127.0.0.1:4479');
    expect(tunnelConfig.bridgeMode).toBe('tunnel');
    expect(tunnelConfig.bridgeUrl).toBe('https://bridge.example.test');
    expect(tunnelConfig.skipBridgeHealth).toBe(true);
  });

  it('fails closed for invalid sink, level, bridge mode, and bridge URL configuration', () => {
    const config = createParentLogConfig({
      NODE_ENV: 'development',
      OCENTRA_PARENT_LOG_ENABLED: 'maybe',
      OCENTRA_PARENT_LOG_CONSOLE: 'maybe',
      OCENTRA_PARENT_LOG_STORE: 'maybe',
      OCENTRA_PARENT_LOG_LEVEL: 'verbose',
      OCENTRA_PARENT_LOG_BRIDGE_MODE: 'remote',
      OCENTRA_PARENT_LOG_BRIDGE_URL: 'https://bridge.example.test/path',
    });

    expect(config.enabled).toBe(false);
    expect(config.consoleEnabled).toBe(false);
    expect(config.storeEnabled).toBe(false);
    expect(config.minLevel).toBe('error');
    expect(config.bridgeMode).toBe('disabled');
    expect(config.bridgeUrl).toBeNull();

    const invalidTunnel = createParentLogConfig({
      NODE_ENV: 'development',
      OCENTRA_PARENT_LOG_BRIDGE_MODE: 'tunnel',
      OCENTRA_PARENT_LOG_BRIDGE_URL: 'https://bridge.example.test/path',
    });
    expect(invalidTunnel.bridgeMode).toBe('tunnel');
    expect(invalidTunnel.bridgeUrl).toBeNull();
  });
});
