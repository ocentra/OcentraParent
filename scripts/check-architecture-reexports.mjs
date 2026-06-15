import { pathToFileURL } from 'node:url';
import { main } from './check-no-reexports.mjs';

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main(process.argv.slice(2));
}
