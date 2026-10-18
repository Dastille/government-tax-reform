//! The whitepaper must quote the model, not hand-typed numbers.
use vat_revenue_model::report::{first_principles, key_figures, render};

const PAPER: &str = include_str!("../../tax-whitepaper.md");

/// Numbers in the paper that come straight from a cited source (not computed by the
/// model) or are section labels. Each entry: (number, why it is allowed).
const SOURCED_NOT_COMPUTED: &[(&str, &str)] = &[
    ("0.166", "AFR 2024-25 revenue/GDP ratio, as a divisor"),
    ("0.445", "TE 2026: GST holiday cost, 2024"),
    ("0.840", "TE 2026: GST holiday cost, 2025"),
    ("10.0", "OECD CTT 2024 Annex 1.A.4: NZ VAT % of GDP"),
    ("30.3", "OECD CTT 2024 Annex 1.A.4: NZ VAT % of taxes"),
    ("10.1", "OECD CTT 2024 Annex 1.A.4: Hungary VAT % of GDP"),
    ("22.0", "OECD CTT 2024 Annex 1.A.4: Denmark VAT % of taxes"),
    ("9.2", "OECD CTT 2024 Annex 1.A.4: Denmark VAT % of GDP"),
    ("28.9", "OECD CTT 2024 Annex 1.A.4: Hungary VAT % of taxes"),
    ("41.2", "AFR 2024-25: federal debt % of GDP"),
    ("16.7", "definition: 20 / 120"),
    (
        "1.8",
        "quoted original claim (1.8-2.5 multipliers), shown as deleted",
    ),
];

fn block<'a>(text: &'a str, name: &str) -> &'a str {
    let b = format!("<!-- {name}:begin -->");
    let e = format!("<!-- {name}:end -->");
    let i = text.find(&b).unwrap_or_else(|| panic!("missing {b}"));
    let j = text.find(&e).unwrap_or_else(|| panic!("missing {e}"));
    &text[i..j + e.len()]
}

/// Decimal numbers ("1,050.1", "36.5") with the text just before them.
fn decimals(text: &str) -> Vec<(String, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let starts = chars[i].is_ascii_digit() && (i == 0 || !chars[i - 1].is_ascii_digit());
        if starts {
            let mut j = i;
            while j < chars.len()
                && (chars[j].is_ascii_digit()
                    || ((chars[j] == ',' || chars[j] == '.')
                        && j + 1 < chars.len()
                        && chars[j + 1].is_ascii_digit()))
            {
                j += 1;
            }
            let tok: String = chars[i..j].iter().filter(|c| **c != ',').collect();
            if tok.contains('.') {
                let pre: String = chars[i.saturating_sub(6)..i].iter().collect();
                out.push((tok, pre));
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

#[test]
fn key_figures_block_matches_model() {
    assert_eq!(
        block(PAPER, "key-figures"),
        block(&key_figures(), "key-figures")
    );
}

#[test]
fn first_principles_block_matches_model() {
    assert_eq!(
        block(PAPER, "first-principles"),
        block(&first_principles(), "first-principles")
    );
}

#[test]
fn every_number_in_the_paper_is_model_output_or_sourced() {
    let body = &PAPER[..PAPER.find("## 11. References").expect("references section")];
    let model: Vec<String> = decimals(&render()).into_iter().map(|(t, _)| t).collect();
    let mut missing = Vec::new();
    for (tok, pre) in decimals(body) {
        let label = pre.ends_with('v') || pre.ends_with('§') || pre.ends_with("### ");
        if label || model.contains(&tok) || SOURCED_NOT_COMPUTED.iter().any(|(n, _)| *n == tok) {
            continue;
        }
        missing.push(tok);
    }
    assert!(
        missing.is_empty(),
        "paper numbers not produced by the model: {missing:?}"
    );
}

#[test]
fn every_number_in_the_readme_is_model_output() {
    let readme = include_str!("../../README.md");
    let model: Vec<String> = decimals(&render()).into_iter().map(|(t, _)| t).collect();
    let missing: Vec<String> = decimals(readme)
        .into_iter()
        .filter(|(t, pre)| !(pre.ends_with('v') || pre.ends_with('-') || model.contains(t)))
        .map(|(t, _)| t)
        .collect();
    assert!(
        missing.is_empty(),
        "README numbers not produced by the model: {missing:?}"
    );
}
