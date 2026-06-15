#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

function parseRepoRoot(argv) {
  const flag = argv.find((value) => value.startsWith('--root='));
  return path.resolve(flag == null ? process.cwd() : flag.slice('--root='.length));
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`dev log routing failed: ${message}`);
  }
}

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function main() {
  const repoRoot = parseRepoRoot(process.argv.slice(2));
  const contractsPath = path.join(repoRoot, 'packages', 'logging-domain', 'src', 'contracts.ts');
  const portalLoggerPath = path.join(repoRoot, 'apps', 'portal', 'src', 'dev-logger.ts');
  const bridgeServerPath = path.join(repoRoot, 'packages', 'logging-domain', 'src', 'transport', 'bridgeServer.ts');
  const cargoPath = path.join(repoRoot, 'crates', 'agent-service', 'Cargo.toml');
  const devLogPath = path.join(repoRoot, 'crates', 'agent-service', 'src', 'dev_log.rs');
  const readmePath = path.join(repoRoot, 'packages', 'logging-domain', 'README.md');

  for (const filePath of [contractsPath, portalLoggerPath, bridgeServerPath, cargoPath, devLogPath, readmePath]) {
    ensure(fs.existsSync(filePath), `missing ${path.relative(repoRoot, filePath).replace(/\\/g, '/')}`);
  }

  const contractsText = readText(contractsPath);
  const portalLoggerText = readText(portalLoggerPath);
  const bridgeServerText = readText(bridgeServerPath);
  const cargoText = readText(cargoPath);
  const devLogText = readText(devLogPath);
  const readmeText = readText(readmePath);

  const portalUsesDirectBridgeTransport = portalLoggerText.includes('sendToBridge(');
  const portalUsesSharedLoggerBridge =
    portalLoggerText.includes('sendPortalLoggerMessage(') &&
    portalLoggerText.includes('portalLogger.register(import.meta.url)') &&
    portalLoggerText.includes('portalLogger.flush()');
  const portalUsesImplementedBridgeReceiver =
    (portalUsesDirectBridgeTransport || portalUsesSharedLoggerBridge) &&
    bridgeServerText.includes("url.pathname === '/__logs__'");

  if (contractsText.includes('DevLogEndpoint') && contractsText.includes('Write:')) {
    ensure(
      portalUsesImplementedBridgeReceiver,
      'DevLogEndpoint.Write exists but portal dev logger does not route through the implemented bridge receiver'
    );
  }

  ensure(
    !portalLoggerText.includes('DevLogEndpoint.Write') &&
      !portalLoggerText.includes('/__ocentra-parent-dev-log') &&
      (portalUsesDirectBridgeTransport || portalUsesSharedLoggerBridge),
    'portal dev logger must not post to an unimplemented endpoint'
  );

  ensure(
    cargoText.includes('ocentra-parent-logging-core'),
    'crates/agent-service must depend on ocentra-parent-logging-core after logging-core migration'
  );
  ensure(
    devLogText.includes('ocentra_parent_logging_core') &&
      !devLogText.includes('OpenOptions') &&
      !devLogText.includes('File::create') &&
      !devLogText.includes('write_all('),
    'crates/agent-service must delegate dev logging to logging-core instead of keeping an ad hoc file writer'
  );

  ensure(
    readmeText.includes('/api/dev/log-snapshot') &&
      readmeText.includes('snapshot/status endpoint') &&
      readmeText.includes('not the primary local log store'),
    '/api/dev/log-snapshot must be documented as a snapshot/status endpoint, not the primary logging store'
  );

  process.stdout.write('dev log routing checks passed.\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
}
