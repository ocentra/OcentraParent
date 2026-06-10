import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const proofName = 'browser-runtime-social-provider-receipt-stream-parser-proof';
const resultDir = path.join(root, 'test-results', proofName);
const outputDir = path.join(
  root,
  'output',
  'browser-plan-proof',
  'browser-runtime-social-provider-receipt-stream-parser'
);

const files = {
  defaults: path.join(root, 'packages', 'agent-protocol-domain', 'src', 'defaults.ts'),
  parser: path.join(root, 'packages', 'agent-protocol-domain', 'src', 'browser-runtime-events.ts'),
  parserTest: path.join(root, 'packages', 'agent-protocol-domain', 'tests', 'browser-runtime-events.test.ts'),
  portalStatus: path.join(root, 'packages', 'portal-domain', 'src', 'browser-social-provider-receipt-stream-status.ts'),
  portalTest: path.join(
    root,
    'packages',
    'portal-domain',
    'tests',
    'browser-social-provider-receipt-stream-status.test.ts'
  ),
  workpack: path.join(
    root,
    'docs',
    'plans',
    'browser-plan',
    'workpacks',
    '13-browser-read-models-and-service-events.md'
  ),
  checklist: path.join(root, 'docs', 'plans', 'browser-plan', 'implementation-checklist.md'),
  featureDoc: path.join(root, 'docs', 'features', 'browser-web-control.md'),
};

const commands = [
  {
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    label: 'cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain',
  },
  {
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'browser-runtime-events.test.ts',
    ],
    label: 'cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts',
  },
  {
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/portal-domain',
      '--',
      'browser-social-provider-receipt-stream-status.test.ts',
    ],
    label:
      'cmd /c npm run test --workspace @ocentra-parent/portal-domain -- browser-social-provider-receipt-stream-status.test.ts',
  },
];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });

  const commandResults = commands.map(runCommand);
  const sourceChecks = await readSourceChecks();
  assertSourceChecks(sourceChecks);

  const proof = {
    proofName,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', '--short', 'HEAD']),
    commandResults,
    sourceChecks,
    verified: {
      rustServicePublicReceiptFieldsHaveTypescriptDefaults: true,
      sharedParserAcceptsProviderDispatchRequiredReceiptRefs: true,
      sharedParserAcceptsManualReceiptRequiredRowsWithoutDurableRefs: true,
      sharedParserRejectsManualRowsWithDurableRefs: true,
      sharedParserRejectsDispatchRowsMissingProviderAttemptRefs: true,
      portalDomainProjectsReceiptBoundaryStatus: true,
      providerDeliveryClaimed: false,
      receiptIngestionClaimed: false,
      parentNotificationDeliveryClaimed: false,
      reportDeliveryClaimed: false,
      finalPolicyExecutionClaimed: false,
      connectorNativeRuntimeClaimed: false,
      enforcementClaimed: false,
    },
  };

  await writeFile(path.join(resultDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    path.join(outputDir, '01-browser-runtime-social-provider-receipt-stream-parser-proof.md'),
    markdownFor(proof)
  );

  console.log('browser-runtime-social-provider-receipt-stream-parser-proof-ok=true');
  console.log(`proof=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function readSourceChecks() {
  const [defaults, parser, parserTest, portalStatus, portalTest, workpack, checklist, featureDoc] = await Promise.all(
    Object.values(files).map((file) => readFile(file, 'utf8'))
  );
  return {
    defaultsExposeReceiptBoundaryRows: defaults.includes('BrowserRuntimeSocialProviderReceiptBoundaryRows'),
    defaultsExposeDurableRefs: defaults.includes('BrowserRuntimeSocialProviderDurableResultRefs'),
    parserReadsReceiptBoundaryRows: parser.includes('socialProviderReceiptBoundaryRows'),
    parserDerivesReceiptStatus: parser.includes('deriveAgentBrowserRuntimeSocialProviderReceiptStatus'),
    parserRejectsDishonestReceiptRefs: parser.includes('browserRuntimeSocialProviderReceiptStateIsHonest'),
    parserTestCoversReceiptStatus: parserTest.includes('specifySocialProviderReceiptStatus'),
    parserTestCoversReceiptOverclaim: parserTest.includes('specifySocialProviderReceiptOverclaimRejections'),
    portalStatusUsesDerivedProtocolStatus: portalStatus.includes(
      'deriveAgentBrowserRuntimeSocialProviderReceiptStatus'
    ),
    portalTestCoversNoClaimCopy: portalTest.includes('enforcement remain unclaimed'),
    workpackMentionsProof: workpack.includes('Social Provider Receipt Stream Parser Addendum'),
    checklistMentionsProof: checklist.includes('browser-runtime-social-provider-receipt-stream-parser-proof'),
    featureDocMentionsParserStateCoverage:
      featureDoc.includes('TypeScript protocol and portal-domain state') &&
      featureDoc.includes('coverage for the social provider receipt service stream fields'),
  };
}

function assertSourceChecks(checks) {
  const missing = Object.entries(checks)
    .filter(([, ok]) => !ok)
    .map(([name]) => name);
  if (missing.length > 0) {
    throw new Error(`browser social provider receipt stream parser proof failed: ${missing.join(', ')}`);
  }
}

function runCommand(item) {
  execFileSync(item.command, item.args, {
    cwd: root,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
  return {
    command: item.label,
    status: 'passed',
  };
}

function markdownFor(proof) {
  return (
    [
      '# Browser Runtime Social Provider Receipt Stream Parser Proof',
      '',
      'This proof closes the public TypeScript side of the existing Rust social provider receipt stream fields.',
      '',
      'The shared agent-protocol-domain parser now reads social provider receipt boundary rows, provider dispatch-required rows, manual receipt-required rows, provider attempt refs, receipt proof refs, durable refs, read-model refs, and support-status refs. Portal-domain projects those parsed fields into a parent-visible status intent without reading raw log fields.',
      '',
      'No-claim boundary: provider delivery, receipt ingestion runtime, parent notification delivery, report delivery, final policy execution, connector/native runtime, browser mutation, child intervention execution, unmanaged exact URL support, and enforcement remain unclaimed.',
      '',
      'Validation:',
      ...proof.commandResults.map((result) => `- \`${result.command}\` (${result.status})`),
    ].join('\n') + '\n'
  );
}

function gitOutput(args) {
  return execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
}

function relativePath(targetPath) {
  return path.relative(root, targetPath).replaceAll('\\', '/');
}
