//! The paper, README and CLAIMS.md are generated from `templates/` plus model values.
//! These tests prove that (1) every committed document is byte-for-byte a fresh render,
//! and (2) every number typed into a template outside a placeholder is a whole token on
//! a known line, listed in [`ALLOWLIST`] with its source.

#[path = "docs/allowlist.rs"]
mod allowlist;

use allowlist::ALLOWLIST;
use std::collections::BTreeMap;
use std::path::Path;
use vat_revenue_model::docs::{render_documents, DOCUMENTS};
use vat_revenue_model::report::render;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("analysis/ has a parent")
}

#[test]
fn committed_documents_equal_fresh_render() {
    for (path, fresh) in render_documents().expect("documents render") {
        let committed = std::fs::read_to_string(repo_root().join(path)).expect("committed doc");
        if committed != fresh {
            let line = committed
                .lines()
                .zip(fresh.lines())
                .position(|(a, b)| a != b)
                .map_or_else(
                    || "length differs".to_string(),
                    |i| format!("line {}", i + 1),
                );
            let detail = committed
                .lines()
                .zip(fresh.lines())
                .find(|(a, b)| a != b)
                .map(|(a, b)| format!("\n  committed: {a}\n  model:     {b}"))
                .unwrap_or_default();
            panic!(
                "{path} differs from a fresh render at {line}{detail}\n\
                 Edit analysis/templates/{path}, then run `cargo run --release -- --write`."
            );
        }
    }
}

#[test]
fn committed_output_equals_fresh_render() {
    let committed = include_str!("../output/revenue_tables.md");
    assert_eq!(
        committed,
        render().expect("report renders"),
        "run `cargo run --release -- --write`"
    );
}

/// Numeric tokens in `line` that are not joined to letters.
fn numeric_tokens(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if !c.is_ascii_digit() {
            i += 1;
            continue;
        }
        let prev = if i == 0 { ' ' } else { chars[i - 1] };
        let start = i;
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == ',') {
            i += 1;
        }
        let next = chars.get(i).copied().unwrap_or(' ');
        let mut tok: String = chars[start..i].iter().collect();
        while tok.ends_with('.') || tok.ends_with(',') {
            tok.pop();
        }
        // Joined to letters or identifiers: R4, FY2024-25, v1.1, 2.A.7, ab193f4, x1.58.
        let joined = prev.is_ascii_alphabetic()
            || prev == '_'
            || next.is_ascii_alphabetic()
            || next == '_'
            || (prev == '.' && start >= 2 && chars[start - 2].is_ascii_alphabetic())
            || (prev == '-'
                && start >= 2
                && chars[start - 2].is_ascii_alphanumeric()
                && chars[start - 2] != ' '
                && is_part_of_label(&chars, start));
        if !joined {
            out.push(tok);
        }
    }
    out
}

/// `FY2024-25`: the `25` after a hyphen that follows a letter-joined number.
fn is_part_of_label(chars: &[char], start: usize) -> bool {
    let mut j = start - 1; // the '-'
    while j > 0 && (chars[j - 1].is_ascii_digit()) {
        j -= 1;
    }
    j > 0 && chars[j - 1].is_ascii_alphabetic()
}

/// Remove text the scan does not cover: placeholders, code spans, URLs, link targets,
/// `§` references, heading numbers and ordered-list markers.
fn scrub(line: &str) -> String {
    let mut s = String::new();
    let mut rest = line;
    // Placeholders.
    while let Some(i) = rest.find("{{") {
        s.push_str(&rest[..i]);
        s.push(' ');
        rest = rest[i..].find("}}").map_or("", |j| &rest[i + j + 2..]);
    }
    s.push_str(rest);
    // Code spans.
    let mut out = String::new();
    for (k, part) in s.split('`').enumerate() {
        if k % 2 == 0 {
            out.push_str(part);
        } else {
            out.push(' ');
        }
    }
    // Bracketed citations: [R1, Tables 3.2 and 3.3; R2, Table 4].
    let out = blank_citations(&out);
    // URLs, link targets, ISO dates and table ids (2025-09-11, 10-10-0147-01), and
    // locators after Table / Tables / Part / Annex / WDI / ch.
    let words: Vec<&str> = out.split(' ').collect();
    let mut kept: Vec<&str> = Vec::with_capacity(words.len());
    let mut locator = false;
    for w in words {
        let bare = w.trim_matches(|c: char| ",;:()".contains(c));
        let numeric_id = !bare.is_empty()
            && bare
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == '/' || c == 'A')
            && bare.chars().next().is_some_and(|c| c.is_ascii_digit());
        if w.contains("http")
            || w.contains("](")
            || (numeric_id && (locator || bare.matches('-').count() >= 2))
        {
            kept.push(" ");
            locator = locator && !w.ends_with(',') && !w.ends_with(';');
            continue;
        }
        let lower = bare.to_ascii_lowercase();
        let is_head = ["table", "tables", "part", "annex", "wdi", "ch."].contains(&lower.as_str());
        locator = is_head || (locator && (lower == "and" || lower == "table"));
        kept.push(w);
    }
    let out = kept.join(" ");
    // Section references: §5, §5.4.
    let mut cleaned = String::new();
    let mut chars = out.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '§' {
            while chars
                .peek()
                .is_some_and(|d| d.is_ascii_digit() || *d == '.')
            {
                chars.next();
            }
            cleaned.push(' ');
        } else {
            cleaned.push(c);
        }
    }
    // Heading numbers ("## 5.4 Title", "## 11. References") and list markers ("1. ").
    let trimmed = cleaned.trim_start();
    let lead = cleaned.len() - trimmed.len();
    let body = trimmed.trim_start_matches('#').trim_start();
    let num_end = body
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(0);
    if num_end > 0
        && body[num_end..].starts_with(' ')
        && (trimmed.starts_with('#') || body[..num_end].ends_with('.'))
    {
        return format!("{}{}", &cleaned[..lead], &body[num_end..]);
    }
    cleaned
}

