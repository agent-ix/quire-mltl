//! Bridges tl-mltl's TL-owned evaluation types to quire-observation's
//! owner-assertion views. The one deliberate exception to tl-mltl's
//! independence from the agent-ix/Quire ecosystem.
//!
//! This crate ports `tl-mltl`'s `wire::request`, `wire::observation` (as
//! [`dispatch`]), `wire::report`, and `mapping::contract_ir` modules
//! (TL-178/TL-175). See `spec/spec.md` (MRS-001) and FR-001..FR-003 for the
//! governing requirements.

#![warn(missing_docs)]

pub mod census;
pub mod contract_ir;
pub mod dispatch;
pub mod report;
pub mod request;
