//! Bridges tl-mltl's TL-owned evaluation types to quire-observation's
//! owner-assertion views. The one deliberate exception to tl-mltl's
//! independence from the agent-ix/Quire ecosystem.
//!
//! This crate is scaffolding only (TL-176). The `tl-mltl`, `quire-observation`
//! and `tl-syntax` dependencies declared in `Cargo.toml` are pre-declared and
//! pinned for the bridging implementation that lands in TL-178; nothing here
//! uses them yet.

#![warn(missing_docs)]
// TL-176 is scaffolding only: `tl-mltl`, `quire-observation` and `tl-syntax`
// are pre-declared and pinned in Cargo.toml for the TL-178 port, which is the
// ticket that will actually use them. Remove this allow once that port lands.
#![allow(unused_crate_dependencies)]

/// Placeholder entry point. Removed once the TL-178 port lands real
/// bridging types in its place.
pub fn hello() -> &'static str {
    "hello from quire_mltl"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_greeting() {
        assert!(hello().contains("quire_mltl"));
    }
}
