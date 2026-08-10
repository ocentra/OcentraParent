#!/usr/bin/env node

import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(scriptDirectory, '..');
const fixturePath = path.join(
  root,
  'crates',
  'app-core',
  'tests',
  'contract',
  'fixtures',
  'app-runtime-decision-contracts.json'
);
const targetPath = path.join(root, 'packages', 'schema-domain', 'src', 'app-runtime-decision-contract-data.ts');

const fixture = JSON.parse(await readFile(fixturePath, 'utf8'));
const toDecisionTuple = ({ input, decision }) => [
  input.capability_state,
  input.foreground_state,
  input.classification_state,
  decision.observation_intent,
  decision.runtime_action_state,
  decision.ai_handoff_state,
  decision.policy_handoff_state,
];
const generated = {
  currentSchemaVersion: fixture.current_schema_version,
  currentDecisions: fixture.current_decisions.map(toDecisionTuple),
  legacyV1DecisionDeltas: fixture.legacy_v1_decision_deltas.map(toDecisionTuple),
};
const source = `// Generated from crates/app-core/tests/contract/fixtures/app-runtime-decision-contracts.json.\n// Rust contract tests own and exhaustively verify the current decision matrix.\nexport const RustOwnedAppRuntimeDecisionContracts = ${JSON.stringify(generated, null, 2)} as const;\n`;
const { default: prettier } = await import('prettier');
const formatted = await prettier.format(source, {
  filepath: targetPath,
  parser: 'typescript',
  singleQuote: true,
  trailingComma: 'es5',
  printWidth: 120,
});

if (process.argv.includes('--check')) {
  const existing = await readFile(targetPath, 'utf8');
  if (existing !== formatted) {
    console.error(`Generated app runtime decision data is stale: ${path.relative(root, targetPath)}`);
    process.exitCode = 1;
  }
} else {
  await writeFile(targetPath, formatted, 'utf8');
  console.log(`Generated ${path.relative(root, targetPath)} from ${path.relative(root, fixturePath)}`);
}
