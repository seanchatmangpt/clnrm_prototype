//! Dev-dependency contract for the v26.9.26 dev-dependencies bump (PR #17).
//!
//! The PR claims five dev-dependencies resolve to exact versions. This file turns
//! that claim into falsifiers over the real `Cargo.lock` and the real crates:
//!
//! - lockfile guard: every bumped package resolves exactly once, at the claimed
//!   version, from crates.io, with the claimed sha256 checksum (stale subject,
//!   wrong digest, duplicate delivery);
//! - anti-vacuity: the guard refuses mutated lockfile text (downgrade, digest
//!   flip, duplicated entry, malformed version, missing package);
//! - each bumped crate is exercised through its real API against real clnrm
//!   code or a real subprocess (no test doubles).
//!
//! Chicago style: no mocks; `mockito` is only checked in the lockfile guard.

use assert_cmd::Command;
use clnrm::determinism::DeterministicPortAllocator;
use clnrm::ContainerId;
use predicates::prelude::*;
use proptest::prelude::*;
use std::time::{Duration, Instant};

/// (package, exact version, crates.io sha256 checksum) claimed by PR #17.
const CLAIMED: [(&str, &str, &str); 5] = [
    (
        "proptest",
        "1.11.0",
        "4b45fcc2344c680f5025fe57779faef368840d0bd1f42f216291f0dc4ace4744",
    ),
    (
        "insta",
        "1.48.0",
        "86f0f8fee8c926415c58d6ae43a08523a26faccb2323f5e6b644fe7dd4ef6b82",
    ),
    (
        "assert_cmd",
        "2.2.2",
        "2aa3a22042e45de04255c7bf3626e239f450200fd0493c1e382263544b20aea6",
    ),
    (
        "predicates",
        "3.1.4",
        "ada8f2932f28a27ee7b70dd6c1c39ea0675c55a36879ab92f3a715eaa1e63cfe",
    ),
    (
        "mockito",
        "1.7.2",
        "90820618712cab19cfc46b274c6c22546a82affcb3c3bdf0f29e3db8e1bb92c0",
    ),
];

const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// Typed refusal returned by the lockfile guard.
#[derive(Debug, PartialEq, Eq)]
enum LockRefusal {
    Malformed(String),
    Missing(String),
    Duplicate(String),
    WrongVersion { name: String, found: String },
    WrongSource(String),
    WrongDigest(String),
}

fn parse_semver(v: &str) -> Option<(u64, u64, u64)> {
    let mut it = v.split('.');
    let out = (
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
    );
    if it.next().is_some() {
        return None;
    }
    Some(out)
}

/// Admit a lockfile text only if every claimed package is present exactly once
/// with the claimed version, source and checksum.
fn admit_lock(text: &str) -> Result<(), LockRefusal> {
    let table: toml::Table =
        toml::from_str(text).map_err(|e| LockRefusal::Malformed(e.to_string()))?;
    let packages = table
        .get("package")
        .and_then(|p| p.as_array())
        .ok_or_else(|| LockRefusal::Malformed("no [[package]] array".into()))?;
    for (name, version, checksum) in CLAIMED {
        let hits: Vec<&toml::Table> = packages
            .iter()
            .filter_map(|p| p.as_table())
            .filter(|p| p.get("name").and_then(|n| n.as_str()) == Some(name))
            .collect();
        let pkg = match hits.as_slice() {
            [] => return Err(LockRefusal::Missing(name.into())),
            [one] => *one,
            _ => return Err(LockRefusal::Duplicate(name.into())),
        };
        let found = pkg.get("version").and_then(|v| v.as_str()).unwrap_or("");
        if parse_semver(found).is_none() {
            return Err(LockRefusal::Malformed(format!("{name} version {found:?}")));
        }
        if found != version {
            return Err(LockRefusal::WrongVersion {
                name: name.into(),
                found: found.into(),
            });
        }
        if pkg.get("source").and_then(|s| s.as_str()) != Some(CRATES_IO) {
            return Err(LockRefusal::WrongSource(name.into()));
        }
        if pkg.get("checksum").and_then(|c| c.as_str()) != Some(checksum) {
            return Err(LockRefusal::WrongDigest(name.into()));
        }
    }
    Ok(())
}

fn real_lock() -> String {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"))
        .expect("Cargo.lock readable")
}

