//! Selective Applicative Functor for regex matching.
//!
//! Models regex matching as a selective computation where all possible
//! effects (literal matches, class checks, captures) are declared
//! statically but selected for execution dynamically.
//!
//! This enables:
//! - Static extraction of required/possible literals for prefiltering
//! - Static analysis of which captures are possible
//! - Dynamic short-circuiting of impossible branches
//!
//! Based on "Selective Applicative Functors" (Mokhov et al., ICFP 2019)

use std::collections::HashSet;

// ============================================================================
// Core IR: the selective regex representation
// ============================================================================

/// A regex computation that produces a value of type `bool` (match/no-match)
/// but whose structure can be inspected statically.
#[derive(Debug, Clone)]
pub enum RegexS {
    /// Always succeeds (empty match)
    Pure,
    /// Match a literal string — the most optimizable operation
    Literal(Vec<char>),
    /// Match a single character against a class
    Class(ClassSpec),
    /// Match any character (dot)
    Dot,
    /// Anchor (^, $, \b)
    Anchor(AnchorSpec),
    /// Sequence: match A then B
    Seq(Vec<RegexS>),
    /// Alternation with static branch info: try each, take first match.
    /// The key selective operation — we know ALL branches statically,
    /// but SELECT which to execute based on input.
    Alt(Vec<RegexS>),
    /// Repetition with known bounds
    Repeat {
        sub: Box<RegexS>,
        min: u32,
        max: Option<u32>,
        greedy: bool,
    },
    /// Capture group (index known statically)
    Capture {
        index: u32,
        sub: Box<RegexS>,
    },
    /// Non-capturing group
    Group(Box<RegexS>),
    /// Lookahead (positive or negative) — effect is conditional
    Lookahead {
        sub: Box<RegexS>,
        negative: bool,
    },
    /// Backreference — depends on runtime capture value
    BackRef(u32),
}

#[derive(Debug, Clone)]
pub enum ClassSpec {
    Ranges(Vec<(char, char)>),
    Digit,
    NotDigit,
    Word,
    NotWord,
    Space,
    NotSpace,
}

#[derive(Debug, Clone, Copy)]
pub enum AnchorSpec {
    Start,
    End,
    WordBoundary,
    NotWordBoundary,
}

// ============================================================================
// Static analysis: extract information without executing
// ============================================================================

/// Information extractable from a regex without running it on any input.
#[derive(Debug, Clone)]
pub struct StaticInfo {
    /// Literal strings that MUST appear in any match (conjunction)
    pub required_literals: Vec<String>,
    /// Literal strings that MIGHT appear (for prefilter candidates)
    pub possible_literals: Vec<String>,
    /// First bytes that could start a match
    pub start_bytes: Option<Vec<u8>>,
    /// Whether the pattern is anchored to start
    pub anchored_start: bool,
    /// Whether the pattern is anchored to end
    pub anchored_end: bool,
    /// Minimum possible match length in bytes
    pub min_length: usize,
    /// Maximum possible match length (None = unbounded)
    pub max_length: Option<usize>,
    /// Capture group indices that are always set on match
    pub required_captures: HashSet<u32>,
    /// All capture group indices that could be set
    pub possible_captures: HashSet<u32>,
    /// Whether the pattern can match empty string
    pub can_match_empty: bool,
    /// Whether any branch uses backreferences (requires backtracking)
    pub has_backrefs: bool,
    /// Whether any branch uses lookahead (needs special handling)
    pub has_lookahead: bool,
    /// Whether any repetition uses lazy (non-greedy) quantifiers
    pub has_lazy: bool,
}

impl StaticInfo {
    fn empty() -> Self {
        StaticInfo {
            required_literals: vec![],
            possible_literals: vec![],
            start_bytes: None,
            anchored_start: false,
            anchored_end: false,
            min_length: 0,
            max_length: Some(0),
            required_captures: HashSet::new(),
            possible_captures: HashSet::new(),
            can_match_empty: true,
            has_backrefs: false,
            has_lookahead: false,
            has_lazy: false,
        }
    }
}

