import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import ocentraParentRules from './eslint-rules/index.js';

const architectureRulesEnabled = process.env.OCENTRA_ARCHITECTURE_LINT === '1';
const sharedIgnores = {
  ignores: [
    '**/dist/**',
    '**/node_modules/**',
    '**/coverage/**',
    '**/.turbo/**',
    '**/target/**',
    '**/*.d.ts',
    '**/*.tsbuildinfo',
  ],
};
const architectureRuleConfig = tseslint.config(sharedIgnores, {
  files: ['apps/**/*.{js,jsx,ts,tsx,mjs,mts,cjs,cts}', 'packages/**/*.{js,jsx,ts,tsx,mjs,mts,cjs,cts}'],
  languageOptions: {
    parser: tseslint.parser,
    ecmaVersion: 2022,
    globals: {
      ...globals.browser,
      ...globals.node,
    },
  },
  rules: {
    'no-restricted-syntax': [
      'error',
      {
        selector: 'ExportAllDeclaration',
        message:
          'BARREL/REEXPORT BAN: `export * from ...` and namespace re-exports are forbidden. Import from the concrete module path directly.',
      },
      {
        selector: 'ExportNamedDeclaration[source]',
        message:
          'BARREL/REEXPORT BAN: `export { ... } from ...`, `export type { ... } from ...`, and default re-exports are forbidden. Import from the concrete module path directly.',
      },
    ],
  },
});

const standardConfig = tseslint.config(
  sharedIgnores,
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['packages/**/*.ts', 'apps/**/*.ts'],
    languageOptions: {
      ecmaVersion: 2022,
      globals: globals.node,
    },
    rules: {
      'no-console': 'error',
      '@typescript-eslint/consistent-type-imports': [
        'error',
        {
          fixStyle: 'inline-type-imports',
        },
      ],
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
        },
      ],
      complexity: ['error', 12],
      'max-classes-per-file': ['error', 1],
      'max-depth': ['error', 4],
      'max-lines': [
        'error',
        {
          max: 1000,
          skipBlankLines: true,
          skipComments: true,
        },
      ],
      'max-lines-per-function': [
        'error',
        {
          max: 80,
          skipBlankLines: true,
          skipComments: true,
        },
      ],
      'max-statements': ['error', 35],
      'ocentra-parent/no-naked-domain-string-types': 'error',
    },
    plugins: {
      'ocentra-parent': ocentraParentRules,
    },
  },
  {
    files: ['apps/portal/src/**/*.ts', 'apps/portal/tests/**/*.ts'],
    languageOptions: {
      ecmaVersion: 2022,
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
  {
    files: ['apps/portal/src/**/*.ts'],
    rules: {
      'ocentra-parent/no-app-string-literals': 'error',
      'ocentra-parent/no-runtime-string-types': 'error',
    },
  },
  {
    files: ['scripts/**/*.mjs', '*.config.js'],
    languageOptions: {
      ecmaVersion: 2022,
      globals: globals.node,
    },
    rules: {
      '@typescript-eslint/no-require-imports': 'off',
    },
  },
  {
    files: ['packages/**/tests/**/*.ts'],
    languageOptions: {
      ecmaVersion: 2022,
      globals: globals.node,
    },
  },
  {
    files: ['packages/schema-domain/src/generated/**/*.ts', 'packages/schema-domain/src/generated-*.ts'],
    rules: {
      '@typescript-eslint/consistent-type-imports': 'off',
      complexity: 'off',
      'max-lines': 'off',
      'max-lines-per-function': 'off',
      'ocentra-parent/no-naked-domain-string-types': 'off',
    },
  },
  {
    files: ['packages/portal-domain/src/generated/**/*.ts', 'packages/portal-domain/src/generated-*.ts'],
    rules: {
      'max-lines': 'off',
    },
  },
  {
    files: [
      'packages/schema-domain/src/browser-control-full-catalog-data-*.ts',
      'packages/schema-domain/src/network-control-catalog-data.ts',
      'packages/schema-domain/src/screen-control-catalog-data-*.ts',
      'packages/schema-domain/src/tracking-control-catalog-data.ts',
    ],
    rules: {
      'max-lines': 'off',
    },
  }
);

export default architectureRulesEnabled ? architectureRuleConfig : standardConfig;
