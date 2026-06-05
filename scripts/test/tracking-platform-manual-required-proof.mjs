import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { execFileSync } from 'node:child_process';

const proofRoot = join('output', 'tracking-plan-proof', '31-platform-extension-checklists-and-proof-routing');
const testRoot = join('test-results', 'tracking-platform-manual-required-proof');

function run(command, args) {
  const startedAt = new Date().toISOString();
  execFileSync(command, args, { stdio: 'inherit', shell: process.platform === 'win32' });
  return `${startedAt} ${[command, ...args].join(' ')} PASS`;
}

function gitOutput(args) {
  return execFileSync('git', args, { encoding: 'utf8' }).trim();
}

async function main() {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(testRoot, { recursive: true });

  const commands = [
    run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']),
    run('cmd', [
      '/c',
      'npm',
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tracking-platform-manual-required-proof',
    ]),
  ];

  const module = await import('../../packages/parent-domain/dist/tracking-platform-manual-required-proof.js');
  const proof = module.buildTrackingPlatformManualRequiredProof();
  const summary = module.summarizeTrackingPlatformManualRequiredProof(proof);
  const branch = gitOutput(['branch', '--show-current']);
  const commit = gitOutput(['rev-parse', 'HEAD']);
  const status = gitOutput(['status', '--short']);
  const generatedAt = '2026-06-05T15:50:00.000Z';

  const proofDocument = {
    proofMode: proof.proofMode,
    generatedAt,
    branch,
    commit,
    gitStatusShort: status,
    summary,
    proof,
    commands,
    knownGaps: [
      'Android foreground location needs real permission and adapter evidence.',
      'Android background location and geofence transition proof need real device evidence.',
      'iOS Core Location and region monitoring need Mac/device, entitlement, and permission evidence.',
      'Physical-device, authority-enrolled, provider-delivery, and production claims remain unproved.',
    ],
  };

  await writeFile(join(testRoot, 'proof.json'), `${JSON.stringify(proofDocument, null, 2)}\n`);
  await writeFile(join(proofRoot, 'proof.json'), `${JSON.stringify(proofDocument, null, 2)}\n`);
  await writeFile(
    join(proofRoot, '15-manual-platform-proof.md'),
    [
      '# WP31 Manual Platform Proof',
      '',
      `- Branch: ${branch}`,
      `- Commit: ${commit}`,
      '- Android foreground/background/geofence tracking remains manual-required before real device proof.',
      '- Android device-status rows are emulator-scaffold only and do not prove location.',
      '- iOS Core Location and background region monitoring remain entitlement/device proof gates.',
      '- iOS simulator package routing is package mechanics only and does not prove child tracking runtime.',
      '- No physical-device, authority-enrolled, provider-delivery, or production capability is claimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofRoot, '16-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(
    join(proofRoot, '02-platform-permission-proof.md'),
    [
      '# WP31 Platform Permission Proof',
      '',
      '- Android foreground location: manual_required until a real device records permission and adapter evidence.',
      '- Android background/geofence: manual_required until a real device records background permission and transition evidence.',
      '- iOS foreground Core Location: manual_required until Mac/device permission evidence exists.',
      '- iOS background region/geofence: authority_required until Apple entitlement and device evidence exists.',
      '- Unsupported or unavailable platform states must render as manual-required, authority-required, not-claimed, or scaffold-only.',
      '',
    ].join('\n')
  );

  console.log('tracking-platform-manual-required-proof-ok');
  console.log(`evidence=${join(testRoot, 'proof.json')}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
