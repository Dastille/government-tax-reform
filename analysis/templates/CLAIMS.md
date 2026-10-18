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
| Both assumptions reproduce today's GST at {{gst_rate}}% | Verified | `model.rs` `both_incidence_assumptions_reproduce_todays_gst_at_5_percent` |
| {{start_rate}}% static raises ${{static_hi_a}}B (A), against PIT + CIT of ${{it_b}}B; static rate for the core need {{rate_static_a}} | Verified | `model.rs` `audit_finding_twenty_percent_raises_about_210b` |
| Dynamic central at {{start_rate}}%: ${{dyn_hi_a}}B (A), rate {{rate_dyn_a}}, k = {{k_a}} | Verified | `model.rs` `central_values` |
| No MPC ≤ 1 closes the gap at {{start_rate}}% ({{mpc_be_a}} needed) | Verified | `model.rs` `central_values` |
| Required rates actually meet the need | Verified | `model.rs` `required_rate_actually_meets_need` |
| VAT(k) + k × (PIT + CIT) = need | Verified | `model.rs` `partial_replacement_balances` |
| Rate path {{start_rate}} − {{path_step}}t gives {{start_rate}} / {{path_mid}} / {{end_rate}} at t = 0 / 5 / {{path_years}}; v1.1's formula gave {{v11_end}} | Verified | `model.rs` `rate_path_is_consistent` |
| LVT arithmetic: ${{lvt_claim_hi}}B at {{lvt_rate_hi}}% needs ${{land_needed_lo}}T of land | Verified (arithmetic); land value **UNVERIFIED** | `model.rs` `lvt_land_requirement` |

## Inputs (sourced)

| Claim | Status | Evidence |
|---|---|---|
| PIT {{pit_m}}; CIT {{cit_m}}; net GST {{gst_m}} ($M, FY2024-25) | Verified (source) | Public Accounts 2025 Vol I Tables 3.2/3.3; `model.rs` `baseline_matches_public_accounts` |
| Revenue {{rev_pct_gdp}}% of GDP, so GDP ≈ ${{gdp_b}}B | Verified (source, derived) | AFR 2024-25 |
| Saving rate {{saving_2024}} (Q2 2024), {{saving_2025}} (Q2 2025) | Verified (source) | StatCan Daily 2024-09-12, 2025-09-11 |
| HFCE {{hfce_pct}}% and imports {{imports_pct}}% of GDP (2024) | Verified (source) | World Bank WDI 4.8 |
| Multipliers: PIT {{mult_pit}}, CIT {{mult_cit}} (short run) | Verified (source) | Budget 2009 Annex 1 Table A1.1 |
| VRR: Canada {{vrr_ca}}, OECD average {{vrr_oecd}}, New Zealand {{vrr_nz}} | Verified (source) | OECD CTT 2024 Annex 2.A.7 |
| Tax-expenditure costs | Verified (source) | Federal Tax Expenditures 2026 Part 2 |
| National wealth ${{wealth_b}}B; residential real estate ${{res_re_b}}B | Verified (source) | StatCan Daily 2025-09-11 |
| Recurrent property tax ≈ ${{property_b}}B (provincial + local, 2024) | Verified (source, derived) | Nova Scotia Finance from StatCan Table 10-10-0147-01 (per person) × StatCan population 1 July 2024 |
| New Zealand GST {{nz_prev_rate}}% → {{nz_rate}}% on {{nz_rate_date}} | Verified (source) | Inland Revenue NZ, Taxation (Budget Measures) Act 2010 |
| Saudi VAT {{sa_rate_2018}}% from {{sa_date_2018}}; {{sa_rate_2020}}% from {{sa_date_2020}} | Verified (source) | ZATCA "About VAT"; Saudi Ministry of Finance, 11 May 2020 |
| FairTax rate {{fairtax_rate}}% of gross payments (2027) | Verified (source) | H.R. 25 (119th), introduced text |
| Land-only value ~${{land_t}}T | **UNVERIFIED** | Secondary source; StatCan Table 36-10-0580 not downloaded |
| Growth addition +{{growth_lo}}–{{growth_hi}} pp a year | **UNVERIFIED** (what-if only) | From the original paper; no source found |

## Not quantified / UNVERIFIED

| Claim | Status |
|---|---|
| Full cost of a dividend that offsets the VAT for the bottom {{dividend_lo}}–{{dividend_hi}}% | UNVERIFIED |
| Incidence of PIT cuts by decile | UNVERIFIED |
| Cost of zero-rating children's clothing | UNVERIFIED |
| Compliance leakage at higher rates | Assumption sweep only |
| Which incidence assumption (A or B) holds in practice | Not estimated; both are shown |
| UAE, Bahrain and Oman VAT; Denmark and Australia LVT; constitutional route to entrenchment | UNVERIFIED |
