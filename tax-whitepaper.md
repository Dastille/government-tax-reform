<!-- GENERATED from analysis/templates/tax-whitepaper.md by `cargo run --release -- --write`. Edit the template, not this file. -->
# Fundamental Tax Reform: Toward a Single Consumption-Based System

**Whitepaper · v3.0 (debt-neutral transition) · October 2026**
**Author:** Ashlynn (original design, January 2026). v2.0 fixed the arithmetic and added sources; v3.0 adds a debt-neutral transition rule, an industry spin-up scenario and the infrastructure test (§10, §11 and §12). Neither changes the design goals.

**Scope.** The design is meant for **mid-sized advanced economies**. Canada is the **worked example** because its federal accounts for fiscal year 2024-25 (April 2024 to March 2025, "FY2024-25") are public and detailed. The method carries over to other countries: the VAT rate a country needs equals the revenue it must replace divided by its effective VAT base.

**How the numbers are produced.** This file is generated. Every model-derived number and table is inserted from the revenue model in [`analysis/`](analysis/), and a test fails if this file differs by one byte from a fresh render. Inputs and their sources are in [`analysis/src/data.rs`](analysis/src/data.rs). Every other number in the prose is on a sourced allowlist (`analysis/tests/docs.rs`) that names the line it may appear on. Anything we could not verify is marked **unverified**. See [CLAIMS.md](CLAIMS.md).

---

## 1. Abstract

**Problem.** Income taxes discourage work, saving and investment more than taxes on spending do. The OECD's ranking puts corporate income tax as the most harmful to growth, then personal income tax, then consumption taxes, with recurrent property taxes the least harmful [R7]. This proposal abolishes five taxes: personal income tax (PIT), corporate income tax (CIT), payroll taxes, capital gains tax and recurrent property tax. It replaces them with one broad value-added tax (VAT) that starts at 20% and steps down to 8% over 10 years. A land value tax (LVT), an income-tested citizen dividend, an independent fiscal board and referendum entrenchment complete the design.

**Thesis.** When income tax goes, people keep more of what they earn, so they spend more, and VAT revenue grows. A static one-for-one comparison misses this effect.

**Method.**

- Every Canadian baseline figure is checked against the Public Accounts and the Annual Financial Report [R1, R2].
- A small Rust model (no dependencies, tested in CI) computes:
  - static revenue at each rate;
  - a dynamic scenario in which households and firms keep $331.3B of income tax and spend part of it (the marginal propensity to consume, or MPC) at VAT-inclusive prices, with an optional demand multiplier;
  - every scenario, static included, under one stated assumption about prices and nominal spending (§2), with the alternative shown as a sensitivity;
  - base-broadening, compliance, land-value and property-tax scenarios, and the break-even conditions.

**Result.** Headline figures use incidence assumption A (§2), with assumption B as a sensitivity.

- A 20% VAT on Canada's current federal base raises **$210.0B** statically.
- With the thesis modelled at central values, it raises **$242.2B**, or **$257.9B** if Finance Canada's short-run multipliers are added.
- The revenue needed is **$383.8B**: PIT plus CIT plus today's GST.
- So a 20% VAT replaces about half of the income taxes. Under central values, **47.3%** of today's PIT and CIT must remain (**k**). Full replacement on today's base needs a rate of **32.1%**, or **30.2%** with the multiplier, against 36.5% statically.
- The spending effect is real, but no possible MPC closes the gap on its own: the MPC would have to be 5.13. Only about 61% of new consumer spending falls in today's taxable base, and that spending pays VAT-inclusive prices.
- Full abolition at 20% needs a taxable base about **1.58×** larger relative to spending needs. The 8% endpoint needs about **3.90×**.
- The LVT must first replace about **$92.6B** of recurrent property tax, more than the LVT scenario raises ($32B).
- **Sensitivity B** gives a lower floor: $183.8B static and $215.9B dynamic at 20%, k = 56.1%. The conclusion is the same under either assumption.
- v2.0 therefore keeps every mechanism of the design. Income taxes are reduced as far as VAT and LVT revenue allows, and they are abolished, and the VAT rate stepped down, only when the fiscal board certifies that revenue covers the need.
- **v3.0, debt-neutral.** Under a rule that cuts income tax only from revenue already realised (§10), the deficit never rises above the baseline without a shock. An industry spin-up from lower income tax (§11) does not close the gap in any case: k settles at 46.9% in the central case and 42.7% in the high case. Closing the gap takes base broadening, a Program-Review-sized spending cut and the LVT together (§11.3); with them, the central case can abolish the income tax, and the VAT it then needs is 19.1%.

**Key figures:**

| Key figure (Canada FY2024-25, federal; incidence A unless stated) | Value |
|---|---:|
| Core need (PIT + CIT + net GST), $B | 383.8 |
| VAT revenue at 20%: static / dynamic central (MPC 0.95) / dynamic + multiplier, $B | 210.0 / 242.2 / 257.9 |
| Rate for core need: static / dynamic central / dynamic + multiplier | 36.5% / 32.1% / 30.2% |
| Income tax still needed at 20%, dynamic central (k) | 47.3% |
| MPC needed to close the gap at 20% (maximum possible 1.0) | 5.13 |
| Base growth beyond needs to abolish income tax at 20% / at 8% | x1.58 / x3.90 |
| Rate for core need with a New Zealand-level base (VRR 0.96), static | 19.0% |
| Sensitivity B: revenue at 20% static / dynamic, $B | 183.8 / 215.9 |
| Sensitivity B: rate for core need static / dynamic; k at 20% | 53.4% / 42.1%; 56.1% |
| Recurrent property tax the LVT must replace first, $B | 92.6 |

---

## 2. Problem statement and definitions

**Problem.** Can one broad consumption tax, plus a land value tax, raise the revenue that PIT, CIT, payroll, capital gains and recurrent property taxes raise today? If so, at what rate, and under what conditions can that rate fall to 8%?

**Definitions** (each term is defined once here):

- **VAT / GST.** A tax on final consumption that businesses collect at each stage of production, getting a credit for the tax they paid on their inputs, so the tax falls on final consumption [R3, Part 1]. Canada's federal version is the 5% Goods and Services Tax (GST). The Harmonized Sales Tax (HST) applies the GST rate plus a provincial component; "GST" in this paper means the federal portion only, as in the federal tax-expenditure report [R3, Part 1].
- **Net GST (FY2024-25: $52.5B).** "Net GST from outside parties" in the Public Accounts: GST after the remission order for GST paid by federal ministries and after the quarterly GST credits [R1, Table 3.3]. The Public Accounts table does not itemize HST; we read it as federal revenue only, which excludes provincial HST portions. **This is the base used in the model.**
- **Effective base.** Net GST ÷ 5% = **$1,050.1B**: the spending that actually bears the tax after exemptions, zero-ratings, credits and non-compliance. Gross GST from outside parties ÷ 5% = $1,175.4B, which matches the original paper's "consumption base ~$1.1-1.2T".
- **Core need ($383.8B).** PIT + CIT + net GST: what the VAT must raise if it absorbs the GST and both income taxes are abolished. **Full federal scope ($428.8B)** also counts non-resident income tax ($13.5B) and EI premiums ($31.5B, a federal payroll tax).
- **Revenue-neutral.** Total federal revenue unchanged. This paper uses the word only when the arithmetic shows it.
- **Tax-exclusive rate.** VAT as a percentage of the pre-tax price (Canada quotes the GST this way).
- **VAT Revenue Ratio (VRR).** Actual VAT revenue divided by what the standard rate would raise on all final consumption. Canada: 0.50. OECD average: 0.58. New Zealand: 0.96 (2022) [R4, Annex Table 2.A.7].
- **Incidence assumption.** What happens to prices and nominal spending when the VAT rate rises. Every scenario, static included, uses the same assumption, so "dynamic" differs from "static" only by the spending effect.
  - **A (central): real base fixed.** The VAT is passed into prices and households keep buying the same tax-exclusive basket, so revenue on today's base = rate × base. This is how a static costing scales a sales tax.
  - **B (sensitivity): nominal spending fixed.** Prices rise, but nominal spending on today's base does not, so a higher rate buys less: revenue = rate/(1 + rate) × spending.
  - Under both, new spending from the income tax people keep pays VAT-inclusive prices.
- **MPC.** The share of an extra dollar of after-tax income that is spent rather than saved.
- **Taxable share.** The share of new spending that falls in the VAT base: today's effective base divided by household consumption = **0.61**.
- **LVT.** An annual tax on the value of land alone, excluding buildings.

---

## 3. Baseline (Canada, FY2024-25, $B)

Exact values: PIT 234,319, CIT 96,954, net GST 52,503 ($ millions) [R1, Tables 3.2 and 3.3; R2, Table 4].

