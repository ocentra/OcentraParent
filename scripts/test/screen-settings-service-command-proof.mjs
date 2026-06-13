import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const repoRoot = process.cwd();
const outputRoot = resolve(repoRoot, 'output', 'screen-plan-proof', 'screen-settings-service-command');
const proofPath = join(outputRoot, 'proof-summary.json');

const commands = [
  {
    label: 'activity-domain screen settings export build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/screen-domain'],
  },
  {
    label: 'agent-protocol-domain screen settings adapter tests',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'exec',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'vitest',
      'run',
      'tests/screen-settings-adapter.test.ts',
    ],
  },
  {
    label: 'agent-protocol screen settings transport tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'screen_settings'],
  },
  {
    label: 'agent-service screen settings websocket command tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'screen_settings'],
  },
];

const results = commands.map(runCommand);
const failed = results.filter((result) => result.status !== 0);
assert(
  failed.length === 0,
  `screen settings command validation failed: ${failed.map((result) => result.label).join(', ')}`
);

const sourceFiles = [
  'packages/screen-domain/src/screen-evidence-settings.ts',
  'packages/agent-protocol-domain/src/screen-settings-adapter.ts',
  'packages/agent-protocol-domain/tests/screen-settings-adapter.test.ts',
  'crates/agent-protocol/src/screen_settings.rs',
  'crates/agent-protocol/src/transport.rs',
  'crates/agent-protocol/src/constants/field.rs',
  'crates/agent-service/src/screen_settings_api.rs',
  'crates/agent-service/src/screen_settings_request.rs',
  'crates/agent-service/src/screen_settings_payload.rs',
  'crates/agent-service/src/screen_settings_api_tests.rs',
  'crates/agent-service/src/websocket.rs',
  'crates/agent-service/src/app.rs',
];

for (const path of sourceFiles) {
  assert(existsSync(join(repoRoot, path)), `missing expected source file: ${path}`);
}

const adapter = readFileSync(
  join(repoRoot, 'packages', 'agent-protocol-domain', 'src', 'screen-settings-adapter.ts'),
  'utf8'
);
assert(
  adapter.includes('@ocentra-parent/screen-domain/screen-evidence-settings'),
  'screen settings adapter must import the owning settings schema entrypoint'
);
assert(
  adapter.includes('ScreenAnalysisParentSettingSchema'),
  'screen settings adapter must reuse the activity-domain parent setting schema'
);

const serviceTests = readFileSync(
  join(repoRoot, 'crates', 'agent-service', 'src', 'screen_settings_api_tests.rs'),
  'utf8'
);
assert(
  serviceTests.includes('screen_settings_replace_persists_and_get_reports_after_runtime_restart'),
  'service command persistence/reload test must remain explicit'
);
assert(
  serviceTests.includes('screen_settings_replace_rejects_raw_image_retention_before_persisting'),
  'service command raw-retention rejection test must remain explicit'
);

const summary = {
  proof: 'screen-settings-service-command',
  generatedAt: new Date().toISOString(),
  branchScope: 'codex/screen-ai-full-scope-b',
  sourceFiles,
  validation: results.map((result) => ({
    label: result.label,
    command: [result.command, ...result.args].join(' '),
    status: result.status,
    stdoutTail: tail(result.stdout),
    stderrTail: tail(result.stderr),
  })),
  proves: [
    'The TypeScript protocol layer has explicit screen settings get/replace command names, response event names, payload fields, and an adapter that reuses the activity-domain ScreenAnalysisParentSettingSchema.',
    'The Rust protocol mirrors screen settings get/replace commands and reported/replace accepted/rejected events.',
    'The Rust service WebSocket command handler routes screen settings get/replace commands to the local JSON-backed ScreenSettingsRuntime.',
    'A replace command persists a parent strict dry-run setting, and a later get command after runtime restart reports the persisted setting.',
    'A replace command that asks to retain raw screenshots is rejected before persistence and leaves no local settings store file.',
  ],
  custody: {
    parentSettingSchemaOwner: '@ocentra-parent/screen-domain/screen-evidence-settings',
    transportOwner: '@ocentra-parent/agent-protocol-domain and crates/agent-protocol',
    serviceStore: 'local child-device JSON service store',
    rawImageRetainedDefault: false,
    rawRetentionRejectedBeforePersistence: true,
    ocentraHostedDefaultStore: false,
  },
  nonClaims: [
    'This proof does not yet wire the parent portal Settings form to send the new service command.',
    'This proof does not enable raw screenshot retention, live view, raw remote upload, or cloud AI.',
    'This proof does not claim product-complete retention-control UI, privacy/legal approval, or new live capture behavior.',
  ],
};

mkdirSync(outputRoot, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(`screen-settings-service-command-proof-ok:${proofPath}`);

function runCommand(commandSpec) {
  const result = spawnSync(commandSpec.command, commandSpec.args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  return {
    ...commandSpec,
    status: result.status ?? 1,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

function tail(text) {
  return text.split(/\r?\n/u).filter(Boolean).slice(-12);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
