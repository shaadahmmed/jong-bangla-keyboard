//! Synthetic workloads only. Run with `cargo run --offline --release --example measure`.
use jong_bangla_core::{Composition, transliterate};
use std::hint::black_box;
use std::time::{Duration, Instant};

const SAMPLES: usize = 31;
const TARGET: Duration = Duration::from_millis(5);

// Percentiles describe batch means, not individual keystroke tail latency.
fn measure(label: &str, scalars: usize, actions: usize, mut run: impl FnMut()) {
    let mut iterations = 1_u64;
    loop {
        let start = Instant::now();
        for _ in 0..iterations {
            run();
        }
        if start.elapsed() >= TARGET || iterations >= 1 << 20 {
            break;
        }
        iterations *= 2;
    }
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..iterations {
            run();
        }
        samples.push(start.elapsed().as_nanos() as f64 / iterations as f64 / actions as f64);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "{label},{scalars},{actions},{iterations},{:.1},{:.1},{:.1}",
        samples[0],
        samples[SAMPLES / 2],
        samples[29]
    );
}

fn main() {
    if cfg!(debug_assertions) {
        eprintln!("use --release for measurements");
        std::process::exit(2);
    }
    println!("# 31 samples; adaptive >=5 ms calibration; warm process; no CPU pinning");
    println!(
        "# ns/action includes allocations and output destruction; p95 is a batch-mean percentile"
    );
    println!("workload,raw_scalars,actions_per_iteration,iterations,min_ns,median_ns,p95_ns");
    for word in [
        "ami",
        "bangla",
        "krrishTi",
        "shadhInota",
        "okkhor123",
        "k`a",
        "কি🙂",
    ] {
        measure(&format!("convert:{word}"), word.chars().count(), 1, || {
            black_box(transliterate(black_box(word)));
        });
        measure(
            &format!("type_commit:{word}"),
            word.chars().count(),
            word.chars().count() + 1,
            || {
                let mut composition = Composition::new();
                for key in word.chars() {
                    assert!(composition.insert(black_box(key)));
                    black_box(composition.preedit());
                }
                black_box(composition.commit());
            },
        );
    }
    for length in [32, 128, 1024, 8192] {
        let raw: String = "kShmri12`a🙂".chars().cycle().take(length).collect();
        measure("convert_mixed", length, 1, || {
            black_box(transliterate(black_box(&raw)));
        });
        // Setup is untimed. Measure editing at a long buffer's end rather than
        // averaging cheap early keys into the result for the final buffer size.
        let mut composition = Composition::new();
        for key in raw.chars() {
            assert!(composition.insert(key));
        }
        measure("insert_backspace_pair", length, 2, || {
            assert!(composition.insert(black_box('i')));
            black_box(composition.preedit());
            assert!(composition.backspace());
            black_box(composition.preedit());
        });
    }
}
