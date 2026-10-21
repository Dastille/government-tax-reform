<!-- GENERATED from analysis/templates/CLAIMS.md by `cargo run --release -- --write`. Edit the template, not this file. -->
# Claims ledger

This file lists the quantitative claims in `tax-whitepaper.md` and `README.md` with their evidence:

- the test that backs it (tests are in `analysis/tests/`);
- the primary source, for inputs (see `analysis/src/data.rs`);
- or **UNVERIFIED**.

Reproduce everything with:

```sh
cargo run --release --manifest-path analysis/Cargo.toml
cargo test --manifest-path analysis/Cargo.toml
```

**Model status:** this is an accounting model, not a general-equilibrium model. Figures marked (A) use the central incidence assumption: the real basket is held and the VAT passes into prices. Figures marked (B) assume nominal spending is held. Every scenario is computed under both.

## What the document tests check, and what they do not

The paper, README and this file are generated: `analysis/templates/*.md` plus values from the model (`analysis/src/docs.rs`). The inputs table below is generated from `analysis/src/ledger.rs`.

| Check | Status | Evidence |
|---|---|---|
| Each committed document is byte-for-byte equal to a fresh render of its template, so every number inserted from the model (tables, and inline placeholders) is the model's current value, in its place | Verified | `docs.rs` `committed_documents_equal_fresh_render` |
| `analysis/output/revenue_tables.md` is byte-for-byte equal to a fresh render | Verified | `docs.rs` `committed_output_equals_fresh_render`; CI diff step |
| Every number typed into a template or a ledger row, with its prefix and suffix (`$227.0B`, `45pct`, `13.5%`, `45–90`, `x1.58`), is on the allowlist for the line it appears on | Verified, with the skips below | `docs.rs` `template_literals_are_placeholders_or_allowlisted` |
| Number words typed into a template (the list is `NUMBER_WORDS` in `tests/docs.rs`) are on the allowlist for their line | Verified, with the limits below | same test |
| Each allowlist entry matches exactly one line, the token is on it, and the entry's source quotes the token | Verified | `docs.rs` `allowlist_entries_are_used` |
| Each sourced input equals the value in a committed verbatim excerpt of its source (URL and access date in each file), in the right row and column for tables | Verified | `sources.rs` `sourced_values_appear_in_their_excerpts`, `excerpt_files_have_url_and_access_date`; excerpts in `analysis/tests/sources/` |
| Dates and mechanisms stated as text (GST start date, the FST start year, GST holiday dates, VAT-country date, CGEB dates, input-tax-credit mechanism, HST) appear in the paper and in a committed excerpt | Verified | `sources.rs` `paper_text_appears_in_its_excerpts` |
| Every input constant in `data.rs` and every `FACTS` key has a row in the inputs table; every Verified or Derived row is backed by excerpt checks and names its excerpt; unverified inputs stay unverified | Verified | `sources.rs` `ledger_covers_every_input`, `verified_rows_have_excerpt_checks`, `unverified_inputs_are_not_marked_verified` |
| Comparison words in the prose ("more than", "about half", "about a third", "larger", "lower", "well above", "is enough", "small next to") are generated from the values they compare | Verified | `docs.rs` `comparison_words_match_the_values`; `docs::values` |
| Templates reference only known values, and every placeholder is closed | Verified | `render_template` errors; exercised by every render |

**What the scan reads.** Every line of every template, including HTML comments, code spans, code blocks and the References section, plus every claim and evidence string in `ledger.rs`. Reference years, page and table numbers are allowlisted by line like any other number.

**What the scan skips, and why.**

- Placeholders: their values are generated and byte-checked.
- URLs and link targets (anything containing `http`, `doi.org` or `](`).
- Identifiers: a number preceded by a letter or `_` (`R14`, `FY2024-25`, `v2.0`, `YA2024`, `Q2`, `A1.1`); dotted ids joined to a letter (`2.A.7`); ids whose letters run into more digits (commit ids such as `07e4fb1`); StatCan table ids (`10-10-0147-01`).
- `§` references to sections that exist in the paper (a reference to a missing section is reported); the section number at the start of a heading; ordered-list markers that continue their list (an out-of-sequence marker is reported).

**Known limits.**

