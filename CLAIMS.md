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
- The excerpts were captured on 2026-10-18 (v3.0 additions on 2026-10-21) through the fetch tool's HTML-to-Markdown rendering, `curl` or `pdftotext`, with whitespace collapsed. They are verbatim from those renderings, not byte copies of the pages, and a test cannot show that a live page still says the same thing.

## Within the model

| Claim | Status | Evidence |
|---|---|---|
| Within one incidence assumption, more revenue at a rate means a lower-or-equal required rate | Verified | `model.rs` `higher_revenue_implies_lower_or_equal_required_rate` (all scenarios, A and B, rates 8–25%) |
| Dynamic differs from static only by the spending effect | Verified | `model.rs` `dynamic_differs_from_static_only_by_spending` |
| Both assumptions reproduce today's GST at 5% | Verified | `model.rs` `both_incidence_assumptions_reproduce_todays_gst_at_5_percent` |
| 20% static raises $210.0B (A), against PIT + CIT of $331.3B; static rate for the core need 36.5% | Verified | `model.rs` `audit_finding_twenty_percent_raises_about_210b` |
| Dynamic central at 20%: $242.2B (A), rate 32.1%, k = 47.3% | Verified | `model.rs` `central_values` |
| No MPC ≤ 1 closes the gap at 20% (5.13 needed) | Verified | `model.rs` `central_values` |
| Required rates actually meet the need | Verified | `model.rs` `required_rate_actually_meets_need` |
| VAT(k) + k × (PIT + CIT) = need | Verified | `model.rs` `partial_replacement_balances` |
| Rate path 20 − 1.2t gives 20 / 14.0 / 8 at t = 0 / 5 / 10; v1.1's formula gave 9.2 | Verified | `model.rs` `rate_path_is_consistent` |
| Debt-neutral rule: without a shock, the deficit change against the baseline is never positive, in any case, over 100 years | Verified | `transition.rs` `deficit_never_rises_without_a_shock` |
| The rule cuts k only from realised, structural surplus, and the cut never exceeds it | Verified | `transition.rs` `cuts_never_exceed_the_structural_surplus` |
| Pause and reversal: after a 3% base fall the rule raises k and repays the account in 3 years | Verified | `transition.rs` `recession_triggers_reversal_and_repayment` |
| Spin-up does not close the gap in any case: k settles at 49.8% / 46.9% / 42.7% | Verified (arithmetic); semi-elasticity **UNVERIFIED** | `transition.rs` `no_case_reaches_full_abolition`, `simulation_converges_to_settled_k` |
| With broadening, a Program-Review-sized cut and the LVT, the central case reaches full abolition at a VAT of 19.1% | Verified (arithmetic); LVT land value **UNVERIFIED** | `transition.rs` `package_closes_the_central_case` |
| LVT arithmetic: $100B at 1% needs $5T of land | Verified (arithmetic); land value **UNVERIFIED** | `model.rs` `lvt_land_requirement` |

## Inputs

Generated from `analysis/src/ledger.rs`. Excerpt files are in `analysis/tests/sources/`.

