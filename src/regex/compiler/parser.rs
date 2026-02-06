//! JavaScript regex parser (ECMAScript 2018+).
//!
//! Recursive descent parser that produces a simple AST from JS regex syntax.
//! Supports: literals, character classes, escapes, backreferences, lookahead,
//! lookbehind, capture groups, non-capturing groups, quantifiers, alternation.

use crate::regex::Flags;
use super::{CompilerError, Result};

// ============================================================================
// AST types
// ============================================================================

#[derive(Debug, Clone)]
pub enum Node {
    Empty,
    Literal(char),
    Dot,
    Class { ranges: Vec<ClassRange>, negated: bool },
    Builtin(BuiltinClass),
    Anchor(AnchorKind),
    WordBoundary { negated: bool },
    BackRef(u32),
    Lookahead { sub: Box<Node>, negative: bool },
    Lookbehind { sub: Box<Node>, negative: bool },
    Capture { index: u32, name: Option<String>, sub: Box<Node> },
    Group(Box<Node>),
    Repeat { sub: Box<Node>, min: u32, max: Option<u32>, greedy: bool },
    Concat(Vec<Node>),
    Alternation(Vec<Node>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinClass {
    Digit,    // \d
    NotDigit, // \D
    Word,     // \w
    NotWord,  // \W
    Space,    // \s
    NotSpace, // \S
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorKind {
    Start, // ^
    End,   // $
}

/// A range in a character class: single char or lo-hi inclusive range.
#[derive(Debug, Clone, Copy)]
pub enum ClassRange {
    Single(char),
    Range(char, char),
    Builtin(BuiltinClass),
}

// ============================================================================
// Parser
// ============================================================================

struct Parser {
    chars: Vec<char>,
    pos: usize,
    flags: Flags,
    capture_count: u32,
}

/// Parse a JavaScript regex pattern into an AST.
pub fn parse(pattern: &str, flags: Flags) -> Result<Node> {
    let mut parser = Parser {
        chars: pattern.chars().collect(),
        pos: 0,
        flags,
        capture_count: 0,
    };
    let node = parser.parse_alternation()?;
    if parser.pos < parser.chars.len() {
        return Err(CompilerError::new(format!(
            "unexpected character '{}' at position {}",
            parser.chars[parser.pos], parser.pos
        )));
    }
    Ok(node)
}

/// Get the total number of capture groups (call after parse).
pub fn count_captures(pattern: &str, flags: Flags) -> Result<u32> {
    let mut parser = Parser {
        chars: pattern.chars().collect(),
        pos: 0,
        flags,
        capture_count: 0,
    };
    let _ = parser.parse_alternation()?;
    Ok(parser.capture_count)
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn expect(&mut self, expected: char) -> Result<()> {
        match self.advance() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(CompilerError::new(format!(
                "expected '{}', got '{}' at position {}", expected, c, self.pos - 1
            ))),
            None => Err(CompilerError::new(format!(
                "expected '{}', got end of pattern", expected
            ))),
        }
    }

    // ====================================================================
    // alternation = concat ('|' concat)*
    // ====================================================================
    fn parse_alternation(&mut self) -> Result<Node> {
        let first = self.parse_concat()?;
        if self.peek() != Some('|') {
            return Ok(first);
        }
        let mut alts = vec![first];
        while self.peek() == Some('|') {
            self.advance(); // consume '|'
            alts.push(self.parse_concat()?);
        }
        Ok(Node::Alternation(alts))
    }

    // ====================================================================
    // concat = quantifier*
    // ====================================================================
    fn parse_concat(&mut self) -> Result<Node> {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            nodes.push(self.parse_quantifier()?);
        }
        match nodes.len() {
            0 => Ok(Node::Empty),
            1 => Ok(nodes.pop().unwrap()),
            _ => Ok(Node::Concat(nodes)),
        }
    }

    // ====================================================================
    // quantifier = atom ('*' | '+' | '?' | '{n,m}') '?'?
    // ====================================================================
    fn parse_quantifier(&mut self) -> Result<Node> {
        let atom = self.parse_atom()?;
        let (min, max) = match self.peek() {
            Some('*') => { self.advance(); (0, None) }
            Some('+') => { self.advance(); (1, None) }
            Some('?') => { self.advance(); (0, Some(1)) }
            Some('{') => self.parse_braces()?,
            _ => return Ok(atom),
        };
        let greedy = if self.peek() == Some('?') {
            self.advance();
            false
        } else {
            true
        };
        Ok(Node::Repeat {
            sub: Box::new(atom),
            min,
            max,
            greedy,
        })
    }

    /// Parse {n}, {n,}, {n,m}
    fn parse_braces(&mut self) -> Result<(u32, Option<u32>)> {
        let save = self.pos;
        self.advance(); // consume '{'
        let min = match self.parse_decimal() {
            Some(n) => n,
            None => {
                // Not a valid quantifier — treat '{' as literal
                self.pos = save;
                return Err(CompilerError::new("literal_brace"));
            }
        };
        match self.peek() {
            Some('}') => {
                self.advance();
                Ok((min, Some(min))) // {n}
            }
            Some(',') => {
                self.advance();
                if self.peek() == Some('}') {
                    self.advance();
                    Ok((min, None)) // {n,}
                } else {
                    match self.parse_decimal() {
                        Some(max) => {
                            self.expect('}')?;
                            Ok((min, Some(max))) // {n,m}
                        }
                        None => {
                            self.pos = save;
                            Err(CompilerError::new("literal_brace"))
                        }
                    }
                }
            }
            _ => {
                self.pos = save;
                Err(CompilerError::new("literal_brace"))
            }
        }
    }

    fn parse_decimal(&mut self) -> Option<u32> {
        let start = self.pos;
        let mut n: u32 = 0;
        while let Some(c) = self.peek() {
            if let Some(d) = c.to_digit(10) {
                n = n.checked_mul(10)?.checked_add(d)?;
                self.advance();
            } else {
                break;
            }
        }
        if self.pos == start { None } else { Some(n) }
    }

    // ====================================================================
    // atom = literal | '.' | escape | class | group
    // ====================================================================
    fn parse_atom(&mut self) -> Result<Node> {
        match self.peek() {
            None => Err(CompilerError::new("unexpected end of pattern")),
            Some('.') => { self.advance(); Ok(Node::Dot) }
            Some('^') => { self.advance(); Ok(Node::Anchor(AnchorKind::Start)) }
            Some('$') => { self.advance(); Ok(Node::Anchor(AnchorKind::End)) }
            Some('\\') => self.parse_escape(),
            Some('[') => self.parse_class(),
            Some('(') => self.parse_group(),
            Some(c) if c == '*' || c == '+' || c == '?' => {
                Err(CompilerError::new(format!("nothing to repeat at position {}", self.pos)))
            }
            Some('{') => {
                // Try parsing as quantifier — if it fails, treat as literal
                match self.parse_braces() {
                    Err(_) => { self.advance(); Ok(Node::Literal('{')) }
                    Ok(_) => Err(CompilerError::new("nothing to repeat")),
                }
            }
            Some(c) => { self.advance(); Ok(Node::Literal(c)) }
        }
    }

    // ====================================================================
    // escape sequences
    // ====================================================================
    fn parse_escape(&mut self) -> Result<Node> {
        self.advance(); // consume '\'
        match self.advance() {
            None => Err(CompilerError::new("trailing backslash")),
            Some('d') => Ok(Node::Builtin(BuiltinClass::Digit)),
            Some('D') => Ok(Node::Builtin(BuiltinClass::NotDigit)),
            Some('w') => Ok(Node::Builtin(BuiltinClass::Word)),
            Some('W') => Ok(Node::Builtin(BuiltinClass::NotWord)),
            Some('s') => Ok(Node::Builtin(BuiltinClass::Space)),
            Some('S') => Ok(Node::Builtin(BuiltinClass::NotSpace)),
            Some('b') => Ok(Node::WordBoundary { negated: false }),
            Some('B') => Ok(Node::WordBoundary { negated: true }),
            Some('n') => Ok(Node::Literal('\n')),
            Some('r') => Ok(Node::Literal('\r')),
            Some('t') => Ok(Node::Literal('\t')),
            Some('v') => Ok(Node::Literal('\x0B')),
            Some('f') => Ok(Node::Literal('\x0C')),
            Some('0') if !self.peek().map_or(false, |c| c.is_ascii_digit()) => {
                Ok(Node::Literal('\0'))
            }
            Some(c @ '1'..='9') => {
                // Backreference \1 - \9 (or multi-digit)
                let mut n = c.to_digit(10).unwrap();
                while let Some(d) = self.peek().and_then(|c| c.to_digit(10)) {
                    n = n * 10 + d;
                    self.advance();
                }
                Ok(Node::BackRef(n))
            }
            Some('x') => {
                // \xHH
                let h1 = self.advance().and_then(|c| c.to_digit(16))
                    .ok_or_else(|| CompilerError::new("invalid hex escape"))?;
                let h2 = self.advance().and_then(|c| c.to_digit(16))
                    .ok_or_else(|| CompilerError::new("invalid hex escape"))?;
                let code = (h1 << 4) | h2;
                Ok(Node::Literal(char::from_u32(code).unwrap()))
            }
            Some('u') => self.parse_unicode_escape(),
            // Escaped special characters
            Some(c @ ('\\' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
                    | '^' | '$' | '.' | '*' | '+' | '?')) => {
                Ok(Node::Literal(c))
            }
            // In non-unicode mode, other escaped chars are identity escapes
            Some(c) => Ok(Node::Literal(c)),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<Node> {
        if self.peek() == Some('{') {
            // \u{HHHH} or \u{HHHHH}
            self.advance(); // consume '{'
            let mut code: u32 = 0;
            let mut digits = 0;
            while let Some(c) = self.peek() {
                if c == '}' { break; }
                let d = c.to_digit(16)
                    .ok_or_else(|| CompilerError::new("invalid unicode escape"))?;
                code = code * 16 + d;
                digits += 1;
                self.advance();
            }
            self.expect('}')?;
            if digits == 0 || code > 0x10FFFF {
                return Err(CompilerError::new("invalid unicode escape"));
            }
            Ok(Node::Literal(char::from_u32(code)
                .ok_or_else(|| CompilerError::new("invalid unicode codepoint"))?))
        } else {
            // \uHHHH
            let mut code: u32 = 0;
            for _ in 0..4 {
                let d = self.advance().and_then(|c| c.to_digit(16))
                    .ok_or_else(|| CompilerError::new("invalid unicode escape"))?;
                code = code * 16 + d;
            }
            Ok(Node::Literal(char::from_u32(code)
                .ok_or_else(|| CompilerError::new("invalid unicode codepoint"))?))
        }
    }

    // ====================================================================
    // character class [...]
    // ====================================================================
    fn parse_class(&mut self) -> Result<Node> {
        self.advance(); // consume '['
        let negated = if self.peek() == Some('^') {
            self.advance();
            true
        } else {
            false
        };

        let mut ranges = Vec::new();
        // ']' as first char in class is literal
        if self.peek() == Some(']') {
            ranges.push(ClassRange::Single(']'));
            self.advance();
        }

        while let Some(c) = self.peek() {
            if c == ']' {
                self.advance();
                return Ok(Node::Class { ranges, negated });
            }
            let item = self.parse_class_atom()?;
            // Check for range a-b
            if self.peek() == Some('-') {
                let save = self.pos;
                self.advance(); // consume '-'
                if self.peek() == Some(']') {
                    // Trailing '-' is literal
                    ranges.push(item);
                    ranges.push(ClassRange::Single('-'));
                } else {
                    let end_item = self.parse_class_atom()?;
                    match (item, end_item) {
                        (ClassRange::Single(lo), ClassRange::Single(hi)) => {
                            ranges.push(ClassRange::Range(lo, hi));
                        }
                        _ => {
                            // Can't form range with builtins, revert
                            self.pos = save;
                            ranges.push(item);
                            ranges.push(ClassRange::Single('-'));
                        }
                    }
                }
            } else {
                ranges.push(item);
            }
        }

        Err(CompilerError::new("unterminated character class"))
    }

    fn parse_class_atom(&mut self) -> Result<ClassRange> {
        match self.peek() {
            None => Err(CompilerError::new("unterminated character class")),
            Some('\\') => {
                self.advance();
                match self.advance() {
                    None => Err(CompilerError::new("trailing backslash in class")),
                    Some('d') => Ok(ClassRange::Builtin(BuiltinClass::Digit)),
                    Some('D') => Ok(ClassRange::Builtin(BuiltinClass::NotDigit)),
                    Some('w') => Ok(ClassRange::Builtin(BuiltinClass::Word)),
                    Some('W') => Ok(ClassRange::Builtin(BuiltinClass::NotWord)),
                    Some('s') => Ok(ClassRange::Builtin(BuiltinClass::Space)),
                    Some('S') => Ok(ClassRange::Builtin(BuiltinClass::NotSpace)),
                    Some('n') => Ok(ClassRange::Single('\n')),
                    Some('r') => Ok(ClassRange::Single('\r')),
                    Some('t') => Ok(ClassRange::Single('\t')),
                    Some('v') => Ok(ClassRange::Single('\x0B')),
                    Some('f') => Ok(ClassRange::Single('\x0C')),
                    Some('0') => Ok(ClassRange::Single('\0')),
                    Some('x') => {
                        let h1 = self.advance().and_then(|c| c.to_digit(16))
                            .ok_or_else(|| CompilerError::new("invalid hex escape in class"))?;
                        let h2 = self.advance().and_then(|c| c.to_digit(16))
                            .ok_or_else(|| CompilerError::new("invalid hex escape in class"))?;
                        Ok(ClassRange::Single(char::from_u32((h1 << 4) | h2).unwrap()))
                    }
                    Some('u') => {
                        let node = self.parse_unicode_escape()?;
                        match node {
                            Node::Literal(c) => Ok(ClassRange::Single(c)),
                            _ => Err(CompilerError::new("unexpected escape in class")),
                        }
                    }
                    Some('b') => Ok(ClassRange::Single('\x08')), // backspace in class
                    Some(c) => Ok(ClassRange::Single(c)),
                }
            }
            Some(c) => { self.advance(); Ok(ClassRange::Single(c)) }
        }
    }

    // ====================================================================
    // groups: (...), (?:...), (?=...), (?!...), (?<=...), (?<!...), (?<name>...)
    // ====================================================================
    fn parse_group(&mut self) -> Result<Node> {
        self.advance(); // consume '('

        if self.peek() == Some('?') {
            self.advance(); // consume '?'
            match self.peek() {
                Some(':') => {
                    self.advance();
                    let sub = self.parse_alternation()?;
                    self.expect(')')?;
                    Ok(Node::Group(Box::new(sub)))
                }
                Some('=') => {
                    self.advance();
                    let sub = self.parse_alternation()?;
                    self.expect(')')?;
                    Ok(Node::Lookahead { sub: Box::new(sub), negative: false })
                }
                Some('!') => {
                    self.advance();
                    let sub = self.parse_alternation()?;
                    self.expect(')')?;
                    Ok(Node::Lookahead { sub: Box::new(sub), negative: true })
                }
                Some('<') => {
                    self.advance(); // consume '<'
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            let sub = self.parse_alternation()?;
                            self.expect(')')?;
                            Ok(Node::Lookbehind { sub: Box::new(sub), negative: false })
                        }
                        Some('!') => {
                            self.advance();
                            let sub = self.parse_alternation()?;
                            self.expect(')')?;
                            Ok(Node::Lookbehind { sub: Box::new(sub), negative: true })
                        }
                        _ => {
                            // Named capture group (?<name>...)
                            let name = self.parse_group_name()?;
                            self.capture_count += 1;
                            let index = self.capture_count;
                            let sub = self.parse_alternation()?;
                            self.expect(')')?;
                            Ok(Node::Capture {
                                index,
                                name: Some(name),
                                sub: Box::new(sub),
                            })
                        }
                    }
                }
                _ => {
                    Err(CompilerError::new(format!(
                        "invalid group syntax at position {}", self.pos
                    )))
                }
            }
        } else {
            // Capturing group
            self.capture_count += 1;
            let index = self.capture_count;
            let sub = self.parse_alternation()?;
            self.expect(')')?;
            Ok(Node::Capture {
                index,
                name: None,
                sub: Box::new(sub),
            })
        }
    }

    fn parse_group_name(&mut self) -> Result<String> {
        let mut name = String::new();
        while let Some(c) = self.peek() {
            if c == '>' {
                self.advance();
                if name.is_empty() {
                    return Err(CompilerError::new("empty group name"));
                }
                return Ok(name);
            }
            if c.is_alphanumeric() || c == '_' || c == '$' {
                name.push(c);
                self.advance();
            } else {
                return Err(CompilerError::new(format!("invalid character in group name: '{}'", c)));
            }
        }
        Err(CompilerError::new("unterminated group name"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(pattern: &str) -> Node {
        parse(pattern, Flags::empty()).expect(&format!("failed to parse: {}", pattern))
    }

    #[test]
    fn test_literal() {
        match p("abc") {
            Node::Concat(nodes) => {
                assert_eq!(nodes.len(), 3);
                assert!(matches!(nodes[0], Node::Literal('a')));
            }
            _ => panic!("expected Concat"),
        }
    }

    #[test]
    fn test_dot() {
        assert!(matches!(p("."), Node::Dot));
    }

    #[test]
    fn test_alternation() {
        match p("a|b") {
            Node::Alternation(alts) => assert_eq!(alts.len(), 2),
            _ => panic!("expected Alternation"),
        }
    }

    #[test]
    fn test_quantifiers() {
        match p("a*") {
            Node::Repeat { min: 0, max: None, greedy: true, .. } => {}
            other => panic!("expected Repeat, got {:?}", other),
        }
        match p("a+?") {
            Node::Repeat { min: 1, max: None, greedy: false, .. } => {}
            other => panic!("expected lazy Repeat, got {:?}", other),
        }
        match p("a{2,5}") {
            Node::Repeat { min: 2, max: Some(5), greedy: true, .. } => {}
            other => panic!("expected bounded Repeat, got {:?}", other),
        }
    }

    #[test]
    fn test_capture_group() {
        match p("(a)") {
            Node::Capture { index: 1, name: None, .. } => {}
            other => panic!("expected Capture, got {:?}", other),
        }
    }

    #[test]
    fn test_non_capturing_group() {
        match p("(?:a)") {
            Node::Group(_) => {}
            other => panic!("expected Group, got {:?}", other),
        }
    }

    #[test]
    fn test_backreference() {
        match p(r"(a)\1") {
            Node::Concat(nodes) => {
                assert!(matches!(nodes[1], Node::BackRef(1)));
            }
            other => panic!("expected Concat with BackRef, got {:?}", other),
        }
    }

    #[test]
    fn test_lookahead() {
        match p("a(?=b)") {
            Node::Concat(nodes) => {
                assert!(matches!(nodes[1], Node::Lookahead { negative: false, .. }));
            }
            other => panic!("expected Concat with Lookahead, got {:?}", other),
        }
    }

    #[test]
    fn test_negative_lookahead() {
        match p("a(?!b)") {
            Node::Concat(nodes) => {
                assert!(matches!(nodes[1], Node::Lookahead { negative: true, .. }));
            }
            other => panic!("expected Concat with negative Lookahead, got {:?}", other),
        }
    }

    #[test]
    fn test_lookbehind() {
        match p("(?<=a)b") {
            Node::Concat(nodes) => {
                assert!(matches!(nodes[0], Node::Lookbehind { negative: false, .. }));
            }
            other => panic!("expected Concat with Lookbehind, got {:?}", other),
        }
    }

    #[test]
    fn test_character_class() {
        match p("[a-z]") {
            Node::Class { ranges, negated: false } => {
                assert_eq!(ranges.len(), 1);
                assert!(matches!(ranges[0], ClassRange::Range('a', 'z')));
            }
            other => panic!("expected Class, got {:?}", other),
        }
    }

    #[test]
    fn test_negated_class() {
        match p("[^0-9]") {
            Node::Class { negated: true, .. } => {}
            other => panic!("expected negated Class, got {:?}", other),
        }
    }

    #[test]
    fn test_builtin_escapes() {
        assert!(matches!(p(r"\d"), Node::Builtin(BuiltinClass::Digit)));
        assert!(matches!(p(r"\w"), Node::Builtin(BuiltinClass::Word)));
        assert!(matches!(p(r"\s"), Node::Builtin(BuiltinClass::Space)));
    }

    #[test]
    fn test_anchors() {
        assert!(matches!(p("^"), Node::Anchor(AnchorKind::Start)));
        assert!(matches!(p("$"), Node::Anchor(AnchorKind::End)));
    }

    #[test]
    fn test_unicode_escape() {
        match p(r"\u0041") {
            Node::Literal('A') => {}
            other => panic!("expected Literal('A'), got {:?}", other),
        }
    }

    #[test]
    fn test_named_group() {
        match p("(?<name>a)") {
            Node::Capture { index: 1, name: Some(ref n), .. } if n == "name" => {}
            other => panic!("expected named Capture, got {:?}", other),
        }
    }

    #[test]
    fn test_complex_pattern() {
        // lexer-like pattern with many features
        let _ = p(r"(\r\n|\r|\n)|([\t\v\f ]+)|([a-zA-Z_][0-9a-zA-Z_]*)|([0-9]+)|(.)");
    }

    #[test]
    fn test_aws_keys_pattern() {
        let _ = p(r"((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))");
    }
}