| Item | $B | Source |
|---|---:|---|
| Personal income tax (PIT) | 234.3 | PA 2025 Vol I Table 3.2 |
| Corporate income tax (CIT) | 97.0 | PA 2025 Vol I Table 3.2 |
| PIT + CIT | 331.3 | sum |
| Non-resident income tax | 13.5 | PA 2025 Vol I Table 3.2 |
| EI premiums (payroll) | 31.5 | AFR 2024-25 Table 4 |
| GST before remission order | 62.3 | PA 2025 Vol I Table 3.3 |
| Less remission order (ministries) | 3.6 | PA 2025 Vol I Table 3.3 |
| Gross GST from outside parties | 58.8 | PA 2025 Vol I Table 3.3 |
| Less quarterly GST credits | 6.3 | PA 2025 Vol I Table 3.3 |
| Net GST from outside parties | 52.5 | PA 2025 Vol I Table 3.3 |
| Total federal revenues | 511.0 | AFR 2024-25 Table 4 |
| Nominal GDP | 3078.0 | total revenue / 16.6% of GDP (AFR 2024-25, derived) |

Effective (net) base = net GST / 5% = 1050.1. Gross base = gross GST / 5% = 1175.4. PIT + CIT = 10.8% of GDP. Core need (PIT + CIT + net GST) = **383.8**; full federal scope (adds non-resident tax and EI premiums) = **428.8**.

Notes:

- The temporary GST holiday (14 Dec 2024 to 15 Feb 2025) fell inside FY2024-25 [R1]. Its cost, $1.3B [R3], is shown as a sensitivity in §5.3.
- The GST/HST credit was renamed the **Canada Groceries and Essentials Benefit (CGEB)** in July 2026, with a 25% increase for 2026-2031 [R13]. The FY2024-25 figures predate both changes.

---

## 4. Proposal: policy specification (single source of truth)

This section is normative. Any app or calculator that implements the policy (for example lion-berry) should match it exactly. Items marked *(v2)* are new in this revision; each one exists to keep the original design arithmetically honest.

1. **Taxes abolished (original design):** PIT (including tax on capital gains), CIT, payroll taxes, capital gains tax, and recurrent property taxes.
   - *(v2)* **Order and trigger.** Abolition is completed when the fiscal board certifies that VAT + LVT revenue covers the revenue need without that tax. Until then, all federal PIT and CIT rates are multiplied by a single factor *k* set each year by the board. At launch, *k* is the smallest value that keeps revenue at the core need. Under central assumptions with a 20% VAT, **k = 47.3%**: federal bracket rates of 7.1 / 9.7 / 12.3 / 13.7 / 15.6% on the 2023 schedule [R3; §5.6]. Under sensitivity B, k = 56.1%. The board sets *k* from observed revenue, not from either assumption.
   - *(v2)* **Property tax.** Recurrent property taxes are levied by municipalities and, in some provinces, by the province. Each is abolished once LVT revenue in that jurisdiction replaces it (§5.4).
2. **One broad VAT** replaces the GST.
   - **Launch rate: 20% at t = 0** (t = years since launch).
   - **Scheduled path:** rate(t) = 20 − 1.2·t for t = 0…10, which gives 20.0, 18.8, 17.6, 16.4, 15.2, 14.0, 12.8, 11.6, 10.4, 9.2, 8.0. This corrects v1.1's `20 − 1.2 × (year − 1)`, which reached only 9.2% in year 10.
   - *(v2)* **Step-down rule.** The rate moves to the next step only if the board certifies that revenue at the lower rate covers the need with income tax at *k* = 0. Abolishing income taxes (reducing *k*) comes before cutting the VAT rate. If a step is not certified, the rate holds.
3. **Differentiated rates (original design).**
   - Zero or reduced rates on unprepared food, medicine, education and children's clothing.
   - Higher rates on luxuries, tobacco, alcohol, fossil fuels and carbon-intensive goods. Revenue from the higher rates funds the reductions.
   - *(v2)* The model treats today's zero-ratings as continuing. Their cost at 20% is $26.5B for groceries alone (§6). Higher-rate schedules are not yet specified, so they are **not modelled**. Children's clothing is taxable today, and the cost of zero-rating it is **unverified**.
4. **Land value tax (original design).**
   - 0.5–1% a year on unimproved land value, aimed mainly at vacant urban and speculative non-farm land.
   - Exemptions or phased rates for active farmland, primary residences below a threshold, and conservation land.
   - *(v2)* LVT revenue first replaces recurrent property taxes. Only the surplus counts toward federal need.
5. **Citizen dividend (original design).**
   - An income-tested annual payment per adult, sized to fully offset the VAT's regressivity for the bottom 40–50% of households.
   - *(v2)* **Floor:** at least the current GST credit/CGEB scaled to the VAT rate: $25.1B at 20%. It is already funded inside the model's revenue figures because the net-GST base is net of the credit.
   - The cost of the full offset is **unverified**: the needed distributional microdata were not available.
6. **Investment commitment (original design).** 60–70% of incremental revenue goes to high-multiplier infrastructure. *(v2)* "Incremental" means revenue above the certified need. Under central assumptions there is none in the early years (§5), so the commitment applies once revenue exceeds need.
7. **Independent fiscal board (original design).** The board monitors performance, certifies abolition steps and rate reductions, and calibrates the differentiated rates and LVT parameters. *(v2)* The board publishes the model inputs it used for each certification.
8. **Entrenchment (original design).**
   - A referendum is required to reintroduce an abolished tax, to raise rates above the schedule, to add a new major tax, or to abolish the LVT.
   - Core amendments need a supermajority, and the board's authority is protected except by supermajority.
   - *(v2)* Holding the rate at a step because certification failed is not a rate increase.
9. **Implementation (original design).** Phased over 10 years, with upfront investment in administration and enforcement, and corporate tax eliminated in phases.

---

## 5. Revenue model

Code: [`analysis/src/model.rs`](analysis/src/model.rs). Full output: [`analysis/output/revenue_tables.md`](analysis/output/revenue_tables.md). Reproduce from a clean clone with one command: `cargo run --release --manifest-path analysis/Cargo.toml`. Every table below is inserted from the model.

### 5.0 First-principles derivation

Revenue is built up from measured quantities, not borrowed from other countries' results:

- VAT revenue = consumption base × coverage-and-compliance ratio × rate.
- LVT revenue = land value × taxable share × rate.

The coverage ratio is *measured* from what Canada's GST actually collected. Analogy (New Zealand, the OECD average) appears only as a scenario.

| Step | Quantity | Value | From |
|---:|---|---:|---|
| 1 | GDP, $B | 3078.0 | total revenue 511.0 / revenue share of GDP 16.6% (AFR 2024-25) |
| 2 | Household final consumption (HFCE), $B | 1711.4 | 55.6% x GDP (World Bank WDI 4.8) |
| 3 | Effective taxed base, $B | 1050.1 | net GST 52.5 / statutory rate 5% (PA 2025, Table 3.3) |
| 4 | Coverage-and-compliance ratio | 0.614 | step 3 / step 2 (measured; OECD VRR for Canada, on a wider denominator: 0.50) |
| 5 | Static VAT revenue at 20%, $B | 210.0 | step 2 x step 4 x 20% (prices rise by the tax; same real basket) |
| 6 | Income tax kept by households and firms, $B | 331.3 | PIT 234.3 + CIT 97.0 (PA 2025, Table 3.2) |
| 7 | New taxable spending, $B | 193.1 | MPC 0.95 (1 - saving rate 5.0%, StatCan) x step 4 x step 6 |
| 8 | Extra real base at 20%, $B | 160.9 | step 7 / 1.20 (new spending pays the VAT-inclusive price) |
| 9 | Dynamic VAT revenue at 20%, $B | 242.2 | 20% x (step 3 + step 8) |
| 10 | Core need, $B | 383.8 | PIT + CIT + net GST |
| 11 | Rate that meets the core need | 32.1% | solves rate x (step 3 + step 7 / (1 + rate)) = step 10 |
| 12 | Share of income tax still needed at 20% (k) | 47.3% | solves VAT(20%, 1-k) + k x step 6 = step 10 |
| 13 | LVT scenario, $B | 32.0 | land 6400.0 (unverified) x 50% taxable x 1% |
| 14 | Recurrent property tax to replace first, $B | 92.6 | $2,242 per person x 41,288,599 people (StatCan) |

### 5.1 Static: today's base, no spending effect

| VAT rate | Revenue (A) | Revenue (B) | Net new vs today's GST (A) | Share of PIT+CIT replaced (A) | Gap to core need (A) |
|---:|---:|---:|---:|---:|---:|
| 5% | 52.5 | 52.5 | 0.0 | 0.0% | 331.3 |
| 8% | 84.0 | 81.7 | 31.5 | 9.5% | 299.8 |
| 10% | 105.0 | 100.2 | 52.5 | 15.8% | 278.8 |
| 15% | 157.5 | 143.8 | 105.0 | 31.7% | 226.3 |
| 20% | 210.0 | 183.8 | 157.5 | 47.5% | 173.8 |
| 25% | 262.5 | 220.5 | 210.0 | 63.4% | 121.3 |

