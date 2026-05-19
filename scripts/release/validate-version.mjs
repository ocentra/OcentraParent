import { evaluateReleaseVersionPolicy } from './version-policy.mjs';

const jsonOutput = process.argv.includes('--json');
const printVersion = process.argv.includes('--print-version');
const result = evaluateReleaseVersionPolicy(process.cwd());

if (!result.ok) {
  if (jsonOutput) {
    console.error(JSON.stringify(result, null, 2));
  } else {
    console.error('Release version policy failed:');
    for (const finding of result.findings) {
      console.error(`- ${finding}`);
    }
  }
  process.exit(1);
}

if (printVersion) {
  console.log(result.version);
} else if (jsonOutput) {
  console.log(JSON.stringify(result, null, 2));
} else {
  console.log(`Release version ${result.version} is aligned across ${result.checkedSources.length} source(s).`);
}
