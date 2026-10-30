# Performance measurements

Run these commands from the repository root after the
[development checks](development.md) finish. Recorded results are observations
from specific environments, not release performance guarantees.

## Measure core latency

```sh
cargo run --offline --release --example measure > target/core-latency.csv
```

The dependency-free runner measures seven short inputs and mixed buffers of
32, 128, 1,024 and 8,192 Unicode scalars. It includes one-shot conversion,
short-word typing/commit and insert/Backspace pairs at long buffers' ends.
It prints nanoseconds per action: minimum, median and 95th percentile of 31
batch means after adaptive calibration. Those percentiles describe batches,
not individual keystroke tail latency. Allocations/output destruction are timed;
long-buffer setup is excluded. The runner rejects debug builds. Run it without
concurrent builds/tests and record hardware, toolchain and load conditions.
The [local measurements](v1.md#local-resource-measurement-and-leak-check)
record two core release runs. Separate
[adapter measurements](v1.md#synthetic-adapter-resource-measurements)
compare dispatch time and memory with an explicit Fcitx baseline.

## Measure synthetic adapter resources

On Linux/glibc, with Python 3 and `nm` from binutils available:

```sh
cmake -S adapters/fcitx5 -B target/fcitx5-release -G Ninja \
  -DCMAKE_BUILD_TYPE=Release -DJONG_BUILD_RESOURCE_PROBE=ON
cmake --build target/fcitx5-release
ctest --test-dir target/fcitx5-release --output-on-failure
python3 tools/measure-adapter.py target/fcitx5-release --output target/adapter-resources
```

Run measurements after builds/checks finish. The optional probe uses the real
dispatcher and addon, isolated temporary XDG paths and no desktop frontend.
Three sequential rounds alternate baseline/Jong order. Each mode/workload class
starts in a fresh process. The baseline loads only a static passthrough engine;
Jong adds its shared addon and input-method entry. Both receive identical keys
through identical synthetic contexts, with no retained document or effect log.
The runner rejects non-Release and sanitizer-instrumented builds, and
`LD_PRELOAD`/`LD_AUDIT`; the probe is disabled in normal builds.

`timing.csv` records nanoseconds per key press for four typing/Space cycles and
insert/Backspace pairs at 32/128/1,024/8,192-scalar buffer ends. It uses 31 batch
means with adaptive >=5 ms calibration; setup is excluded. The Jong-minus-baseline
difference includes conversion, the C ABI, adapter and synchronous preedit work;
it does not isolate bridge cost or individual-event tail latency.

`memory.csv` records eight checkpoints: idle, short word, short cleared, long
word, long cleared, 1,000/2,000 repeated sessions and context destruction. The
parent reads `/proc/PID/smaps_rollup` while the probe waits, capturing RSS, PSS and
private pages. `os_hwm_kib` preserves the kernel's approximate `VmHWM` counter;
the maximum snapshot RSS is only an observed peak, not an exact transient peak.
glibc `mallinfo2` records allocator-accounted used/reserved bytes, including
allocator caches rather than exact Rust capacities. No allocator trimming is
forced. `metadata.json` records build/source hashes, versions and method.
These results measure an isolated dispatcher, not a running desktop daemon or
frontend. See the [recorded results and limits](v1.md#synthetic-adapter-resource-measurements).
