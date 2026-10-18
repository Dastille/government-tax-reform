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

/// Number words the scan reports. "one" is excluded (it is mostly a pronoun: "one of the
/// worst taxes"), as are ordinals ("second quarter"); both are known limits in CLAIMS.md.
const NUMBER_WORDS: [&str; 41] = [
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
    "thousand",
    "million",
    "billion",
    "trillion",
    "half",
    "halves",
    "thirds",
    "quarters",
    "twice",
    "double",
    "doubled",
    "triple",
    "tripled",
    "dozen",
];

/// Every numeric token in `line`, with its prefix and suffix: `$227.0B`, `45pct`, `13.5%`,
/// `45–90`, `$1.1-1.2T`, `x1.58`, `1,050.1`, `2024-25`.
///
/// Skipped (identifiers, not quantities): a number preceded by a letter or `_` (`R14`,
/// `FY2024-25`, `v2.0`, `YA2024`, `Q2`, `A1.1`); a number after `.` or `-` that follows a letter
/// (`AGPL-3.0`,
/// (the `7` in `2.A.7`); a number followed by `.` and a letter (the `2` in `2.A.7`); a token
/// whose letter suffix runs into more digits (commit ids such as `07e4fb1`); `StatCan` table
/// ids of the form `NN-NN-NNNN(-NN)`.
fn numeric_tokens(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let at = |k: usize| chars.get(k).copied().unwrap_or(' ');
    let mut out = Vec::new();
    let mut i = 0;
    while i < n {
        if !chars[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        let prev = if start == 0 { ' ' } else { chars[start - 1] };
        let prev2 = if start < 2 { ' ' } else { chars[start - 2] };
        // Prefix: `$`, or a multiplication sign `x` / `×` that is not part of a word.
        let mut prefix = String::new();
        let mut id = false;
        if prev == '$' {
            prefix.push('$');
        } else if (prev == 'x' || prev == '×') && !prev2.is_alphanumeric() {
            prefix.push(prev);
        } else if prev.is_alphabetic()
            || prev == '_'
            || ((prev == '.' || prev == '-') && prev2.is_alphabetic())
        {
            id = true;
        }
        // Number, then any `-` / `–` / `/` joined numbers (ranges, dates, compounds).
        let mut j = i;
        loop {
            while j < n && (chars[j].is_ascii_digit() || chars[j] == '.' || chars[j] == ',') {
                j += 1;
            }
            while j > start && (chars[j - 1] == '.' || chars[j - 1] == ',') {
                j -= 1;
            }
            if (at(j) == '-' || at(j) == '–' || at(j) == '/') && at(j + 1).is_ascii_digit() {
                j += 1;
                continue;
            }
            if at(j) == '-' && at(j + 1) == '$' && at(j + 2).is_ascii_digit() {
                j += 2;
                continue;
            }
            break;
        }
        let number: String = chars[start..j].iter().collect();
        if at(j) == '.' && at(j + 1).is_alphabetic() {
            id = true; // 2.A.7, 1.A.4
        }
        // Suffix: letters, `%` or `×` directly after the number.
        let mut k = j;
        while k < n && (chars[k].is_alphabetic() || chars[k] == '%' || chars[k] == '×') {
            k += 1;
        }
        if k > j && at(k).is_ascii_digit() {
            id = true; // ab193f4-style ids that start with a digit: 07e4fb1, 3ecc821
        }
        let suffix: String = chars[j..k].iter().collect();
        if is_statcan_table_id(&number) {
            id = true;
        }
        // Skip the rest of an identifier so its later digits are not re-read.
        let mut end = k;
        if id {
            while end < n
                && (chars[end].is_alphanumeric()
                    || chars[end] == '.' && at(end + 1).is_alphanumeric())
            {
                end += 1;
            }
        } else {
            out.push(format!("{prefix}{number}{suffix}"));
        }
        i = end.max(start + 1);
    }
    out
}

/// `10-10-0147-01`, `36-10-0580-01`, `17-10-0009-01`.
fn is_statcan_table_id(t: &str) -> bool {
    let parts: Vec<&str> = t.split('-').collect();
    let lens: Vec<usize> = parts.iter().map(|p| p.len()).collect();
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
        && (lens == [2, 2, 4, 2] || lens == [2, 2, 4])
}

/// Number words in `line` (lower-cased, whole words, hyphenated parts split).
fn number_words(line: &str) -> Vec<String> {
    line.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-'))
        .filter(|w| !w.contains('_')) // identifiers such as test names
        .flat_map(|w| w.split('-'))
        .map(str::to_lowercase)
        .filter(|w| NUMBER_WORDS.contains(&w.as_str()))
        .collect()
}

/// Section numbers that exist in the paper ("1" … "11", "5.4", "8.1").
fn paper_sections() -> Vec<String> {
    vat_revenue_model::docs::PAPER_TEMPLATE
        .lines()
        .filter(|l| l.starts_with("## ") || l.starts_with("### "))
        .filter_map(|l| {
            let body = l.trim_start_matches('#').trim_start();
            let num: String = body
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            let num = num.trim_end_matches('.').to_string();
            (!num.is_empty()).then_some(num)
        })
        .collect()
}

/// Remove what the scan does not read: placeholders (their values are generated and
/// byte-checked), URLs, `§` references to sections that exist, the section number of a
/// heading, and ordered-list markers that continue their list. Everything else is read,
/// including code spans, code blocks, HTML comments and the References section.
fn scrub(line: &str, list_next: &mut usize, sections: &[String]) -> String {
    // Placeholders.
    let mut s = String::new();
    let mut rest = line;
    while let Some(i) = rest.find("{{") {
        s.push_str(&rest[..i]);
        s.push(' ');
        rest = rest[i..].find("}}").map_or("", |j| &rest[i + j + 2..]);
    }
    s.push_str(rest);
    // URLs and link targets.
    let s: String = s
        .split(' ')
        .map(|w| {
            if w.contains("http") || w.contains("](") || w.contains("doi.org") {
                " "
            } else {
                w
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    // `§` references: removed only when the section exists.
    let mut cleaned = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '§' {
            let mut j = i + 1;
            while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                j += 1;
            }
            let num: String = chars[i + 1..j].iter().collect();
            let num = num.trim_end_matches('.');
            if sections.iter().any(|x| x == num) {
                cleaned.push(' ');
                // keep a trailing '.' that ends the sentence
                i = i + 1 + num.chars().count();
                continue;
            }
        }
        cleaned.push(chars[i]);
        i += 1;
    }
    // Heading section number ("## 5.4 Title", "## 11. References").
    let trimmed = cleaned.trim_start();
    if trimmed.starts_with('#') {
        let body = trimmed.trim_start_matches('#').trim_start();
        let num: String = body
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        if !num.is_empty() && body[num.len()..].starts_with(' ') {
            return body[num.len()..].to_string();
        }
        return cleaned;
    }
    // Ordered-list marker that continues its list: 1., 2., 3. … (a "7." out of sequence is read).
    let lead = cleaned.len() - trimmed.len();
    let digits: String = trimmed.chars().take_while(char::is_ascii_digit).collect();
    if !digits.is_empty() && trimmed[digits.len()..].starts_with(". ") {
        let k: usize = digits.parse().unwrap_or(0);
        if k == 1 || k == *list_next {
            *list_next = k + 1;
            return format!("{}{}", &cleaned[..lead], &trimmed[digits.len() + 1..]);
        }
    }
    cleaned
}

/// (1-based line number, scanned text, raw line) for every line of a template.
fn scanned_lines(template: &str) -> Vec<(usize, String, &str)> {
    let sections = paper_sections();
    let mut list_next = 1;
    template
        .lines()
        .enumerate()
        .map(|(i, raw)| (i + 1, scrub(raw, &mut list_next, &sections), raw))
        .collect()
}

/// Every token the scan reports on a scanned line: numbers, then number words.
fn tokens(text: &str) -> Vec<String> {
    let mut t = numeric_tokens(text);
    t.extend(number_words(text));
    t
}

/// The CLAIMS inputs ledger (`src/ledger.rs`) as scannable lines: claim, then evidence.
fn ledger_text() -> String {
    vat_revenue_model::ledger::INPUTS
        .iter()
        .map(|c| format!("{} | {}", c.claim, c.evidence))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Scanned sources: each generated document's template, plus the ledger rows.
fn templates() -> Vec<(&'static str, String)> {
    let mut v: Vec<(&'static str, String)> = DOCUMENTS
        .iter()
        .map(|(t, p)| (*p, (*t).to_string()))
        .collect();
    v.push(("ledger.rs", ledger_text()));
    v
}

#[test]
fn template_literals_are_placeholders_or_allowlisted() {
    let mut missing = Vec::new();
    for (file, template) in templates() {
        for (n, text, raw) in scanned_lines(&template) {
            for tok in tokens(&text) {
                let covered = ALLOWLIST
                    .iter()
                    .any(|e| e.file == file && e.token == tok && raw.contains(e.anchor));
                if !covered {
                    missing.push(format!("{file}:{n}: `{tok}` in: {}", raw.trim()));
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
    let by_file: BTreeMap<&str, String> = templates().into_iter().collect();
    let mut bad = Vec::new();
    for e in ALLOWLIST {
        if !e.source.contains(e.token) {
            bad.push(format!(
                "{}: entry for `{}` must quote the token in its source (got {:?})",
                e.file, e.token, e.source
            ));
        }
        let template = by_file.get(e.file).map_or("", String::as_str);
        let lines: Vec<(usize, String, &str)> = scanned_lines(template)
            .into_iter()
            .filter(|(_, _, raw)| raw.contains(e.anchor))
            .collect();
        match lines.as_slice() {
            [(_, text, _)] if tokens(text).iter().any(|t| *t == e.token) => {}
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

/// No tokens.
const NONE: [&str; 0] = [];

#[test]
fn scanner_reads_prefixes_suffixes_ranges_and_words() {
    let t = |s: &str| tokens(s);
    assert_eq!(
        t("raises 210.0 and 175 countries, 1,050.1"),
        ["210.0", "175", "1,050.1"]
    );
    assert_eq!(t("a typed $227.0B or $999B"), ["$227.0B", "$999B"]);
    assert_eq!(
        t("45pct and 13.5% and 2.5pp and $1.1-1.2T and 45–90"),
        ["45pct", "13.5%", "2.5pp", "$1.1-1.2T", "45–90"]
    );
    assert_eq!(t("about x1.58 or 3.90×"), ["x1.58", "3.90×"]);
    assert_eq!(
        t("in 2024-25, 2025-09-11 and 5/12/2020"),
        ["2024-25", "2025-09-11", "5/12/2020"]
    );
    assert_eq!(t("`audit_finding_twenty_percent`"), NONE);
    assert_eq!(
        t("roughly half, twice, two-thirds"),
        ["half", "twice", "two", "thirds"]
    );
    // Identifiers are skipped.
    assert_eq!(
        t("AGPL-3.0 R4 FY2024-25 v2.0 2.A.7 1.A.4 ab193f4 07e4fb1 YA2024 Q2 A1.1 10-10-0147-01"),
        NONE
    );
}

#[test]
fn scrub_reads_comments_code_and_references_but_not_urls_or_placeholders() {
    let sections = paper_sections();
    let mut next = 1;
    let mut s = |l: &str| tokens(&scrub(l, &mut next, &sections));
    assert_eq!(s("<!-- note: 45 -->"), ["45"]);
    assert_eq!(s("`20 − 1.2 × (year − 1)`"), ["20", "1.2", "1"]);
    assert_eq!(
        s("- [R1] Public Accounts 2025, Tables 3.2 and 3.3. https://a.b/c/2025/1"),
        ["2025", "3.2", "3.3"]
    );
    assert_eq!(s("## 5.4 Land {{x}} §5.3 and §2"), NONE);
    assert_eq!(s("see §99"), ["99"]);
    assert_eq!(s("1. first"), NONE);
    assert_eq!(s("2. second"), NONE);
    assert_eq!(s("7. out of sequence"), ["7"]);
}

/// Allowlist maintenance: `cargo test --test docs dump_scan -- --ignored --nocapture`
/// prints every reported token as `file<TAB>line<TAB>token<TAB>raw line`.
#[test]
#[ignore = "maintenance helper"]
fn dump_scan() {
    for (file, template) in templates() {
        for (n, text, raw) in scanned_lines(&template) {
            for tok in tokens(&text) {
                println!("SCAN\t{file}\t{n}\t{tok}\t{raw}");
            }
        }
    }
}

/// Every comparison word in the documents is generated; this recomputes each comparison
/// from the model and checks the word, so a changed input that flips a comparison changes
/// the prose (and the committed-file test) instead of leaving a false sentence.
#[test]
#[allow(clippy::too_many_lines)] // one assertion per generated word
fn comparison_words_match_the_values() {
    use vat_revenue_model::data::{
        END_RATE, GROWTH_ADDITION_PP, LAND_VALUE_UNVERIFIED, LVT_RATES, LVT_TAXABLE_SHARE_SCENARIO,
        PATH_YEARS, START_RATE, VRR_NEW_ZEALAND,
    };
    use vat_revenue_model::docs::{fraction_words, more_or_less, values};
    use vat_revenue_model::model::{
        annualised, base0, gdp, lvt_rate_to_replace_property_tax, lvt_scenario, need_core,
        property_tax_total, vrr_factor, Incidence, Scenario,
    };

    // Unit cases for the word helpers.
    assert_eq!(more_or_less(2.0, 1.0), "more than");
    assert_eq!(more_or_less(1.0, 2.0), "less than");
    assert_eq!(more_or_less(1.0, 1.0), "the same as");
    assert_eq!(fraction_words(0.52), "about half");
    assert_eq!(fraction_words(0.35), "about a third");
    assert_eq!(fraction_words(0.70), "about two-thirds");
    assert_eq!(fraction_words(0.41), "41%");

    let v = values().expect("values");
    let get = |k: &str| v.get(k).unwrap_or_else(|| panic!("no value {k}")).as_str();
    let core = need_core();
    let (hi, lo) = (START_RATE, END_RATE);
    let cen = Scenario::central();
    let cb = Scenario::dynamic(Incidence::NominalSpendingFixed);
    let st = Scenario::static_case(Incidence::RealBaseFixed);
    let mul = Scenario {
        multiplier: true,
        ..cen
    };
    let half = Scenario { mpc: 0.5, ..cen };
    let nz = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..st
    };
    let k_a = cen.income_tax_kept(hi, core);
    let k_b = cb.income_tax_kept(hi, core);

    // The ordering the prose relies on.
    assert!(
        cen.revenue(hi) > st.revenue(hi),
        "dynamic must exceed static for the prose"
    );
    assert!(
        mul.revenue(hi) > cen.revenue(hi),
        "the multiplier must add revenue for the prose"
    );
    assert!(
        core - mul.revenue(hi) > 0.0,
        "the prose says a shortfall remains at {hi}%"
    );
    assert!(cen.breakeven_mpc(hi, core) > 1.0 || get("mpc_verdict_a").starts_with("an MPC"));

    assert_eq!(get("replaced_frac_a"), fraction_words(1.0 - k_a));
    assert_eq!(get("base_frac_gdp"), fraction_words(base0() / gdp()));
    let gfac = cen.breakeven_growth_factor(hi, core);
    assert_eq!(
        get("g_hi_a_dir"),
        if gfac > 1.0 { "larger" } else { "smaller" }
    );
    assert_eq!(
        get("property_vs_lvt"),
        more_or_less(property_tax_total(), lvt_scenario())
    );
    assert_eq!(
        get("b_vs_a"),
        if cb.revenue(hi) < cen.revenue(hi) {
            "lower"
        } else {
            "higher"
        }
    );
    assert_eq!(get("k_b_dir"), if k_b > k_a { "rises" } else { "falls" });
    let ratio =
        lvt_rate_to_replace_property_tax(LAND_VALUE_UNVERIFIED * LVT_TAXABLE_SHARE_SCENARIO)
            / LVT_RATES[1];
    let lvt_word = if ratio >= 2.0 {
        "well above"
    } else if ratio > 1.0 {
        "above"
    } else {
        "within"
    };
    assert_eq!(get("lvt_vs_design"), lvt_word);
    let nz_needed = nz.required_rate(core);
    assert_eq!(get("nz_verdict").starts_with("is enough"), nz_needed <= hi);
    assert_eq!(
        get("nz_condition").contains("like New Zealand's"),
        nz_needed <= hi
    );
    let g_lo_pa = 100.0 * annualised(cen.breakeven_growth_factor(lo, core), PATH_YEARS);
    let growth_word = if g_lo_pa > GROWTH_ADDITION_PP[1] {
        "faster than even the high end of"
    } else if g_lo_pa >= GROWTH_ADDITION_PP[0] {
        "within the range of"
    } else {
        "below"
    };
    assert_eq!(get("growth_vs_original"), growth_word);
    let effect = cen.revenue(hi) - half.revenue(hi);
    let short = core - mul.revenue(hi);
    assert_eq!(
        get("mpc_effect_size"),
        if effect < 0.25 * short {
            "small"
        } else {
            "large"
        }
    );
}
