#!/usr/bin/env node

import { listAdminBillingAccounts } from '../src/fixtures.js';

console.log(
  JSON.stringify(
    {
      generatedAt: '2026-06-14T00:00:00.000Z',
      accounts: listAdminBillingAccounts(null),
    },
    null,
    2
  )
);
