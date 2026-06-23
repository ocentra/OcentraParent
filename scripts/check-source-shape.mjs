import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, resolve, sep } from 'node:path';
import { pathToFileURL } from 'node:url';
import ts from 'typescript';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const repoRoot = process.cwd();
const ignoredSegments = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger', 'target']);
const warningRatio = 0.8;
const fileLineWarningStep = 250;
const scriptName = 'node scripts/check-source-shape.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const policies = [
  {
    roots: ['apps'],
    extensions: new Set(['.ts', '.tsx']),
    kind: 'typescript',
    maxClasses: 1,
    maxExports: 35,
    maxFunctionLines: 80,
    maxLines: 1000,
  },
  {
    roots: ['packages'],
    extensions: new Set(['.ts', '.tsx']),
    kind: 'typescript',
    maxClasses: 1,
    maxExports: 45,
    maxFunctionLines: 80,
    maxLines: 1000,
  },
  {
    roots: ['crates'],
    extensions: new Set(['.rs']),
    kind: 'rust',
    maxFunctionLines: 80,
    maxFunctions: 18,
    maxLines: 1000,
    maxTypes: 24,
  },
];

const schemaDomainTypeScriptRoot = 'packages/schema-domain/src/';
const schemaDomainCatalogDataPattern = /^packages\/schema-domain\/src\/.*catalog-data(?:-[a-z0-9-]+)?\.ts$/u;
const agentProtocolRustRoot = 'crates/agent-protocol/';
const agentProtocolRustTestPattern =
  /^crates\/agent-protocol\/(?:src\/(?:.*_tests|tests)\.rs|tests\/contract\/.*\.rs)$/u;
const crateRustTestPattern = /^crates\/.*(?:_tests?\.rs|tests\/.*\.rs)$/u;
const sourceShapePolicyOverrides = new Map([
  [
    'apps/portal/e2e/tracking-hosted-ui-proof.spec.ts',
    {
      maxLines: 1100,
    },
  ],
  [
    'crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request.rs',
    {
      maxFunctions: 22,
    },
  ],
  [
    'crates/agent-service/src/browser_runtime_stream_payload.rs',
    {
      maxFunctions: 20,
    },
  ],
  [
    'crates/agent-service/src/enforcement_api.rs',
    {
      maxFunctions: 22,
    },
  ],
  [
    'crates/tracking-core/src/runtime_flow.rs',
    {
      maxFunctions: 60,
    },
  ],
  [
    'crates/agent-service/src/websocket/tracking_retention_settings_write.rs',
    {
      maxFunctionLines: 100,
    },
  ],
  [
    'crates/agent-core/src/activity_store_policy_preview.rs',
    {
      maxFunctions: 24,
    },
  ],
  [
    'crates/agent-core/src/activity_store_policy_preview_tests.rs',
    {
      maxFunctionLines: 130,
    },
  ],
  [
    'crates/agent-core/src/browser_event_runtime.rs',
    {
      maxFunctions: 24,
    },
  ],
  [
    'crates/billing-core/src/billing_subscription.rs',
    {
      maxFunctions: 40,
      maxTypes: 32,
    },
  ],
  [
    'crates/billing-core/tests/unit/subscription_lifecycle.rs',
    {
      maxFunctionLines: 120,
    },
  ],
  [
    'crates/family-identity-core/src/household_authority.rs',
    {
      maxFunctionLines: 90,
    },
  ],
  [
    'crates/parent-runtime-core/src/tracking_config_update_flow.rs',
    {
      maxFunctions: 50,
      maxFunctionLines: 160,
    },
  ],
  [
    'crates/parent-runtime-core/src/tracking_dispatch.rs',
    {
      maxFunctions: 20,
    },
  ],
  [
    'crates/policy-control-core/src/policy_compiler.rs',
    {
      maxFunctions: 36,
      maxFunctionLines: 220,
    },
  ],
  [
    'crates/policy-control-core/src/policy_conflict.rs',
    {
      maxFunctionLines: 150,
    },
  ],
  [
    'crates/policy-control-core/src/policy_delivery.rs',
    {
      maxFunctions: 36,
      maxFunctionLines: 180,
    },
  ],
  [
    'crates/policy-control-core/src/policy_event.rs',
    {
      maxFunctions: 36,
      maxFunctionLines: 100,
    },
  ],
  [
    'crates/policy-control-core/src/policy_preview.rs',
    {
      maxFunctions: 32,
    },
  ],
  [
    'crates/policy-control-core/src/policy_request.rs',
    {
      maxFunctions: 32,
      maxFunctionLines: 90,
    },
  ],
  [
    'crates/policy-control-core/src/policy_source.rs',
    {
      maxFunctions: 50,
      maxLines: 1300,
      maxTypes: 30,
    },
  ],
  [
    'crates/policy-control-core/tests/unit/policy_compiler.rs',
    {
      maxFunctions: 24,
      maxFunctionLines: 200,
      maxLines: 1300,
    },
  ],
  [
    'crates/policy-control-core/tests/unit/policy_conflict.rs',
    {
      maxFunctionLines: 130,
    },
  ],
  [
    'crates/policy-control-core/tests/unit/policy_event.rs',
    {
      maxFunctionLines: 115,
    },
  ],
  [
    'crates/policy-control-core/tests/unit/policy_source.rs',
    {
      maxFunctionLines: 90,
      maxFunctions: 30,
    },
  ],
  [
    'crates/policy-control-core/tests/version-skew/policy_compiler.rs',
    {
      maxFunctionLines: 200,
    },
  ],
  [
    'crates/policy-control-core/tests/version-skew/policy_event.rs',
    {
      maxFunctionLines: 150,
    },
  ],
  [
    'crates/provisioning-core/src/provisioning_install.rs',
    {
      maxFunctions: 34,
      maxFunctionLines: 180,
      maxLines: 1300,
      maxTypes: 28,
    },
  ],
  [
    'crates/screen-capture-adapter/examples/screen_capture_real_proof_support/mod.rs',
    {
      maxFunctionLines: 90,
    },
  ],
  [
    'crates/tracking-core/src/status.rs',
    {
      maxFunctionLines: 90,
    },
  ],
]);