The static rate for the core need is **36.5%** under A and 53.4% under B; for the full federal scope it is 40.8% (A).

### 5.2 The thesis: households keep the money and spend it

All $331.3B of PIT and CIT stays with households and firms. They spend a share, the MPC. Of that spending, a taxable share lands in the VAT base and pays VAT-inclusive prices. Today's base is treated exactly as in the static case (A or B); the only difference from static is the new spending.

- **MPC evidence.** Canada's household saving rate was 7.2% in Q2 2024 and 5.0% in Q2 2025 [R9, R10], so the average propensity to consume was 0.928–0.950. A one-off windfall is usually spent at a lower rate than average income, so 0.95 is a generous upper bound.
- **Taxable share.** 0.61: the effective base divided by household consumption (55.6% of GDP [R11]).
- **Imports.** Imports are 32.7% of Canada's GDP [R11]. They do **not** shrink a destination-based VAT base, because imports are taxed at the border. They matter only for how much domestic income the new spending generates, and Finance Canada's multipliers already include saving and import leakages [R8].
- **Optional multiplier.** Short-run multipliers of 1.0 for PIT measures and 0.3 for CIT measures [R8, Table A1.1]. Finance Canada also notes that CIT measures have among the highest multipliers in the long run, which this short-run figure does not capture.

All 331.3 of PIT + CIT stays with households and firms. A share (MPC) is spent; a share of that spending (taxable share = 0.61) lands in the VAT base and pays rate-inclusive prices. Imports (32.7% of GDP) do not leak from a destination-based base; they matter only for the multiplier. "+ mult" adds Finance Canada's short-run multipliers (PIT 1.0, CIT 0.3). MPC 0.000 is the static case.

| MPC | Rev. at 20% (A) | Rate for core need (A) | Rev., share = 1 (A) | Rate (A) | Rev., + mult (A) | Rate (A) | Rev. at 20% (B) | Rate for core need (B) |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0.000 | 210.0 | 36.5% | 210.0 | 36.5% | 225.7 | 34.3% | 183.8 | 53.4% |
| 0.500 | 227.0 | 34.1% | 237.6 | 32.7% | 242.7 | 32.0% | 200.7 | 46.8% |
| 0.600 | 230.3 | 33.6% | 243.1 | 32.0% | 246.1 | 31.6% | 204.1 | 45.6% |
| 0.700 | 233.7 | 33.2% | 248.7 | 31.3% | 249.5 | 31.2% | 207.5 | 44.6% |
| 0.800 | 237.1 | 32.7% | 254.2 | 30.6% | 252.8 | 30.8% | 210.9 | 43.5% |
| 0.900 | 240.5 | 32.3% | 259.7 | 30.0% | 256.2 | 30.4% | 214.2 | 42.6% |
| 0.928 | 241.4 | 32.2% | 261.2 | 29.8% | 257.2 | 30.3% | 215.2 | 42.3% |
| 0.950 | 242.2 | 32.1% | 262.5 | 29.7% | 257.9 | 30.2% | 215.9 | 42.1% |

MPC rows 0.928 and 0.950 are 1 minus the household saving rate in Q2 2024 (7.2%) and Q2 2025 (5.0%); an average propensity is an upper bound for spending out of a tax cut. Real taxable consumption at 20%, central MPC: 1211.0 (A), 1079.7 (B), versus 1050.1 today.

Every scenario under both assumptions:

Within each assumption, higher revenue at a given rate always means a lower or equal required rate (tested). Comparisons across A and B are not like-for-like: B's revenue curve flattens as the rate rises.

| Scenario | Rev. at 20% (A) | Rate for core need (A) | Rev. at 20% (B) | Rate for core need (B) |
|---|---:|---:|---:|---:|
| Static (no spending effect) | 210.0 | 36.5% | 183.8 | 53.4% |
| Dynamic, MPC 0.50 | 227.0 | 34.1% | 200.7 | 46.8% |
| Dynamic, central (MPC 0.95) | 242.2 | 32.1% | 215.9 | 42.1% |
| Dynamic, every new dollar taxable | 262.5 | 29.7% | 236.2 | 37.1% |
| Dynamic + multiplier | 257.9 | 30.2% | 231.7 | 38.1% |
| Static, 20% compliance leakage | 178.5 | 44.4% | 157.5 | 73.4% |
| Static, all exemptions and rebates taxed | 305.4 | 25.1% | 267.2 | 31.5% |
| Static, New Zealand VRR | 403.2 | 19.0% | 352.8 | 22.1% |

**What this shows.**

- **Consistency.** Under one assumption, more revenue at a given rate always means a lower required rate; the test suite checks this across every scenario, both assumptions and several rates. An earlier draft of v2.0 computed static under A and dynamic under B, which produced an apparent contradiction; it is fixed.
- **The thesis is right about direction.** At the central MPC, real taxable consumption at 20% is $1,211.0B under A and $1,079.7B under B, against $1,050.1B today.
- **Break-even.** The MPC would need to be **5.13** (A), **3.15** even if every new dollar were taxable, or 5.90 under B; an MPC cannot exceed 1. What can close the gap is a **larger taxable base**: **1.58×** today's relative to spending needs, 4.7% a year faster than needs for 10 years. With the multiplier it is 1.49× (4.1% a year); under B it is 1.78× (5.9% a year).
- The original paper's growth addition of +1.5 to +2.5 points a year is **unsourced** and is used only as a what-if: it gives 1.16× to 1.28× over 10 years.

| Condition | At 20% | At 8% |
|---|---:|---:|
| MPC needed, central share (A) (max possible 1.0) | 5.13 | 19.91 |
| MPC needed, every new dollar taxable (A) (max possible 1.0) | 3.15 | 12.22 |
| MPC needed, central share (B) (max possible 1.0) | 5.90 | 20.06 |
| Base growth beyond needs, central (A): cumulative | x1.58 | x3.90 |
| ... same, per year over 10 years | 4.7% | 14.6% |
| Base growth beyond needs, central + multiplier (A): cumulative | x1.49 | x3.64 |
| ... same, per year over 10 years | 4.1% | 13.8% |
| Base growth beyond needs, central (B): cumulative | x1.78 | x4.00 |
| ... same, per year over 10 years | 5.9% | 14.9% |
| What-if growth addition 1.5 pp/yr (unsourced) gives, after 10 years | x1.16 | x1.16 |
| What-if growth addition 2.5 pp/yr (unsourced) gives, after 10 years | x1.28 | x1.28 |

### 5.3 Base broadening and compliance

Tax-expenditure costs are calendar 2024 values at the 5% rate (Report on Federal Tax Expenditures 2026, Part 2). Finance Canada notes that removing a measure would not yield its full cost.

| Base | Extra base, $B at 5% | Core need (A) | Core need (B) | Full scope (A) |
|---|---:|---:|---:|---:|
| Today's base (FY2024-25) | 0.0 | 36.5% | 53.4% | 40.8% |
| Add back the Dec 2024-Feb 2025 GST holiday | 1.3 | 35.7% | 51.5% | 39.9% |
| Tax basic groceries | 6.6 | 32.5% | 44.7% | 36.3% |
| Tax all 11 household zero-ratings/exemptions | 16.6 | 27.7% | 35.9% | 31.0% |
| Households + public-body rebates + small-supplier threshold | 23.8 | 25.1% | 31.5% | 28.1% |
| OECD-average VAT revenue ratio (0.58) | n/a | 31.5% | 42.9% | 35.2% |
| New Zealand VAT revenue ratio (0.96) | n/a | 19.0% | 22.1% | 21.3% |

Taxing groceries conflicts with the design's own zero rate on food; the row only sizes the trade-off.

Leakage = share of revenue above today's net GST lost to evasion or avoidance at higher rates. No Canadian estimate is used; these are what-ifs.

| Leakage | Revenue at 20% (A) | Rate for core need (A) | Revenue at 20% (B) | Rate for core need (B) |
|---:|---:|---:|---:|---:|
| 0% | 210.0 | 36.5% | 183.8 | 53.4% |
| 10% | 194.3 | 40.1% | 170.6 | 61.7% |
| 20% | 178.5 | 44.4% | 157.5 | 73.4% |
| 30% | 162.8 | 50.1% | 144.4 | 91.1% |

### 5.4 Land value tax and the property tax it replaces

Taxable (unimproved) land value needed for the original $50.0-100.0B a year ($B of land):

| LVT rate | For $50.0B | For $100.0B |
|---:|---:|---:|
| 0.5% | 10,000.0 | 20,000.0 |
| 1% | 5,000.0 | 10,000.0 |