- "one" and ordinals ("second quarter", "third bracket") are not scanned: they are mostly not quantities, so a quantity written that way needs review by eye.
- The scan checks that a literal sits on a known line; whether it is correct rests on its source. Literals that are model inputs are instead checked against source excerpts (above).
- Text inside the Rust table code (`report.rs`, `docs.rs`) is not scanned. Tables are built from constants; a few labels hold fixed numbers that are definitions (for example "maximum possible 1.0" for the MPC).
- The excerpts were captured on 2026-10-18 through the fetch tool's HTML-to-Markdown rendering, `curl` or `pdftotext`, with whitespace collapsed. They are verbatim from those renderings, not byte copies of the pages, and a test cannot show that a live page still says the same thing.

## Within the model

| Claim | Status | Evidence |
|---|---|---|
| Within one incidence assumption, more revenue at a rate means a lower-or-equal required rate | Verified | `model.rs` `higher_revenue_implies_lower_or_equal_required_rate` (all scenarios, A and B, rates 8–25%) |
| Dynamic differs from static only by the spending effect | Verified | `model.rs` `dynamic_differs_from_static_only_by_spending` |
| Both assumptions reproduce today's GST at {{gst_rate}}% | Verified | `model.rs` `both_incidence_assumptions_reproduce_todays_gst_at_5_percent` |
| {{start_rate}}% static raises ${{static_hi_a}}B (A), against PIT + CIT of ${{it_b}}B; static rate for the core need {{rate_static_a}} | Verified | `model.rs` `audit_finding_twenty_percent_raises_about_210b` |
| Dynamic central at {{start_rate}}%: ${{dyn_hi_a}}B (A), rate {{rate_dyn_a}}, k = {{k_a}} | Verified | `model.rs` `central_values` |
| No MPC ≤ 1 closes the gap at {{start_rate}}% ({{mpc_be_a}} needed) | Verified | `model.rs` `central_values` |
| Required rates actually meet the need | Verified | `model.rs` `required_rate_actually_meets_need` |
| VAT(k) + k × (PIT + CIT) = need | Verified | `model.rs` `partial_replacement_balances` |
| Rate path {{start_rate}} − {{path_step}}t gives {{start_rate}} / {{path_mid}} / {{end_rate}} at t = 0 / 5 / {{path_years}}; v1.1's formula gave {{v11_end}} | Verified | `model.rs` `rate_path_is_consistent` |
| LVT arithmetic: ${{lvt_claim_hi}}B at {{lvt_rate_hi}}% needs ${{land_needed_lo}}T of land | Verified (arithmetic); land value **UNVERIFIED** | `model.rs` `lvt_land_requirement` |

## Inputs

Generated from `analysis/src/ledger.rs`. Excerpt files are in `analysis/tests/sources/`.

{{table:claims_inputs}}

## Source caveats

- **Meade Report [R17].** The paper says the committee "favoured an expenditure-tax base, setting out universal (ch. 9) and two-tier (ch. 10) variants". The chapter titles come from the book's table of contents; the report's text was not read for this revision, so the characterisation is secondary.
- **Hall-Rabushka [R18], Bradford [R19] and the Carter Commission [R21]** are described from publisher pages and a secondary summary; the primary texts were not opened. The paper dates the Carter report 1966, as The Canadian Encyclopedia does; the external audit notes it was dated December 1966 and tabled in February 1967, which was not checked against the report itself.
- **Saudi Ministry of Finance [R15b].** The page URL encodes 11 May 2020 and its dateline reads 5/12/2020; the paper uses 11 May 2020. The quoted sentence is in the excerpt.

## Not quantified / UNVERIFIED

| Claim | Status |
|---|---|
| Full cost of a dividend that offsets the VAT for the bottom {{dividend_lo}}–{{dividend_hi}}% | UNVERIFIED |
| Incidence of PIT cuts by decile | UNVERIFIED |
| Cost of zero-rating children's clothing | UNVERIFIED |
| Compliance leakage at higher rates | Assumption sweep only |
| Which incidence assumption (A or B) holds in practice | Not estimated; both are shown |
| UAE, Bahrain and Oman VAT; Denmark and Australia LVT; constitutional route to entrenchment | UNVERIFIED |
