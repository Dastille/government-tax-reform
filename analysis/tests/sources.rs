//! Every `Verified (source excerpt)` value in `src/data.rs` is checked against a short
//! verbatim excerpt of its source, committed in `tests/sources/<stem>.txt` with its URL and
//! access date. Each expected string is built from the constant, so changing a constant
//! (or an excerpt) turns these tests red. The ledger tests make every input appear in
//! `src/ledger.rs`, and stop an input from being marked verified without a check.

// The check helpers take the expected string by value so call sites can pass `format!(…)`.
#![allow(clippy::needless_pass_by_value)]

use std::collections::BTreeSet;
use std::path::Path;
use vat_revenue_model::data::*;
use vat_revenue_model::ledger::{Status, INPUTS};

const ACCESSED: &str = "Accessed: 2026-10-18 (America/New_York)";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// The excerpt body (after the `---` line) of `tests/sources/<stem>.txt`.
fn excerpt(stem: &str) -> String {
    let path = root().join("tests/sources").join(format!("{stem}.txt"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let (_, body) = text
        .split_once("\n---\n")
        .unwrap_or_else(|| panic!("{stem}: no `---` line after the header"));
    body.to_string()
}

fn header(stem: &str) -> String {
    let path = root().join("tests/sources").join(format!("{stem}.txt"));
    let text = std::fs::read_to_string(path).expect("excerpt file");
    text.split_once("\n---\n")
        .map(|(h, _)| h.to_string())
        .unwrap_or_default()
}

fn fact(key: &str) -> &'static str {
    FACTS
        .iter()
        .find(|(k, _)| *k == key)
        .map_or_else(|| panic!("no FACTS key {key}"), |(_, v)| *v)
}

/// `234319.0` → `234,319`; `17267.4` → `17,267.4`.
fn thousands(x: f64) -> String {
    let s = if x.fract() == 0.0 {
        format!("{x:.0}")
    } else {
        format!("{x:.1}")
    };
    let (int, frac) = s
        .split_once('.')
        .map_or((s.as_str(), None), |(i, f)| (i, Some(f)));
    let mut out = String::new();
    for (i, ch) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if let Some(f) = frac {
        out.push('.');
        out.push_str(f);
    }
    out
}

/// `15.0` → `15`; `20.5` → `20.5`.
fn plain(x: f64) -> String {
    if x.fract() == 0.0 {
        format!("{x:.0}")
    } else {
        format!("{x}")
    }
}

/// Cells of the first `| … |` row of `text` whose first cell is `label`.
fn row(text: &str, label: &str) -> Option<Vec<String>> {
    text.lines()
        .find(|l| l.starts_with(&format!("| {label} |")))
        .map(|l| {
            l.trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_string())
                .collect()
        })
}

/// Whitespace tokens of the first line of `text` that starts with `prefix ` and is followed
/// by a number, with `prefix` removed.
fn number_row<'a>(text: &'a str, prefix: &str) -> Option<Vec<&'a str>> {
    text.lines()
        .find(|l| {
            l.strip_prefix(prefix)
                .and_then(|r| r.strip_prefix(' '))
                .is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit() || c == '.'))
        })
        .map(|l| l[prefix.len()..].split_whitespace().collect())
}

/// One check of an input against an excerpt.
struct Check {
    input: &'static str,
    stem: &'static str,
    ok: bool,
    detail: String,
}

fn contains(input: &'static str, stem: &'static str, needle: String) -> Check {
    let ok = excerpt(stem).contains(&needle);
    Check {
        input,
        stem,
        ok,
        detail: format!("expected verbatim `{needle}`"),
    }
}

fn cell(input: &'static str, stem: &'static str, label: &str, idx: usize, want: String) -> Check {
    let got = row(&excerpt(stem), label).and_then(|c| c.get(idx).cloned());
    let ok = got.as_deref() == Some(want.as_str());
    Check {
        input,
        stem,
        ok,
        detail: format!("row `{label}` cell {idx}: expected `{want}`, got {got:?}"),
    }
}

