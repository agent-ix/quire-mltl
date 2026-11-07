//! Outside-consumer contract for the public request input (FR-001-AC-10).
//!
//! A generated crate declares the `branch = "main"` tl-syntax, tl-mltl and
//! quire-observation dependencies itself and hands their types to
//! `quire_mltl::request`. It only compiles when those dependencies resolve to
//! the same packages quire-mltl uses.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ix_trace_rs::trace;

const CONSUMER_MAIN: &str = r#"use quire_mltl::request::{ObservationInputs, RequestInput, TemporalInput};
use quire_observation::authority::{availability, clock, closure, completeness, progress};
use tl_mltl::past::history::ValidatedPositionHistory;
use tl_mltl::wire::ValidatedTrace;
use tl_syntax::{FormulaDocument, PropositionMapDocument};

pub fn observations<'a>(
    progress_view: &'a progress::View,
    closure_view: &'a closure::View,
    completeness_view: &'a completeness::View,
    availability_view: &'a availability::View,
) -> ObservationInputs<'a> {
    ObservationInputs {
        decision_scope_progress: progress_view,
        decision_scope_closure: closure_view,
        surrounding_execution_progress: progress_view,
        surrounding_execution_closure: closure_view,
        completeness: completeness_view,
        availability: availability_view,
    }
}

pub fn request<'a>(
    formula: &'a FormulaDocument,
    proposition_map: &'a PropositionMapDocument,
    temporal_input: TemporalInput<'a>,
    clock_view: &'a clock::View,
    observations: ObservationInputs<'a>,
) -> RequestInput<'a> {
    RequestInput {
        formula,
        proposition_map,
        input: temporal_input,
        clock: clock_view,
        subject_identity: "subject",
        correspondence_identity: "correspondence",
        anchor: 0,
        observations,
    }
}

pub fn future(trace: &ValidatedTrace) -> TemporalInput<'_> {
    TemporalInput::Future(trace)
}

pub fn past(history: &ValidatedPositionHistory) -> TemporalInput<'_> {
    TemporalInput::Past(history)
}
"#;

fn consumer_manifest(quire_mltl: &Path) -> String {
    format!(
        r#"[package]
name = "outside-consumer"
version = "0.0.0"
edition = "2021"
publish = false

[workspace]

[lib]
path = "src/lib.rs"

[dependencies]
quire-mltl = {{ path = {path:?} }}
tl-mltl = {{ version = "=0.3.0", git = "https://github.com/agent-ix/tl-mltl", branch = "main" }}
quire-observation = {{ version = "=0.1.0", git = "https://github.com/agent-ix/quire-observation", branch = "main" }}
tl-syntax = {{ version = "=0.3.0", git = "https://github.com/agent-ix/tl-syntax.git", branch = "main", default-features = false, features = ["alloc", "serde"] }}
"#,
        path = quire_mltl.display().to_string()
    )
}

#[trace("FR-001-AC-10")]
#[test]
fn outside_consumer_passes_main_cohort_types_into_request_input() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let consumer = root.join("target").join("outside-consumer");
    fs::create_dir_all(consumer.join("src")).expect("create consumer src dir");
    fs::write(consumer.join("Cargo.toml"), consumer_manifest(&root)).expect("write manifest");
    fs::write(consumer.join("src/lib.rs"), CONSUMER_MAIN).expect("write consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--offline", "--manifest-path"])
        .arg(consumer.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", consumer.join("target"))
        .output()
        .expect("spawn nested cargo");

    assert!(
        output.status.success(),
        "outside consumer failed to compile against RequestInput:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