fn real_manifest() -> String {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("Cargo.toml readable")
}

#[test]
fn real_lockfile_is_admitted() {
    assert_eq!(admit_lock(&real_lock()), Ok(()));
}

#[test]
fn manifest_floors_match_claimed_versions() {
    // A manifest floor below the lock lets `cargo update` silently downgrade to a
    // pre-bump version; the floors must pin at least the claimed releases.
    let manifest: toml::Table = toml::from_str(&real_manifest()).expect("manifest parses");
    let dev = manifest["dev-dependencies"]
        .as_table()
        .expect("dev-dependencies");
    let floors = [
        ("proptest", "1.11"),
        ("insta", "1.48"),
        ("assert_cmd", "2.2"),
        ("predicates", "3.1.4"),
        ("mockito", "1.7.2"),
    ];
    for (name, floor) in floors {
        let req = dev[name]
            .as_str()
            .unwrap_or_else(|| panic!("{name} is a plain version"));
        assert_eq!(req, floor, "{name} manifest floor");
    }
}

#[test]
fn guard_refuses_downgraded_lock() {
    let mutated = real_lock().replacen(
        "name = \"proptest\"\nversion = \"1.11.0\"",
        "name = \"proptest\"\nversion = \"1.9.0\"",
        1,
    );
    assert_eq!(
        admit_lock(&mutated),
        Err(LockRefusal::WrongVersion {
            name: "proptest".into(),
            found: "1.9.0".into()
        })
    );
}

#[test]
fn guard_refuses_flipped_digest() {
    let good = CLAIMED[1].2;
    let mut bad = good.to_string();
    bad.replace_range(0..1, if good.starts_with('0') { "1" } else { "0" });
    let mutated = real_lock().replacen(good, &bad, 1);
    assert_eq!(
        admit_lock(&mutated),
        Err(LockRefusal::WrongDigest("insta".into()))
    );
}

#[test]
fn guard_refuses_duplicate_delivery() {
    let dup = format!(
        "{}\n[[package]]\nname = \"mockito\"\nversion = \"1.7.2\"\nsource = \"{CRATES_IO}\"\n",
        real_lock()
    );
    assert_eq!(
        admit_lock(&dup),
        Err(LockRefusal::Duplicate("mockito".into()))
    );
}

#[test]
fn guard_refuses_missing_and_malformed() {
    let missing = real_lock().replace("name = \"assert_cmd\"", "name = \"assert_cmd_renamed\"");
    assert_eq!(
        admit_lock(&missing),
        Err(LockRefusal::Missing("assert_cmd".into()))
    );

    let bad_version = real_lock().replacen(
        "name = \"predicates\"\nversion = \"3.1.4\"",
        "name = \"predicates\"\nversion = \"3.1\"",
        1,
    );
    assert!(matches!(
        admit_lock(&bad_version),
        Err(LockRefusal::Malformed(_))
    ));

    assert!(matches!(
        admit_lock("[[package"),
        Err(LockRefusal::Malformed(_))
    ));
    assert!(matches!(admit_lock(""), Err(LockRefusal::Malformed(_))));
}

#[test]
fn guard_refuses_foreign_source() {
    let mutated = real_lock().replacen(
        &format!("name = \"predicates\"\nversion = \"3.1.4\"\nsource = \"{CRATES_IO}\""),
        "name = \"predicates\"\nversion = \"3.1.4\"\nsource = \"git+https://example.invalid/predicates\"",
        1,
    );
    assert_eq!(
        admit_lock(&mutated),
        Err(LockRefusal::WrongSource("predicates".into()))
    );
}

#[test]
fn guard_latency_regression_bound() {
    // Deterministic timing bound: admitting the real lockfile 20 times must keep a
    // median under 250ms per admission even in an unoptimized test build.
    let text = real_lock();
    let mut samples: Vec<Duration> = (0..20)
        .map(|_| {
            let t = Instant::now();
            admit_lock(&text).expect("admitted");
            t.elapsed()
        })
        .collect();
    samples.sort();
    let median = samples[samples.len() / 2];
    assert!(median < Duration::from_millis(250), "median {median:?}");
}