fn token(
    input: &'static str,
    stem: &'static str,
    text: &str,
    prefix: &str,
    idx: usize,
    want: String,
) -> Check {
    let got = number_row(text, prefix).and_then(|t| t.get(idx).map(|s| (*s).to_string()));
    let ok = got.as_deref() == Some(want.as_str());
    Check {
        input,
        stem,
        ok,
        detail: format!("row `{prefix}` token {idx}: expected `{want}`, got {got:?}"),
    }
}

fn arithmetic(input: &'static str, stem: &'static str, ok: bool, what: &str) -> Check {
    Check {
        input,
        stem,
        ok,
        detail: format!("arithmetic: {what}"),
    }
}

/// Data labels → the verbatim measure names in Federal Tax Expenditures 2026, Part 2.
const TE_LABELS: [(&str, &str); 17] = [
    (
        "Zero-rating of basic groceries",
        "Zero-rating of basic groceries",
    ),
    (
        "Exemption: certain residential rent",
        "Exemption from GST for certain residential rent",
    ),
    (
        "Exemption: certain supplies by charities and NPOs",
        "Exemption from GST for certain supplies made by charities and non-profit organizations",
    ),
    (
        "Zero-rating of prescription drugs",
        "Zero-rating of prescription drugs",
    ),
    (
        "Exemption: tuition and educational services",
        "Exemption from GST for tuition and educational services",
    ),
    (
        "Exemption: health care services",
        "Exemption from GST for health care services",
    ),
    (
        "Zero-rating of medical and assistive devices",
        "Zero-rating of medical and assistive devices",
    ),
    (
        "Exemption: water, sewage and basic garbage",
        "Exemption from GST for water, sewage and basic garbage collection services",
    ),
    (
        "Exemption: municipal transit",
        "Exemption from GST for municipal transit",
    ),
    ("Exemption: child care", "Exemption from GST for child care"),
    (
        "Zero-rating of feminine hygiene products",
        "Zero-rating of feminine hygiene products",
    ),
    ("Rebate for municipalities", "Rebate for municipalities"),
    (
        "Rebate for hospitals, facility operators",
        "Rebate for hospitals, facility operators and external suppliers",
    ),
    (
        "Rebate for schools, colleges and universities",
        "Rebate for schools, colleges and universities",
    ),
    (
        "Rebate for registered charities",
        "Rebate for registered charities",
    ),
    ("Small suppliers' threshold", "Small suppliers' threshold"),
    (
        "Rebate for qualifying NPOs",
        "Rebate for qualifying non-profit organizations",
    ),
];

/// Column of the 2024 and 2025 cost in a TE row (cells: measure, tax, 2020, …).
const TE_2024: usize = 6;
const TE_2025: usize = 7;

