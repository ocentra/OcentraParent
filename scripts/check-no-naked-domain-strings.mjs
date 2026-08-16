#!/usr/bin/env node
import { runLegacyCheck } from './enforcer/run-legacy-check.mjs';

runLegacyCheck('check-no-naked-domain-strings');