proptest! {
    // proptest 1.11: property over real clnrm determinism code.
    #[test]
    fn deterministic_ports_are_sequential_and_releasable(base in 1024u16..60000, n in 1usize..64) {
        let mut alloc = DeterministicPortAllocator::new(base);
        let ports: Vec<u16> = (0..n).map(|_| alloc.allocate_port().expect("port")).collect();
        let expected: Vec<u16> = (0..n as u16).map(|i| base + i).collect();
        prop_assert_eq!(&ports, &expected);
        prop_assert_eq!(alloc.get_allocated_ports(), expected.as_slice());
        for p in &ports {
            prop_assert!(alloc.release_port(*p).is_ok());
        }
        prop_assert!(alloc.release_port(base).is_err(), "double release must be refused");
        prop_assert!(alloc.get_allocated_ports().is_empty());
    }

    #[test]
    fn container_id_value_round_trips_and_zero_is_refused(v in 1u64..u64::MAX) {
        let id = ContainerId::from_value(v).expect("nonzero admitted");
        prop_assert_eq!(id.as_value(), v);
        prop_assert_eq!(u64::from(id), v);
        prop_assert_eq!(ContainerId::try_from(v).map(|i| i.value()).ok(), Some(v));
        prop_assert!(ContainerId::from_value(0).is_none(), "zero id must be refused");
    }
}

#[test]
fn insta_inline_snapshot_of_refusal_shape() {
    let refusal = admit_lock(&real_lock().replacen(
        "name = \"assert_cmd\"\nversion = \"2.2.2\"",
        "name = \"assert_cmd\"\nversion = \"2.1.1\"",
        1,
    ));
    insta::assert_snapshot!(
        format!("{refusal:?}"),
        @r#"Err(WrongVersion { name: "assert_cmd", found: "2.1.1" })"#
    );
}

#[test]
fn assert_cmd_runs_real_micro_cli_in_isolated_dir() {
    let dir = tempfile::tempdir().expect("tempdir");
    Command::cargo_bin("micro_cli")
        .expect("micro_cli built")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "File content matches expected content",
        ))
        .stdout(predicate::str::contains("File successfully removed"));
    // Consequence check: the CLI left no residue in its working directory.
    assert_eq!(std::fs::read_dir(dir.path()).expect("readable").count(), 0);
}

#[test]
fn predicates_reject_what_they_should() {
    let p = predicate::str::contains("1.11.0").and(predicate::str::starts_with("proptest"));
    assert!(p.eval("proptest 1.11.0"));
    assert!(!p.eval("proptest 1.9.0"));
    assert!(!p.eval("insta 1.11.0"));
}

#[test]
fn port_allocator_refuses_u16_overflow_instead_of_panicking() {
    // Adversarial boundary found while hardening PR #17: base + counter used an
    // unchecked u16 add, which panicked (debug) or wrapped to a privileged port
    // (release) at the top of the port range.
    let mut alloc = DeterministicPortAllocator::new(u16::MAX);
    assert_eq!(alloc.allocate_port().expect("last port"), u16::MAX);
    assert!(
        alloc.allocate_port().is_err(),
        "overflow must be a typed refusal"
    );
    assert_eq!(alloc.get_allocated_ports(), &[u16::MAX]);
}

#[test]
fn port_allocator_counter_survives_release_churn_to_top_of_range() {
    // Releasing keeps len() under the 1000 cap, so the counter itself can walk the
    // whole u16 range; the last port must still be issued, then refused.
    let mut alloc = DeterministicPortAllocator::new(0);
    let mut last = 0;
    for _ in 0..=u16::MAX as u32 {
        last = alloc.allocate_port().expect("in range");
        alloc.release_port(last).expect("release");
    }
    assert_eq!(last, u16::MAX);
    assert!(alloc.allocate_port().is_err());
}

#[test]
fn port_alloc_release_latency_regression_bound() {
    // Criterion release-profile baseline (docs/bench/dev-deps-contract-v26.9.26.json):
    // ~73us per 1000 allocate+release. Unoptimized bound: median < 50ms.
    let mut samples: Vec<Duration> = (0..11)
        .map(|_| {
            let t = Instant::now();
            let mut a = DeterministicPortAllocator::new(20000);
            for _ in 0..1000 {
                a.allocate_port().expect("port");
            }
            for p in 20000u16..21000 {
                a.release_port(p).expect("release");
            }
            t.elapsed()
        })
        .collect();
    samples.sort();
    let median = samples[samples.len() / 2];
    assert!(median < Duration::from_millis(50), "median {median:?}");
}
