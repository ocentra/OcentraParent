import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const ciWorkflowPath = join(repoRoot, '.github', 'workflows', 'ci.yml');

function readCiWorkflow() {
  return readFileSync(ciWorkflowPath, 'utf8');
}

test('CI gate runs for documentation and expectation changes', () => {
  const workflow = readCiWorkflow();

  assert.match(workflow, /pull_request:\s+branches:\s+- main\s+- production/u);
  assert.match(workflow, /push:\s+branches:\s+- main/u);
  assert.equal(workflow.includes('paths-ignore'), false);
});

test('CI gate builds package previews but does not publish releases from main', () => {
  const workflow = readCiWorkflow();

  assert.match(workflow, /detect-docs-hub-only:\s+name: Detect docs\/hub-only change/u);
  assert.match(
    workflow,
    /package-preview:\s+needs: \[detect-docs-hub-only, validate, build, dependency-policy\]\s+uses: \.\/\.github\/workflows\/package-preview\.yml/u
  );
  assert.match(
    workflow,
    /dependency-policy:\s+needs: \[detect-docs-hub-only, secret-scan\]\s+uses: \.\/\.github\/workflows\/dependency-policy\.yml/u
  );
  assert.equal(workflow.includes('Create GitHub release'), false);
});