For scale (verified): national wealth, all non-financial assets, Q2 2025 = $17,267.4B; household residential real estate, land plus buildings, Q2 2025 = $8,375.9B. Revenue ($B a year) if total land value is $6,400.0B (**unverified**) and only part of it is taxable:

| Taxable share of land | At 0.5% | At 1% |
|---:|---:|---:|
| 10% | 3.2 | 6.4 |
| 25% | 8.0 | 16.0 |
| 50% | 16.0 | 32.0 |
| 100% | 32.0 | 64.0 |

The original $50–100B a year needs $5–20 trillion of taxable land. Total land value of about $6.4T is **unverified** (a secondary source citing StatCan Table 36-10-0580-01, which we could not download).

**Why abolish property tax at all?** The OECD ranks recurrent property tax as the *least* growth-harmful tax [R7], so abolishing it moves against that evidence. The original design's reason (ab193f4) is that "the land value tax captures rents from scarcity without discouraging improvements". A recurrent property tax has two parts: a tax on land value and a tax on buildings. The Mirrlees Review quotes William Vickrey: the property tax is "a combination of one of the worst taxes—the part that is assessed on real estate improvements …—and one of the best taxes—the tax on land or site value"; land "is not a produced input; its supply is fixed" [R12, ch. 16]. The design keeps the land part (the LVT) and drops the buildings part, so construction and upkeep are no longer taxed.

**The counter-argument, stated fairly.** R7's ranking is about recurrent property taxes as they exist, buildings included, and those taxes are hard to avoid because property cannot move. Mirrlees argues for taxing land *and* the consumption value of housing, not land alone [R12, ch. 16]. Dropping the buildings part gives up a low-distortion revenue source, and the LVT has to raise more from land alone to make up for it.

**The cost of that choice.** The LVT must first replace today's recurrent property taxes:

| Quantity | Value | From |
|---|---:|---|
| Property tax per person, provincial + local, 2024 | $2,,242 | StatCan CGFS Table 10-10-0147-01, via Nova Scotia Finance |
| Population, 1 July 2024 | 41,288,599 | StatCan Daily 2024-09-25 |
| Recurrent property tax revenue, $B | 92.6 | per person x population (derived) |
| LVT scenario revenue, $B | 32.0 | 1% on 50% of land (land value unverified) |
| LVT rate needed to replace property tax, scenario land | 2.9% | property tax / (50% of land) |
| LVT rate needed to replace property tax, all land | 1.4% | property tax / all land |

At the scenario's taxable land, the LVT would need a rate of 2.9%, well above the design's 1%, just to replace property tax. Until LVT revenue covers property tax in a jurisdiction, the property tax stays (§4).

### 5.5 The rate path and the conditions for 8%

rate(t) = 20 - 1.2 t, t = years since launch (0..10). The v1.1 formula `20 - 1.2 x (year - 1)` for years 1-10 reaches 9.2% in year 10. Coverage = VAT revenue / core need, all income tax abolished, dynamic central. Growth columns are what-ifs (unsourced): the base grows that much faster than the revenue requirement every year.

| t | Rate | Revenue, no growth (A) | Coverage (A) | Coverage, +1.5 pp/yr (A) | Coverage, +2.5 pp/yr (A) | Coverage (B) |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 20.0% | 242.2 | 63.1% | 63.1% | 63.1% | 56.3% |
| 1 | 18.8% | 228.0 | 59.4% | 60.3% | 60.9% | 53.4% |
| 2 | 17.6% | 213.7 | 55.7% | 57.4% | 58.5% | 50.5% |
| 3 | 16.4% | 199.4 | 52.0% | 54.3% | 56.0% | 47.6% |
| 4 | 15.2% | 185.1 | 48.2% | 51.2% | 53.2% | 44.5% |
| 5 | 14.0% | 170.7 | 44.5% | 47.9% | 50.3% | 41.5% |
| 6 | 12.8% | 156.3 | 40.7% | 44.5% | 47.2% | 38.3% |
| 7 | 11.6% | 141.9 | 37.0% | 41.0% | 43.9% | 35.1% |
| 8 | 10.4% | 127.4 | 33.2% | 37.4% | 40.4% | 31.8% |
| 9 | 9.2% | 112.9 | 29.4% | 33.6% | 36.7% | 28.4% |
| 10 | 8.0% | 98.3 | 25.6% | 29.7% | 32.8% | 25.0% |

Each row stacks one more lever on the dynamic central case. The LVT row assumes none of the LVT is needed to replace property taxes (optimistic; see Table 8b).

| Stack | Revenue at 8%, no growth (A) | LVT | Base growth beyond needs over 10 yrs (A) | Per year (A) | Over 10 yrs (B) | Per year (B) |
|---|---:|---:|---:|---:|---:|---:|
| Dynamic central (her mechanism) | 98.3 | 0.0 | x3.90 | 14.6% | x4.00 | 14.9% |
| + Finance Canada multiplier | 105.3 | 0.0 | x3.64 | 13.8% | x3.73 | 14.1% |
| + LVT | 105.3 | 32.0 | x3.34 | 12.8% | x3.42 | 13.1% |
| + tax every exemption and rebate in Table 3 | 153.1 | 32.0 | x2.30 | 8.7% | x2.35 | 8.9% |
| + New Zealand-level base and compliance instead | 202.2 | 32.0 | x1.74 | 5.7% | x1.78 | 5.9% |

### 5.6 Income tax that must remain while the VAT phases in

Dynamic central case, no growth, no LVT. k = share of today's PIT + CIT still needed; the removed share (1 - k) drives the extra spending. Bracket rates scale the 2023 federal schedule (15/20.5/26/29/33%) by k.

| VAT rate | VAT revenue (A) | k (A) | Kept, $B (A) | Federal bracket rates (A) | k (B) |
|---:|---:|---:|---:|---|---:|
| 10% | 107.9 | 83.3% | 275.8 | 12.5 / 17.1 / 21.6 / 24.1 / 27.5 | 84.8% |
| 15% | 166.1 | 65.7% | 217.6 | 9.9 / 13.5 / 17.1 / 19.1 / 21.7 | 70.2% |
| 20% | 227.0 | 47.3% | 156.8 | 7.1 / 9.7 / 12.3 / 13.7 / 15.6 | 56.1% |
| 25% | 290.2 | 28.2% | 93.5 | 4.2 / 5.8 / 7.3 / 8.2 / 9.3 | 42.6% |
| 30% | 355.8 | 8.4% | 28.0 | 1.3 / 1.7 / 2.2 / 2.4 / 2.8 | 29.6% |

**Conclusion of the model.** Under FY2024-25 Canadian numbers:

- 20% at launch supports partial replacement, with *k* ≈ 47% (A) to 56% (B).
- Full abolition at 20% needs a VAT that performs like New Zealand's (static rate needed with a New Zealand-level base: 19.0% under A, 22.1% under B).
- 8% needs that plus a base 3.90× today's relative to needs: 14.6% a year faster than needs for 10 years, faster than even the high end of the original paper's unsourced growth addition (+1.5 to +2.5 points).
- Property tax cannot be abolished on the scenario LVT alone.

The fiscal board's certification rule keeps the original goal (abolition, then 8%) as a target that can be met, rather than a promise that cannot.

---

## 6. Distributional analysis

- **Who bears a VAT.** Measured as a share of *income*, VATs are regressive. Measured as a share of *spending*, they are roughly proportional or slightly progressive [R6]. A 2020 OECD reassessment reaches the same conclusion [R6b].
- **Reduced rates are a poor targeting tool.** On aggregate, richer households get as much benefit from reduced rates as poorer ones, or more [R6]. Mirrlees et al. recommend removing zero and reduced rates and compensating through direct transfers [R12, ch. 9].
- The original design used **both** reduced rates and a dividend. v2.0 keeps both, and adds that the dividend is the stronger tool per dollar.
- **Canada already does this.** The GST credit was introduced with the GST in 1991 [R14b] and cost $6.3B in FY2024-25 [R1]. Now called the CGEB, it was increased 25% from July 2026 [R13].
- **Who gains from abolishing income tax.** Federal PIT is progressive (bracket rates 15–33% in 2023 [R3]), so the dollar gains from cutting it go mostly to higher earners. The shares by decile are **unverified** here. This is why the dividend has to be in place **at launch**.

Static scaling (A) of two existing measures: the GST credit (PA 2025 Table 3.3) and zero-rating of basic groceries (Tax Expenditures 2026).

| VAT rate | GST-credit equivalent | Cost of zero-rating groceries |
|---:|---:|---:|
| 5% | 6.3 | 6.6 |
| 10% | 12.5 | 13.2 |
| 15% | 18.8 | 19.9 |
| 20% | 25.1 | 26.5 |
| 25% | 31.3 | 33.1 |

---

## 7. Counter-arguments, answered with numbers