#[allow(clippy::too_many_lines)]
fn checks() -> Vec<Check> {
    let pa = "pa2025_vol1_s3";
    let afr = "afr2024_25";
    let te = "te2026_part2";
    let oecd = "oecd_ctt_2024";
    let ctt = excerpt(oecd);
    let (ctt_shares, ctt_vrr) = ctt.split_once("Annex Table 2.A.7").expect("2.A.7 section");
    let wdi = excerpt("wdi_table_4_8");
    let budget = excerpt("budget2009_annex1");
    let last =
        |prefix: &str| number_row(&budget, prefix).and_then(|t| t.last().map(|s| (*s).to_string()));

    let mut v = vec![
        contains("PIT", pa, format!("| Personal | {} |", thousands(PIT))),
        contains("CIT", pa, format!("| Corporate | {} |", thousands(CIT))),
        contains(
            "NON_RESIDENT",
            pa,
            format!("| Non-resident | {} |", thousands(NON_RESIDENT)),
        ),
        cell(
            "GST_BEFORE_REMISSION",
            pa,
            "GST",
            1,
            thousands(GST_BEFORE_REMISSION),
        ),
        contains(
            "GST_REMISSION",
            pa,
            format!(
                "purchased from outside parties | {} |",
                thousands(GST_REMISSION)
            ),
        ),
        cell(
            "GST_GROSS_EXTERNAL",
            pa,
            "Gross GST from outside parties",
            1,
            thousands(GST_GROSS_EXTERNAL),
        ),
        cell(
            "GST_CREDIT",
            pa,
            "Less: quarterly tax credits",
            1,
            thousands(GST_CREDIT),
        ),
        cell(
            "GST_NET",
            pa,
            "Net GST from outside parties",
            1,
            thousands(GST_NET),
        ),
        arithmetic(
            "GST_GROSS_EXTERNAL",
            pa,
            GST_BEFORE_REMISSION - GST_REMISSION == GST_GROSS_EXTERNAL,
            "GST less remission equals gross GST from outside parties",
        ),
        arithmetic(
            "GST_NET",
            pa,
            GST_GROSS_EXTERNAL - GST_CREDIT == GST_NET,
            "gross GST less credits equals net GST",
        ),
        contains(
            "GST_RATE",
            pa,
            format!("applied at a rate of {}% on most goods", plain(GST_RATE)),
        ),
        cell(
            "EI_PREMIUMS",
            afr,
            "Employment Insurance premiums",
            1,
            thousands(EI_PREMIUMS),
        ),
        cell(
            "TOTAL_REVENUE",
            afr,
            "Total revenues",
            1,
            thousands(TOTAL_REVENUE),
        ),
        contains(
            "REVENUE_PCT_GDP",
            afr,
            format!("The ratio stood at {REVENUE_PCT_GDP} per cent in 2024-25"),
        ),
        contains(
            "debt_pct_gdp",
            afr,
            format!(
                "federal debt stood at {} per cent of GDP in 2024-25",
                fact("debt_pct_gdp")
            ),
        ),
        cell(
            "GST_HOLIDAY_COST_2024",
            te,
            "Targeted Temporary GST/HST Relief",
            TE_2024,
            thousands(GST_HOLIDAY_COST_2024),
        ),
        cell(
            "GST_HOLIDAY_COST_2025",
            te,
            "Targeted Temporary GST/HST Relief",
            TE_2025,
            thousands(GST_HOLIDAY_COST_2025),
        ),
        arithmetic(
            "GST_HOLIDAY_COST",
            te,
            GST_HOLIDAY_COST == GST_HOLIDAY_COST_2024 + GST_HOLIDAY_COST_2025,
            "holiday cost is the 2024 plus 2025 columns",
        ),
        contains(
            "SAVING_RATE_Q2_2024",
            "statcan_daily_2024-09-12",
            format!(
                "increased to {:.1}% in the second quarter",
                SAVING_RATE_Q2_2024 * 100.0
            ),
        ),
        contains(
            "SAVING_RATE_Q2_2025",
            "statcan_daily_2025-09-11",
            format!(
                "fell to {:.1}% in the second quarter of 2025",
                SAVING_RATE_Q2_2025 * 100.0
            ),
        ),
        token(
            "HFCE_PCT_GDP",
            "wdi_table_4_8",
            &wdi,
            "Canada",
            5,
            format!("{HFCE_PCT_GDP:.1}"),
        ),
        token(
            "IMPORTS_PCT_GDP",
            "wdi_table_4_8",
            &wdi,
            "Canada",
            11,
            format!("{IMPORTS_PCT_GDP:.1}"),
        ),
        Check {
            input: "MULTIPLIER_PIT",
            stem: "budget2009_annex1",
            ok: last("Personal income tax measures") == Some(format!("{MULTIPLIER_PIT:.1}")),
            detail: format!("last column of `Personal income tax measures` = {MULTIPLIER_PIT:.1}"),
        },
        Check {
            input: "MULTIPLIER_CIT",
            stem: "budget2009_annex1",
            ok: last("Corporate income tax measures 1") == Some(format!("{MULTIPLIER_CIT:.1}")),
            detail: format!("last column of `Corporate income tax measures` = {MULTIPLIER_CIT:.1}"),
        },
        contains(
            "NATIONAL_WEALTH_Q2_2025",
            "statcan_daily_2025-09-11",
            format!(
                "national wealth, dipped to ${} billion",
                thousands(NATIONAL_WEALTH_Q2_2025 / 1000.0)
            ),
        ),
        contains(
            "RESIDENTIAL_REAL_ESTATE_Q2_2025",
            "statcan_daily_2025-09-11",
            format!(
                "residential real estate stood at ${} billion",
                thousands(RESIDENTIAL_REAL_ESTATE_Q2_2025 / 1000.0)
            ),
        ),
        contains(
            "PROPERTY_TAX_PER_CAPITA_2024",
            "ns_finance_gfs_2024",
            format!(
                "of the national average (${})",
                thousands(PROPERTY_TAX_PER_CAPITA_2024)
            ),
        ),
        contains(
            "POPULATION_2024",
            "statcan_daily_2024-09-25",
            format!("\n{}\nJuly 1, 2024", thousands(POPULATION_2024)),
        ),
        contains(
            "nz_rate",
            "ird_nz_gst_2010",
            format!(
                "increase from {}% to {}% from {}",
                fact("nz_prev_rate"),
                fact("nz_rate"),
                fact("nz_rate_date")
            ),
        ),
        contains(
            "nz_prev_rate",
            "ird_nz_gst_2010",
            format!("increase from {}% to", fact("nz_prev_rate")),
        ),
        contains(
            "nz_rate_date",
            "ird_nz_gst_2010",
            format!("% from {},", fact("nz_rate_date")),
        ),
        contains(
            "sa_rate_2018",
            "zatca_about_vat",
            format!("standard rate of {}%", fact("sa_rate_2018")),
        ),
        contains(
            "sa_date_2018",
            "zatca_about_vat",
            format!("imposed VAT on {}", us_date(fact("sa_date_2018"))),
        ),
        contains(
            "sa_rate_2020",
            "saudi_mof_2020-05-11",
            format!(
                "VAT shall increase from {}% to {}% as of {}",
                fact("sa_rate_2018"),
                fact("sa_rate_2020"),
                fact("sa_date_2020")
            ),
        ),
        contains(
            "sa_date_2020",
            "saudi_mof_2020-05-11",
            format!("% as of {}.", fact("sa_date_2020")),
        ),
        contains(
            "sg_pit_top",
            "iras_rates",
            format!(
                "highest personal income tax rate is at {}%.",
                fact("sg_pit_top")
            ),
        ),
        contains(
            "sg_gst",
            "iras_rates",
            format!("GST rate in Singapore is {}%.", fact("sg_gst")),
        ),
        contains(
            "fst_rate",
            "te2017_part10",
            format!(
                "applied at a general rate of {}% on goods",
                fact("fst_rate")
            ),
        ),
        contains(
            "gst_1991_rate",
            "te2017_part10",
            format!(
                "lower flat rate of {}% (since reduced to {}%)",
                fact("gst_1991_rate"),
                plain(GST_RATE)
            ),
        ),
        contains(
            "vat_countries",
            oecd,
            format!(
                "As of 1 July 2024, {} countries and territories",
                fact("vat_countries")
            ),
        ),
        contains(
            "cgeb_increase",
            "cra_cgeb",
            format!(
                "there was a {}% increase to the benefit amount",
                fact("cgeb_increase")
            ),
        ),
        contains(
            "fairtax_rate",
            "hr25_119th",
            format!(
                "the rate of tax is {} percent of the gross payments",
                fact("fairtax_rate")
            ),
        ),
        // 2.A.7 rows: standard rate (countries only), the VRR series ending 2022, change.
        token(
            "VRR_CANADA",
            oecd,
            ctt_vrr,
            "Canada",
            17,
            format!("{VRR_CANADA:.2}"),
        ),
        token(
            "VRR_NEW_ZEALAND",
            oecd,
            ctt_vrr,
            "New Zealand",
            17,
            format!("{VRR_NEW_ZEALAND:.2}"),
        ),
        token(
            "VRR_OECD_AVERAGE",
            oecd,
            ctt_vrr,
            "OECD average",
            16,
            format!("{VRR_OECD_AVERAGE:.2}"),
        ),
        token(
            "dk_rate",
            oecd,
            ctt_vrr,
            "Denmark",
            0,
            fact("dk_rate").to_string(),
        ),
        token(
            "hu_rate",
            oecd,
            ctt_vrr,
            "Hungary",
            0,
            fact("hu_rate").to_string(),
        ),
    ];
    for (key, country) in [("nz", "New Zealand"), ("dk", "Denmark"), ("hu", "Hungary")] {
        let gdp: &'static str = Box::leak(format!("{key}_vat_gdp").into_boxed_str());
        let taxes: &'static str = Box::leak(format!("{key}_vat_taxes").into_boxed_str());
        // Columns: 1975 2005 2010 2015 2019 2020 2021 2022, then the same years again.
        v.push(token(
            gdp,
            oecd,
            ctt_shares,
            country,
            7,
            fact(gdp).to_string(),
        ));
        v.push(token(
            taxes,
            oecd,
            ctt_shares,
            country,
            15,
            fact(taxes).to_string(),
        ));
    }
    let names = ["First", "Second", "Third", "Fourth", "Fifth"];
    for (name, rate) in names.iter().zip(BRACKETS_2023) {
        v.push(cell(
            "BRACKETS_2023",
            te,
            &format!("{name} bracket"),
            2,
            format!("{}%", plain(rate)),
        ));
    }
    for (data_label, cost) in TE_HOUSEHOLD.iter().chain(TE_PUBLIC_AND_THRESHOLD.iter()) {
        let input = if TE_HOUSEHOLD.iter().any(|(l, _)| l == data_label) {
            "TE_HOUSEHOLD"
        } else {
            "TE_PUBLIC_AND_THRESHOLD"
        };
        let verbatim = TE_LABELS.iter().find(|(d, _)| d == data_label).map_or_else(
            || panic!("no verbatim label for `{data_label}` in TE_LABELS"),
            |(_, s)| *s,
        );
        v.push(cell(input, te, verbatim, 1, "GST".to_string()));
        v.push(cell(input, te, verbatim, TE_2024, thousands(*cost)));
    }
    v
}

