import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const ciWorkflowPath = join(repoRoot, '.github', 'workflows', 'ci.yml');
const workflowsRoot = join(repoRoot, '.github', 'workflows');

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

  assert.match(workflow, /detect-targets:\s+name: Detect CI targets/u);
  assert.match(workflow, /preflight:[\s\S]+uses: \.\/\.github\/workflows\/ci-preflight\.yml/u);
  assert.match(workflow, /portal-typescript:[\s\S]+uses: \.\/\.github\/workflows\/ci-portal-typescript\.yml/u);
  assert.match(workflow, /rust-agent-service:[\s\S]+uses: \.\/\.github\/workflows\/ci-rust-agent-service\.yml/u);
  assert.match(workflow, /child-android:[\s\S]+uses: \.\/\.github\/workflows\/ci-child-android\.yml/u);
  assert.match(workflow, /child-ios:[\s\S]+uses: \.\/\.github\/workflows\/ci-child-ios\.yml/u);
  assert.match(workflow, /static-analysis:[\s\S]+uses: \.\/\.github\/workflows\/ci-codeql\.yml/u);
  assert.match(workflow, /package-windows:[\s\S]+uses: \.\/\.github\/workflows\/ci-package-windows\.yml/u);
  assert.match(workflow, /package-android:[\s\S]+uses: \.\/\.github\/workflows\/ci-package-android\.yml/u);
  assert.match(workflow, /package-preview:\s+name: Package Preview Gate/u);
  assert.match(workflow, /package-preview:[\s\S]+if: \$\{\{ always\(\) && needs\.validate\.result == 'success' \}\}/u);
  assert.equal(workflow.includes('Create GitHub release'), false);
});

test('non-doc product pull requests force the full merge proof graph', () => {
  const workflow = readCiWorkflow();

  assert.match(workflow, /product_pr_full_merge_proof=false/u);
  assert.match(
    workflow,
    /if \[\[ "\$\{\{ github\.event_name \}\}" == "pull_request" && "\$docs_hub_only" != "true" \]\]/u
  );
  assert.match(workflow, /product_pr_full_merge_proof=true/u);
  assert.match(workflow, /parent_mobile_changed=true/u);
  assert.match(workflow, /child_android_changed=true/u);
  assert.match(workflow, /child_ios_changed=true/u);
  assert.match(workflow, /package_parent_android_changed=true/u);
  assert.match(workflow, /package_parent_ios_changed=true/u);
  assert.match(workflow, /package_android_changed=true/u);
  assert.match(workflow, /package_ios_changed=true/u);
});

test('static analysis covers workflow, TypeScript, and Rust surfaces', () => {
  const staticAnalysisWorkflow = readFileSync(join(workflowsRoot, 'ci-codeql.yml'), 'utf8');

  assert.match(staticAnalysisWorkflow, /language:\s+actions/u);
  assert.match(staticAnalysisWorkflow, /language:\s+javascript-typescript/u);
  assert.match(staticAnalysisWorkflow, /language:\s+rust/u);
  assert.match(staticAnalysisWorkflow, /queries:\s+security-and-quality/u);
});

test('CI target workflows are split by runnable area', () => {
  const expectedWorkflows = [
    'ci-child-android.yml',
    'ci-child-ios.yml',
    'ci-codeql.yml',
    'ci-docs-hub.yml',
    'ci-domain-packages.yml',
    'ci-format.yml',
    'ci-local-transport.yml',
    'ci-package-android.yml',
    'ci-package-ios.yml',
    'ci-package-linux.yml',
    'ci-package-macos.yml',
    'ci-package-parent-android.yml',
    'ci-package-parent-ios.yml',
    'ci-package-windows.yml',
    'ci-parent-desktop-tauri.yml',
    'ci-parent-mobile.yml',
    'ci-portal-e2e.yml',
    'ci-preflight.yml',
    'ci-portal-typescript.yml',
    'ci-release-version.yml',
    'ci-rust-adapters.yml',
    'ci-rust-agent-core.yml',
    'ci-rust-agent-protocol.yml',
    'ci-rust-agent-service.yml',
    'ci-tooling.yml',
  ];

  for (const workflowName of expectedWorkflows) {
    assert.equal(existsSync(join(workflowsRoot, workflowName)), true, `${workflowName} should exist`);
  }
});

test('CI aggregate gates parse needs results as JSON', () => {
  const workflow = readCiWorkflow();

  assert.equal(workflow.includes('grep -q \'"result":"failure"'), false);
  assert.match(workflow, /const needs = JSON\.parse\(process\.env\.RESULTS\);/u);
  assert.match(workflow, /\['failure', 'cancelled'\]\.includes\(value\.result\)/u);
});
