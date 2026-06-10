import { execFileSync } from 'node:child_process';

const generatedRoots = ['output', 'test-results', 'playwright-report'];

const trackedArtifacts = execFileSync('git', ['ls-files', ...generatedRoots], {
  encoding: 'utf8',
})
  .split(/\r?\n/u)
  .filter(Boolean);

if (trackedArtifacts.length > 0) {
  console.error('Generated proof and test artifacts must not be tracked in git.');
  console.error(`Found ${trackedArtifacts.length} tracked artifact(s):`);

  for (const artifactPath of trackedArtifacts.slice(0, 80)) {
    console.error(`- ${artifactPath}`);
  }

  if (trackedArtifacts.length > 80) {
    console.error(`... ${trackedArtifacts.length - 80} more`);
  }

  console.error(
    'Keep proof scripts, fixtures, and docs in source; upload generated proof output as CI artifacts or inspect it locally.'
  );
  process.exit(1);
}

console.log('No tracked generated proof or test artifacts found.');