1. **"20% raises only about $210.0B, not $331.3B" (static).** Correct. With the thesis modelled at central values, 20% raises $242.2B, or $257.9B with the multiplier. That still falls about $126–142B short of the core need, so v2.0 makes abolition conditional (§4) instead of claiming it.
2. **"People will save the windfall."** Even at MPC = 0.50, revenue at 20% is $227.0B, against $242.2B at 0.95. The MPC moves revenue by about $15B, which is small next to the gap.
3. **"Imports leak."** Imports are 32.7% of GDP, but a destination-based VAT taxes them at the border, so they do not reduce the base. They reduce only the domestic multiplier, and the Finance Canada multipliers already account for that.
4. **"High rates invite evasion."** At 20% compliance leakage, the required rate rises from 36.5% to 44.4% (§5.3). This is why the design puts upfront investment in enforcement and administration first.
5. **"It is regressive."** The credit floor costs $25.1B at 20%; zero-rating groceries costs $26.5B. Evidence favours the transfer [R6, R12].
6. **"Retirees pay twice"** (taxed on income while working, then on spending in retirement). This is valid for spending out of savings accumulated before the switch. It is **not quantified** here. A phase-in and the dividend reduce the problem but do not remove it.
7. **"Property tax is the least harmful tax; why abolish it?"** [R7]. Answered in §5.4: the LVT keeps the land part and drops the buildings part, but it must first raise about $92.6B, so property tax stays until it does.
8. **The strongest counter-argument: the base is too narrow for the rate to be credible.** Canada's effective VAT base is about a third of GDP ($1,050.1B of $3,078B). The taxes to be replaced are worth 10.8% of GDP. So the rate has to be about 37% statically, or about 32% counting the spending effect (42% under assumption B). The highest standard VAT rate in the OECD is Hungary's 27% [R4, Annex Table 2.A.7]. *Answer:* the design does not need 32% if the base becomes as broad and well-collected as New Zealand's (VRR 0.96, which needs 19.0%). Until then, the board keeps part of the income tax. The counter-argument limits the *speed* of reform, not its direction.

---

## 8. Prior work and precedents

### 8.1 Earlier proposals to replace income tax with consumption tax

| Proposal | What it proposed | How this design differs |
|---|---|---|
| Meade Report (UK, 1978) [R17] | A review of the whole UK tax system that favoured an expenditure-tax base, setting out universal (ch. 9) and two-tier (ch. 10) variants: tax what people spend rather than what they earn. | Meade kept a progressive personal tax on spending. This design uses a VAT collected from businesses, and gets progressivity from the dividend rather than from rates. |
| Hall-Rabushka flat tax (1985; 2nd ed. 1995) [R18] | One low flat rate replacing the federal income tax, taxing all income once, either at the business or at the worker, on a consumption-type base [R18, R19]. | Hall-Rabushka keeps a tax return for workers. This design removes personal returns and relies on a VAT plus transfers. |
| Bradford X-tax (1986; 2004) [R19] | A consumption-type tax built like Hall-Rabushka, designed to replace the income tax on corporations and individuals while keeping "substantial scope for progressivity" at the worker level. | The X-tax keeps progressivity inside the tax. This design moves it into the dividend. |
| US FairTax, H.R. 25 (119th Congress) [R20] | A national retail sales tax at 23% of gross (tax-inclusive) payments in 2027, replacing federal income, payroll, estate and gift taxes, with a monthly rebate (the "family consumption allowance"). | Closest in spirit (replace income and payroll taxes with a consumption tax plus a rebate). This design uses a multi-stage VAT rather than a retail sales tax, adds an LVT and a fiscal board, and its model states the rate needed instead of assuming one. |
| Mirrlees Review, *Tax by Design* (2011) [R12] | Keep income tax but exempt the normal return to saving; broaden the VAT base and compensate through benefits. | Mirrlees improves the income tax; this design removes it. Both agree on a broad VAT base with compensation through transfers. |
| Carter Commission (Canada, 1966) [R21] | The opposite direction: a comprehensive *income* base, taxing every increase in economic power alike ("a buck is a buck"). | Carter's goal was fairness through a broad income base. This design seeks it through a broad spending base plus a dividend. Carter is the strongest Canadian argument for keeping income tax. |

**What is new here.** The design combines four things: abolishing income *and* property taxes; a falling VAT path; an LVT that keeps the land part of property tax; and a fiscal board that has to certify each step. It also publishes a model that states the rate needed instead of assuming it.

### 8.2 Precedents

- **New Zealand (GST, 1986).** The rate rose from 12.5% to 15% on 1 October 2010, explicitly as "a switch in the tax mix from income tax to consumption tax" [R22]. The base is very broad and includes public services [R4]. VRR 0.96, the highest in the OECD; VAT is 10.0% of GDP and 30.3% of all taxes (2022) [R4, Annex 1.A.4]. New Zealand **kept** its income taxes: it shows how broad a base can be, not that income tax can be fully replaced.
- **Denmark and Hungary.** Standard rates of 25% and 27% (2022) [R4, Annex 2.A.7]. VAT is 9.2% of GDP / 22.0% of taxes in Denmark and 10.1% / 28.9% in Hungary [R4, Annex 1.A.4]. Both still levy income taxes. Denmark's land tax is **unverified** here.
- **Gulf states.** Saudi Arabia introduced VAT at 5% on 1 January 2018 [R15] and announced on 11 May 2020 that it would rise to 15% from July 2020 [R15b]: a large, fast rate change. Statements about personal income tax and about UAE, Bahrain and Oman VAT are **unverified** here.
- **Canada 1991.** The 7% GST replaced the 13.5% federal sales tax (FST), in place since 1924, on 1 January 1991, together with the GST credit [R14, R14b]. It is now 5%.
- **Singapore.** v1.1 described it as having "minimal income taxes". That is incorrect: IRAS gives the current highest personal income tax rate as 24% and the current GST rate as 9% [R16].
- **Breadth of VAT.** 175 countries and territories had a VAT as of 1 July 2024 [R4].
- Land value taxation in parts of Australia is **unverified** here. The Pennsylvania cities example (added in v1.1, not part of the original design) has been removed.

---

## 9. Vision vs feasible: requirements questioned

Five steps: question each requirement and name who set it; delete what adds nothing; simplify; make the model fast to rerun; automate the checks last. "Ashlynn (ab193f4)" is the original design (January 2026); "Bot" rows are automated edits from August 2026; "Audit" is the October 2026 audits.

| Requirement | From | Question asked | Decision |
|---|---|---|---|
| Abolish PIT, CIT, payroll, capital gains and property tax | Ashlynn (ab193f4) | Does a 20% VAT pay for it? | **Kept as the goal, made conditional.** k = 47.3% of income tax remains at launch; full abolition needs a base about 1.58× larger |
| Abolish recurrent property tax | Ashlynn (ab193f4) | The OECD ranks it least harmful [R7]: why abolish? | **Kept, reason stated** (§5.4): the LVT keeps the land part. Property tax stays until the LVT replaces about $92.6B |
| VAT 20% → 8% over 10 years | Ashlynn (ab193f4) | Is the path consistent, and is 8% reachable? | **Kept.** Formula fixed; steps need certification; 8% needs about 3.90× the base |
| Thesis: kept income is spent, so VAT revenue grows | Ashlynn (ab193f4; steering 2026-10-18) | How large is the effect? | **Central and modelled.** It adds about $32–48B at 20% (A); the MPC needed to close the gap alone is 5.13 |
| LVT, $50–100B | Ashlynn (ab193f4) | How much land does that need? | **Kept; amount flagged.** It needs $5–20T of taxable land; scenario $32–64B |
| Dividend fully offsets the bottom 40–50% | Ashlynn (ab193f4) | What does it cost? | **Kept.** Floor $25.1B at 20%; full cost **unverified** |
| Fiscal board; referendum and supermajority | Ashlynn (ab193f4) | Does it conflict with a conditional path? | **Kept.** Certification is the mechanism that makes the path conditional |
| 60–70% of incremental revenue to infrastructure | Ashlynn (ab193f4) | Is there incremental revenue early on? | **Kept, re-timed.** It applies once revenue exceeds need |
| "Neutral within a decade" | Ashlynn (ab193f4) | Does the arithmetic support it? | **Deleted as a claim.** Replaced by the certification rule |
| Velocity, investment multipliers, base elasticity | Ashlynn (ab193f4) | Is there a source? | **Deleted.** None found; replaced by the MPC sweep and Finance Canada multipliers (0.3–1.0) |
| Growth +1.5–2.5 pp a year | Ashlynn (ab193f4) | Is there a source? | **What-if only, unverified.** It gives 1.16–1.28× over 10 years |
| "Over 170 countries"; "Singapore: minimal income tax" | Ashlynn (ab193f4) | Still true? | **Corrected** to 175 [R4]; Singapore's top PIT rate is 24% [R16] |
| "Primary jurisdiction: Canada" | Bot (d7a99d9) | Is that the original scope? | **Replaced.** Scope is mid-sized advanced economies; Canada is the worked example |
| "Illustrative, not a CGE run" table; `20 − 1.2(year − 1)` | Bot (d7a99d9) | Do the numbers come from anything? | **Deleted.** Replaced by model output |
| "Dividend before the VAT bite" | Bot (d7a99d9) | Is it needed? | **Kept.** The distributional evidence supports paying it at launch (§6) |
| Pennsylvania cities as an LVT precedent | Bot (d7a99d9) | Is there a source? | **Deleted** |
| "Not viable as written"; credit-plus-split-rate counter-proposal | Bot (07e4fb1/3ecc821) | Does the model support replacing the design? | **Deleted.** Enlarging the credit survives as the dividend floor |
| Static scaling as the only test | Audit | Does it ignore behaviour? | **Kept as the floor**, with the dynamic model next to it |
| Static computed under one price assumption, dynamic under another | v2 draft (caught in review) | Comparable? | **Fixed.** One assumption per scenario; monotonicity tested |
| Paper numbers checked only for presence somewhere in the output | v2 draft (caught in audit) | Does the test prove each number is in the right place? | **Fixed.** Paper, README and CLAIMS are generated from templates and compared byte for byte; literals need a sourced allowlist entry |