/// The "Const" interpreter — extracts static information (over-approximation).
/// This is the selective functor's `Const m` instance where `m` is `StaticInfo`.
pub fn analyze(node: &RegexS) -> StaticInfo {
    match node {
        RegexS::Pure => StaticInfo::empty(),

        RegexS::Literal(chars) => {
            let s: String = chars.iter().collect();
            let byte_len = s.len();
            let start = chars.first().map(|c| {
                let mut buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut buf);
                vec![encoded.as_bytes()[0]]
            });
            StaticInfo {
                required_literals: vec![s.clone()],
                possible_literals: vec![s],
                start_bytes: start,
                anchored_start: false,
                anchored_end: false,
                min_length: byte_len,
                max_length: Some(byte_len),
                required_captures: HashSet::new(),
                possible_captures: HashSet::new(),
                can_match_empty: false,
                has_lazy: false,
                has_backrefs: false,
                has_lookahead: false,
            }
        }

        RegexS::Class(spec) => {
            let start = match spec {
                ClassSpec::Digit => Some((b'0'..=b'9').collect()),
                ClassSpec::Space => Some(vec![b' ', b'\t', b'\n', b'\r']),
                ClassSpec::Ranges(ranges) if !ranges.is_empty() && ranges.len() <= 6 => {
                    // Only generate start_bytes for small, ASCII-dominated classes.
                    // Large Unicode classes (like \p{L}) span too many byte ranges
                    // and would produce an incomplete/misleading prefilter.
                    let mut bytes = vec![];
                    for &(lo, _hi) in ranges {
                        if (lo as u32) < 128 {
                            bytes.push(lo as u8);
                        }
                    }
                    if bytes.is_empty() || bytes.len() > 8 { None } else { Some(bytes) }
                }
                _ => None,
            };
            StaticInfo {
                required_literals: vec![],
                possible_literals: vec![],
                start_bytes: start,
                min_length: 1,
                max_length: Some(4), // max UTF-8 char
                can_match_empty: false,
                ..StaticInfo::empty()
            }
        }

        RegexS::Dot => StaticInfo {
            min_length: 1,
            max_length: Some(4),
            can_match_empty: false,
            ..StaticInfo::empty()
        },

        RegexS::Anchor(spec) => {
            let mut info = StaticInfo::empty();
            match spec {
                AnchorSpec::Start => info.anchored_start = true,
                AnchorSpec::End => info.anchored_end = true,
                _ => {}
            }
            info
        }

        RegexS::Seq(subs) => {
            let mut info = StaticInfo::empty();
            let mut all_literals = String::new();
            let mut collecting_prefix = true;

            for sub in subs {
                let sub_info = analyze(sub);

                // Accumulate required literals from sequence
                info.required_literals.extend(sub_info.required_literals);
                info.possible_literals.extend(sub_info.possible_literals);

                // Collect contiguous literal prefix for start_bytes
                // Skip zero-width assertions (lookahead/lookbehind/anchors) — they don't consume input
                if collecting_prefix {
                    let is_zero_width = matches!(sub,
                        RegexS::Lookahead { .. } | RegexS::Anchor(_));
                    if let RegexS::Literal(chars) = sub {
                        all_literals.extend(chars);
                    } else if is_zero_width {
                        // Zero-width: skip, keep collecting prefix
                    } else {
                        if info.start_bytes.is_none() && !all_literals.is_empty() {
                            info.start_bytes = Some(vec![all_literals.as_bytes()[0]]);
                        } else if info.start_bytes.is_none() {
                            info.start_bytes = sub_info.start_bytes;
                        }
                        collecting_prefix = false;
                    }
                }

                // Length accumulates
                info.min_length += sub_info.min_length;
                info.max_length = match (info.max_length, sub_info.max_length) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                };

                info.can_match_empty = info.can_match_empty && sub_info.can_match_empty;
                info.required_captures.extend(sub_info.required_captures);
                info.possible_captures.extend(sub_info.possible_captures);
                info.has_backrefs = info.has_backrefs || sub_info.has_backrefs;
                info.has_lookahead = info.has_lookahead || sub_info.has_lookahead;
                info.has_lazy = info.has_lazy || sub_info.has_lazy;

                if sub_info.anchored_start && subs.first().map(|s| std::ptr::eq(s, sub)).unwrap_or(false) {
                    info.anchored_start = true;
                }
                if sub_info.anchored_end {
                    info.anchored_end = true;
                }
            }

            if collecting_prefix && !all_literals.is_empty() {
                info.start_bytes = Some(vec![all_literals.as_bytes()[0]]);
                info.required_literals.push(all_literals);
            }

            info
        }

        // The KEY selective operation: alternation.
        // Statically we see ALL branches. The over-approximation unions their info.
        // Required literals become empty (no single literal is required across all branches).
        // Possible literals union all branches.
        RegexS::Alt(alts) => {
            let mut info = StaticInfo::empty();
            let mut all_start_bytes: Vec<u8> = vec![];
            let mut min_len = usize::MAX;
            let mut max_len: Option<usize> = Some(0);
            let mut any_can_match_empty = false;

            // Over-approximation: union of all branch possibilities
            for alt in alts {
                let alt_info = analyze(alt);

                // Possible literals = union of all branches
                info.possible_literals.extend(alt_info.possible_literals);

                // Start bytes = union of all branch start bytes
                if let Some(bytes) = alt_info.start_bytes {
                    all_start_bytes.extend(bytes);
                } else {
                    // One branch has unknown start → can't constrain
                    all_start_bytes.clear();
                }

                min_len = min_len.min(alt_info.min_length);
                max_len = match (max_len, alt_info.max_length) {
                    (Some(a), Some(b)) => Some(a.max(b)),
                    _ => None,
                };

                any_can_match_empty = any_can_match_empty || alt_info.can_match_empty;

                // Captures: possible = union, required = intersection
                info.possible_captures.extend(alt_info.possible_captures);

                info.has_backrefs = info.has_backrefs || alt_info.has_backrefs;
                info.has_lookahead = info.has_lookahead || alt_info.has_lookahead;
                info.has_lazy = info.has_lazy || alt_info.has_lazy;
            }

            // Required literals: only those required by ALL branches
            // (for alternation, typically nothing is universally required)
            info.required_literals.clear();

            info.start_bytes = if all_start_bytes.is_empty() {
                None
            } else {
                all_start_bytes.sort();
                all_start_bytes.dedup();
                Some(all_start_bytes)
            };

            info.min_length = if min_len == usize::MAX { 0 } else { min_len };
            info.max_length = max_len;
            info.can_match_empty = any_can_match_empty;
            info
        }

        RegexS::Repeat { sub, min, max, greedy, .. } => {
            let sub_info = analyze(sub);
            let mut info = StaticInfo::empty();

            info.possible_literals = sub_info.possible_literals;
            info.start_bytes = if *min > 0 { sub_info.start_bytes } else { None };
            info.min_length = sub_info.min_length * (*min as usize);
            info.max_length = match (sub_info.max_length, max) {
                (Some(sub_max), Some(max_count)) => Some(sub_max * (*max_count as usize)),
                _ => None,
            };
            info.can_match_empty = *min == 0 || sub_info.can_match_empty;
            info.possible_captures = sub_info.possible_captures;
            info.has_backrefs = sub_info.has_backrefs;
            info.has_lookahead = sub_info.has_lookahead;
            info.has_lazy = sub_info.has_lazy || !greedy;

            // Required literals: only if min > 0
            if *min > 0 {
                info.required_literals = sub_info.required_literals;
            }

            info
        }

        RegexS::Capture { index, sub } => {
            let mut info = analyze(sub);
            info.possible_captures.insert(*index);
            info.required_captures.insert(*index);
            info
        }

        RegexS::Group(sub) => analyze(sub),

        RegexS::Lookahead { sub, .. } => {
            let mut info = analyze(sub);
            // Lookahead doesn't consume input
            info.min_length = 0;
            info.max_length = Some(0);
            info.can_match_empty = true;
            info.has_lookahead = true;
            info
        }

        RegexS::BackRef(_) => {
            let mut info = StaticInfo::empty();
            info.min_length = 0; // backref could match empty if group matched empty
            info.max_length = None;
            info.can_match_empty = true;
            info.has_backrefs = true;
            info
        }
    }
}

