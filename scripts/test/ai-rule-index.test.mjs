import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import assert from 'node:assert/strict';

import { inspectRuleIndex } from '../check-ai-rule-index.mjs';

const tempRoot = join(process.cwd(), '.tmp-ai-rule-index-test');

test.afterEach(() => {
  rmSync(tempRoot, { recursive: true, force: true });
});

test('ai rule index check rejects unlinked rule files', () => {
  mkdirSync(tempRoot, { recursive: true });
  const missingRule = join(tempRoot, 'missing-rule.mdc');
  writeFileSync(missingRule, '# Missing\n');

  const result = inspectRuleIndex({
    agentsText: '.ocentra-ai/rules/ocentra-parent-rules.mdc',
    indexText: '# Index\n',
    ruleFiles: [missingRule],
    rootDir: process.cwd(),
    ruleDir: tempRoot,
  });

  assert.equal(result.failures.length, 1);
  assert.match(result.failures[0], /not linked/);
});

test('ai rule index check rejects missing AGENTS reference', () => {
  mkdirSync(tempRoot, { recursive: true });
  const linkedRule = join(tempRoot, 'linked-rule.mdc');
  writeFileSync(linkedRule, '# Linked\n');

  const result = inspectRuleIndex({
    agentsText: '# Agent guide',
    indexText: '.ocentra-ai/rules/linked-rule.mdc',
    ruleFiles: [linkedRule],
    rootDir: process.cwd(),
    ruleDir: tempRoot,
  });

  assert.equal(result.failures.length, 1);
  assert.match(result.failures[0], /AGENTS\.md/);
});

test('ai rule index check accepts indexed granular rules', () => {
  mkdirSync(tempRoot, { recursive: true });
  const linkedRule = join(tempRoot, 'linked-rule.mdc');
  writeFileSync(linkedRule, '# Linked\n');

  const result = inspectRuleIndex({
    agentsText: '.ocentra-ai/rules/ocentra-parent-rules.mdc',
    indexText: '.ocentra-ai/rules/linked-rule.mdc',
    ruleFiles: [linkedRule],
    rootDir: process.cwd(),
    ruleDir: tempRoot,
  });

  assert.deepEqual(result.failures, []);
});
