import { readdirSync, readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

function readSource(relativePath: string): string {
  return readFileSync(new URL(`../../src/${relativePath}`, import.meta.url), 'utf8');
}

function listPolicySourceFiles(): string[] {
  return readdirSync(new URL('../../src', import.meta.url))
    .filter((entry) => /^policy.*\.ts$|^generated-policy.*\.ts$/.test(entry))
    .sort();
}

describe('policy thin boundary', () => {
  it('keeps the policy source set split into generated files and thin adapters only', () => {
    const policySourceFiles = listPolicySourceFiles();
    const handwrittenPolicyFiles = policySourceFiles.filter((file) => !file.startsWith('generated-policy'));
    const generatedPolicyFiles = policySourceFiles.filter((file) => file.startsWith('generated-policy'));

    expect(handwrittenPolicyFiles).toEqual([
      'policy-authority.ts',
      'policy-compiler.ts',
      'policy-contracts.ts',
      'policy-literal-contracts.ts',
      'policy.ts',
    ]);
    expect(generatedPolicyFiles).toEqual([
      'generated-policy-control-helpers-contracts.ts',
      'generated-policy-control-helpers.ts',
      'generated-policy.ts',
    ]);
    expect(handwrittenPolicyFiles).toHaveLength(5);
    expect(generatedPolicyFiles).toHaveLength(3);
  });

  it('keeps policy surfaces as generated or thin adapters only', () => {
    const policySource = readSource('policy.ts');
    const policyAuthoritySource = readSource('policy-authority.ts');
    const policyCompilerSource = readSource('policy-compiler.ts');
    const policyLiteralContractsSource = readSource('policy-literal-contracts.ts');

    expect(policySource).toContain("from './generated-policy'");
    expect(policySource).toContain("from './policy-contracts'");
    expect(policySource).toContain('compareGeneratedPolicyActionStrictness');
    expect(policySource).toContain('selectGeneratedStricterPolicyAction');
    expect(policySource).not.toContain('allow: 0');
    expect(policySource).not.toContain('block: 50');

    expect(policyAuthoritySource).toContain("from './generated-policy-control-helpers-contracts'");
    expect(policyAuthoritySource).toContain('literalRecordFromValues(GeneratedPolicyApprovalOriginValues)');
    expect(policyAuthoritySource).toContain('parsedLiteralRecord(');
    expect(policyAuthoritySource).not.toContain("Schema.Literal('child-request'");
    expect(policyAuthoritySource).not.toContain("Schema.Literal('assistant-draft'");

    expect(policyCompilerSource).toContain("from './generated-policy-control-helpers-contracts'");
    expect(policyCompilerSource).toContain('literalRecordFromValues(GeneratedPolicyCompilerNoClaimLabelValues)');
    expect(policyCompilerSource).toContain('literalRecordFromValues(GeneratedPolicyCompilerTargetKindValues)');
    expect(policyCompilerSource).toContain('parsedLiteralRecord(');
    expect(policyCompilerSource).not.toContain("Schema.Literal('compiled-artifact-not-source-truth'");
    expect(policyCompilerSource).not.toContain("Schema.Literal('runtime-mutation-not-claimed'");

    expect(policyLiteralContractsSource).toContain('literalRecordFromValues');
    expect(policyLiteralContractsSource).toContain('literalSchema');
    expect(policyLiteralContractsSource).toContain('parsedLiteralRecord');
    expect(policyLiteralContractsSource).not.toContain('allow');
    expect(policyLiteralContractsSource).not.toContain('manual-required');
  });
});