---

## 10. Debt-neutral transition

Ashlynn's requirement for v3.0: the transition must not add to federal debt. This section states a rule that keeps the deficit at or below the baseline in every year, and tests how far it gets.

### 10.1 The rule

Each year the fiscal board compares realised federal revenue with the core need ($383.8B, today's PIT + CIT + GST). Realised revenue is the VAT, plus the income tax still in force (share k), plus any growth in the land and property base.

1. **Hold the rate.** The VAT stays at 20% until k reaches zero. The step down to 8% (§5.5) waits until then: on today's base, a VAT of 8% would need k = 90.1% under central values, so the step down cannot be debt-neutral while income tax remains.
2. **Cut only from realised surplus.** k falls by realised revenue above the need, after any balance owed in a compensation account is repaid, and never by more than the structural surplus. Structural revenue leaves out temporary demand from the investment build-out, as the Swiss rule leaves out the cycle.
3. **Pause.** If realised revenue is below the need, the shortfall is booked in the compensation account and k is not cut.
4. **Reverse.** If structural revenue is below the need, k rises by the shortfall. While the account is owed, k is raised enough to repay it over 3 years.
5. **Property and land.** The federal government levies no property tax, so this term is nil unless the LVT (§5.4) is adopted. LVT revenue then counts under the same test.

Without a shock, the deficit change against the baseline is zero or negative in every simulated year of every case (`transition.rs` test `deficit_never_rises_without_a_shock`). No rule can prevent a deficit in a year when the base falls unexpectedly. The rule's answer is the account and the reversal (§12.1).

### 10.2 Precedents for the rule

| Country | Rule | Relevance | Source |
|---|---|---|---|
| Canada, Program Review | Spending under review fell 18.9% relative to 1994-95; it covered about $52B of spending, excluding major statutory transfers. Interim deficit target 3% of GDP ($24.3B), pursued mainly through $25.3B of cumulative spending reductions: "nearly seven dollars of spending reduction for every dollar of tax revenue increase" | Spending review at this scale has been done in Canada | Budget 1995 [R23] |
| Switzerland, debt brake | Ordinary spending is capped at receipts adjusted by the ratio of trend GDP to current GDP; shortfalls go to a compensation account that must be cleared in later years; a tax cut must be matched by spending cuts. Approved by 85% on 2 December 2001; debt ratio from 25.3% (2003) to 13.5% (2019) | Source of the structural test, the account and the matching rule | Federal Finance Administration [R24] |
| Germany, Schuldenbremse | Structural borrowing up to 0.35% of GDP; a control account whose negative balance may not exceed 1.5% of GDP | Source of the control-account trigger | Article 115 Act [R25] |
| Sweden, fiscal framework | Surplus target over the business cycle and a debt anchor of 35% of GDP; the framework dates from the crisis of the early 1990s | A surplus rule can last decades | Government Offices of Sweden [R26] |
| New Zealand, GST | GST at a single rate of 10% from 1 October 1986; personal income tax rates cut on the same date; new top rate 48c | A broad VAT replaced part of income tax in one step | Inland Revenue [R27, R28] |

### 10.3 Result

| Case | Spending effect | Capital per point of METR cut | METR cut, points | Speed per year | k at launch | k after 10 years | k settles at | Capital stock, long run | Years to k = 0 (within 100) | Gap left with k = 0, $B |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|
| Low | MPC 0.50 | 0.1% | 5 | 0.1 | 49.9% | 49.8% | 49.8% | +0.3% | never | 156.4 |
| Central | MPC 0.95 | 0.2% | 8 | 0.2 | 47.3% | 46.9% | 46.9% | +0.8% | never | 140.0 |
| High | MPC 0.95 + multipliers | 0.4% | 13.2 | 0.3 | 44.4% | 42.8% | 42.7% | +3.0% | never | 120.2 |

The rule keeps the transition debt-free, but the spin-up does not close the gap in any case. Within 100 years, k settles at 49.8% (low), 46.9% (central) and 42.7% (high), against 49.9%, 47.3% and 44.4% at launch. Years to full abolition within that horizon: never, never and never. With k at zero, the gap left is $156.4B, $140.0B and $120.2B a year.

Central case, year by year:

| Year | k | Capital stock vs launch | Build-out investment, $B | Revenue, $B | Structural revenue, $B | Deficit change vs baseline, $B |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 47.34% | +0.00% | 0.0 | 383.8 | 383.8 | 0.0 |
| 1 | 47.34% | +0.17% | 15.0 | 385.6 | 384.0 | -1.8 |
| 2 | 47.26% | +0.30% | 12.0 | 385.3 | 384.0 | -1.5 |
| 3 | 47.18% | +0.41% | 9.6 | 385.0 | 384.0 | -1.2 |
| 5 | 47.08% | +0.57% | 6.2 | 384.5 | 383.9 | -0.8 |
| 10 | 46.95% | +0.76% | 2.1 | 384.0 | 383.8 | -0.3 |
| 20 | 46.89% | +0.84% | 0.2 | 383.8 | 383.8 | 0.0 |
| 50 | 46.88% | +0.85% | 0.0 | 383.8 | 383.8 | 0.0 |

---

## 11. Industry spin-up scenario

This is a scenario, not a forecast. It asks how much new investment lower income tax could bring, and how much of the gap that investment's tax base would close.

### 11.1 Mechanism and inputs

- **Cost of capital.** Canada's marginal effective tax rate (METR) on new business investment is 15.6%, or 13.2% with the Budget 2025 super-deduction [R29]. The high case removes all of the latter. The low and central cases remove less, because part of the METR is provincial; the federal share is not in the sources used here.
- **Response.** The capital stock rises by 0.2% per point of METR cut in the central case, less in the low case and more in the high case (table below). This semi-elasticity is **unverified**: it is the figure reported for an IMF working paper on Canadian capital stocks [R30], whose text could not be opened. Capital moves toward its new level at a set speed each year.
- **Base.** A larger capital stock raises output in proportion to the capital share of GDP, 0.418 (gross operating surplus over GDP per dollar of final demand), and the VAT base and remaining income tax rise with it. The capital stock proxy is national wealth less residential real estate, $8.9T; it includes land and government assets, so it overstates business capital and makes the spin-up larger.
- **Build-out.** Investment during the build-out is demand: 0.833 of GDP and 6.199 jobs per $1M of final demand, from Statistics Canada's input-output multipliers, Table 36-10-0013-01 [R31]. These values are **unverified**: the Statistics Canada site was down on the access date, and they were read from a search index's copy of the table. The capital share above comes from the same table. This is temporary, so the rule does not count it as structural. In the central case it peaks at $15.0B of investment in the first year, supporting about 93k jobs.

| Input | Value | Source |
|---|---:|---|
| Output per $ of final demand | 1.560 | StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy) |
| GDP at market prices per $ of final demand | 0.833 | StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy) |
| Gross operating surplus per $ of final demand | 0.348 | StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy) |
| International imports per $ of final demand | 0.167 | StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy) |
| Jobs per $1M of final demand | 6.199 | StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy) |
| Capital share of GDP (surplus / GDP) | 0.418 | derived from the two rows above (**unverified**) |
| Capital proxy (national wealth less residential real estate), $T | 8.9 | StatCan national balance sheet (derived) |