/// `1 January 2018` → `January 1, 2018` (the form ZATCA uses).
fn us_date(d: &str) -> String {
    let parts: Vec<&str> = d.split(' ').collect();
    assert_eq!(parts.len(), 3, "date `{d}`");
    format!("{} {}, {}", parts[1], parts[0], parts[2])
}

/// Phrases the paper states as text (dates, mechanisms), each with its verbatim support.
/// `(stem, excerpt phrase, paper phrase)`.
const TEXT_CHECKS: &[(&str, &str, &str)] = &[
    (
        "lop_prb0003",
        "introduced on 1 January 1991",
        "on 1 January 1991",
    ),
    (
        "lop_prb0003",
        "in existence since 1924",
        "in place since 1924",
    ),
    (
        "pa2025_vol1_s3",
        "from December 14, 2024 to February 15, 2025",
        "(14 Dec 2024 to 15 Feb 2025)",
    ),
    ("oecd_ctt_2024", "As of 1 July 2024", "as of 1 July 2024"),
    (
        "cra_cgeb",
        "replaced the GST/HST credit in July 2026",
        "in July 2026",
    ),
    ("cra_cgeb", "from 2026 to 2031", "increase for 2026-2031"),
    (
        "te2026_part1",
        "the HST is applied at a rate equal to the rate of the GST plus a provincial component",
        "applies the GST rate plus a provincial component",
    ),
    (
        "te2026_part1",
        "claim tax credits to recover the GST paid on their business inputs",
        "getting a credit for the tax they paid on their inputs",
    ),
    (
        "te2026_part1",
        "the GST is effectively imposed on final consumption",
        "the tax falls on final consumption",
    ),
];