// ============================================================================
// Prefilter generation from static analysis
// ============================================================================

/// A prefilter strategy derived from static analysis of the regex.
#[derive(Debug, Clone)]
pub enum Prefilter {
    /// No useful prefilter — must scan everything
    None,
    /// Pattern is anchored to start — only try position 0
    AnchoredStart,
    /// Look for a single byte to find candidates (byte must start the match)
    SingleByte(u8),
    /// Look for any of these bytes to find candidates (byte starts the match)
    ByteSet(Vec<u8>),
    /// Look for a literal string using memmem (literal starts the match)
    MemmemStart(Vec<u8>),
    /// Look for a literal string that appears INSIDE the match — back up by min_prefix
    MemmemInner { needle: Vec<u8>, min_prefix: usize },
    /// Look for any of these literal strings using Aho-Corasick (literals start the match)
    AhoCorasickStart(Vec<Vec<u8>>),
    /// Look for any of these literals INSIDE the match — back up by min_prefix
    AhoCorasickInner { patterns: Vec<Vec<u8>>, min_prefix: usize },
}

/// Derive the best prefilter from static analysis.
pub fn derive_prefilter(info: &StaticInfo) -> Prefilter {
    if info.anchored_start {
        return Prefilter::AnchoredStart;
    }

    // PRIORITY 1: Required inner literals (most selective — memmem is fast)
    if let Some(best) = info.required_literals.iter()
        .max_by_key(|s| s.len())
        .filter(|s| s.len() >= 3) // At least 3 chars to be worth memmem
    {
        let needle = best.as_bytes().to_vec();
        let at_start = info.start_bytes.as_ref()
            .map(|sb| sb.contains(&needle[0]))
            .unwrap_or(false)
            && info.min_length <= needle.len() + 4;

        if at_start {
            return Prefilter::MemmemStart(needle);
        } else {
            return Prefilter::MemmemInner {
                needle,
                min_prefix: info.min_length,
            };
        }
    }

    // PRIORITY 2: Start bytes (only if highly selective — single byte)
    if let Some(ref bytes) = info.start_bytes {
        if bytes.len() == 1 {
            return Prefilter::SingleByte(bytes[0]);
        }
    }

    // Possible literals from alternation at the start of the pattern
    if info.possible_literals.len() >= 2 {
        let patterns: Vec<Vec<u8>> = info.possible_literals.iter()
            .filter(|s| s.len() >= 2)
            .map(|s| s.as_bytes().to_vec())
            .collect();
        if patterns.len() >= 2 {
            // If min_length == literal length, these are at the start
            let max_pattern_len = patterns.iter().map(|p| p.len()).max().unwrap_or(0);
            if max_pattern_len >= info.min_length.saturating_sub(2) {
                return Prefilter::AhoCorasickStart(patterns);
            } else {
                return Prefilter::AhoCorasickInner {
                    patterns,
                    min_prefix: info.min_length.saturating_sub(max_pattern_len),
                };
            }
        }
    }

    // Required literals from sequence — check if at start or inner
    if let Some(best) = info.required_literals.iter()
        .max_by_key(|s| s.len())
        .filter(|s| s.len() >= 2)
    {
        let needle = best.as_bytes().to_vec();
        // Check if this literal starts the match (start_bytes match literal's first byte)
        let at_start = info.start_bytes.as_ref()
            .map(|sb| sb.contains(&needle[0]))
            .unwrap_or(false)
            && info.min_length <= needle.len() + 4; // small prefix tolerance

        if at_start {
            return Prefilter::MemmemStart(needle);
        } else {
            // Inner literal — need to back up when we find it
            return Prefilter::MemmemInner {
                needle,
                min_prefix: info.min_length,
            };
        }
    }

    Prefilter::None
}

