import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'v0-8-enforcement-control-plan-proof');
const proofDir = join(repoRoot, 'output', 'v0-8-enforcement-control-plan-proof', '18-proof-command-and-matrix');
const proofPath = join(testOutputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  await runCommand('node', ['scripts/test/v0-8-supported-adapter-runtime-proof.mjs']);
  await runCommand('node', ['scripts/test/v0-8-enforcement-product-control-spine.mjs']);
  await runCommand('node', ['scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs']);
  await runCommand('node', ['scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs']);
  await runCommand('node', ['scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs']);
  await runCommand(
    ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/policy-domain', '--', 'policy-approval-override'])
  );

  const supportedProof = await readJson('test-results/v0-8-supported-adapter-runtime-proof/proof.json');
  const productProof = await readJson('test-results/v0-8-enforcement-product-control-spine/proof.json');
  const integrityProof = await readJson('test-results/v0-8-enforcement-integrity-runtime-audit/proof.json');
  const platformProof = await readJson('test-results/v0-8-cross-platform-enforcement-capability-proof/proof.json');
  const runtimeProof = await readJson('test-results/v0-8-broad-os-adapter-runtime-proof/proof.json');
  const browserProof = await readJson('test-results/v0-8-browser-domain-adapter-proof/proof.json');

  assertClaimBoundaries(supportedProof, integrityProof, platformProof, runtimeProof, browserProof);

  const rows = [
    {
      rowId: 'app-game',
      proofLevel: 'implemented-boundary',
      state: 'supported-boundary-proved',
      summary:
        'Windows app/game owned-process time-limit remains the only app/game enforcement path proved as a supported boundary.',
      evidence: [
        supportedProof.evidence.proofHarness,
        supportedProof.evidence.tsRuntimeContract,
        productProof.evidence.proofHarness,
      ],
      knownGaps: ['global installed-app blocking', 'mobile child-device enforcement'],
    },
    {
      rowId: 'managed-browser',
      proofLevel: 'implemented-boundary',
      state: 'executes-real-service',
      summary:
        'Managed browser intervention is proved only for the owned managed-session path and stays separate from exact URL enforcement.',
      evidence: [browserProof.evidence.proofHarness, productProof.evidence.proofHarness],
      knownGaps: ['managed browser exact active-tab URL enforcement'],
    },
    {
      rowId: 'unmanaged-browser',
      proofLevel: 'implemented-boundary-plus-no-claim',
      state: 'process-scoped-with-exact-evidence-not-claimed',
      summary:
        'Unmanaged browser terminate/warn states are process-scoped only; exact URL, title, page, download, HTTPS content, and intent evidence remain unclaimed.',
      evidence: [browserProof.evidence.proofHarness, runtimeProof.evidence.proofHarness],
      knownGaps: [
        'unmanaged browser URL, active tab, title, page, download source, HTTPS content, or intent certainty',
      ],
    },
    {
      rowId: 'network-domain',
      proofLevel: 'manual-required',
      state: 'returns-manual-required',
      summary:
        'Host network/domain blocking remains manual-required and does not upgrade into real blocking without host adapter artifacts.',
      evidence: [runtimeProof.evidence.proofHarness, supportedProof.evidence.proofHarness],
      knownGaps: ['host network or domain blocking'],
    },
    {
      rowId: 'timers',
      proofLevel: 'implemented-boundary',
      state: 'recovery-rollback-expiry-audit-visible',
      summary:
        'Timer create/recovery/rollback/expiry states are represented in the product-control and integrity audit surfaces without expanding into broad blocking.',
      evidence: [integrityProof.evidence.proofHarness, productProof.evidence.proofHarness],
      knownGaps: ['service restart timer persistence beyond recovery-needed state'],
    },
    {
      rowId: 'approvals',
      proofLevel: 'contract-tested-and-surfaced',
      state: 'policy-approval-override-preview-approved-expired-replay-covered',
      summary:
        'Parent approval/override boundaries are covered by policy-domain contract tests and surfaced as a distinct product-control row.',
      evidence: [
        'packages/policy-domain/tests/unit/policy-approval-override.test.ts',
        productProof.evidence.proofHarness,
      ],
      knownGaps: ['notification delivery', 'portal UI rendering'],
    },
    {
      rowId: 'integrity',
      proofLevel: 'service-payload-boundary',
      state: 'permission-loss-heartbeat-stop-tamper-manual-required',
      summary:
        'Integrity audit and alert bridge rows expose permission-loss, stale heartbeat, stopped-or-removed, and tamper-manual-required states without anti-tamper or privilege claims.',
      evidence: [integrityProof.evidence.proofHarness, 'test-results/v0-8-integrity-alert-status-bridge/proof.json'],
      knownGaps: ['notification provider delivery', 'anti-tamper or uninstall resistance', 'privilege escalation'],
    },
    {
      rowId: 'platform',
      proofLevel: 'cross-platform-capability-matrix',
      state: 'windows-implemented-boundary-linux-unavailable-mobile-manual-required',
      summary:
        'Windows stays implemented-boundary only, Linux stays unavailable, and Android/iOS privileged surfaces stay manual-required or planned.',
      evidence: [platformProof.evidence.proofHarness],
      knownGaps: [
        'Linux, macOS, Android, or iOS child enforcement support',
        'device-owner policy, Family Controls entitlement, signing, TestFlight, Google Play, or App Store production readiness',
      ],
    },
  ];

  const knownGaps = unique([
    ...supportedProof.claimsNotProved,
    ...integrityProof.claimsNotProved,
    ...platformProof.claimsNotProved,
    ...runtimeProof.claimsNotProved,
    ...browserProof.claimsNotProved,
  ]);

  const claimUpgradeChecks = [
    zeroCheck(
      'supported-adapter-runtime-proof',
      supportedProof.counts.broadInstalledAppBlockingClaimed +
        supportedProof.counts.networkDomainBlockingClaimed +
        supportedProof.counts.exactActiveTabEnforcementClaimed +
        supportedProof.counts.notificationDeliveryClaimed +
        supportedProof.counts.tamperHardeningClaimed +
        supportedProof.counts.mobileControlClaimed +
        supportedProof.counts.unsupportedPlatformBehaviorClaimed
    ),
    zeroCheck(
      'enforcement-integrity-runtime-audit',
      integrityProof.counts.broadInstalledAppBlockingClaimed +
        integrityProof.counts.hostNetworkDomainBlockingClaimed +
        integrityProof.counts.exactActiveTabEnforcementClaimed +
        integrityProof.counts.notificationDeliveryClaimed +
        integrityProof.counts.tamperHardeningClaimed +
        integrityProof.counts.mobilePrivilegeClaimed +
        integrityProof.counts.stealthPersistenceClaimed +
        integrityProof.counts.privilegeEscalationClaimed +
        integrityProof.counts.integrityAlertProviderDeliveryClaimed +
        integrityProof.counts.integrityAlertTamperResistanceClaimed
    ),
    zeroCheck(
      'cross-platform-enforcement-capability-proof',
      platformProof.counts.broadBlockingClaimed +
        platformProof.counts.exactUrlClaimed +
        platformProof.counts.privilegedMobileClaimed +
        platformProof.counts.productionDistributionClaimed
    ),
    zeroCheck(
      'broad-os-adapter-runtime-proof',
      runtimeProof.counts.broadInstalledAppBlockingClaimed +
        runtimeProof.counts.networkDomainBlockingClaimed +
        runtimeProof.counts.managedBrowserExactUrlClaimed +
        runtimeProof.counts.unmanagedBrowserExactEvidenceClaimed +
        runtimeProof.counts.unsupportedPlatformClaimed +
        runtimeProof.counts.mobilePrivilegeClaimed
    ),
    zeroCheck(
      'browser-domain-adapter-proof',
      browserProof.counts.managedExactUrlClaimed +
        browserProof.counts.unmanagedExactUrlClaimed +
        browserProof.counts.networkDomainBlockingClaimed +
        browserProof.counts.broadBrowserControlClaimed +
        browserProof.counts.unsupportedOsClaimed +
        browserProof.counts.appControlPreventionClaimed +
        browserProof.counts.appControlPolicyCreationClaimed +
        browserProof.counts.appControlPolicyUpdateClaimed +
        browserProof.counts.appControlRollbackClaimed
    ),
  ];

  const proof = {
    schemaVersion: 1,
    proofMode: 'v0-8-enforcement-control-plan-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    platform: process.platform,
    command: 'node scripts/test/v0-8-enforcement-control-plan-proof.mjs',
    commands,
    proofLevel: 'composed-plan-matrix',
    rows,
    sourceProofs: {
      supportedAdapterRuntimeProof: 'test-results/v0-8-supported-adapter-runtime-proof/proof.json',
      enforcementProductControlSpine: 'test-results/v0-8-enforcement-product-control-spine/proof.json',
      enforcementIntegrityRuntimeAudit: 'test-results/v0-8-enforcement-integrity-runtime-audit/proof.json',
      crossPlatformEnforcementCapabilityProof:
        'test-results/v0-8-cross-platform-enforcement-capability-proof/proof.json',
      broadOsAdapterRuntimeProof: 'test-results/v0-8-broad-os-adapter-runtime-proof/proof.json',
      browserDomainAdapterProof: 'test-results/v0-8-browser-domain-adapter-proof/proof.json',
      policyApprovalOverrideTest: 'packages/policy-domain/tests/unit/policy-approval-override.test.ts',
    },
    claimUpgradeChecks,
    knownGaps,
  };

  await writeJson(proofPath, proof);
  await writeProofPack(proofDir, proof);

  console.log(`v0-8-enforcement-control-plan-proof-ok:${rows.map((row) => row.rowId).join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function assertClaimBoundaries(supportedProof, integrityProof, platformProof, runtimeProof, browserProof) {
  if (supportedProof.counts.byRuntimeState['implemented-boundary'] < 2) {
    throw new Error(
      'supported-adapter runtime proof is missing implemented-boundary rows for app/game and network handoff.'
    );
  }
  if (integrityProof.counts.byExecution['executed-supported-boundary'] < 4) {
    throw new Error('integrity runtime audit proof is missing executed-supported-boundary evidence.');
  }
  if (platformProof.counts.byProductClaimState['implemented-boundary'] < 4) {
    throw new Error('cross-platform capability proof is missing Windows implemented-boundary coverage.');
  }
  if (runtimeProof.counts.byProductClaimState['manual-required'] < 6) {
    throw new Error('broad OS runtime proof is missing manual-required runtime boundaries.');
  }
  if (browserProof.counts.byProductClaimState['implemented-boundary'] < 5) {
    throw new Error('browser/domain adapter proof is missing implemented-boundary coverage.');
  }
}

function zeroCheck(sourceProof, count) {
  if (count !== 0) {
    throw new Error(`${sourceProof} promoted ${count} claim upgrade flags without proof.`);
  }
  return { sourceProof, claimUpgradeCount: 0, result: 'pass' };
}

async function writeProofPack(dir, proof) {
  await writeText(
    join(dir, '00-scope-summary.md'),
    [
      '# Scope Summary',
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      `- Command: ${proof.command}`,
      `- Platform: ${proof.platform}`,
      `- Proof JSON: ${relative(repoRoot, proofPath)}`,
      '- Covered rows: app-game, managed browser, unmanaged browser, network/domain, timers, approvals, integrity, platform.',
      '',
    ].join('\n')
  );
  await writeText(
    join(dir, '01-negative-case-proof.md'),
    [
      '# Negative Case Proof',
      '',
      'The composed proof fails when any source proof reports claim-upgrade counters above zero.',
      '',
      ...proof.claimUpgradeChecks.map((check) => `- ${check.sourceProof}: ${check.result}`),
      '',
    ].join('\n')
  );
  await writeText(
    join(dir, '02-no-claim-boundary.md'),
    ['# No-Claim Boundary', '', ...proof.knownGaps.map((gap) => `- ${gap}`), ''].join('\n')
  );
  await writeJson(join(dir, '03-composed-proof.json'), proof);
  await writeText(join(dir, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`);
}

async function readJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function writeText(path, value) {
  await writeFile(path, value.endsWith('\n') ? value : `${value}\n`);
}

function unique(values) {
  return [...new Set(values)].sort();
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function gitBranch() {
  return gitOutput(['branch', '--show-current']);
}

async function gitHead() {
  return gitOutput(['rev-parse', 'HEAD']);
}

async function gitOutput(args) {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`git ${args.join(' ')} failed`))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
