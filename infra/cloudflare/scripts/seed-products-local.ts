#!/usr/bin/env node

import { LOCAL_PRICING_PLANS } from '../src/fixtures.js';

console.log(
  JSON.stringify(
    {
      generatedAt: '2026-06-14T00:00:00.000Z',
      pricingPlans: LOCAL_PRICING_PLANS,
    },
    null,
    2
  )
);
