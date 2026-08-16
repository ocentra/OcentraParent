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
  const contractsPath = path.join(repoRoot, 'packages', 'logging-domain', 'src', 'logging-contracts.ts');
  const portalLoggerPath = path.join(repoRoot, 'packages', 'portal-domain', 'src', 'dev-logger.ts');
  const portalLoggerImplementationPath = path.join(repoRoot, 'packages', 'portal-domain', 'src', 'dev-logger-impl.ts');
  const portalLoggerWrapperPath = path.join(repoRoot, 'apps', 'portal', 'src', 'dev-logger.ts');
  const bridgeServerPath = path.join(repoRoot, 'packages', 'logging-domain', 'src', 'transport', 'bridgeServer.ts');
  const cargoPath = path.join(repoRoot, 'crates', 'agent-service', 'Cargo.toml');
  const devLogPath = path.join(repoRoot, 'crates', 'agent-service', 'src', 'dev_log.rs');
  const readmePath = path.join(repoRoot, 'packages', 'logging-domain', 'README.md');

  for (const filePath of [
    contractsPath,
    portalLoggerPath,
    portalLoggerImplementationPath,
    portalLoggerWrapperPath,
    bridgeServerPath,
    cargoPath,
    devLogPath,
    readmePath,
  ]) {
    ensure(fs.existsSync(filePath), `missing ${path.relative(repoRoot, filePath).replace(/\\/g, '/')}`);
  }

  const contractsText = readText(contractsPath);
  const portalLoggerText = readText(portalLoggerImplementationPath);
  const portalLoggerWrapperText = readText(portalLoggerWrapperPath);
  const bridgeServerText = readText(bridgeServerPath);
  const cargoText = readText(cargoPath);
  const devLogText = readText(devLogPath);
  const readmeText = readText(readmePath);

  const portalUsesSharedLoggerBridge =
    portalLoggerText.includes('portalLogger.register(context.moduleUrl)') &&
    portalLoggerText.includes('portalLogger.flush()') &&
    portalLoggerText.includes('sendPortalBridgeMessage(') &&
    portalLoggerText.includes('buildPortalLoggerConfiguration(');
  const portalWrapperDelegatesToPortalDomain =
    portalLoggerWrapperText.includes("from '@ocentra-parent/portal-domain/dev-logger'") &&
    portalLoggerWrapperText.includes('sendPortalDevLogWithContext') &&
    portalLoggerWrapperText.includes('sendPortalProofTraceLogWithContext');
  const portalUsesCompatibilityEndpoint =
    portalLoggerText.includes('DevLogEndpoint.Write') &&
    portalLoggerText.includes('resolvePortalCompatibilityUrl(') &&
    portalLoggerText.includes('sendPortalCompatibilityLog(');
  const portalPrefersBridgeBeforeCompatibility =
    /return\s*\(\s*\(await sendPortalBridgeMessage\([\s\S]*?\)\s*\|\|\s*\(await sendPortalCompatibilityLog\(/.test(
      portalLoggerText
    );
  const bridgeServerImplementsReceiver = bridgeServerText.includes("case '/__logs__':");

  if (contractsText.includes('DevLogEndpoint') && contractsText.includes('Write:')) {
    ensure(
      portalWrapperDelegatesToPortalDomain,
      'portal dev logger wrapper must delegate to the portal-domain dev logger implementation'
    );
    ensure(
      portalUsesSharedLoggerBridge && bridgeServerImplementsReceiver,
      'DevLogEndpoint.Write exists but portal dev logger does not route through the implemented bridge receiver'
    );
    ensure(
      portalUsesCompatibilityEndpoint && portalPrefersBridgeBeforeCompatibility,
      'portal dev logger implementation must keep the compatibility endpoint behind a bridge short-circuit fallback'
    );
  }

  ensure(
    !portalLoggerWrapperText.includes('DevLogEndpoint.Write') &&
      !portalLoggerWrapperText.includes('/__ocentra-parent-dev-log'),
    'portal dev logger wrapper must not post directly to the compatibility endpoint'
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
