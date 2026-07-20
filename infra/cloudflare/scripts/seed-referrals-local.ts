#!/usr/bin/env node

import { LOCAL_REFERRALS } from '../src/fixtures.js';
import { runLocalSeedMutation } from './local-seed-runtime.js';

const mutationReceipt = await runLocalSeedMutation('referral-test-graph');

console.log(
  JSON.stringify(
    {
      generatedAt: '2026-06-14T00:00:00.000Z',
      referrals: LOCAL_REFERRALS,
      mutationReceipt,
    },
    null,
    2
  )
);