#[test]
fn sourced_values_appear_in_their_excerpts() {
    let bad: Vec<String> = checks()
        .iter()
        .filter(|c| !c.ok)
        .map(|c| format!("{} vs tests/sources/{}.txt: {}", c.input, c.stem, c.detail))
        .collect();
    assert!(
        bad.is_empty(),
        "values not found in their source excerpts:\n{}",
        bad.join("\n")
    );
}

#[test]
fn paper_text_appears_in_its_excerpts() {
    let paper = std::fs::read_to_string(root().join("../tax-whitepaper.md")).expect("paper");
    for (stem, source_phrase, paper_phrase) in TEXT_CHECKS {
        assert!(
            excerpt(stem).contains(source_phrase),
            "tests/sources/{stem}.txt lacks `{source_phrase}`"
        );
        assert!(
            paper.contains(paper_phrase),
            "tax-whitepaper.md lacks `{paper_phrase}`"
        );
    }
}

#[test]
fn excerpt_files_have_url_and_access_date() {
    let dir = root().join("tests/sources");
    let mut stems = BTreeSet::new();
    for entry in std::fs::read_dir(&dir).expect("tests/sources") {
        let path = entry.expect("entry").path();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("stem")
            .to_string();
        let h = header(&stem);
        assert!(
            h.lines().any(|l| l.starts_with("URL: http")),
            "{stem}: no `URL: http…` line"
        );
        assert!(
            h.lines().any(|l| l == ACCESSED),
            "{stem}: no `{ACCESSED}` line"
        );
        assert!(
            h.lines().any(|l| l.starts_with("Retrieved: ")),
            "{stem}: no `Retrieved:` line"
        );
        assert!(!excerpt(&stem).trim().is_empty(), "{stem}: empty excerpt");
        stems.insert(stem);
    }
    let used: BTreeSet<String> = checks()
        .iter()
        .map(|c| c.stem.to_string())
        .chain(TEXT_CHECKS.iter().map(|(s, _, _)| (*s).to_string()))
        .collect();
    let unused: Vec<_> = stems.difference(&used).collect();
    assert!(
        unused.is_empty(),
        "excerpt files no check reads: {unused:?}"
    );
}