| Claim | Status | Evidence |
|---|---|---|
| PIT 234,319; CIT 96,954; non-resident income tax 13,528 ($M, FY2024-25) | Verified (source excerpt) | Public Accounts 2025, Vol I, Table 3.2 [R1]; `pa2025_vol1_s3` |
| GST 62,334, less the ministries' remission 3,565, gives gross GST from outside parties 58,769; less quarterly credits 6,266, net GST 52,503 ($M) | Verified (source excerpt) | Public Accounts 2025, Vol I, Table 3.3 [R1]; `pa2025_vol1_s3` |
| GST rate 5% | Verified (source excerpt) | Public Accounts 2025, Vol I, Section 3 text [R1]; `pa2025_vol1_s3` |
| EI premiums 31,530; total revenues 510,951 ($M); revenues 16.6% of GDP | Verified (source excerpt) | Annual Financial Report 2024-25, Table 4 and text [R2]; `afr2024_25` |
| GDP ≈ $3,078B (total revenues ÷ revenue ratio) | Derived from verified inputs | From the AFR values above; `afr2024_25` |
| Tax-expenditure costs, calendar 2024 column: 11 household zero-ratings and exemptions, public-body rebates and the small-supplier threshold | Verified (source excerpt) | Federal Tax Expenditures 2026, Part 2 [R3]; `te2026_part2` (each row and column checked) |
| GST holiday cost $1.3B (445 + 840 $M) | Verified (source excerpt) | Federal Tax Expenditures 2026, Part 2, "Targeted Temporary GST/HST Relief" [R3]; `te2026_part2` |
| Federal PIT bracket rates 15 / 20.5 / 26 / 29 / 33% (2023) | Verified (source excerpt) | Federal Tax Expenditures 2026, Part 2, background table [R3]; `te2026_part2` |
| VRR: Canada 0.50, OECD average 0.58, New Zealand 0.96 (2022) | Verified (source excerpt) | OECD Consumption Tax Trends 2024, Annex Table 2.A.7 [R4]; `oecd_ctt_2024` |
| Household saving rate 7.2% (Q2 2024), 5.0% (Q2 2025) | Verified (source excerpt) | StatCan, The Daily [R9, R10]; `statcan_daily_2024-09-12`, `statcan_daily_2025-09-11` |
| Household consumption 55.6% and imports 32.7% of GDP (Canada, 2024) | Verified (source excerpt) | World Bank WDI, Table 4.8 [R11]; `wdi_table_4_8` |
| Short-run multipliers: PIT measures 1.0, CIT measures 0.3 | Verified (source excerpt) | Budget 2009, Annex 1, Table A1.1, last column [R8]; `budget2009_annex1` |
| National wealth $17,267.4B; residential real estate $8,375.9B (Q2 2025) | Verified (source excerpt) | StatCan, The Daily [R9]; `statcan_daily_2025-09-11` |
| Provincial-local property tax $2,242 per person (2024); population 41,288,599 (1 July 2024) | Verified (source excerpt) | Nova Scotia Finance, from StatCan CGFS; StatCan, The Daily; `ns_finance_gfs_2024`, `statcan_daily_2024-09-25` |
| Recurrent property tax ≈ $92.6B (per person × population) | Derived from verified inputs | From the values above; `ns_finance_gfs_2024`, `statcan_daily_2024-09-25` |
| New Zealand GST 12.5% → 15% from 1 October 2010 | Verified (source excerpt) | Inland Revenue NZ [R22]; `ird_nz_gst_2010` |
| VAT as % of GDP / % of all taxes (2022): New Zealand 10.0 / 30.3, Denmark 9.2 / 22.0, Hungary 10.1 / 28.9; standard rates Denmark 25%, Hungary 27% | Verified (source excerpt) | OECD Consumption Tax Trends 2024, Annex Tables 1.A.4 and 2.A.7 [R4]; `oecd_ctt_2024` |
| Saudi VAT 5% from 1 January 2018; 15% from July 2020 | Verified (source excerpt) | ZATCA "About VAT" [R15]; Saudi Ministry of Finance [R15b]; `zatca_about_vat`, `saudi_mof_2020-05-11` |
| Singapore: top resident PIT rate 24%; GST 9% | Verified (source excerpt) | IRAS [R16]; `iras_rates` |
| Canada 1991: 7% GST replaced the 13.5% FST | Verified (source excerpt) | Federal Tax Expenditures 2017, Part 10 [R14b]; Library of Parliament [R14]; `te2017_part10`, `lop_prb0003` |
| 175 countries and territories with a VAT | Verified (source excerpt) | OECD Consumption Tax Trends 2024 [R4]; `oecd_ctt_2024` |
| CGEB increase 25% (July 2026) | Verified (source excerpt) | Canada Revenue Agency [R13]; `cra_cgeb` |
| FairTax rate 23% of gross payments (2027) | Verified (source excerpt) | H.R. 25 (119th Congress), introduced text [R20]; `hr25_119th` |
| Federal debt 41.2% of GDP | Verified (source excerpt) | Annual Financial Report 2024-25 [R2]; `afr2024_25` |
| Land-only value ~$6.4T | **UNVERIFIED** | Secondary source citing StatCan Table 36-10-0580-01; the table was not downloaded |
| Taxable share of land after exemptions 50% | Scenario parameter, not data | Chosen to illustrate the design's exemptions |
| Growth addition +1.5–2.5 pp a year | **UNVERIFIED** | From the original paper; no source found; what-if only |
| VAT 20% → 8% over 10 years; LVT 0.5–1% raising $50–100B; dividend for the bottom 40–50%; 60–70% of incremental revenue to infrastructure | Design choice (ab193f4), not data | Ashlynn's original design (ab193f4) |
| METR on new business investment 15.6%, 13.2% with the super-deduction | Verified (source excerpt) | Federal budget, chapter on growth [R29]; excerpt `budget2025_ch1` |
| Input-output multipliers (latest year): GDP 0.833 and 6.199 jobs per $M of final demand; capital share 0.418 | **UNVERIFIED** | StatCan Table 36-10-0013-01 [R31], read from a search index copy; the live site was down on the access date, so no excerpt |
| Capital semi-elasticity 0.2% per point of METR | **UNVERIFIED** | IMF working paper [R30], abstract as summarised by a search index; the IMF pages refused the fetch |
| GST/HST gap: net 5.8%, gross 13.5% of revenue | Verified (source excerpt) | CRA overall federal tax gap report [R34]; excerpt `cra_tax_gap_2026` |
| Direct program expenses $237.6B; deficit $36.3B; average rate on unmatured debt 3.6% | Verified (source excerpt) | Annual Financial Report, expense and debt tables and statement of financial position [R2]; excerpt `afr2024_25_expenses` |
| Program Review: about $52B covered; spending down 18.9% relative to 1994-95; $25.3B of cuts; interim target 3% of GDP ($24.3B) | Verified (source excerpt) | Program Review budget plan [R23]; excerpt `budget1995` |
| Swiss debt brake: approved by 85% on 2 December 2001; debt ratio from 25.3% (2003) to 13.5% (2019) | Verified (source excerpt) | Swiss Federal Finance Administration [R24]; excerpt `efv_debt_brake` |
| Germany: structural borrowing up to 0.35% of GDP; control account limit 1.5% | Verified (source excerpt) | Act implementing the German debt brake [R25]; excerpt `bmf_art115` |
| Sweden: debt anchor 35% of GDP; framework from the early 1990s | Verified (source excerpt) | Government Offices of Sweden [R26]; excerpt `gov_se_framework` |
| New Zealand: GST at 10% from 1 October 1986; top income tax rate 48c | Verified (source excerpt) | Inland Revenue [R27, R28]; excerpts `ird_gst_bill_1985`, `ird_ita2_1986` |
| Ireland: GNI* 57.1% of GDP (latest year) | Verified (source excerpt) | CSO Annual National Accounts [R32]; excerpt `cso_ana2024_gni` |
| Estonia: profits taxed when distributed, at 22/78 | Verified (source excerpt) | Estonian Tax and Customs Board [R33]; excerpt `emta_companies` |
| Spin-up cases: semi-elasticity scale, METR cut and speed; recession of 3% in one year | Scenario parameter, not data | Chosen to span low, central and high cases; not estimates |
| Rule: repay the compensation account over 3 years; simulate 100 years | Design choice (ab193f4), not data | Design choice of v3.0 (not in ab193f4); the Swiss rule [R24] sets no fixed repayment period |

