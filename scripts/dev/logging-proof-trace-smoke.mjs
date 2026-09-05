#!/usr/bin/env node

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const generatedLoggingContractsArtifact = path.join(
  workspaceRoot,
  'packages/schema-domain/dist/generated-logging-contracts.js'
);
const contractBuildInfoFiles = [
  'packages/schema-domain/tsconfig.tsbuildinfo',
  'packages/logging-domain/tsconfig.tsbuildinfo',
  'packages/portal-domain/tsconfig.tsbuildinfo',
].map((relativePath) => path.join(workspaceRoot, relativePath));
const expectedSteps = ['portal.route.opened', 'portal.action.clicked', 'portal.ui.rendered'];

function optionValue(argv, name) {
  const prefix = `${name}=`;
  const exactIndex = argv.indexOf(name);
  if (exactIndex !== -1) {
    return argv[exactIndex + 1] ?? null;
  }
  const inline = argv.find((value) => value.startsWith(prefix));
  return inline == null ? null : inline.slice(prefix.length);
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`logging proof-trace smoke failed: ${message}`);
  }
}

function ensureSafeRoot(rootDir) {
  const resolvedRoot = path.resolve(rootDir);
  const allowedRoots = [
    path.resolve(os.tmpdir()),
    path.resolve(path.join(workspaceRoot, 'test-results')),
    path.resolve(path.join(workspaceRoot, 'output', 'logging-domain-parity-proof')),
  ];
  const allowed = allowedRoots.some((allowedRoot) => resolvedRoot.startsWith(allowedRoot));
  ensure(
    allowed,
    `smoke root must stay inside temp, test-results, or output/logging-domain-parity-proof: ${resolvedRoot}`
  );
}

function makeTempDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-proof-trace-smoke-'));
}

function removeDir(targetPath) {
  fs.rmSync(targetPath, { force: true, recursive: true });
}

function ensureGeneratedLoggingContractsArtifact() {
  if (!fs.existsSync(generatedLoggingContractsArtifact)) {
    for (const buildInfoFile of contractBuildInfoFiles) {
      fs.rmSync(buildInfoFile, { force: true });
    }
    const result = spawnSync('npm', ['run', 'build:contracts'], {
      cwd: workspaceRoot,
      encoding: 'utf8',
      shell: process.platform === 'win32',
      windowsHide: true,
    });
    if (result.status !== 0) {
      throw new Error(
        `schema-domain prerequisite build failed\nstdout:\n${result.stdout ?? ''}\nstderr:\n${result.stderr ?? ''}`
      );
    }
  }
  ensure(fs.existsSync(generatedLoggingContractsArtifact), 'schema-domain generated logging contracts are unavailable');
}

async function loadSmokeDependencies() {
  const [
    ndjsonWriter,
    ndjsonPaths,
    testLogTypes,
    bridgeServer,
    bridgeTransport,
    bridgeQueueStorage,
    schemaContracts,
    portalLogger,
    query,
  ] = await Promise.all([
    import('@ocentra-parent/logging-domain/test-log/ndjsonWriter'),
    import('@ocentra-parent/logging-domain/test-log/ndjsonPaths'),
    import('@ocentra-parent/logging-domain/test-log/types'),
    import('@ocentra-parent/logging-domain/transport/bridgeServer'),
    import('@ocentra-parent/logging-domain/transport/bridgeTransport'),
    import('@ocentra-parent/logging-domain/core/localArtifactBridgeQueueStorage'),
    import('@ocentra-parent/schema-domain/generated/logging-contracts'),
    import('../../apps/portal/src/dev-logger.ts'),
    import('./lib/log-query-service.mjs'),
  ]);
  return {
    ...ndjsonWriter,
    ...ndjsonPaths,
    ...testLogTypes,
    ...bridgeServer,
    ...bridgeTransport,
    ...bridgeQueueStorage,
    ...schemaContracts,
    ...portalLogger,
    ...query,
  };
}

function runAgentQuery(args, env) {
  const result = spawnSync(process.execPath, [path.join(workspaceRoot, 'scripts/dev/agent-query.mjs'), ...args], {
    cwd: workspaceRoot,
    env,
    encoding: 'utf8',
    windowsHide: true,
  });

  if (result.status !== 0) {
    throw new Error(`agent-query failed\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`);
  }

  return result.stdout.trimEnd();
}

