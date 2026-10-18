# Claims ledger

This file lists every quantitative claim in `tax-whitepaper.md` and `README.md` with its evidence:

- the test that backs it (tests are in `analysis/tests/`);
- the primary source, for inputs (see `analysis/src/data.rs`);
- or **UNVERIFIED**.

Reproduce everything with:

```sh
cargo run --release --manifest-path analysis/Cargo.toml
cargo test --manifest-path analysis/Cargo.toml
```

**Model status:** this is an accounting model, not a general-equilibrium model. Figures marked (A) use the central incidence assumption: the real basket is held and the VAT passes into prices. Figures marked (B) assume nominal spending is held. Every scenario is computed under both.

## Blanket checks

| Claim | Status | Evidence |
|---|---|---|
| Every decimal number in the paper body is model output or a listed sourced input | Verified | `paper.rs` `every_number_in_the_paper_is_model_output_or_sourced` |
| Every decimal number in the README is model output | Verified | `paper.rs` `every_number_in_the_readme_is_model_output` |
| The paper's key-figures and first-principles blocks equal the model output exactly | Verified | `paper.rs` `key_figures_block_matches_model`, `first_principles_block_matches_model` |
| `analysis/output/revenue_tables.md` is current | Verified | `model.rs` `committed_output_is_current`; CI diff step |
| Within one incidence assumption, more revenue at a rate means a lower-or-equal required rate | Verified | `model.rs` `higher_revenue_implies_lower_or_equal_required_rate` (all scenarios, A and B, rates 8–25%) |
| Dynamic differs from static only by the spending effect | Verified | `model.rs` `dynamic_differs_from_static_only_by_spending` |
| Both assumptions reproduce today's GST at 5% | Verified | `model.rs` `both_incidence_assumptions_reproduce_todays_gst_at_5_percent` |

## Inputs (sourced)

| Claim | Status | Evidence |
|---|---|---|
| PIT 234,319; CIT 96,954; net GST 52,503 ($M, FY2024-25) | Verified (source) | Public Accounts 2025 Vol I Tables 3.2/3.3; `model.rs` `baseline_matches_public_accounts` |
| GST components reconcile: 62,334 − 3,565 = 58,769, and 58,769 − 6,266 = 52,503 | Verified | `baseline_matches_public_accounts` |
| Revenue 16.6% of GDP, so GDP ≈ $3,078B | Verified (source, derived) | AFR 2024-25 |
| Saving rate 7.2% (Q2 2024), 5.0% (Q2 2025) | Verified (source) | StatCan Daily 2024-09-12, 2025-09-11 |
| HFCE 55.6% and imports 32.7% of GDP (2024) | Verified (source) | World Bank WDI 4.8 |
| Multipliers: PIT 1.0, CIT 0.3 (short run) | Verified (source) | Budget 2009 Annex 1 Table A1.1 |
| VRR: Canada 0.50, OECD average 0.58, New Zealand 0.96 | Verified (source) | OECD CTT 2024 Annex 2.A.7 |
| Tax-expenditure costs (groceries 6,620, etc.) | Verified (source) | Federal Tax Expenditures 2026 Part 2 |
| National wealth $17,267.4B; residential real estate $8,375.9B | Verified (source) | StatCan Daily 2025-09-11 |
| Land-only value ~$6.4T | **UNVERIFIED** | Secondary source; StatCan Table 36-10-0580 not downloaded |

## Results

| Claim | Status | Evidence |
|---|---|---|
| 20% static raises $210.0B (A), against PIT + CIT of $331.3B | Verified | `audit_finding_twenty_percent_raises_about_210b` |
| Static rate for the core need is 36.5% (A) | Verified | same test |
| Dynamic central at 20%: $242.2B (A), rate 32.1%, k = 47.3% | Verified | `central_values` |
| No MPC ≤ 1 closes the gap at 20% (5.13 needed) | Verified | `central_values` (`breakeven_mpc > 1`) |
| Required rates actually meet the need | Verified | `required_rate_actually_meets_need` |
| VAT(k) + k × (PIT + CIT) = need | Verified | `partial_replacement_balances` |
| Rate path 20 − 1.2t gives 20 / 14 / 8 at t = 0 / 5 / 10; v1.1 formula gave 9.2 | Verified | `rate_path_is_consistent` |
| $50B at 1% needs $5T of land; 1% of $6.4T is $64B | Verified (arithmetic); land value **UNVERIFIED** | `lvt_land_requirement` |
| Base growth 1.58× (20%) and 3.90× (8%); Table 9 stacks | Verified via output | `committed_output_is_current` + paper number test |

## Not quantified / UNVERIFIED

| Claim | Status |
|---|---|
| Full cost of a dividend that offsets the VAT for the bottom 40–50% | UNVERIFIED |
| Incidence of PIT cuts by decile | UNVERIFIED |
| Municipal property tax revenue to be replaced by the LVT | UNVERIFIED |
| Cost of zero-rating children's clothing | UNVERIFIED |
| Compliance leakage at higher rates | Assumption sweep only |
| Which incidence assumption (A or B) holds in practice | Not estimated; both are shown |
| UAE, Bahrain and Oman VAT; Denmark and Australia LVT; constitutional route to entrenchment | UNVERIFIED |
