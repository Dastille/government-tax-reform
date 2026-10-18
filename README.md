# Government tax reform

A proposal for **fundamental tax reform in mid-sized advanced economies**: replace income, payroll, capital gains and recurrent property taxes with a single broad consumption tax (VAT), a land value tax and a citizen dividend. Canada (federal, FY2024-25) is the worked example with numbers.

This is independent analysis. It is not a bill, not advice, and not affiliated with any government.

Paper: **[tax-whitepaper.md](tax-whitepaper.md)** (v2.0). Model output: [analysis/output/revenue_tables.md](analysis/output/revenue_tables.md).

## The proposal

The full specification is §4 of the paper, which is the single source of truth.

1. **Abolish** personal and corporate income tax, payroll taxes, capital gains tax and recurrent property taxes. Income tax rates are scaled down by a factor *k* until an independent fiscal board certifies that VAT + LVT revenue covers the need without them.
2. **One broad VAT** replaces the GST. It starts at **20%** and follows rate(t) = 20 − 1.2·t to **8%** at t = 10, with each step taken only when certified.
3. **Differentiated rates.** Zero or reduced on unprepared food, medicine, education and children's clothing. Higher on luxuries, tobacco, alcohol, fossil fuels and carbon-intensive goods.
4. **Land value tax** of 0.5–1% on unimproved land value, aimed at vacant urban and speculative non-farm land.
5. **Income-tested citizen dividend.** Its floor is the current GST credit/CGEB scaled to the VAT rate.
6. **Independent fiscal board**, plus **referendum and supermajority entrenchment**.

## What the numbers say (Canada, FY2024-25, federal, $B)

| | $B | Source |
|---|---:|---|
| Personal income tax | 234.3 | Public Accounts 2025, Vol I, Table 3.2 |
| Corporate income tax | 97.0 | Public Accounts 2025, Vol I, Table 3.2 |
| Net GST (5%) | 52.5 | Public Accounts 2025, Vol I, Table 3.3 |
| **Core need** (PIT + CIT + GST) | **383.8** | sum |
| VAT at 20%, static | 210.0 | model |
| VAT at 20%, households spend the income tax they keep (MPC 0.95) | 215.9 | model |
| ...plus Finance Canada short-run multipliers | 231.7 | model |

- **Headline.** A 20% VAT, even counting the extra spending from the income tax people keep, replaces about half of federal income tax: 56.1% of it must remain at launch under central assumptions.
- **Full abolition** at 20% needs a taxable base about 1.78× larger than today's relative to spending needs, for example a New Zealand-style broad base. With such a base, the static rate needed is 19.0%.
- **The 8% endpoint** needs about 4× the base.
- The paper's §9 lists every original requirement, who it came from, and whether it was kept, reframed or deleted.

## Run the model

From a clean clone, with a Rust toolchain installed:

```sh
cargo run --release --manifest-path analysis/Cargo.toml   # prints all tables
cargo test --manifest-path analysis/Cargo.toml            # model checks, and paper == model output
```

**Why Rust?** The model is a small, dependency-free crate, so it runs the same way everywhere. `cargo test` fails if the committed tables or any number in the paper drift from the code. CI (`.github/workflows/analysis.yml`) runs fmt, clippy (warnings are errors), the tests, a check that the committed tables match fresh output, and `cargo audit`.

Inputs, each with its source, are in [`analysis/src/data.rs`](analysis/src/data.rs). Figures we could not verify are labelled **unverified** in both the code and the paper.

## Filename note

The original file was committed as `tax-whitepaper. md`, with a space before `md`. It now lives at `tax-whitepaper.md`.

## License

[AGPL-3.0](LICENSE)
