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

The paper, README and this file are generated: `analysis/templates/*.md` plus values from the model (`analysis/src/docs.rs`).

| Check | Status | Evidence |
|---|---|---|
| Each committed document is byte-for-byte equal to a fresh render of its template, so every number inserted from the model (tables, and inline placeholders) is the model's current value, in its place, integers included | Verified | `docs.rs` `committed_documents_equal_fresh_render` |
| `analysis/output/revenue_tables.md` is byte-for-byte equal to a fresh render | Verified | `docs.rs` `committed_output_equals_fresh_render`; CI diff step |
| Every number typed directly into a template (outside a placeholder) is on the allowlist, with a source and the line it appears on | Verified, with exclusions below | `docs.rs` `template_literals_are_placeholders_or_allowlisted` |
| No allowlist entry is stale (each one matches a line in its template) | Verified | `docs.rs` `allowlist_entries_are_used` |
| Templates reference only known values, and every placeholder is closed | Verified | `render_template` errors; exercised by every render |

**Not scanned for literals:** the References section of the paper (it holds citation years, table numbers and URLs); tokens joined to letters (`R4`, `FY2024-25`, `2.A.7`, commit ids); section numbers in headings and `§` references; numbered-list markers; URLs; HTML comments; code spans and code blocks. Each allowlist entry names its template, an anchor that must match exactly one line, and the token; the token must appear on that line as a whole token (not as part of a longer number), and every literal on every scanned line must be covered by an entry. The test checks the literal's **location**, not its correctness; its correctness rests on the cited source.

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
| LVT arithmetic: $100B at 1% needs $5T of land | Verified (arithmetic); land value **UNVERIFIED** | `model.rs` `lvt_land_requirement` |

## Inputs (sourced)

| Claim | Status | Evidence |
|---|---|---|
| PIT 234,319; CIT 96,954; net GST 52,503 ($M, FY2024-25) | Verified (source) | Public Accounts 2025 Vol I Tables 3.2/3.3; `model.rs` `baseline_matches_public_accounts` |
| Revenue 16.6% of GDP, so GDP ≈ $3,078B | Verified (source, derived) | AFR 2024-25 |
| Saving rate 7.2% (Q2 2024), 5.0% (Q2 2025) | Verified (source) | StatCan Daily 2024-09-12, 2025-09-11 |
| HFCE 55.6% and imports 32.7% of GDP (2024) | Verified (source) | World Bank WDI 4.8 |
| Multipliers: PIT 1.0, CIT 0.3 (short run) | Verified (source) | Budget 2009 Annex 1 Table A1.1 |
| VRR: Canada 0.50, OECD average 0.58, New Zealand 0.96 | Verified (source) | OECD CTT 2024 Annex 2.A.7 |
| Tax-expenditure costs | Verified (source) | Federal Tax Expenditures 2026 Part 2 |
| National wealth $17,267.4B; residential real estate $8,375.9B | Verified (source) | StatCan Daily 2025-09-11 |
| Recurrent property tax ≈ $92.6B (provincial + local, 2024) | Verified (source, derived) | Nova Scotia Finance from StatCan Table 10-10-0147-01 (per person) × StatCan population 1 July 2024 |
| New Zealand GST 12.5% → 15% on 1 October 2010 | Verified (source) | Inland Revenue NZ, Taxation (Budget Measures) Act 2010 |
| Saudi VAT 5% from 1 January 2018; 15% from July 2020 | Verified (source) | ZATCA "About VAT"; Saudi Ministry of Finance, 11 May 2020 |
| FairTax rate 23% of gross payments (2027) | Verified (source) | H.R. 25 (119th), introduced text |
| Land-only value ~$6.4T | **UNVERIFIED** | Secondary source; StatCan Table 36-10-0580 not downloaded |
| Growth addition +1.5–2.5 pp a year | **UNVERIFIED** (what-if only) | From the original paper; no source found |

## Not quantified / UNVERIFIED

| Claim | Status |
|---|---|
| Full cost of a dividend that offsets the VAT for the bottom 40–50% | UNVERIFIED |
| Incidence of PIT cuts by decile | UNVERIFIED |
| Cost of zero-rating children's clothing | UNVERIFIED |
| Compliance leakage at higher rates | Assumption sweep only |
| Which incidence assumption (A or B) holds in practice | Not estimated; both are shown |
| UAE, Bahrain and Oman VAT; Denmark and Australia LVT; constitutional route to entrenchment | UNVERIFIED |
