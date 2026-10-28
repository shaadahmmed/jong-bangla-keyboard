// MIT. Runs our authored fixtures against an external, pinned reference file.
// The reference is not bundled with or used by the Rust library.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import vm from "node:vm";

if (process.argv.length !== 3) {
  console.error("Usage: node tools/check-reference.mjs /path/to/avro-lib.js");
  process.exit(2);
}

const source = readFileSync(process.argv[2]);
assert.equal(
  createHash("sha256").update(source).digest("hex"),
  "312ed3438ebf0a69f6de38036572ec39cdbafe3ef53f500caa402aaa704c2b32",
  "Reference must be src/avro-lib.js from the pinned v1.1.4 revision",
);
const context = vm.createContext({});
vm.runInContext(source.toString("utf8"), context, { timeout: 5000 });
const parse = (input) => context.OmicronLab.Avro.Phonetic.parse(input);
const codePoints = (text) =>
  [...text]
    .map((ch) => `U+${ch.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")}`)
    .join(" ");

let count = 0;
for (const name of [
  "reference.tsv",
  "vowels.tsv",
  "vowels-u.tsv",
  "vowels-i-long.tsv",
  "vowels-e.tsv",
  "vowels-o.tsv",
  "vowels-oi-ou.tsv",
  "vowels-rri.tsv",
  "consonants-r.tsv",
]) {
  const fixture = readFileSync(
    new URL(`../tests/fixtures/${name}`, import.meta.url),
    "utf8",
  );
  let fileCount = 0;
  for (const line of fixture.trimEnd().split("\n")) {
    if (line.startsWith("#")) continue;
    const columns = line.split("\t");
    assert.equal(columns.length, 3, `Invalid fixture: ${line}`);
    const [input, expected, points] = columns;
    const actual = parse(input);
    assert.equal(actual, expected, `Reference output for ${JSON.stringify(input)}`);
    assert.equal(codePoints(actual), points, `Code points for ${JSON.stringify(input)}`);
    console.log(`${JSON.stringify(input)} -> ${actual} (${points})`);
    fileCount += 1;
  }
  assert.ok(fileCount > 0, `${name} must contain cases`);
  count += fileCount;
}
console.log(`PASS: ${count} authored cases; runtime ${process.version}`);

// These probes expose missing behavior; they are not supported-slice fixtures.
for (const input of ["kr", "rrk", "rrr", "Rri", "rri`"]) {
  const actual = parse(input);
  console.log(`GAP reference ${JSON.stringify(input)} -> ${actual} (${codePoints(actual)})`);
}
