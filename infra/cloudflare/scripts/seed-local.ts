#!/usr/bin/env node

import { buildLocalSeedSnapshot } from '../src/billing-binding-read-model.js';
import type { Env } from '../src/env.js';

const env: Env = {
  ENVIRONMENT: 'development',
  APP_ORIGIN: 'http://localhost:3000',
  CORS_ALLOWED_ORIGINS: 'http://localhost:3000',
  AUTH_ADAPTER_MODE: 'local-safe-fixture',
  ENTITLEMENT_SIGNING_KEY_REF: 'manual-required-local-ref',
};

console.log(JSON.stringify(buildLocalSeedSnapshot(env), null, 2));