| Case | Peak build-out investment, $B/yr | GDP from it, $B | Imports, $B | Jobs supported (thousands) | Long-run base gain with k = 0 | Revenue gain with k = 0, $B/yr | Share of the cost of abolition it covers |
|---|---:|---:|---:|---:|---:|---:|---:|
| Low | 2.2 | 1.9 | 0.4 | 14 | +0.21% | 0.5 | 0.3% |
| Central | 15.0 | 12.5 | 2.5 | 93 | +0.67% | 1.6 | 1.1% |
| High | 78.3 | 65.2 | 13.1 | 485 | +2.21% | 5.7 | 4.5% |

### 11.2 Precedents

- **Ireland.** Low corporate tax drew large booked profits. Ireland's statistics office publishes modified GNI (GNI*) because globalisation effects distort GDP; GNI* was 57.1% of GDP [R32]. Under this design no CIT is levied, so booked profits would raise measured GDP but add no federal revenue beyond the VAT on domestic spending. The spin-up above counts only domestic capital.
- **Estonia.** Estonia taxes company profits only when distributed, at 22/78 of the net distribution [R33]. It is a middle option: it removes the tax on retained, reinvested profit while keeping a tax on payouts. It is not modelled; its revenue for Canada is **unverified**.

### 11.3 Levers that would close the gap

| Case | Gap, $B | VAT rate that closes it alone | Tax household GST exemptions at the rule rate, $B | Program-Review-sized spending cut, $B | LVT scenario, $B (land value unverified) | VAT rate needed with all three |
|---|---:|---:|---:|---:|---:|---:|
| Low | 156.4 | 34.0% | 66.7 | 44.9 | 32.0 | 20.5% |
| Central | 140.0 | 31.9% | 67.0 | 44.9 | 32.0 | 19.1% |
| High | 120.2 | 29.5% | 68.1 | 44.9 | 32.0 | 17.6% |

| Case, with all three levers | k at launch | Years to k = 0 under the rule (within 100) | VAT rate that then meets the need |
|---|---:|---|---:|
| Low | 2.6% | never | 20.5% |
| Central | 0.0% | 0 | 19.1% |
| High | 0.0% | 0 | 17.6% |

The arithmetic, central case (each figure is in the model output and reproduced by `cargo run --release`):

- **Rate.** The VAT that closes the gap alone, with no income tax and the long-run base, is 31.9%.
- **Base broadening.** Taxing the household GST exemptions and zero-rating [R3] at the rule rate: their cost at today's rate, times the rule rate over today's rate.
- **Spending review.** A cut the size of the Program Review: 18.9% of today's direct program expenses ($237.6B [R2]) is $44.9B. Direct program expenses include transfers to Indigenous Peoples and other programs; which items to cut is a political choice this paper does not make.
- **LVT.** The scenario in §5.4, $32B; the land value is **unverified**.

With all of these levers, the VAT that meets the need with no income tax is 20.5% (low), 19.1% (central) and 17.6% (high). Under the rule at 20%, the central case reaches full abolition immediately once the levers are in force. The low case still falls short: k stays at about 2.6% (years to zero: never). The levers take time to enact; the Program Review cuts were phased in over several budget years [R23].

---

## 12. The system as infrastructure

Ashlynn's framing is that the tax system is infrastructure. The test for infrastructure finance is that borrowing is matched by a measured return, not used to fund operating deficits. Finance Canada is moving to a capital budget that "distinguishes its day-to-day operational spending from expenditure that stimulates public and private sector capital formation" [R2].

Applied here: abolishing the income tax at once, on borrowed money, would cost the gap every year. The spin-up's extra revenue covers 1.1% of that cost in the central case and 4.5% in the high case ($5.7B a year), before interest. The average rate on federal unmatured debt is about 3.6% (public debt charges over unmatured debt [R2]). Borrowing for the tax cut therefore fails the infrastructure test: the debt would compound, on top of today's deficit of $36.3B. The rule in §10 is the infrastructure version of the transition: each step is paid for by revenue already measured.

### 12.1 Counter-arguments, with who bears them

