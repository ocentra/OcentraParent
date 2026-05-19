#!/usr/bin/env node
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const ROOT_DIR = fileURLToPath(new URL('..', import.meta.url));
const RULES_ROOT_NAME = '.ocentra-ai';
const RULE_DIR = join(ROOT_DIR, RULES_ROOT_NAME, 'rules');
const AGENTS_PATH = join(ROOT_DIR, 'AGENTS.md');
const INDEX_NAME = 'ocentra-parent-rules.mdc';
const WARNING_LINE_COUNT = 160;
const MAX_LINE_COUNT = 220;

export function collectMarkdownFiles(directory) {
  return readdirSync(directory)
    .filter((entry) => entry.endsWith('.md') || entry.endsWith('.mdc'))
    .map((entry) => join(directory, entry))
    .filter((entryPath) => statSync(entryPath).isFile());
}

export function inspectRuleIndex({ agentsText, indexText, ruleFiles, rootDir = ROOT_DIR, ruleDir = RULE_DIR }) {
  const failures = [];
  const warnings = [];

  if (!agentsText.includes(`${RULES_ROOT_NAME}/rules/${INDEX_NAME}`)) {
    failures.push(`AGENTS.md must reference ${RULES_ROOT_NAME}/rules/${INDEX_NAME}.`);
  }

  for (const ruleFile of ruleFiles) {
    const relativePath = relative(rootDir, ruleFile).replaceAll('\\', '/');
    const ruleName = relative(ruleDir, ruleFile).replaceAll('\\', '/');
    const lineCount = readFileSync(ruleFile, 'utf8').split(/\r?\n/).length;

    if (ruleName !== INDEX_NAME && !indexText.includes(`${RULES_ROOT_NAME}/rules/${ruleName}`)) {
      failures.push(`${relativePath} is not linked from ${INDEX_NAME}.`);
    }

    if (lineCount > MAX_LINE_COUNT) {
      failures.push(`${relativePath} has ${lineCount} lines; split rule files above ${MAX_LINE_COUNT}.`);
    } else if (lineCount >= WARNING_LINE_COUNT) {
      warnings.push(`${relativePath} has ${lineCount} lines; warning starts at ${WARNING_LINE_COUNT}.`);
    }
  }

  return { failures, warnings };
}

export function runRuleIndexCheck() {
  const ruleFiles = collectMarkdownFiles(RULE_DIR);
  const agentsText = readFileSync(AGENTS_PATH, 'utf8');
  const indexText = readFileSync(join(RULE_DIR, INDEX_NAME), 'utf8');
  return inspectRuleIndex({ agentsText, indexText, ruleFiles });
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const result = runRuleIndexCheck();

  for (const warning of result.warnings) {
    console.warn(warning);
  }

  if (result.failures.length > 0) {
    console.error('AI rule index check failed:');
    for (const failure of result.failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('AI rule index check passed.');
}
