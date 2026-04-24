//! Orquestracao de alto nivel do pipeline do scanner.

pub mod steps;

pub use steps::{
    AnnotatedRuleRegex, UnifiedRegexSpec, build_unified_regex_spec, generate_scanner_artifacts,
};
