<!-- GENERATED from analysis/templates/README.md by `cargo run --release -- --write`. Edit the template, not this file. -->
# Government tax reform

A proposal for **fundamental tax reform in mid-sized advanced economies**: replace income, payroll, capital gains and recurrent property taxes with a single broad consumption tax (VAT), a land value tax and a citizen dividend. Canada (federal, FY2024-25) is the worked example with numbers.

This is independent analysis. It is not a bill, not advice, and not affiliated with any government.

Paper: **[tax-whitepaper.md](tax-whitepaper.md)** (v2.0). Model output: [analysis/output/revenue_tables.md](analysis/output/revenue_tables.md).

## The proposal

The full specification is §4 of the paper, which is the single source of truth.

1. **Abolish** personal and corporate income tax, payroll taxes, capital gains tax and recurrent property taxes. Income tax rates are scaled down by a factor *k* until an independent fiscal board certifies that VAT + LVT revenue covers the need without them.
2. **One broad VAT** replaces the GST. It starts at **20%** and follows rate(t) = 20 − 1.2·t to **8%** at t = 10, with each step taken only when certified.
3. **Differentiated rates.** Zero or reduced on unprepared food, medicine, education and children's clothing. Higher on luxuries, tobacco, alcohol, fossil fuels and carbon-intensive goods.
4. **Land value tax** of 0.5–1% on unimproved land value, aimed at vacant urban and speculative non-farm land. It is the land part of today's property tax; buildings are no longer taxed.
5. **Income-tested citizen dividend.** Its floor is the current GST credit/CGEB scaled to the VAT rate.
6. **Independent fiscal board**, plus **referendum and supermajority entrenchment**.

## What the numbers say (Canada, FY2024-25, federal, $B)

Every scenario uses one stated price assumption. **A (central):** the VAT is passed into prices and the real basket is held. **B (sensitivity):** nominal spending is held. See the paper's §2.

| | A | B | Source |
|---|---:|---:|---|
| Personal income tax | 234.3 | | Public Accounts 2025, Vol I, Table 3.2 |
| Corporate income tax | 97.0 | | Public Accounts 2025, Vol I, Table 3.2 |
| Net GST (5%) | 52.5 | | Public Accounts 2025, Vol I, Table 3.3 |
| **Core need** (PIT + CIT + GST) | **383.8** | | sum |
| VAT at 20%, static | 210.0 | 183.8 | model |
| VAT at 20%, households spend the income tax they keep (MPC 0.95) | 242.2 | 215.9 | model |
| Rate needed for the core need, static / dynamic | 36.5% / 32.1% | 53.4% / 42.1% | model |
| Share of income tax that must remain at 20% (k) | 47.3% | 56.1% | model |
| Recurrent property tax the LVT must replace first | 92.6 | | StatCan CGFS (derived) |

- **Headline.** A 20% VAT, counting the extra spending from the income tax people keep, replaces about half of federal income tax. Under central assumptions, 47.3% of it must remain at launch.
- **Full abolition** at 20% needs a taxable base about 1.58× larger than today's relative to spending needs. A New Zealand-style broad base is enough: with it, the static rate needed is 19.0%.
- **The 8% endpoint** needs about 3.90× the base.
- **Property tax** (about $92.6B) stays until LVT revenue replaces it.
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