## Source caveats

- **Meade Report [R17].** The paper says the committee "favoured an expenditure-tax base, setting out universal (ch. 9) and two-tier (ch. 10) variants". The chapter titles come from the book's table of contents; the report's text was not read for this revision, so the characterisation is secondary.
- **Hall-Rabushka [R18], Bradford [R19] and the Carter Commission [R21]** are described from publisher pages and a secondary summary; the primary texts were not opened. The paper dates the Carter report 1966, as The Canadian Encyclopedia does; the external audit notes it was dated December 1966 and tabled in February 1967, which was not checked against the report itself.
- **Statistics Canada Table 36-10-0013-01 [R31].** The live site returned an outage page on the access date; the values were read from the search tool's index copy of the table page. No verbatim excerpt is committed, so these inputs are marked unverified.
- **IMF working paper [R30].** The IMF pages refused the fetch; the semi-elasticity is from a search summary of the abstract and is unverified.
- **Saudi Ministry of Finance [R15b].** The page URL encodes 11 May 2020 and its dateline reads 5/12/2020; the paper uses 11 May 2020. The quoted sentence is in the excerpt.

## Not quantified / UNVERIFIED

| Claim | Status |
|---|---|
| Full cost of a dividend that offsets the VAT for the bottom 40–50% | UNVERIFIED |
| Incidence of PIT cuts by decile | UNVERIFIED |
| Cost of zero-rating children's clothing | UNVERIFIED |
| Compliance leakage at higher rates | Assumption sweep only |
| Which incidence assumption (A or B) holds in practice | Not estimated; both are shown |
| Capital semi-elasticity to the METR (0.2% per point, IMF working paper [R30]) | UNVERIFIED |
| Federal share of the METR; the METR cut in the low and central spin-up cases | Scenario |
| Cost of a universal VAT prebate; Estonian-style distributed-profit tax revenue for Canada | UNVERIFIED |
| UAE, Bahrain and Oman VAT; Denmark and Australia LVT; constitutional route to entrenchment | UNVERIFIED |