function toPosix(path) {
  return path.split(sep).join('/');
}

function extensionOf(path) {
  const match = path.match(/\.[^.]+$/u);
  return match?.[0] ?? '';
}

function shouldSkip(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  return relativePath.split('/').some((part) => ignoredSegments.has(part));
}

function countLines(text) {
  return text.length === 0 ? 0 : text.split(/\r?\n/u).length;
}

function walk(path, files) {
  if (!existsSync(path) || shouldSkip(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stats.isFile()) {
    files.push(path);
  }
}

function policyFor(relativePath) {
  const normalized = toPosix(relativePath);
  const basePolicy = policies.find(
    (policy) =>
      policy.roots.some((root) => normalized.startsWith(`${root}/`)) && policy.extensions.has(extensionOf(normalized))
  );
  if (basePolicy === undefined) {
    return undefined;
  }

  const override = sourceShapePolicyOverrides.get(normalized);
  if (override !== undefined) {
    return {
      ...basePolicy,
      ...override,
    };
  }

  if (basePolicy.kind === 'typescript') {
    if (!normalized.startsWith(schemaDomainTypeScriptRoot)) {
      return basePolicy;
    }

    const schemaDomainPolicy = {
      ...basePolicy,
      // Central schema ownership naturally carries a wider public contract surface.
      maxExports: 140,
      maxLines: 1500,
    };

    if (schemaDomainCatalogDataPattern.test(normalized)) {
      return {
        ...schemaDomainPolicy,
        // Generated catalog slabs are data-only; line count is not a useful ownership signal here.
        maxLines: 10000,
      };
    }

    return schemaDomainPolicy;
  }

  if (!normalized.startsWith(agentProtocolRustRoot)) {
    if (crateRustTestPattern.test(normalized)) {
      return {
        ...basePolicy,
        // Rust test surfaces naturally carry more scenario coverage and slightly longer assertions.
        maxFunctionLines: 110,
        maxFunctions: 24,
      };
    }
    return basePolicy;
  }

  const agentProtocolPolicy = {
    ...basePolicy,
    // Central protocol crates mirror large contract surfaces and compatibility bridges.
    maxFunctions: 120,
    maxLines: 2000,
    maxTypes: 60,
  };

  if (agentProtocolRustTestPattern.test(normalized)) {
    return {
      ...agentProtocolPolicy,
      // Contract tests intentionally exercise many mirror and payload combinations in one place.
      maxFunctionLines: 140,
      maxFunctions: 32,
    };
  }

  return agentProtocolPolicy;
}

const sourceShapeRoots = [...new Set(policies.flatMap((policy) => policy.roots))];

function nearLimit(value, limit) {
  return value >= Math.ceil(limit * warningRatio) && value <= limit;
}

function fileLineWarningBand(lines, policy) {
  if (lines < fileLineWarningStep || lines > policy.maxLines) {
    return null;
  }
  return Math.floor(lines / fileLineWarningStep) * fileLineWarningStep;
}

function reportFileLines(findings, warnings, relativePath, text, policy) {
  const lines = countLines(text);
  if (lines > policy.maxLines) {
    findings.push({
      path: relativePath,
      line: policy.maxLines + 1,
      reason: `file has ${lines} lines; maximum is ${policy.maxLines}`,
    });
    return;
  }
  const warningBand = fileLineWarningBand(lines, policy);
  if (warningBand !== null) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${lines} lines; crossed ${warningBand}-line advisory band; maximum is ${policy.maxLines}`,
    });
  }
}

export function inspectTypeScriptSource(relativePath, text, policy = policies[0]) {
  const findings = [];
  const warnings = [];
  const source = ts.createSourceFile(relativePath, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  let classCount = 0;
  let exportCount = 0;

  reportFileLines(findings, warnings, relativePath, text, policy);

  function lineSpan(node) {
    const start = source.getLineAndCharacterOfPosition(node.getStart(source)).line;
    const end = source.getLineAndCharacterOfPosition(node.getEnd()).line;
    return end - start + 1;
  }

  function hasExportModifier(node) {
    return node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword) ?? false;
  }

  function inspectFunctionLike(node) {
    const lines = lineSpan(node);
    if (lines > policy.maxFunctionLines) {
      const position = source.getLineAndCharacterOfPosition(node.getStart(source));
      findings.push({
        path: relativePath,
        line: position.line + 1,
        reason: `function has ${lines} lines; maximum is ${policy.maxFunctionLines}`,
      });
      return;
    }
    if (nearLimit(lines, policy.maxFunctionLines)) {
      const position = source.getLineAndCharacterOfPosition(node.getStart(source));
      warnings.push({
        path: relativePath,
        line: position.line + 1,
        reason: `function has ${lines} lines; warning starts at ${Math.ceil(policy.maxFunctionLines * warningRatio)} of ${policy.maxFunctionLines}`,
      });
    }
  }

  function visit(node) {
    if (ts.isClassDeclaration(node)) {
      classCount += 1;
    }
    if (hasExportModifier(node) || ts.isExportDeclaration(node) || ts.isExportAssignment(node)) {
      exportCount += 1;
    }
    if (
      ts.isFunctionDeclaration(node) ||
      ts.isFunctionExpression(node) ||
      ts.isArrowFunction(node) ||
      ts.isMethodDeclaration(node)
    ) {
      inspectFunctionLike(node);
    }
    ts.forEachChild(node, visit);
  }

  visit(source);

  if (classCount > policy.maxClasses) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${classCount} classes; maximum is ${policy.maxClasses}`,
    });
  }
  if (exportCount > policy.maxExports) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${exportCount} exports; maximum is ${policy.maxExports}`,
    });
  }

  return { findings, warnings };
}

export function inspectRustSource(relativePath, text, policy = policies[2]) {
  const findings = [];
  const warnings = [];
  const lines = text.split(/\r?\n/u);
  const functionStarts = [];
  let typeCount = 0;

  reportFileLines(findings, warnings, relativePath, text, policy);

  lines.forEach((line, index) => {
    if (/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+/u.test(line)) {
      functionStarts.push(index);
    }
    if (/^\s*(?:pub\s+)?(?:struct|enum)\s+\w+/u.test(line)) {
      typeCount += 1;
    }
  });

  if (functionStarts.length > policy.maxFunctions) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${functionStarts.length} functions; maximum is ${policy.maxFunctions}`,
    });
  } else if (nearLimit(functionStarts.length, policy.maxFunctions)) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${functionStarts.length} functions; warning starts at ${Math.ceil(policy.maxFunctions * warningRatio)} of ${policy.maxFunctions}`,
    });
  }
  if (typeCount > policy.maxTypes) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${typeCount} structs/enums; maximum is ${policy.maxTypes}`,
    });
  } else if (nearLimit(typeCount, policy.maxTypes)) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${typeCount} structs/enums; warning starts at ${Math.ceil(policy.maxTypes * warningRatio)} of ${policy.maxTypes}`,
    });
  }

  for (const start of functionStarts) {
    const end = findRustFunctionEnd(lines, start);
    const functionLines = end - start + 1;
    if (functionLines > policy.maxFunctionLines) {
      findings.push({
        path: relativePath,
        line: start + 1,
        reason: `function has ${functionLines} lines; maximum is ${policy.maxFunctionLines}`,
      });
    } else if (nearLimit(functionLines, policy.maxFunctionLines)) {
      warnings.push({
        path: relativePath,
        line: start + 1,
        reason: `function has ${functionLines} lines; warning starts at ${Math.ceil(policy.maxFunctionLines * warningRatio)} of ${policy.maxFunctionLines}`,
      });
    }
  }

  return { findings, warnings };
}

function findRustFunctionEnd(lines, start) {
  let depth = 0;
  let seenBody = false;
  for (let index = start; index < lines.length; index += 1) {
    for (const char of lines[index]) {
      if (char === '{') {
        seenBody = true;
        depth += 1;
      } else if (char === '}') {
        depth -= 1;
      }
    }
    if (seenBody && depth === 0) {
      return index;
    }
  }
  return start;
}

export function collectSourceShapeReport(root = repoRoot) {
  const files = [];
  for (const policy of policies) {
    for (const sourceRoot of policy.roots) {
      walk(join(root, sourceRoot), files);
    }
  }

  const findings = [];
  const warnings = [];
  for (const file of files) {
    const relativePath = toPosix(relative(root, file));
    const policy = policyFor(relativePath);
    if (policy === undefined) {
      continue;
    }

    const text = readFileSync(file, 'utf8');
    const result =
      policy.kind === 'rust'
        ? inspectRustSource(relativePath, text, policy)
        : inspectTypeScriptSource(relativePath, text, policy);
    findings.push(...result.findings);
    warnings.push(...result.warnings);
  }
  return { findings, warnings };
}

export function collectSourceShapeReportForFiles(files) {
  const findings = [];
  const warnings = [];

  for (const file of files) {
    const relativePath = toPosix(file);
    const policy = policyFor(relativePath);
    if (policy === undefined) {
      continue;
    }

    const text = readFileSync(repoAbsolutePath(relativePath), 'utf8');
    const result =
      policy.kind === 'rust'
        ? inspectRustSource(relativePath, text, policy)
        : inspectTypeScriptSource(relativePath, text, policy);
    findings.push(...result.findings);
    warnings.push(...result.warnings);
  }

  return { findings, warnings };
}

export function collectSourceShapeFindings(root = repoRoot) {
  return collectSourceShapeReport(root).findings;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const rawArgs = process.argv.slice(2);
  const scopedFiles =
    rawArgs.length === 0
      ? null
      : resolveScopedFiles(rawArgs, {
          scriptName,
          usageLines,
          roots: sourceShapeRoots,
          acceptPath: (filePath) => policyFor(filePath) !== undefined,
        }).files;
  const { findings, warnings } =
    scopedFiles === null
      ? collectSourceShapeReport(
          process.argv[2] && !process.argv[2].startsWith('--') ? resolve(repoRoot, process.argv[2]) : repoRoot
        )
      : collectSourceShapeReportForFiles(scopedFiles);
  if (warnings.length > 0) {
    console.log('Source shape warnings: files/functions are near their size limits.');
    for (const warning of warnings) {
      console.log(`${warning.path}:${warning.line} ${warning.reason}`);
    }
  }

  if (findings.length > 0) {
    console.error('Source shape guard failed. Split oversized files/functions/classes before adding behavior.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.reason}`);
    }
    process.exit(1);
  }

  if (scopedFiles === null) {
    console.log('Source shape guard passed.');
    process.exit(0);
  }

  console.log(`Source shape guard passed for ${scopedFiles.length} file(s).`);
}
