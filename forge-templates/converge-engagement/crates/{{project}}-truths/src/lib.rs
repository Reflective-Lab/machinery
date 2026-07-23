//! Compiled truths — business invariants this engagement promises to enforce.
//!
//! Today these are reference constants exposed via the dashboard.
//! When a truth graduates to a gate, run `cz codegen` (Axiom) to emit
//! Rust invariants and register them as Suggestors in `-app`.

#[derive(Debug, Clone, Copy)]
pub struct TruthSpec {
    pub key: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub source: &'static str,
}

pub const TRUTHS: &[TruthSpec] = &[
    // TruthSpec {
    //     key: "example-invariant",
    //     title: "Example Invariant",
    //     summary: "One-sentence statement of what must always hold.",
    //     source: "kb/rules/example-invariant.md",
    // },
];

#[must_use]
pub fn truth_by_key(key: &str) -> Option<TruthSpec> {
    TRUTHS.iter().copied().find(|t| t.key == key)
}
