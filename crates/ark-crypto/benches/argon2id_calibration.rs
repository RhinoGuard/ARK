//! Reproducible Argon2id calibration harness.
//!
//! Run with:
//!   cargo bench -p ark-crypto --bench argon2id_calibration
//!
//! Optional environment variables:
//!   ARK_BENCH_WARMUPS=2
//!   ARK_BENCH_SAMPLES=9
//!
//! This reports wall-clock derivation latency and the configured Argon2 memory
//! cost. The latter is the KDF memory-block allocation target, not a measurement
//! of total process RSS or peak system memory.

use std::env;
use std::hint::black_box;
use std::time::{Duration, Instant};

use ark_crypto::kdf::{Argon2idConfig, derive_kek};

const PASSWORD: &[u8] = b"ARK Argon2id calibration input - non-secret benchmark fixture";
const SALT: &[u8; 32] = b"ARK-bench-salt-2026-11-24-000001";

#[derive(Clone, Copy)]
struct Profile {
    name: &'static str,
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
}

const PROFILES: [Profile; 6] = [
    Profile {
        name: "minimum-floor",
        memory_kib: 16 * 1024,
        iterations: 1,
        lanes: 1,
    },
    Profile {
        name: "low-memory",
        memory_kib: 32 * 1024,
        iterations: 2,
        lanes: 1,
    },
    Profile {
        name: "baseline-default",
        memory_kib: 64 * 1024,
        iterations: 3,
        lanes: 1,
    },
    Profile {
        name: "parallel-baseline",
        memory_kib: 64 * 1024,
        iterations: 3,
        lanes: 2,
    },
    Profile {
        name: "high-memory-desktop",
        memory_kib: 128 * 1024,
        iterations: 3,
        lanes: 2,
    },
    Profile {
        name: "upper-policy-profile",
        memory_kib: 256 * 1024,
        iterations: 4,
        lanes: 4,
    },
];

fn env_count(name: &str, default: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| (1..=100).contains(value))
        .unwrap_or(default)
}

fn percentile(sorted: &[u128], numerator: usize, denominator: usize) -> u128 {
    let index = ((sorted.len() - 1) * numerator).div_ceil(denominator);
    sorted[index]
}

fn run_profile(profile: Profile, warmups: usize, samples: usize) -> Result<(), String> {
    let config = Argon2idConfig::new(profile.memory_kib, profile.iterations, profile.lanes)
        .map_err(|_| format!("invalid benchmark profile: {}", profile.name))?;

    for _ in 0..warmups {
        let output = derive_kek(black_box(PASSWORD), black_box(SALT), config)
            .map_err(|_| format!("Argon2id warm-up failed: {}", profile.name))?;
        black_box(output);
    }

    let mut elapsed_ns = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        let output = derive_kek(black_box(PASSWORD), black_box(SALT), config)
            .map_err(|_| format!("Argon2id sample failed: {}", profile.name))?;
        let elapsed = started.elapsed();
        black_box(output);
        elapsed_ns.push(elapsed.as_nanos());
    }

    elapsed_ns.sort_unstable();
    let total: u128 = elapsed_ns.iter().sum();
    let mean = total / samples as u128;
    let median = percentile(&elapsed_ns, 1, 2);
    let p95 = percentile(&elapsed_ns, 95, 100);
    let min = elapsed_ns[0];
    let max = elapsed_ns[elapsed_ns.len() - 1];

    println!(
        "{},{},{},{},{},{:.3},{:.3},{:.3},{:.3},{:.3}",
        profile.name,
        profile.memory_kib,
        profile.iterations,
        profile.lanes,
        profile.memory_kib / 1024,
        mean as f64 / 1_000_000.0,
        median as f64 / 1_000_000.0,
        p95 as f64 / 1_000_000.0,
        min as f64 / 1_000_000.0,
        max as f64 / 1_000_000.0,
    );

    Ok(())
}

fn main() {
    let warmups = env_count("ARK_BENCH_WARMUPS", 2);
    let samples = env_count("ARK_BENCH_SAMPLES", 9);
    let started = Instant::now();

    println!("# ark-crypto Argon2id calibration");
    println!("# warmups={warmups}, samples={samples}");
    println!(
        "# rustc={}, os={}, arch={}",
        option_env!("RUSTC_VERSION").unwrap_or("record rustc --version separately"),
        env::consts::OS,
        env::consts::ARCH
    );
    println!(
        "profile,memory_kib,iterations,lanes,configured_memory_mib,mean_ms,median_ms,p95_ms,min_ms,max_ms"
    );

    for profile in PROFILES {
        if let Err(error) = run_profile(profile, warmups, samples) {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }

    let total: Duration = started.elapsed();
    eprintln!("total_elapsed_seconds={:.3}", total.as_secs_f64());
}