// ============================================================================
// Convert from parser AST to selective IR
// ============================================================================

use super::compiler::parser::{Node, BuiltinClass, AnchorKind, ClassRange};

pub fn from_ast(node: &Node) -> RegexS {
    match node {
        Node::Empty => RegexS::Pure,
        Node::Literal(c) => RegexS::Literal(vec![*c]),
        Node::Dot => RegexS::Dot,

        Node::Class { ranges, negated } => {
            let mut pairs = vec![];
            for r in ranges {
                match r {
                    ClassRange::Single(c) => pairs.push((*c, *c)),
                    ClassRange::Range(lo, hi) => pairs.push((*lo, *hi)),
                    ClassRange::Builtin(b) => return RegexS::Class(builtin_to_spec(*b)),
                }
            }
            if *negated {
                // For negated classes, we can't easily represent the complement
                // Just mark as generic class
                RegexS::Class(ClassSpec::Ranges(pairs))
            } else {
                RegexS::Class(ClassSpec::Ranges(pairs))
            }
        }

        Node::Builtin(b) => RegexS::Class(builtin_to_spec(*b)),

        Node::Anchor(AnchorKind::Start) => RegexS::Anchor(AnchorSpec::Start),
        Node::Anchor(AnchorKind::End) => RegexS::Anchor(AnchorSpec::End),

        Node::WordBoundary { negated } => {
            if *negated {
                RegexS::Anchor(AnchorSpec::NotWordBoundary)
            } else {
                RegexS::Anchor(AnchorSpec::WordBoundary)
            }
        }

        Node::BackRef(n) => RegexS::BackRef(*n),

        Node::Lookahead { sub, negative } => RegexS::Lookahead {
            sub: Box::new(from_ast(sub)),
            negative: *negative,
        },

        Node::Lookbehind { sub, negative } => RegexS::Lookahead {
            sub: Box::new(from_ast(sub)),
            negative: *negative,
        },

        Node::Capture { index, sub, .. } => RegexS::Capture {
            index: *index,
            sub: Box::new(from_ast(sub)),
        },

        Node::Group(sub) => RegexS::Group(Box::new(from_ast(sub))),

        Node::Repeat { sub, min, max, greedy } => RegexS::Repeat {
            sub: Box::new(from_ast(sub)),
            min: *min,
            max: *max,
            greedy: *greedy,
        },

        Node::Concat(nodes) => {
            // Merge adjacent literals
            let mut subs = vec![];
            let mut lit_buf: Vec<char> = vec![];
            for n in nodes {
                if let Node::Literal(c) = n {
                    lit_buf.push(*c);
                } else {
                    if !lit_buf.is_empty() {
                        subs.push(RegexS::Literal(lit_buf.clone()));
                        lit_buf.clear();
                    }
                    subs.push(from_ast(n));
                }
            }
            if !lit_buf.is_empty() {
                subs.push(RegexS::Literal(lit_buf));
            }
            if subs.len() == 1 {
                subs.pop().unwrap()
            } else {
                RegexS::Seq(subs)
            }
        }

        Node::Alternation(alts) => {
            RegexS::Alt(alts.iter().map(from_ast).collect())
        }
    }
}

