#!/usr/bin/env node
import { runLegacyCheck } from './enforcer/run-legacy-check.mjs';

runLegacyCheck('check-no-skipped-focused-tests');
