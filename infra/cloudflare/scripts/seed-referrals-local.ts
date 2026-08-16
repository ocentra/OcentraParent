#!/usr/bin/env node

import { LOCAL_REFERRALS } from '../src/fixtures.js';

console.log(
  JSON.stringify(
    {
      generatedAt: '2026-06-14T00:00:00.000Z',
      referrals: LOCAL_REFERRALS,
    },
    null,
    2
  )
);
