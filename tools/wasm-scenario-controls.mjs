// Keep baseline/scenario JSON as strings: JS numbers cannot preserve all i64 inputs.
import fs from 'node:fs';
import readline from 'node:readline';
import {pathToFileURL} from 'node:url';
import {once} from 'node:events';

const [glue, wasm, controls, destination] = process.argv.slice(2);
if (!destination) throw new Error('Expected glue, wasm, controls and new output paths');
const adapter = await import(pathToFileURL(glue));
adapter.initSync({module: fs.readFileSync(wasm)});
const output = fs.createWriteStream(destination, {flags: 'wx'});
const lines = readline.createInterface({input: fs.createReadStream(controls), crlfDelay: Infinity});
for await (const line of lines) {
  const control = JSON.parse(line);
  let result;
  try { result = {Ok: adapter.compare_json(control.baseline, control.scenario)}; }
  catch (error) {
    if (typeof error !== 'string') throw error;
    result = {Err: error};
  }
  if (!output.write(JSON.stringify(result) + '\n')) await once(output, 'drain');
}
output.end();
await once(output, 'finish');
