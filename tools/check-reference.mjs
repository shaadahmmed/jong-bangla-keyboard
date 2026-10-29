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
  "consonants-p-ph-f.tsv",
  "consonants-c-ch-j-jh.tsv",
  "consonants-d-dh.tsv",
  "consonants-l-h.tsv",
  "consonants-s-sh.tsv",
  "consonants-r-retroflex.tsv",
  "consonant-z.tsv",
  "consonants-ng-nj.tsv",
  "sign-ng.tsv",
  "nasal-rri.tsv",
  "consonant-y-uppercase.tsv",
  "consonant-y-lowercase.tsv",
  "consonant-z-uppercase.tsv",
  "ya-following-r.tsv",
  "uppercase-y-following-r.tsv",
  "consonant-q.tsv",
  "consonant-v.tsv",
  "consonant-w.tsv",
  "w-following-r.tsv",
  "consonant-x.tsv",
  "x-following-r.tsv",
  "conjunct-ngx.tsv",
  "conjunct-ngk.tsv",
  "conjunct-ngkh.tsv",
  "conjunct-ngkx.tsv",
  "conjunct-ngkkh.tsv",
  "conjunct-ngksh.tsv",
  "conjunct-kkh.tsv",
  "conjunct-ksh.tsv",
  "conjunct-kkhn.tsv",
  "conjunct-kshn.tsv",
  "conjunct-kxn.tsv",
  "conjunct-kkhm.tsv",
  "conjunct-kshm.tsv",
  "conjunct-kxm.tsv",
  "conjunct-kk.tsv",
  "conjunct-shn.tsv",
  "conjunct-shm.tsv",
  "k-following-r.tsv",
  "kh-following-r.tsv",
  "sh-following-r.tsv",
  "g-following-r.tsv",
  "gh-following-r.tsv",
  "conjunct-ngg.tsv",
  "conjunct-nggh.tsv",
  "conjunct-gg.tsv",
  "conjunct-g-n.tsv",
  "conjunct-nn.tsv",
  "conjunct-h-n.tsv",
  "conjunct-gn.tsv",
  "conjunct-ghn.tsv",
  "conjunct-hn.tsv",
  "conjunct-dg.tsv",
  "conjunct-dgh.tsv",
  "conjunct-r-g.tsv",
  "h-following-r.tsv",
  "n-following-r.tsv",
  "n-retroflex-following-r.tsv",
  "conjunct-ngh.tsv",
  "conjunct-n-lower-pair.tsv",
  "conjunct-n-mixed-pair.tsv",
  "conjunct-mn.tsv",
  "conjunct-hm.tsv",
  "conjunct-hl.tsv",
  "conjunct-nm.tsv",
  "conjunct-n-retroflex-m.tsv",
  "conjunct-mm.tsv",
  "conjunct-ml.tsv",
  "conjunct-ng-m.tsv",
  "conjunct-gm.tsv",
  "m-following-r.tsv",
  "l-following-r.tsv",
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
for (const input of [
  "ShNG", "ddghr", "shn", "bhl", "phl", "shl", "ShM", "rrShN", "Shm`a",
]) {
  const actual = parse(input);
  console.log(`GAP reference ${JSON.stringify(input)} -> ${actual} (${codePoints(actual)})`);
}
