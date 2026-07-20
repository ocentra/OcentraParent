#!/usr/bin/env node

import { listAdminBillingAccounts } from '../src/fixtures.js';
import { runLocalSeedMutation } from './local-seed-runtime.js';

const mutationReceipt = await runLocalSeedMutation('support-admin-test-accounts');

console.log(
  JSON.stringify(
    {
      generatedAt: '2026-06-14T00:00:00.000Z',
      accounts: listAdminBillingAccounts(null),
      mutationReceipt,
    },
    null,
    2
  )
);