| Risk | Measure | Value | Who bears it |
|---|---|---:|---|
| Regressivity | GST credit scaled to 20%, $B/yr (already netted out of the model's base) | 25.1 | low-income households if the credit is not scaled |
| Regressivity | Same, share of VAT revenue at 20% | 10.3% | |
| Evasion | k at launch if the net GST/HST gap (5.8%) applies to new VAT revenue | 50.7% | compliant firms and buyers |
| Evasion | k at launch if the gross GST/HST gap (13.5%) applies to new VAT revenue | 55.1% | compliant firms and buyers |
| Recession | Deficit change in a year the base falls 3%, $B | 10.8 | taxpayers, through the reversal |
| Recession | Years to repay it under the rule | 3 | |
| Profit shifting | Ireland 2024: GNI* / GDP | 57.1% | measured GDP overstates domestic income |

- **Regressivity and the credit.** A VAT takes a larger share of income from low-income households, though roughly the same share of their spending [R6, R6b]. Scaling the GST credit to 20% costs $25.1B a year; the model already nets it out of revenue. A universal prebate paid to every household would cost more; its cost is **unverified**.
- **Evasion and the VAT gap.** The CRA estimates the net GST/HST gap at 5.8% and the gross gap at 13.5% of revenue [R34]. If either applied to the new VAT revenue, k at launch would be higher (table above), and the shortfall falls on compliant firms and buyers.
- **Provincial harmonization.** In HST provinces the federal and provincial rates share one base [R3]. A federal rate of 20% on top of provincial sales taxes needs agreements with each province, and Quebec administers its own QST. Not modelled.
- **Capital flight.** Removing CIT and PIT lowers the incentive to move capital out. The risk runs the other way: profit booked in Canada without real activity, as in Ireland. Non-resident income tax stays outside the core need and is kept.
- **Recession.** A base 3% lower for a year raises that year's deficit by $10.8B in the central case. The rule pauses cuts, raises k, and repays the account in 3 years (table below). The cost falls on income-tax payers through the higher k, and on households, since the consumption base falls with spending.

| Year | k | Revenue, $B | Deficit change vs baseline, $B | Compensation account owed, $B | Rule action for next year |
|---:|---:|---:|---:|---:|---|
| 4 | 47.12% | 384.7 | -1.0 | 0.0 | cut k |
| 5 | 47.08% | 373.0 | 10.8 | 10.8 | reverse: raise k |
| 6 | 48.16% | 387.7 | -3.9 | 6.8 | reverse: raise k |
| 7 | 48.84% | 389.7 | -5.9 | 0.9 | reverse: raise k |
| 8 | 48.93% | 390.0 | -6.2 | 0.0 | cut k |
| 9 | 47.34% | 385.2 | -1.4 | 0.0 | cut k |

---

## 13. Limitations

- **Accounting model, not a general-equilibrium model.** The MPC, taxable share and multipliers are fixed. There are no wage, price-level, exchange-rate or interest-rate responses beyond the stated incidence assumption.
- **Incidence.** Headline figures assume full pass-through with the real basket held (A). If nominal spending does not rise with prices (B), revenue at 20% is about $26B lower and k rises from 47.3% to 56.1%. Reality is likely between the two; it is not estimated here.
- **Federal only.** Provincial PIT, CIT and sales taxes are excluded, as is the provincial share of HST. A full Canadian reform would need provincial agreement. The constitutional route to referendum entrenchment in Canada is **unverified**.
- **Property tax total** is derived from a per-person figure (Nova Scotia Finance, from StatCan CGFS) times the 1 July 2024 population; it covers provincial and local property taxes.
- **One base year** (FY2024-25), which includes the GST holiday ($1.3B).
- **Tax-expenditure costs** are not the same as revenue gained, and they are 2024 projections [R3].
- **Multipliers** are short-run estimates from 2009 [R8], with leakage built in. Long-run CIT effects may be larger.
- **The land value total is unverified.**
- **Compliance leakage** at higher rates is a what-if, not an estimate. The growth addition is a what-if, not an estimate.
- **Distribution by decile** and the full dividend cost are **unverified**.
- **Transition financing.** The rule in §10 keeps each year's deficit at or below the baseline, so no transition debt is modelled; federal debt is 41.2% of GDP [R2]. The rule's result depends on the spin-up inputs below.
- **Spin-up inputs.** The capital semi-elasticity is **unverified**. The input-output values, and the capital share derived from them, are **unverified**: they were read from a search index's copy of the Statistics Canada table because the live site was down when accessed. The capital proxy includes land and government assets. The METR cut in the low and central cases is a scenario, not an estimate of the federal share.
- **Portability.** Other mid-sized advanced economies have different VRRs and income-tax shares. Rerun the model with their inputs.

---

## 14. References

- [R1] Receiver General for Canada, *Public Accounts of Canada 2025*, Vol. I, Section 3 "Revenues", Tables 3.2 and 3.3. https://www.tpsgc-pwgsc.gc.ca/recgen/cpc-pac/2025/vol1/s3/rvnu-eng.html
- [R2] Department of Finance Canada, *Annual Financial Report of the Government of Canada, Fiscal Year 2024–2025*, Tables 2 and 4. https://www.canada.ca/en/department-finance/services/publications/annual-financial-report/2025.html
- [R3] Department of Finance Canada, *Report on Federal Tax Expenditures – Concepts, Estimates and Evaluations 2026*, Part 1 (GST benchmark: input tax credits; HST = GST rate plus a provincial component; "GST" = federal portion) and Part 2 (GST measures; PIT rate schedule). https://www.canada.ca/en/department-finance/services/publications/federal-tax-expenditures/2026/part-1.html · https://www.canada.ca/en/department-finance/services/publications/federal-tax-expenditures/2026/part-2.html
- [R4] OECD (2024), *Consumption Tax Trends 2024*, Annex Tables 1.A.4, 2.A.1, 2.A.7. https://doi.org/10.1787/dcd4dd36-en
- [R6] OECD/KIPF (2014), *The Distributional Effects of Consumption Taxes in OECD Countries*. https://www.oecd.org/content/dam/oecd/en/publications/reports/2014/12/the-distributional-effects-of-consumption-taxes-in-oecd-countries_g1g4a7fc/9789264224520-en.pdf
- [R6b] OECD (2020), *Reassessing the Regressivity of the VAT*. https://www.oecd.org/content/dam/oecd/en/publications/reports/2020/08/reassessing-the-regressivity-of-the-vat_39f90653/b76ced82-en.pdf
- [R7] Johansson, Å. et al. (2008), "Taxation and Economic Growth", OECD Economics Department Working Paper No. 620. https://www.oecd.org/en/publications/taxation-and-economic-growth_241216205486.html
- [R8] Department of Finance Canada, *Budget 2009*, Annex 1, Table A1.1 (multipliers). https://budget.canada.ca/2009/plan/bpa1-eng.html
- [R9] Statistics Canada, *The Daily*, 11 September 2025 (household saving rate Q2 2025; national balance sheet). https://www150.statcan.gc.ca/n1/daily-quotidien/250911/dq250911a-eng.htm
- [R10] Statistics Canada, *The Daily*, 12 September 2024 (household saving rate Q2 2024). https://www150.statcan.gc.ca/n1/daily-quotidien/240912/dq240912a-eng.htm
- [R11] World Bank, World Development Indicators, Table 4.8 "Structure of demand" (Canada 2024). https://wdi.worldbank.org/table/4.8
- [R12] Mirrlees, J. et al. (2011), *Tax by Design*, Institute for Fiscal Studies. https://ifs.org.uk/books/tax-design
- [R13] Canada Revenue Agency, "Canada Groceries and Essentials Benefit (CGEB)". https://www.canada.ca/en/revenue-agency/services/child-family-benefits/canada-groceries-essentials-benefit.html
- [R14] Library of Parliament, *The Goods and Services Tax* (PRB 00-03E). https://publications.gc.ca/Collection-R/LoPBdP/BP/prb0003-e.htm
- [R14b] Department of Finance Canada, *Report on Federal Tax Expenditures 2017*, Part 10. https://www.canada.ca/en/department-finance/services/publications/federal-tax-expenditures/2017/part-10.html
- [R15] Zakat, Tax and Customs Authority (Saudi Arabia), "About VAT" (5% from 1 January 2018). https://zatca.gov.sa/en/RulesRegulations/VAT/Pages/About-Vat.aspx
- [R15b] Ministry of Finance (Saudi Arabia), "Additional Measures to Confront the Financial and Economic Impact of the Coronavirus Pandemic", 11 May 2020 ("VAT shall increase from 5% to 15% as of July 2020"). https://www.mof.gov.sa/en/MediaCenter/news/Pages/News_11052020.aspx
- [R16] Inland Revenue Authority of Singapore, "Individual Income Tax rates" and "Current GST rates". https://www.iras.gov.sg/taxes/individual-income-tax/basics-of-individual-income-tax/tax-residency-and-tax-rates/individual-income-tax-rates · https://www.iras.gov.sg/taxes/goods-services-tax-(gst)/basics-of-gst/current-gst-rates
- [R17] Meade, J. E. (chair) (1978), *The Structure and Reform of Direct Taxation*, Institute for Fiscal Studies / Allen and Unwin, ch. 9 "A Universal Expenditure Tax" and ch. 10 "A Two-Tier Expenditure Tax" (chapter titles from the table of contents; Routledge reissue, https://doi.org/10.4324/9780203120798-22). https://ifs.org.uk/books/structure-and-reform-direct-taxation
- [R18] Hall, R. E. and Rabushka, A., *The Flat Tax* (Hoover Institution Press; 1st ed. 1985, 2nd ed. 1995, reissued 2007). https://www.hoover.org/research/flat-tax
- [R19] Bradford, D. F. (1986), *Untangling the Income Tax*, Harvard University Press; and (2004) *The X Tax in the World Economy*, AEI Press. https://www.aei.org/research-products/book/the-x-tax-in-the-world-economy/
- [R20] U.S. Congress, H.R. 25 (119th Congress), *FairTax Act of 2025*, introduced text (Title I repeals; proposed sec. 101(b) rate; ch. 3 family consumption allowance). https://www.govinfo.gov/content/pkg/BILLS-119hr25ih/html/BILLS-119hr25ih.htm
- [R21] Royal Commission on Taxation (Carter Commission), *Report* (1966); summary in *The Canadian Encyclopedia*, "Royal Commission on Taxation". https://thecanadianencyclopedia.ca/en/article/royal-commission-on-taxation
- [R22] Inland Revenue (New Zealand), "GST rate increase", Taxation (Budget Measures) Act 2010. https://www.taxtechnical.ird.govt.nz/new-legislation/act-articles/taxation-budget-measures-act-2010/gst-rate-increase
- [R23] Department of Finance Canada, *Budget 1995*, Budget Plan (Program Review). https://www.budget.canada.ca/archives/budget95/binb/budget1995-eng.pdf
- [R24] Swiss Federal Finance Administration, "Federal Government's debt brake". https://www.efv.admin.ch/en/federal-governments-debt-brake
- [R25] German Federal Ministry of Finance, "Act to implement Article 115 of the Basic Law" (English translation). https://www.bundesfinanzministerium.de/Content/EN/Gesetze/Laws/2009-08-10-article-115-act.html
- [R26] Government Offices of Sweden, "The fiscal policy framework". https://www.government.se/government-of-sweden/ministry-of-finance/central-government-budget/the-fiscal-policy-framework/
- [R27] Inland Revenue (New Zealand), "Goods and Services Tax Bill 1985" (archived commentary). https://www.taxtechnical.ird.govt.nz/pib-reviews/archived-legislative-commentary/gst/goods-and-services-tax-bill-1985
- [R28] Inland Revenue (New Zealand), "Income Tax Amendment Act (No 2) 1986" (archived commentary). https://www.taxtechnical.ird.govt.nz/pib-reviews/archived-legislative-commentary/income-tax/income-tax-amendment-act-no-2-1986
- [R29] Department of Finance Canada, *Budget 2025*, Chapter 1. https://budget.canada.ca/2025/report-rapport/chap1-en.html
- [R30] International Monetary Fund, Working Paper WP/20/77, "Tax Elasticity Estimates for Capital Stocks in Canada" (abstract only; **unverified**). https://www.imf.org/en/Publications/WP/Issues/2020/06/05/Tax-Elasticity-Estimates-for-Capital-Stocks-in-Canada-49461
- [R31] Statistics Canada, Table 36-10-0013-01, "Input-output multipliers, summary level". https://www150.statcan.gc.ca/t1/tbl1/en/tv.action?pid=3610001301
- [R32] Central Statistics Office (Ireland), *Annual National Accounts 2024*, "GNI and De-Globalised Results". https://www.cso.ie/en/releasesandpublications/ep/p-ana/annualnationalaccounts2024/gniandde-globalisedresults/
- [R33] Estonian Tax and Customs Board, "Tax liabilities of companies". https://www.emta.ee/en/business-client/registration-business/non-residents-e-residents/tax-liabilities-companies
- [R34] Canada Revenue Agency, "Overall federal tax gap report 2026". https://www.canada.ca/en/revenue-agency/corporate/about-canada-revenue-agency-cra/tax-canada-a-conceptual-study/tax-gap-brief-overview/2026-overall-federal-tax-gap-report.html
- Nova Scotia Department of Finance, "Government Financial Statistics, 2024" (21 November 2025), from Statistics Canada Table 10-10-0147-01. https://novascotia.ca/finance/statistics/archive_news.asp?df=&dg=&dti=3&dto=0&id=21517 · Statistics Canada, *The Daily*, 25 September 2024 (population, 1 July 2024). https://www150.statcan.gc.ca/n1/daily-quotidien/240925/dq240925d-eng.htm

---

*License: AGPL-3.0 · Repo: Dastille/government-tax-reform*
