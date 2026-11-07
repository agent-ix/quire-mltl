//! Compile-time consumer contract for the public request input (FR-001-AC-10).

use quire_mltl::request::{ObservationInputs, RequestInput, TemporalInput};
use quire_observation::authority::{availability, clock, closure, completeness, progress};
use tl_mltl::past::history::ValidatedPositionHistory;
use tl_mltl::wire::ValidatedTrace;
use tl_syntax::{FormulaDocument, PropositionMapDocument};

/// Trace: FR-001-AC-10
#[test]
fn request_input_accepts_cohort_owner_types() {
    fn observations<'a>(
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

    fn request<'a>(
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

    fn future(trace: &ValidatedTrace) -> TemporalInput<'_> {
        TemporalInput::Future(trace)
    }

    fn past(history: &ValidatedPositionHistory) -> TemporalInput<'_> {
        TemporalInput::Past(history)
    }

    let _ = (observations, request, future, past);
}
