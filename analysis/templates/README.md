<!-- GENERATED from analysis/templates/README.md by `cargo run --release -- --write`. Edit the template, not this file. -->
# Government tax reform

A proposal for **fundamental tax reform in mid-sized advanced economies**: replace income, payroll, capital gains and recurrent property taxes with a single broad consumption tax (VAT), a land value tax and a citizen dividend. Canada (federal, FY2024-25) is the worked example with numbers.

This is independent analysis. It is not a bill, not advice, and not affiliated with any government.

Paper: **[tax-whitepaper.md](tax-whitepaper.md)** (v3.0). Model output: [analysis/output/revenue_tables.md](analysis/output/revenue_tables.md).

## The proposal

The full specification is §4 of the paper, which is the single source of truth.

1. **Abolish** personal and corporate income tax, payroll taxes, capital gains tax and recurrent property taxes. Income tax rates are scaled down by a factor *k* until an independent fiscal board certifies that VAT + LVT revenue covers the need without them.
2. **One broad VAT** replaces the GST. It starts at **{{start_rate}}%** and follows rate(t) = {{start_rate}} − {{path_step}}·t to **{{end_rate}}%** at t = {{path_years}}, with each step taken only when certified.
3. **Differentiated rates.** Zero or reduced on unprepared food, medicine, education and children's clothing. Higher on luxuries, tobacco, alcohol, fossil fuels and carbon-intensive goods.
4. **Land value tax** of {{lvt_rate_lo}}–{{lvt_rate_hi}}% on unimproved land value, aimed at vacant urban and speculative non-farm land. It is the land part of today's property tax; buildings are no longer taxed.
5. **Income-tested citizen dividend.** Its floor is the current GST credit/CGEB scaled to the VAT rate.
6. **Independent fiscal board**, plus **referendum and supermajority entrenchment**.

## What the numbers say (Canada, FY2024-25, federal, $B)

Every scenario uses one stated price assumption. **A (central):** the VAT is passed into prices and the real basket is held. **B (sensitivity):** nominal spending is held. See the paper's §2.

{{table:readme}}

- **Headline.** A {{start_rate}}% VAT, counting the extra spending from the income tax people keep, replaces {{replaced_frac_a}} of federal income tax. Under central assumptions, {{k_a}} of it must remain at launch.
- **Full abolition** at {{start_rate}}% needs a taxable base about {{g_hi_a}}× {{g_hi_a_dir}} than today's relative to spending needs. A New Zealand-style broad base {{nz_verdict}}: with it, the static rate needed is {{nz_rate_needed_a}}.
- **The {{end_rate}}% endpoint** needs about {{g_lo_a}}× the base.
- **Property tax** (about ${{property_b}}B) stays until LVT revenue replaces it.

## Debt-neutral transition (v3.0)

The paper's §10, §11 and §12 add a rule under which income tax is cut only by revenue already realised above the need, with a pause and a reversal when revenue falls short. Without a shock, the deficit never rises above the baseline.

- **Does it close the gap without debt?** An industry spin-up from lower income tax (a scenario, not a forecast) {{v3_verdict}}. k settles at {{v3_settle_low}} / {{v3_settle_central}} / {{v3_settle_high}} (low / central / high); years to full abolition: {{v3_years_low}} / {{v3_years_central}} / {{v3_years_high}}.
- **What would close it.** Base broadening, a cut the size of Canada's Program Review (${{v3_pr_cut}}B) and the LVT together; with them, the central case can abolish the income tax, and the VAT it then needs is {{v3_pkg_rate_central}}. The LVT's land value is unverified.
- **Infrastructure test.** Borrowing to abolish income tax at once fails it: the spin-up revenue covers {{v3_cover_central}} of the cost in the central case.

## End state: no income tax (v3.0, §13)

- **Balanced packages, no income tax.** VAT and spending review: {{es_rate_a}}. Adding the LVT: {{es_rate_b}}. Adding the PBO wealth tax: {{es_rate_c}}. Each includes a prebate that removes the VAT on spending up to the MBM poverty line (net ${{es_prebate_net_b}}B a year in package B); without it the rates are {{es_noprebate_a}} / {{es_noprebate_b}} / {{es_noprebate_c}}.
- **Who pays.** Filers living on current income gain; those spending from savings, borrowing or unrealised gains pay more. The MBM line, the age share and the land value are unverified; consumption by income is a scenario.
- Within each assumption, more revenue always means a lower required rate, and a test enforces this.
- The paper's §9 lists every original requirement, who it came from, and whether it was kept, reframed or deleted. [CLAIMS.md](CLAIMS.md) maps every claim to its test or source.

## Run the model

From a clean clone, with a Rust toolchain installed:

```sh
cargo run --release --manifest-path analysis/Cargo.toml             # prints all tables
cargo test --manifest-path analysis/Cargo.toml                      # model checks; documents == fresh render
cargo run --release --manifest-path analysis/Cargo.toml -- --write  # regenerate the paper, README, CLAIMS and tables
```

The paper, this README and CLAIMS.md are **generated** from `analysis/templates/`. Edit a template, then run `--write`. `cargo test` fails if any committed document differs by one byte from a fresh render, or if a template contains a number that is neither a placeholder nor on the sourced allowlist in `analysis/tests/docs.rs`.

**Why Rust?** The model is a small crate with no dependencies, so it runs the same way everywhere. CI (`.github/workflows/analysis.yml`) runs fmt, clippy (pedantic, warnings are errors), the tests on stable and on the minimum supported Rust version, a check that the committed documents match a fresh render, `cargo deny` and `cargo audit`.

Inputs, each with its source, are in [`analysis/src/data.rs`](analysis/src/data.rs). Figures we could not verify are labelled **unverified** in both the code and the paper.

## Filename note

The original file was committed as `tax-whitepaper. md`, with a space before `md`. It now lives at `tax-whitepaper.md`.

## License

[AGPL-3.0](LICENSE)
