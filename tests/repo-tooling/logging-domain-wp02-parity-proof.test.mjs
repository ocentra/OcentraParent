import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';
import { afterEach, describe, it } from 'node:test';
import { fileURLToPath } from 'node:url';
import {
  Wp02ProofArtifactNames,
  blockedScopeDefaultsProof,
  collectPackageExportProof,
  collectParityFileMapProof,
  createQueryScriptSmokeProof,
  formatValidationCommandLog,
  resolveNpmInvocation,
  writeWp02ProofArtifacts,
} from '../../scripts/test/logging-domain-wp02-parity-proof.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const temporaryDirectories = [];

describe('logging-domain WP02 parity proof runner', () => {
  afterEach(() => {
    for (const directory of temporaryDirectories.splice(0, temporaryDirectories.length)) {
      fs.rmSync(directory, { force: true, recursive: true });
    }
  });

  it('collects the real current package export and TypeScript parity surfaces', () => {
    const exportProof = collectPackageExportProof(repoRoot, 'HEAD');
    const fileMap = collectParityFileMapProof(repoRoot);

    assert.equal(exportProof.status, 'passed');
    assert.deepEqual(exportProof.removed, []);
    assert.equal(exportProof.after.exports.length, exportProof.before.exports.length + exportProof.added.length);
    assert.deepEqual(exportProof.missingRequiredExports, []);
    assert.equal(fileMap.status, 'passed');
    assert.deepEqual(fileMap.missingGroups, []);
    assert.notEqual(fileMap.groups.testLog.files.indexOf('src/test-log/testLogDuckDb.ts'), -1);
    assert.notEqual(fileMap.groups.transport.files.indexOf('src/transport/bridgeServer.ts'), -1);
    assert.notEqual(fileMap.groups.appLog.files.indexOf('src/app-log/appNdjsonWriter.ts'), -1);
  });

  it('launches the npm CLI through Node when npm provides its executable module path', () => {
    const invocation = resolveNpmInvocation(['run', '--silent', 'build'], 'C:\\tools\\npm\\bin\\npm-cli.js');

    assert.equal(invocation.command, process.execPath);
    assert.deepEqual(invocation.args, ['C:\\tools\\npm\\bin\\npm-cli.js', 'run', '--silent', 'build']);
  });

  it('writes only the five retained artifacts with explicit blocked and no-claim states', () => {
    const outputDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-wp02-proof-schema-'));
    temporaryDirectories.push(outputDirectory);
    const command = spawnSync(
      process.execPath,
      [
        '-e',
        'process.stdout.write(JSON.stringify({distinctRuns:0,distinctTests:0,errorLogs:0,newestTimestamp:null,totalLogs:0,warnLogs:0}))',
      ],
      {
        encoding: 'utf8',
      }
    );
    const commandResult = {
      command: 'node bounded-query-result',
      exitCode: command.status ?? 1,
      status: command.status === 0 ? 'passed' : 'failed',
      stdout: command.stdout,
      diagnostics: 'bounded query schema command completed',
    };
    const exportProof = collectPackageExportProof(repoRoot, 'HEAD');
    const fileMap = collectParityFileMapProof(repoRoot);
    const scopeProof = blockedScopeDefaultsProof('package build intentionally not invoked by schema unit test');
    const queryProof = createQueryScriptSmokeProof(commandResult);
    const validationLog = formatValidationCommandLog([commandResult]);

    writeWp02ProofArtifacts(outputDirectory, {
      '00-package-export-before-after.json': exportProof,
      '01-typescript-parity-file-map.json': fileMap,
      '02-scope-defaults-proof.json': scopeProof,
      '03-query-script-smoke.json': queryProof,
      '16-validation-commands.log': validationLog,
    });

    assert.deepEqual(fs.readdirSync(outputDirectory).sort(), [...Wp02ProofArtifactNames].sort());
    assert.equal(
      JSON.parse(fs.readFileSync(path.join(outputDirectory, '02-scope-defaults-proof.json'), 'utf8')).status,
      'blocked'
    );
    assert.equal(
      JSON.parse(fs.readFileSync(path.join(outputDirectory, '03-query-script-smoke.json'), 'utf8')).status,
      'passed'
    );
    assert.match(
      fs.readFileSync(path.join(outputDirectory, '16-validation-commands.log'), 'utf8'),
      /no_claim: local TypeScript package parity only/u
    );
  });
});