function seedStaleProofTrace(dependencies, rootDir, proofId) {
  dependencies.appendTestLogEntries(
    [
      {
        schemaVersion: 1,
        type: 'log',
        scope: dependencies.TestLogScope.ParentPortal,
        runId: proofId,
        runType: dependencies.RunType.Single,
        suiteType: 'integration',
        testName: 'logging-proof-trace-smoke',
        timestamp: Date.now() - 10_000,
        level: 'info',
        source: 'stale-proof',
        context: 'stale-proof.seed',
        message: 'stale proof trace before wipe',
        data: JSON.stringify({
          proofId,
          traceStep: 'portal.route.opened',
          eventType: 'route',
          status: 'ok',
        }),
        file: 'logging-proof-trace-smoke.mjs',
        filePath: 'scripts/dev/logging-proof-trace-smoke.mjs',
        line: null,
        column: null,
        correlationId: `${proofId}-correlation`,
        tags: [],
        stack: null,
        origin: dependencies.TestLogOrigin.Test,
        environment: 'test',
      },
    ],
    rootDir
  );
}

async function listen(server) {
  await new Promise((resolve) => {
    server.listen(0, '127.0.0.1', resolve);
  });
  return server.address();
}

async function closeServer(server) {
  await new Promise((resolve, reject) => {
    server.close((error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

async function main() {
  const argv = process.argv.slice(2);
  ensureGeneratedLoggingContractsArtifact();
  if (argv.includes('--verify-schema-prerequisite')) {
    process.stdout.write(`${JSON.stringify({ generatedLoggingContractsArtifact })}\n`);
    return;
  }
  const dependencies = await loadSmokeDependencies();
  const explicitRoot = optionValue(argv, '--root');
  const keepRoot = argv.includes('--keep-root') || explicitRoot != null;
  const rootDir = explicitRoot == null ? makeTempDir() : path.resolve(explicitRoot);
  const previousStructuredRoot = process.env['OCENTRA_PARENT_LOG_DIR'];

  ensureSafeRoot(rootDir);
  removeDir(rootDir);
  fs.mkdirSync(rootDir, { recursive: true });
  process.env['OCENTRA_PARENT_LOG_DIR'] = rootDir;

  const proofId = 'wp10-proof-trace-smoke';
  const staleProofId = 'wp10-stale-proof-trace';
  const bridgeServer = dependencies.createBridgeServer({ rootDir });
  const portalRuntime = {
    localStorage: dependencies.createLocalArtifactBridgeQueueStorage(rootDir),
  };

  try {
    seedStaleProofTrace(dependencies, rootDir, staleProofId);
    const staleFilesBeforeStart = dependencies.listNdjsonFiles(
      dependencies.getTestLogScopeDir(dependencies.TestLogScope.ParentPortal, rootDir)
    ).length;
    ensure(staleFilesBeforeStart > 0, 'stale proof trace seed did not create any NDJSON rows');

    const address = await listen(bridgeServer);
    const endpoint = `http://127.0.0.1:${address.port}`;

    const started = await dependencies.notifyBridgeRunStarted(endpoint, {
      runId: proofId,
      runType: dependencies.RunType.Single,
      suiteType: 'integration',
      scope: dependencies.TestLogScope.ParentPortal,
    });
    ensure(started, 'bridge run-start request did not succeed');

    const runInfo = await dependencies.fetchRunInfoFromBridge(endpoint);
    ensure(runInfo?.runId === proofId, 'bridge run info does not reflect the current proof-trace run');

    const staleFilesAfterStart = dependencies.listNdjsonFiles(
      dependencies.getTestLogScopeDir(dependencies.TestLogScope.ParentPortal, rootDir)
    ).length;
    ensure(staleFilesAfterStart === 0, 'bridge run-start did not wipe stale proof-trace rows');

    let staleProofRemoved = false;
    try {
      await dependencies.getProofTrace({
        scope: dependencies.TestLogScope.ParentPortal,
        proofId: staleProofId,
        limit: 10,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      staleProofRemoved =
        message.includes('No structured logs found for scope') ||
        message.includes(`Proof trace not found: ${staleProofId}`);
    }
    ensure(staleProofRemoved, 'stale proof trace remained queryable after the fresh run started');

    const routeOpened = await dependencies.sendPortalProofTraceLog(
      dependencies.GeneratedDevLogMessage.PortalStarted,
      {
        proofId,
        traceStep: 'portal.route.opened',
        eventType: 'route',
        status: 'ok',
        expectedNext: 'portal.action.clicked',
      },
      {},
      endpoint,
      portalRuntime
    );
    const actionClicked = await dependencies.sendPortalProofTraceLog(
      dependencies.GeneratedDevLogMessage.PortalCommandSent,
      {
        proofId,
        traceStep: 'portal.action.clicked',
        eventType: 'action',
        action: 'clicked',
        status: 'ok',
        expectedNext: 'portal.ui.rendered',
      },
      {
        uiTarget: 'open-dev-panel',
      },
      endpoint,
      portalRuntime
    );
    const uiRendered = await dependencies.sendPortalProofTraceLog(
      dependencies.GeneratedDevLogMessage.PortalEventReceived,
      {
        proofId,
        traceStep: 'portal.ui.rendered',
        eventType: 'ui',
        status: 'ok',
        artifactRef: 'artifact://portal/ui-rendered',
        correlationId: 'wp10-proof-trace-correlation',
      },
      {
        renderState: 'visible',
      },
      endpoint,
      portalRuntime
    );

    ensure(
      routeOpened && actionClicked && uiRendered,
      'portal proof-trace rows were not written through the local bridge'
    );
    ensure(await dependencies.flushBridgeRun(endpoint, proofId), 'bridge flush request did not succeed');

    const trace = await dependencies.getProofTrace({
      scope: dependencies.TestLogScope.ParentPortal,
      proofId,
      limit: 10,
    });
    ensure(
      JSON.stringify(trace.rows.map((row) => row.traceStep)) === JSON.stringify(expectedSteps),
      'query service returned unexpected proof-trace steps'
    );

    const gaps = await dependencies.getProofTraceGaps({
      scope: dependencies.TestLogScope.ParentPortal,
      proofId,
      expectedSteps,
      limit: 10,
    });
    ensure(gaps.missingSteps.length === 0, 'proof-trace gap query reported missing steps');
    ensure(gaps.outOfOrderSteps.length === 0, 'proof-trace gap query reported out-of-order steps');
    ensure(gaps.unexpectedErrorRows.length === 0, 'proof-trace gap query reported unexpected warn/error rows');

    const queryEnv = {
      ...process.env,
      OCENTRA_PARENT_LOG_DIR: rootDir,
    };
    const cliTrace = runAgentQuery(
      ['proof-trace', `--scope=${dependencies.TestLogScope.ParentPortal}`, `--proof-id=${proofId}`, '--limit=10'],
      queryEnv
    );
    ensure(cliTrace.includes(`proof_id: ${proofId}`), 'CLI proof-trace output did not include the proof id');
    ensure(cliTrace.includes('[portal.route.opened]'), 'CLI proof-trace output did not include the first trace step');
    ensure(cliTrace.includes('[portal.ui.rendered]'), 'CLI proof-trace output did not include the final trace step');

    const cliGaps = runAgentQuery(
      [
        'proof-trace-gaps',
        `--scope=${dependencies.TestLogScope.ParentPortal}`,
        `--proof-id=${proofId}`,
        `--expected-steps-json=${JSON.stringify(expectedSteps)}`,
        '--limit=10',
      ],
      queryEnv
    );
    ensure(cliGaps.includes('matched_steps: 3'), 'CLI proof-trace gap output did not report all matched steps');
    ensure(cliGaps.includes('missing_steps: 0'), 'CLI proof-trace gap output did not report zero missing steps');
    ensure(
      cliGaps.includes('unexpected_warn_or_error_rows: 0'),
      'CLI proof-trace gap output did not report zero unexpected warn/error rows'
    );

    process.stdout.write(
      `${JSON.stringify(
        {
          smoke: 'proof-trace',
          scope: dependencies.TestLogScope.ParentPortal,
          proofId,
          staleProofRemoved,
          rootDir: keepRoot ? rootDir.replace(/\\/g, '/') : null,
          runInfo,
          rows: trace.rows,
          gapSummary: {
            matchedSteps: gaps.matchedSteps.length,
            missingSteps: gaps.missingSteps.length,
            outOfOrderSteps: gaps.outOfOrderSteps.length,
            unexpectedWarnOrErrorRows: gaps.unexpectedErrorRows.length,
          },
          cli: {
            proofTrace: cliTrace,
            proofTraceGaps: cliGaps,
          },
        },
        null,
        2
      )}\n`
    );
  } finally {
    if (previousStructuredRoot == null) {
      delete process.env['OCENTRA_PARENT_LOG_DIR'];
    } else {
      process.env['OCENTRA_PARENT_LOG_DIR'] = previousStructuredRoot;
    }

    await closeServer(bridgeServer).catch(() => {});
    if (!keepRoot) {
      removeDir(rootDir);
    }
  }
}

void main();