/// Names of every `pub const` in `src/data.rs` (except the `FACTS` table itself) and every
/// `FACTS` key.
fn all_inputs() -> BTreeSet<String> {
    let src = std::fs::read_to_string(root().join("src/data.rs")).expect("data.rs");
    let mut names: BTreeSet<String> = src
        .lines()
        .filter_map(|l| l.strip_prefix("pub const "))
        .filter_map(|r| r.split(':').next())
        .map(str::to_string)
        .filter(|n| n != "FACTS")
        .collect();
    names.extend(FACTS.iter().map(|(k, _)| (*k).to_string()));
    names
}

#[test]
fn ledger_covers_every_input() {
    let inputs = all_inputs();
    let listed: BTreeSet<String> = INPUTS
        .iter()
        .flat_map(|c| c.inputs.iter().map(|s| (*s).to_string()))
        .collect();
    let missing: Vec<_> = inputs.difference(&listed).collect();
    let unknown: Vec<_> = listed.difference(&inputs).collect();
    assert!(
        missing.is_empty(),
        "inputs with no row in src/ledger.rs: {missing:?}"
    );
    assert!(
        unknown.is_empty(),
        "ledger names that are not inputs: {unknown:?}"
    );
}

#[test]
fn verified_rows_have_excerpt_checks() {
    let checks = checks();
    let mut bad = Vec::new();
    for claim in INPUTS
        .iter()
        .filter(|c| matches!(c.status, Status::Source | Status::Derived))
    {
        for input in claim.inputs {
            let mine: Vec<&Check> = checks.iter().filter(|c| c.input == *input).collect();
            if mine.is_empty() {
                bad.push(format!(
                    "`{input}` is {:?} but has no excerpt check",
                    claim.status
                ));
            }
            for c in mine {
                if !claim.evidence.contains(&format!("`{}`", c.stem)) {
                    bad.push(format!(
                        "`{input}` is checked against `{}`, which its evidence does not name",
                        c.stem
                    ));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn unverified_inputs_are_not_marked_verified() {
    for name in ["LAND_VALUE_UNVERIFIED", "GROWTH_ADDITION_PP"] {
        let rows: Vec<_> = INPUTS.iter().filter(|c| c.inputs.contains(&name)).collect();
        assert!(!rows.is_empty(), "{name} has no ledger row");
        for r in rows {
            assert_eq!(
                r.status,
                Status::Unverified,
                "{name} must stay UNVERIFIED (no primary source)"
            );
        }
    }
    for c in INPUTS {
        if c.status == Status::Unverified {
            assert!(
                !c.evidence.is_empty(),
                "an UNVERIFIED row must say why: {}",
                c.claim
            );
        }
    }
}

#[test]
fn helpers_format_like_the_sources() {
    assert_eq!(thousands(234_319.0), "234,319");
    assert_eq!(thousands(17_267.4), "17,267.4");
    assert_eq!(thousands(445.0), "445");
    assert_eq!(plain(20.5), "20.5");
    assert_eq!(us_date("1 January 2018"), "January 1, 2018");
}