fn builtin_to_spec(b: BuiltinClass) -> ClassSpec {
    match b {
        BuiltinClass::Digit => ClassSpec::Digit,
        BuiltinClass::NotDigit => ClassSpec::NotDigit,
        BuiltinClass::Word => ClassSpec::Word,
        BuiltinClass::NotWord => ClassSpec::NotWord,
        BuiltinClass::Space => ClassSpec::Space,
        BuiltinClass::NotSpace => ClassSpec::NotSpace,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::compiler::parser;
    use crate::regex::Flags;

    fn analyze_pattern(pattern: &str) -> StaticInfo {
        let ast = parser::parse(pattern, Flags::empty()).unwrap();
        let ir = from_ast(&ast);
        analyze(&ir)
    }

    #[test]
    fn test_literal_extraction() {
        let info = analyze_pattern("hello world");
        assert!(info.required_literals.contains(&"hello world".to_string()));
        assert_eq!(info.min_length, 11);
        assert!(!info.can_match_empty);
    }

    #[test]
    fn test_alternation_possible_literals() {
        let info = analyze_pattern("foo|bar|baz");
        assert!(info.required_literals.is_empty(), "no literal required across all branches");
        assert!(info.possible_literals.len() >= 3);
    }

    #[test]
    fn test_anchored_detection() {
        let info = analyze_pattern("^hello");
        assert!(info.anchored_start);
        assert!(!info.anchored_end);
    }

    #[test]
    fn test_prefilter_memmem() {
        let info = analyze_pattern("hello.*world");
        let pf = derive_prefilter(&info);
        // "hello" starts the match → SingleByte('h') or MemmemStart
        assert!(!matches!(pf, Prefilter::None), "should have a prefilter, got {:?}", pf);
    }

    #[test]
    fn test_prefilter_alternation() {
        let info = analyze_pattern("Sherlock|Holmes|Watson");
        let pf = derive_prefilter(&info);
        // Should have start bytes S, H, W or AhoCorasick
        assert!(!matches!(pf, Prefilter::None), "should have a prefilter, got {:?}", pf);
    }

    #[test]
    fn test_prefilter_anchored() {
        let info = analyze_pattern("^test");
        let pf = derive_prefilter(&info);
        assert!(matches!(pf, Prefilter::AnchoredStart));
    }

    #[test]
    fn test_capture_tracking() {
        let info = analyze_pattern("(a)(b)|(c)");
        assert!(info.possible_captures.contains(&1));
        assert!(info.possible_captures.contains(&2));
        assert!(info.possible_captures.contains(&3));
    }

    #[test]
    fn test_backreference_detection() {
        let info = analyze_pattern(r"(a)\1");
        assert!(info.has_backrefs);
    }

    #[test]
    fn test_lookahead_detection() {
        let info = analyze_pattern("foo(?=bar)");
        assert!(info.has_lookahead);
    }

    #[test]
    fn test_repeat_min_length() {
        let info = analyze_pattern("[a-z]{8,13}");
        assert_eq!(info.min_length, 8);
        assert!(info.can_match_empty == false);
    }

    #[test]
    fn test_complex_prefilter() {
        // Pattern like aws-keys: has literal prefixes in alternation
        let info = analyze_pattern("(?:ASIA|AKIA|AROA|AIDA)[A-Z0-7]{16}");
        let pf = derive_prefilter(&info);
        // All branches start with 'A' → SingleByte('A') or AhoCorasick
        assert!(!matches!(pf, Prefilter::None), "should have a prefilter, got {:?}", pf);
    }

    #[test]
    fn test_noseyparker_like() {
        // Many alternation branches with different literal prefixes
        let info = analyze_pattern(r"age1[0-9a-z]{58}|AGE-SECRET-KEY|AKIA[A-Z0-9]{16}");
        let pf = derive_prefilter(&info);
        // Should find multiple literal candidates
        assert!(!matches!(pf, Prefilter::None));
    }

    #[test]
    fn test_star_can_match_empty() {
        let info = analyze_pattern("a*");
        assert!(info.can_match_empty);
    }

    #[test]
    fn test_timeout_patterns() {
        // These are the patterns that timeout on rebar. Let's see what
        // the selective analysis finds.

        // aws-keys quick: ((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))
        let info = analyze_pattern(r"((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))");
        let pf = derive_prefilter(&info);
        eprintln!("aws-keys/quick:");
        eprintln!("  prefilter: {:?}", pf);
        eprintln!("  min_length: {}", info.min_length);
        eprintln!("  required_literals: {:?}", info.required_literals);
        eprintln!("  possible_literals: {:?}", info.possible_literals);
        eprintln!("  start_bytes: {:?}", info.start_bytes);

        // aws-keys full
        let info2 = analyze_pattern(
            r#"(('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|").*?(\n^.*?){0,4}(('|")[a-zA-Z0-9+/]{40}('|"))+|('|")[a-zA-Z0-9+/]{40}('|").*?(\n^.*?){0,3}('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|"))+"#
        );
        let pf2 = derive_prefilter(&info2);
        eprintln!("\naws-keys/full:");
        eprintln!("  prefilter: {:?}", pf2);
        eprintln!("  min_length: {}", info2.min_length);
        eprintln!("  start_bytes: {:?}", info2.start_bytes);
        eprintln!("  possible_literals: {:?}", info2.possible_literals);

        // bounded-repeat/context
        let info3 = analyze_pattern(
            r"[A-Za-z]{10}\s+[\s\S]{0,100}Result[\s\S]{0,100}\s+[A-Za-z]{10}"
        );
        let pf3 = derive_prefilter(&info3);
        eprintln!("\nbounded-repeat/context:");
        eprintln!("  prefilter: {:?}", pf3);
        eprintln!("  required_literals: {:?}", info3.required_literals);
        eprintln!("  min_length: {}", info3.min_length);

        // bounded-repeat/capitals: (?:[A-Z][a-z]+\s*){10,100}
        let info4 = analyze_pattern(r"(?:[A-Z][a-z]+\s*){10,100}");
        let pf4 = derive_prefilter(&info4);
        eprintln!("\nbounded-repeat/capitals:");
        eprintln!("  prefilter: {:?}", pf4);
        eprintln!("  min_length: {}", info4.min_length);
        eprintln!("  start_bytes: {:?}", info4.start_bytes);

        // dictionary/single: huge alternation
        let mut words: Vec<String> = (0..100).map(|i| format!("word{:04}", i)).collect();
        let dict_pattern = words.join("|");
        let info5 = analyze_pattern(&dict_pattern);
        let pf5 = derive_prefilter(&info5);
        eprintln!("\ndictionary/single (100 words):");
        eprintln!("  prefilter: {:?}", pf5);
        eprintln!("  possible_literals count: {}", info5.possible_literals.len());

        // cloudflare redos: .*(?:.*=.*)
        let info6 = analyze_pattern(r".*(?:.*=.*)");
        let pf6 = derive_prefilter(&info6);
        eprintln!("\ncloudflare-redos:");
        eprintln!("  prefilter: {:?}", pf6);
        eprintln!("  required_literals: {:?}", info6.required_literals);
        eprintln!("  can_match_empty: {}", info6.can_match_empty);
        eprintln!("  min_length: {}", info6.min_length);

        // ruff-noqa
        let info7 = analyze_pattern(
            r"(\s*)((?:# [Nn][Oo][Qq][Aa])(?::\s?(([A-Z]+[0-9]+(?:[,\s]+)?)+))?)"
        );
        let pf7 = derive_prefilter(&info7);
        eprintln!("\nruff-noqa:");
        eprintln!("  prefilter: {:?}", pf7);
        eprintln!("  required_literals: {:?}", info7.required_literals);
        eprintln!("  min_length: {}", info7.min_length);
        eprintln!("  start_bytes: {:?}", info7.start_bytes);
    }

    #[test]
    fn test_plus_cannot_match_empty() {
        let info = analyze_pattern("a+");
        assert!(!info.can_match_empty);
    }
}
