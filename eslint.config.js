import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import ocentraParentRules from './eslint-rules/index.js';

export default tseslint.config(
  {
    ignores: [
      '**/dist/**',
      '**/node_modules/**',
      '**/coverage/**',
      '**/.turbo/**',
      '**/target/**',
      '**/*.d.ts',
      '**/*.tsbuildinfo',
    ],
  },
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
          max: 240,
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
  }
);
