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
  "conjunct-sth.tsv",
  "conjunct-st.tsv",
  "conjunct-s-t-retroflex.tsv",
  "conjunct-sh-th-retroflex.tsv",
  "conjunct-sh-t-retroflex.tsv",
  "conjunct-sh-lowercase-t.tsv",
  "conjunct-s-uppercase-t.tsv",
  "s-stop-controls.tsv",
  "conjunct-ns.tsv",
  "conjunct-nsh.tsv",
  "conjunct-ps.tsv",
  "conjunct-psh.tsv",
  "ns-ps-controls.tsv",
  "conjunct-bb.tsv",
  "conjunct-pp.tsv",
  "conjunct-pn.tsv",
  "conjunct-sn.tsv",
  "conjunct-sh-lowercase-n.tsv",
  "conjunct-s-uppercase-n.tsv",
  "repeat-n-controls.tsv",
  "conjunct-gl.tsv",
  "conjunct-ll.tsv",
  "conjunct-bl.tsv",
  "conjunct-pl.tsv",
  "conjunct-sl.tsv",
  "conjunct-sh-lowercase-l.tsv",
  "conjunct-bhl.tsv",
  "conjunct-phl.tsv",
  "conjunct-vl.tsv",
  "conjunct-fl.tsv",
  "conjunct-s-uppercase-l.tsv",
  "p-following-r.tsv",
  "ph-following-r.tsv",
  "f-following-r.tsv",
  "l-ending-controls.tsv",
  "conjunct-mbh.tsv",
  "conjunct-mb.tsv",
  "conjunct-mv.tsv",
  "conjunct-mph.tsv",
  "conjunct-mpl.tsv",
  "conjunct-mp.tsv",
  "conjunct-mf.tsv",
  "conjunct-lm.tsv",
  "conjunct-sm.tsv",
  "conjunct-sh-lowercase-m.tsv",
  "conjunct-s-uppercase-m.tsv",
  "m-labial-controls.tsv",
  "conjunct-lgh.tsv",
  "conjunct-lg.tsv",
  "conjunct-lbh.tsv",
  "conjunct-lb.tsv",
  "conjunct-lv.tsv",
  "conjunct-lph.tsv",
  "conjunct-lp.tsv",
  "conjunct-l-t-retroflex.tsv",
  "conjunct-l-d-retroflex.tsv",
  "conjunct-ldh.tsv",
  "l-prefix-controls.tsv",
  "conjunct-sph.tsv",
  "conjunct-spl.tsv",
  "conjunct-sp.tsv",
  "conjunct-sf.tsv",
  "conjunct-sh-ph.tsv",
  "conjunct-sh-p.tsv",
  "conjunct-sh-f.tsv",
  "s-labial-controls.tsv",
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
  "conjunct-ddh.tsv",
  "conjunct-dd.tsv",
  "conjunct-dhn.tsv",
  "conjunct-dhm.tsv",
  "conjunct-dm.tsv",
  "conjunct-dbh.tsv",
  "conjunct-dv.tsv",
  "conjunct-d-retroflex-pair.tsv",
  "d-following-r.tsv",
  "dh-following-r.tsv",
  "d-retroflex-following-r.tsv",
  "dh-retroflex-following-r.tsv",
  "b-following-r.tsv",
  "bh-following-r.tsv",
  "v-following-r.tsv",
  "conjunct-ndh.tsv",
  "conjunct-nd.tsv",
  "conjunct-n-d-retroflex.tsv",
  "conjunct-n-retroflex-dh-retroflex.tsv",
  "conjunct-n-retroflex-d-retroflex.tsv",
  "conjunct-nth.tsv",
  "conjunct-nt.tsv",
  "conjunct-n-th-retroflex.tsv",
  "conjunct-n-t-retroflex.tsv",
  "conjunct-n-retroflex-th-retroflex.tsv",
  "conjunct-n-retroflex-t-retroflex.tsv",
  "t-following-r.tsv",
  "th-following-r.tsv",
  "t-retroflex-following-r.tsv",
  "th-retroflex-following-r.tsv",
  "conjunct-tth.tsv",
  "conjunct-tt.tsv",
  "conjunct-tn.tsv",
  "conjunct-tm.tsv",
  "conjunct-t-retroflex-pair.tsv",
  "conjunct-t-retroflex-m.tsv",
  "conjunct-kt.tsv",
  "conjunct-k-t-retroflex.tsv",
  "conjunct-kl.tsv",
  "k-family-controls.tsv",
  "conjunct-nkh.tsv",
  "conjunct-nk.tsv",
  "conjunct-lkh.tsv",
  "conjunct-lk.tsv",
  "conjunct-skl.tsv",
  "conjunct-skh.tsv",
  "conjunct-sk.tsv",
  "conjunct-sh-k.tsv",
  "conjunct-ksh-priority.tsv",
  "conjunct-ks.tsv",
  "s-following-r.tsv",
  "sh-lowercase-following-r.tsv",
  "s-uppercase-following-r.tsv",
  "consuming-k-controls.tsv",
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
  "ShNG", "bdd", "ShM", "rrShN", "Shm`a",
]) {
  const actual = parse(input);
  console.log(`GAP reference ${JSON.stringify(input)} -> ${actual} (${codePoints(actual)})`);
}
