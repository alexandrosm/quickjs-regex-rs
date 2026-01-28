//! # quickjs-regex
//!
//! A JavaScript-compatible regex engine translated from QuickJS.
//!
//! This crate provides a regex engine that is fully compatible with
//! ECMAScript regex semantics, translated from Fabrice Bellard's QuickJS.

// Required for c2rust generated code (temporary - will be removed during refinement)
#![feature(c_variadic)]

mod generated;
mod regex;

// Re-export the public API
pub use regex::*;