/// Blank `[R…]` citation brackets.
fn blank_citations(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("[R") {
        out.push_str(&rest[..i]);
        if let Some(j) = rest[i..].find(']') {
            out.push(' ');
            rest = &rest[i + j + 1..];
        } else {
            rest = &rest[i..];
            break;
        }
    }
    out.push_str(rest);
    out
}

/// (1-based line number, scanned text) for every line the literal scan covers.
fn scanned_lines(template: &str) -> Vec<(usize, String, &str)> {
    let mut out = Vec::new();
    let mut in_code = false;
    for (i, raw) in template.lines().enumerate() {
        if raw.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if raw.starts_with("## 11. References") {
            break; // citation years, table numbers and URLs
        }
        if in_code || raw.trim_start().starts_with("<!--") {
            continue;
        }
        out.push((i + 1, scrub(raw), raw));
    }
    out
}

fn templates() -> Vec<(&'static str, &'static str)> {
    DOCUMENTS.iter().map(|(t, p)| (*p, *t)).collect()
}

#[test]
fn template_literals_are_placeholders_or_allowlisted() {
    let mut missing = Vec::new();
    for (file, template) in templates() {
        for (n, text, raw) in scanned_lines(template) {
            for tok in numeric_tokens(&text) {
                let covered = ALLOWLIST
                    .iter()
                    .any(|e| e.file == file && e.token == tok && raw.contains(e.anchor));
                if !covered {
                    missing.push(format!("templates/{file}:{n}: `{tok}` in: {}", raw.trim()));
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "{} literal number(s) are neither placeholders nor allowlisted with a source:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

#[test]
fn allowlist_entries_are_used() {
    let by_file: BTreeMap<&str, &str> = templates().into_iter().collect();
    let mut bad = Vec::new();
    for e in ALLOWLIST {
        assert!(
            !e.source.trim().is_empty(),
            "allowlist entry `{}` has no source",
            e.token
        );
        let template = by_file.get(e.file).copied().unwrap_or("");
        let lines: Vec<(usize, String, &str)> = scanned_lines(template)
            .into_iter()
            .filter(|(_, _, raw)| raw.contains(e.anchor))
            .collect();
        match lines.as_slice() {
            [(_, text, _)] if numeric_tokens(text).iter().any(|t| *t == e.token) => {}
            [(n, _, _)] => bad.push(format!(
                "{}:{n}: token `{}` is not a whole token on the anchor line",
                e.file, e.token
            )),
            [] => bad.push(format!(
                "{}: anchor {:?} matches no scanned line",
                e.file, e.anchor
            )),
            many => bad.push(format!(
                "{}: anchor {:?} matches {} lines; make it unique",
                e.file,
                e.anchor,
                many.len()
            )),
        }
    }
    assert!(
        bad.is_empty(),
        "stale or ambiguous allowlist entries:\n{}",
        bad.join("\n")
    );
}

#[test]
fn scanner_sees_integers_and_decimals_as_whole_tokens() {
    assert_eq!(
        numeric_tokens("raises 210.0 and 175 countries, 1,050.1"),
        ["210.0", "175", "1,050.1"]
    );
    assert_eq!(
        numeric_tokens("R4 FY2024-25 v1.1 2.A.7 ab193f4 x1.58"),
        [] as [std::string::String; 0]
    );
    assert_eq!(numeric_tokens("13.5% and 262.5"), ["13.5", "262.5"]);
    assert_eq!(
        numeric_tokens(&scrub(
            "## 5.4 Land {{x}} §5.3 `20 - 1.2` see https://a.b/1 [R1, Table 3.2]"
        )),
        [] as [std::string::String; 0]
    );
    assert_eq!(
        numeric_tokens(&scrub("1. raises 210.0 in 2025-09-11 data")),
        ["210.0"]
    );
}
