<!-- GENERATED from analysis/templates/tax-whitepaper.md by `cargo run --release -- --write`. Edit the template, not this file. -->
# Fundamental Tax Reform: Toward a Single Consumption-Based System

**Whitepaper · v3.0 (debt-neutral transition) · October 2026**
**Author:** Ashlynn (original design, January 2026). v2.0 fixed the arithmetic and added sources; v3.0 adds a debt-neutral transition rule, an industry spin-up scenario and the infrastructure test (§10, §11 and §12). Neither changes the design goals.

**Scope.** The design is meant for **mid-sized advanced economies**. Canada is the **worked example** because its federal accounts for fiscal year 2024-25 (April 2024 to March 2025, "FY2024-25") are public and detailed. The method carries over to other countries: the VAT rate a country needs equals the revenue it must replace divided by its effective VAT base.

**How the numbers are produced.** This file is generated. Every model-derived number and table is inserted from the revenue model in [`analysis/`](analysis/), and a test fails if this file differs by one byte from a fresh render. Inputs and their sources are in [`analysis/src/data.rs`](analysis/src/data.rs). Every other number in the prose is on a sourced allowlist (`analysis/tests/docs.rs`) that names the line it may appear on. Anything we could not verify is marked **unverified**. See [CLAIMS.md](CLAIMS.md).

---

## 1. Abstract

**Problem.** Income taxes discourage work, saving and investment more than taxes on spending do. The OECD's ranking puts corporate income tax as the most harmful to growth, then personal income tax, then consumption taxes, with recurrent property taxes the least harmful [R7]. This proposal abolishes five taxes: personal income tax (PIT), corporate income tax (CIT), payroll taxes, capital gains tax and recurrent property tax. It replaces them with one broad value-added tax (VAT) that starts at {{start_rate}}% and steps down to {{end_rate}}% over {{path_years}} years. A land value tax (LVT), an income-tested citizen dividend, an independent fiscal board and referendum entrenchment complete the design.

**Thesis.** When income tax goes, people keep more of what they earn, so they spend more, and VAT revenue grows. A static one-for-one comparison misses this effect.

**Method.**

- Every Canadian baseline figure is checked against the Public Accounts and the Annual Financial Report [R1, R2].
- A small Rust model (no dependencies, tested in CI) computes:
  - static revenue at each rate;
  - a dynamic scenario in which households and firms keep ${{it_b}}B of income tax and spend part of it (the marginal propensity to consume, or MPC) at VAT-inclusive prices, with an optional demand multiplier;
  - every scenario, static included, under one stated assumption about prices and nominal spending (§2), with the alternative shown as a sensitivity;
  - base-broadening, compliance, land-value and property-tax scenarios, and the break-even conditions.

**Result.** Headline figures use incidence assumption A (§2), with assumption B as a sensitivity.

- A {{start_rate}}% VAT on Canada's current federal base raises **${{static_hi_a}}B** statically.
- With the thesis modelled at central values, it raises **${{dyn_hi_a}}B**, or **${{mult_hi_a}}B** if Finance Canada's short-run multipliers are added.
- The revenue needed is **${{need_core_b}}B**: PIT plus CIT plus today's GST.
- So a {{start_rate}}% VAT replaces {{replaced_frac_a}} of the income taxes. Under central values, **{{k_a}}** of today's PIT and CIT must remain (**k**). Full replacement on today's base needs a rate of **{{rate_dyn_a}}**, or **{{rate_mult_a}}** with the multiplier, against {{rate_static_a}} statically.
- The spending effect is real, but {{mpc_verdict_a}}: the MPC would have to be {{mpc_be_a}}. Only about {{taxable_share_pct}}% of new consumer spending falls in today's taxable base, and that spending pays VAT-inclusive prices.
- Full abolition at {{start_rate}}% needs a taxable base about **{{g_hi_a}}×** {{g_hi_a_dir}} relative to spending needs. The {{end_rate}}% endpoint needs about **{{g_lo_a}}×**.
- The LVT must first replace about **${{property_b}}B** of recurrent property tax, {{property_vs_lvt}} the LVT scenario raises (${{lvt_scenario}}B).
- **Sensitivity B** gives a {{b_vs_a}} floor: ${{static_hi_b}}B static and ${{dyn_hi_b}}B dynamic at {{start_rate}}%, k = {{k_b}}. The conclusion is the same under either assumption.
- v2.0 therefore keeps every mechanism of the design. Income taxes are reduced as far as VAT and LVT revenue allows, and they are abolished, and the VAT rate stepped down, only when the fiscal board certifies that revenue covers the need.
- **v3.0, debt-neutral.** Under a rule that cuts income tax only from revenue already realised (§10), the deficit never rises above the baseline without a shock. An industry spin-up from lower income tax (§11) {{v3_verdict}}: k settles at {{v3_settle_central}} in the central case and {{v3_settle_high}} in the high case. Closing the gap takes base broadening, a Program-Review-sized spending cut and the LVT together (§11.3); with them, the central case can abolish the income tax, and the VAT it then needs is {{v3_pkg_rate_central}}.

**Key figures:**

{{table:key_figures}}

---

## 2. Problem statement and definitions

**Problem.** Can one broad consumption tax, plus a land value tax, raise the revenue that PIT, CIT, payroll, capital gains and recurrent property taxes raise today? If so, at what rate, and under what conditions can that rate fall to {{end_rate}}%?

**Definitions** (each term is defined once here):

- **VAT / GST.** A tax on final consumption that businesses collect at each stage of production, getting a credit for the tax they paid on their inputs, so the tax falls on final consumption [R3, Part 1]. Canada's federal version is the {{gst_rate}}% Goods and Services Tax (GST). The Harmonized Sales Tax (HST) applies the GST rate plus a provincial component; "GST" in this paper means the federal portion only, as in the federal tax-expenditure report [R3, Part 1].
- **Net GST (FY2024-25: ${{gst_b}}B).** "Net GST from outside parties" in the Public Accounts: GST after the remission order for GST paid by federal ministries and after the quarterly GST credits [R1, Table 3.3]. The Public Accounts table does not itemize HST; we read it as federal revenue only, which excludes provincial HST portions. **This is the base used in the model.**
- **Effective base.** Net GST ÷ {{gst_rate}}% = **${{base0_b}}B**: the spending that actually bears the tax after exemptions, zero-ratings, credits and non-compliance. Gross GST from outside parties ÷ {{gst_rate}}% = ${{gross_base_b}}B, which matches the original paper's "consumption base ~$1.1-1.2T".
- **Core need (${{need_core_b}}B).** PIT + CIT + net GST: what the VAT must raise if it absorbs the GST and both income taxes are abolished. **Full federal scope (${{need_full_b}}B)** also counts non-resident income tax (${{nr_b}}B) and EI premiums (${{ei_b}}B, a federal payroll tax).
- **Revenue-neutral.** Total federal revenue unchanged. This paper uses the word only when the arithmetic shows it.
- **Tax-exclusive rate.** VAT as a percentage of the pre-tax price (Canada quotes the GST this way).
- **VAT Revenue Ratio (VRR).** Actual VAT revenue divided by what the standard rate would raise on all final consumption. Canada: {{vrr_ca}}. OECD average: {{vrr_oecd}}. New Zealand: {{vrr_nz}} (2022) [R4, Annex Table 2.A.7].
- **Incidence assumption.** What happens to prices and nominal spending when the VAT rate rises. Every scenario, static included, uses the same assumption, so "dynamic" differs from "static" only by the spending effect.
  - **A (central): real base fixed.** The VAT is passed into prices and households keep buying the same tax-exclusive basket, so revenue on today's base = rate × base. This is how a static costing scales a sales tax.
  - **B (sensitivity): nominal spending fixed.** Prices rise, but nominal spending on today's base does not, so a higher rate buys less: revenue = rate/(1 + rate) × spending.
  - Under both, new spending from the income tax people keep pays VAT-inclusive prices.
- **MPC.** The share of an extra dollar of after-tax income that is spent rather than saved.
- **Taxable share.** The share of new spending that falls in the VAT base: today's effective base divided by household consumption = **{{taxable_share}}**.
- **LVT.** An annual tax on the value of land alone, excluding buildings.

---

## 3. Baseline (Canada, FY2024-25, $B)

Exact values: PIT {{pit_m}}, CIT {{cit_m}}, net GST {{gst_m}} ($ millions) [R1, Tables 3.2 and 3.3; R2, Table 4].

{{table:baseline}}

Notes:

- The temporary GST holiday (14 Dec 2024 to 15 Feb 2025) fell inside FY2024-25 [R1]. Its cost, ${{holiday_b}}B [R3], is shown as a sensitivity in §5.3.
- The GST/HST credit was renamed the **Canada Groceries and Essentials Benefit (CGEB)** in July 2026, with a {{cgeb_increase}}% increase for 2026-2031 [R13]. The FY2024-25 figures predate both changes.

---

## 4. Proposal: policy specification (single source of truth)

This section is normative. Any app or calculator that implements the policy (for example lion-berry) should match it exactly. Items marked *(v2)* are new in this revision; each one exists to keep the original design arithmetically honest.

1. **Taxes abolished (original design):** PIT (including tax on capital gains), CIT, payroll taxes, capital gains tax, and recurrent property taxes.
   - *(v2)* **Order and trigger.** Abolition is completed when the fiscal board certifies that VAT + LVT revenue covers the revenue need without that tax. Until then, all federal PIT and CIT rates are multiplied by a single factor *k* set each year by the board. At launch, *k* is the smallest value that keeps revenue at the core need. Under central assumptions with a {{start_rate}}% VAT, **k = {{k_a}}**: federal bracket rates of {{brackets_k_a}}% on the 2023 schedule [R3; §5.6]. Under sensitivity B, k = {{k_b}}. The board sets *k* from observed revenue, not from either assumption.
   - *(v2)* **Property tax.** Recurrent property taxes are levied by municipalities and, in some provinces, by the province. Each is abolished once LVT revenue in that jurisdiction replaces it (§5.4).
2. **One broad VAT** replaces the GST.
   - **Launch rate: {{start_rate}}% at t = 0** (t = years since launch).
   - **Scheduled path:** rate(t) = {{start_rate}} − {{path_step}}·t for t = 0…{{path_years}}, which gives {{path_list}}. This corrects v1.1's `20 − 1.2 × (year − 1)`, which reached only {{v11_end}}% in year {{path_years}}.
   - *(v2)* **Step-down rule.** The rate moves to the next step only if the board certifies that revenue at the lower rate covers the need with income tax at *k* = 0. Abolishing income taxes (reducing *k*) comes before cutting the VAT rate. If a step is not certified, the rate holds.
3. **Differentiated rates (original design).**
   - Zero or reduced rates on unprepared food, medicine, education and children's clothing.
   - Higher rates on luxuries, tobacco, alcohol, fossil fuels and carbon-intensive goods. Revenue from the higher rates funds the reductions.
   - *(v2)* The model treats today's zero-ratings as continuing. Their cost at {{start_rate}}% is ${{groceries_hi}}B for groceries alone (§6). Higher-rate schedules are not yet specified, so they are **not modelled**. Children's clothing is taxable today, and the cost of zero-rating it is **unverified**.
4. **Land value tax (original design).**
   - {{lvt_rate_lo}}–{{lvt_rate_hi}}% a year on unimproved land value, aimed mainly at vacant urban and speculative non-farm land.
   - Exemptions or phased rates for active farmland, primary residences below a threshold, and conservation land.
   - *(v2)* LVT revenue first replaces recurrent property taxes. Only the surplus counts toward federal need.
5. **Citizen dividend (original design).**
   - An income-tested annual payment per adult, sized to fully offset the VAT's regressivity for the bottom {{dividend_lo}}–{{dividend_hi}}% of households.
   - *(v2)* **Floor:** at least the current GST credit/CGEB scaled to the VAT rate: ${{credit_hi}}B at {{start_rate}}%. It is already funded inside the model's revenue figures because the net-GST base is net of the credit.
   - The cost of the full offset is **unverified**: the needed distributional microdata were not available.
6. **Investment commitment (original design).** {{invest_lo}}–{{invest_hi}}% of incremental revenue goes to high-multiplier infrastructure. *(v2)* "Incremental" means revenue above the certified need. Under central assumptions there is none in the early years (§5), so the commitment applies once revenue exceeds need.
7. **Independent fiscal board (original design).** The board monitors performance, certifies abolition steps and rate reductions, and calibrates the differentiated rates and LVT parameters. *(v2)* The board publishes the model inputs it used for each certification.
8. **Entrenchment (original design).**
   - A referendum is required to reintroduce an abolished tax, to raise rates above the schedule, to add a new major tax, or to abolish the LVT.
   - Core amendments need a supermajority, and the board's authority is protected except by supermajority.
   - *(v2)* Holding the rate at a step because certification failed is not a rate increase.
9. **Implementation (original design).** Phased over {{path_years}} years, with upfront investment in administration and enforcement, and corporate tax eliminated in phases.

---

## 5. Revenue model

Code: [`analysis/src/model.rs`](analysis/src/model.rs). Full output: [`analysis/output/revenue_tables.md`](analysis/output/revenue_tables.md). Reproduce from a clean clone with one command: `cargo run --release --manifest-path analysis/Cargo.toml`. Every table below is inserted from the model.

### 5.0 First-principles derivation

Revenue is built up from measured quantities, not borrowed from other countries' results:

- VAT revenue = consumption base × coverage-and-compliance ratio × rate.
- LVT revenue = land value × taxable share × rate.

The coverage ratio is *measured* from what Canada's GST actually collected. Analogy (New Zealand, the OECD average) appears only as a scenario.

{{table:first_principles}}

### 5.1 Static: today's base, no spending effect

{{table:static}}

The static rate for the core need is **{{rate_static_a}}** under A and {{rate_static_b}} under B; for the full federal scope it is {{rate_static_full_a}} (A).

### 5.2 The thesis: households keep the money and spend it

All ${{it_b}}B of PIT and CIT stays with households and firms. They spend a share, the MPC. Of that spending, a taxable share lands in the VAT base and pays VAT-inclusive prices. Today's base is treated exactly as in the static case (A or B); the only difference from static is the new spending.

- **MPC evidence.** Canada's household saving rate was {{saving_2024}} in Q2 2024 and {{saving_2025}} in Q2 2025 [R9, R10], so the average propensity to consume was {{apc_lo}}–{{apc_hi}}. A one-off windfall is usually spent at a lower rate than average income, so {{mpc_central}} is a generous upper bound.
- **Taxable share.** {{taxable_share}}: the effective base divided by household consumption ({{hfce_pct}}% of GDP [R11]).
- **Imports.** Imports are {{imports_pct}}% of Canada's GDP [R11]. They do **not** shrink a destination-based VAT base, because imports are taxed at the border. They matter only for how much domestic income the new spending generates, and Finance Canada's multipliers already include saving and import leakages [R8].
- **Optional multiplier.** Short-run multipliers of {{mult_pit}} for PIT measures and {{mult_cit}} for CIT measures [R8, Table A1.1]. Finance Canada also notes that CIT measures have among the highest multipliers in the long run, which this short-run figure does not capture.

{{table:dynamic}}

Every scenario under both assumptions:

{{table:sensitivity}}

**What this shows.**

- **Consistency.** Under one assumption, more revenue at a given rate always means a lower required rate; the test suite checks this across every scenario, both assumptions and several rates. An earlier draft of v2.0 computed static under A and dynamic under B, which produced an apparent contradiction; it is fixed.
- **The thesis is right about direction.** At the central MPC, real taxable consumption at {{start_rate}}% is ${{real_base_a}}B under A and ${{real_base_b}}B under B, against ${{base0_b}}B today.
- **Break-even.** The MPC would need to be **{{mpc_be_a}}** (A), **{{mpc_be_s1}}** even if every new dollar were taxable, or {{mpc_be_b}} under B; an MPC cannot exceed 1. What can close the gap is a **{{g_hi_a_dir}} taxable base**: **{{g_hi_a}}×** today's relative to spending needs, {{g_hi_a_pa}} a year faster than needs for {{path_years}} years. With the multiplier it is {{g_hi_mult}}× ({{g_hi_mult_pa}} a year); under B it is {{g_hi_b}}× ({{g_hi_b_pa}} a year).
- The original paper's growth addition of +{{growth_lo}} to +{{growth_hi}} points a year is **unsourced** and is used only as a what-if: it gives {{growth_f_lo}}× to {{growth_f_hi}}× over {{path_years}} years.

{{table:breakeven}}

### 5.3 Base broadening and compliance

{{table:broadening}}

Taxing groceries conflicts with the design's own zero rate on food; the row only sizes the trade-off.

{{table:leakage}}

### 5.4 Land value tax and the property tax it replaces

{{table:lvt}}

The original ${{lvt_claim_lo}}–{{lvt_claim_hi}}B a year needs ${{land_needed_lo}}–{{land_needed_hi}} trillion of taxable land. Total land value of about ${{land_t}}T is **unverified** (a secondary source citing StatCan Table 36-10-0580-01, which we could not download).

**Why abolish property tax at all?** The OECD ranks recurrent property tax as the *least* growth-harmful tax [R7], so abolishing it moves against that evidence. The original design's reason (ab193f4) is that "the land value tax captures rents from scarcity without discouraging improvements". A recurrent property tax has two parts: a tax on land value and a tax on buildings. The Mirrlees Review quotes William Vickrey: the property tax is "a combination of one of the worst taxes—the part that is assessed on real estate improvements …—and one of the best taxes—the tax on land or site value"; land "is not a produced input; its supply is fixed" [R12, ch. 16]. The design keeps the land part (the LVT) and drops the buildings part, so construction and upkeep are no longer taxed.

**The counter-argument, stated fairly.** R7's ranking is about recurrent property taxes as they exist, buildings included, and those taxes are hard to avoid because property cannot move. Mirrlees argues for taxing land *and* the consumption value of housing, not land alone [R12, ch. 16]. Dropping the buildings part gives up a low-distortion revenue source, and the LVT has to raise more from land alone to make up for it.

**The cost of that choice.** The LVT must first replace today's recurrent property taxes:

{{table:property}}

At the scenario's taxable land, the LVT would need a rate of {{lvt_replace_scen}}, {{lvt_vs_design}} the design's {{lvt_rate_hi}}%, just to replace property tax. Until LVT revenue covers property tax in a jurisdiction, the property tax stays (§4).

### 5.5 The rate path and the conditions for {{end_rate}}%

{{table:path}}

{{table:stack}}

### 5.6 Income tax that must remain while the VAT phases in

{{table:k}}

**Conclusion of the model.** Under FY2024-25 Canadian numbers:

- {{start_rate}}% at launch supports partial replacement, with *k* ≈ {{k_a_round}}% (A) to {{k_b_round}}% (B).
- Full abolition at {{start_rate}}% {{nz_condition}} (static rate needed with a New Zealand-level base: {{nz_rate_needed_a}} under A, {{nz_rate_needed_b}} under B).
- {{end_rate}}% needs that plus a base {{g_lo_a}}× today's relative to needs: {{g_lo_a_pa}} a year faster than needs for {{path_years}} years, {{growth_vs_original}} the original paper's unsourced growth addition (+{{growth_lo}} to +{{growth_hi}} points).
- Property tax cannot be abolished on the scenario LVT alone.

The fiscal board's certification rule keeps the original goal (abolition, then {{end_rate}}%) as a target that can be met, rather than a promise that cannot.

---

## 6. Distributional analysis

- **Who bears a VAT.** Measured as a share of *income*, VATs are regressive. Measured as a share of *spending*, they are roughly proportional or slightly progressive [R6]. A 2020 OECD reassessment reaches the same conclusion [R6b].
- **Reduced rates are a poor targeting tool.** On aggregate, richer households get as much benefit from reduced rates as poorer ones, or more [R6]. Mirrlees et al. recommend removing zero and reduced rates and compensating through direct transfers [R12, ch. 9].
- The original design used **both** reduced rates and a dividend. v2.0 keeps both, and adds that the dividend is the stronger tool per dollar.
- **Canada already does this.** The GST credit was introduced with the GST in 1991 [R14b] and cost ${{credit_b}}B in FY2024-25 [R1]. Now called the CGEB, it was increased {{cgeb_increase}}% from July 2026 [R13].
- **Who gains from abolishing income tax.** Federal PIT is progressive (bracket rates {{bracket_lo}}–{{bracket_hi}}% in 2023 [R3]), so the dollar gains from cutting it go mostly to higher earners. The shares by decile are **unverified** here. This is why the dividend has to be in place **at launch**.

{{table:protect}}

---

## 7. Counter-arguments, answered with numbers

1. **"{{start_rate}}% raises only about ${{static_hi_a}}B, not ${{it_b}}B" (static).** Correct. With the thesis modelled at central values, {{start_rate}}% raises ${{dyn_hi_a}}B, or ${{mult_hi_a}}B with the multiplier. That still falls about ${{short_lo}}–{{short_hi}}B short of the core need, so v2.0 makes abolition conditional (§4) instead of claiming it.
2. **"People will save the windfall."** Even at MPC = {{mpc_half}}, revenue at {{start_rate}}% is ${{mpc_half_rev}}B, against ${{dyn_hi_a}}B at {{mpc_central}}. The MPC moves revenue by about ${{mpc_effect}}B, which is {{mpc_effect_size}} next to the gap.
3. **"Imports leak."** Imports are {{imports_pct}}% of GDP, but a destination-based VAT taxes them at the border, so they do not reduce the base. They reduce only the domestic multiplier, and the Finance Canada multipliers already account for that.
4. **"High rates invite evasion."** At {{leak20_pct}}% compliance leakage, the required rate rises from {{rate_static_a}} to {{leak20_rate}} (§5.3). This is why the design puts upfront investment in enforcement and administration first.
5. **"It is regressive."** The credit floor costs ${{credit_hi}}B at {{start_rate}}%; zero-rating groceries costs ${{groceries_hi}}B. Evidence favours the transfer [R6, R12].
6. **"Retirees pay twice"** (taxed on income while working, then on spending in retirement). This is valid for spending out of savings accumulated before the switch. It is **not quantified** here. A phase-in and the dividend reduce the problem but do not remove it.
7. **"Property tax is the least harmful tax; why abolish it?"** [R7]. Answered in §5.4: the LVT keeps the land part and drops the buildings part, but it must first raise about ${{property_b}}B, so property tax stays until it does.
8. **The strongest counter-argument: the base is too narrow for the rate to be credible.** Canada's effective VAT base is {{base_frac_gdp}} of GDP (${{base0_b}}B of ${{gdp_b}}B). The taxes to be replaced are worth {{it_pct_gdp}} of GDP. So the rate has to be about {{rate_static_a_round}}% statically, or about {{rate_dyn_a_round}}% counting the spending effect ({{rate_dyn_b_round}}% under assumption B). The highest standard VAT rate in the OECD is Hungary's {{hu_rate}}% [R4, Annex Table 2.A.7]. *Answer:* the design does not need {{rate_dyn_a_round}}% if the base becomes as broad and well-collected as New Zealand's (VRR {{vrr_nz}}, which needs {{nz_rate_needed_a}}). Until then, the board keeps part of the income tax. The counter-argument limits the *speed* of reform, not its direction.

---

## 8. Prior work and precedents

### 8.1 Earlier proposals to replace income tax with consumption tax

| Proposal | What it proposed | How this design differs |
|---|---|---|
| Meade Report (UK, 1978) [R17] | A review of the whole UK tax system that favoured an expenditure-tax base, setting out universal (ch. 9) and two-tier (ch. 10) variants: tax what people spend rather than what they earn. | Meade kept a progressive personal tax on spending. This design uses a VAT collected from businesses, and gets progressivity from the dividend rather than from rates. |
| Hall-Rabushka flat tax (1985; 2nd ed. 1995) [R18] | One low flat rate replacing the federal income tax, taxing all income once, either at the business or at the worker, on a consumption-type base [R18, R19]. | Hall-Rabushka keeps a tax return for workers. This design removes personal returns and relies on a VAT plus transfers. |
| Bradford X-tax (1986; 2004) [R19] | A consumption-type tax built like Hall-Rabushka, designed to replace the income tax on corporations and individuals while keeping "substantial scope for progressivity" at the worker level. | The X-tax keeps progressivity inside the tax. This design moves it into the dividend. |
| US FairTax, H.R. 25 (119th Congress) [R20] | A national retail sales tax at {{fairtax_rate}}% of gross (tax-inclusive) payments in 2027, replacing federal income, payroll, estate and gift taxes, with a monthly rebate (the "family consumption allowance"). | Closest in spirit (replace income and payroll taxes with a consumption tax plus a rebate). This design uses a multi-stage VAT rather than a retail sales tax, adds an LVT and a fiscal board, and its model states the rate needed instead of assuming one. |
| Mirrlees Review, *Tax by Design* (2011) [R12] | Keep income tax but exempt the normal return to saving; broaden the VAT base and compensate through benefits. | Mirrlees improves the income tax; this design removes it. Both agree on a broad VAT base with compensation through transfers. |
| Carter Commission (Canada, 1966) [R21] | The opposite direction: a comprehensive *income* base, taxing every increase in economic power alike ("a buck is a buck"). | Carter's goal was fairness through a broad income base. This design seeks it through a broad spending base plus a dividend. Carter is the strongest Canadian argument for keeping income tax. |

**What is new here.** The design combines four things: abolishing income *and* property taxes; a falling VAT path; an LVT that keeps the land part of property tax; and a fiscal board that has to certify each step. It also publishes a model that states the rate needed instead of assuming it.

### 8.2 Precedents

- **New Zealand (GST, 1986).** The rate rose from {{nz_prev_rate}}% to {{nz_rate}}% on {{nz_rate_date}}, explicitly as "a switch in the tax mix from income tax to consumption tax" [R22]. The base is very broad and includes public services [R4]. VRR {{vrr_nz}}, the highest in the OECD; VAT is {{nz_vat_gdp}}% of GDP and {{nz_vat_taxes}}% of all taxes (2022) [R4, Annex 1.A.4]. New Zealand **kept** its income taxes: it shows how broad a base can be, not that income tax can be fully replaced.
- **Denmark and Hungary.** Standard rates of {{dk_rate}}% and {{hu_rate}}% (2022) [R4, Annex 2.A.7]. VAT is {{dk_vat_gdp}}% of GDP / {{dk_vat_taxes}}% of taxes in Denmark and {{hu_vat_gdp}}% / {{hu_vat_taxes}}% in Hungary [R4, Annex 1.A.4]. Both still levy income taxes. Denmark's land tax is **unverified** here.
- **Gulf states.** Saudi Arabia introduced VAT at {{sa_rate_2018}}% on {{sa_date_2018}} [R15] and announced on 11 May 2020 that it would rise to {{sa_rate_2020}}% from {{sa_date_2020}} [R15b]: a large, fast rate change. Statements about personal income tax and about UAE, Bahrain and Oman VAT are **unverified** here.
- **Canada 1991.** The {{gst_1991_rate}}% GST replaced the {{fst_rate}}% federal sales tax (FST), in place since 1924, on 1 January 1991, together with the GST credit [R14, R14b]. It is now {{gst_rate}}%.
- **Singapore.** v1.1 described it as having "minimal income taxes". That is incorrect: IRAS gives the current highest personal income tax rate as {{sg_pit_top}}% and the current GST rate as {{sg_gst}}% [R16].
- **Breadth of VAT.** {{vat_countries}} countries and territories had a VAT as of 1 July 2024 [R4].
- Land value taxation in parts of Australia is **unverified** here. The Pennsylvania cities example (added in v1.1, not part of the original design) has been removed.

---

## 9. Vision vs feasible: requirements questioned

Five steps: question each requirement and name who set it; delete what adds nothing; simplify; make the model fast to rerun; automate the checks last. "Ashlynn (ab193f4)" is the original design (January 2026); "Bot" rows are automated edits from August 2026; "Audit" is the October 2026 audits.

| Requirement | From | Question asked | Decision |
|---|---|---|---|
| Abolish PIT, CIT, payroll, capital gains and property tax | Ashlynn (ab193f4) | Does a {{start_rate}}% VAT pay for it? | **Kept as the goal, made conditional.** k = {{k_a}} of income tax remains at launch; full abolition needs a base about {{g_hi_a}}× {{g_hi_a_dir}} |
| Abolish recurrent property tax | Ashlynn (ab193f4) | The OECD ranks it least harmful [R7]: why abolish? | **Kept, reason stated** (§5.4): the LVT keeps the land part. Property tax stays until the LVT replaces about ${{property_b}}B |
| VAT {{start_rate}}% → {{end_rate}}% over {{path_years}} years | Ashlynn (ab193f4) | Is the path consistent, and is {{end_rate}}% reachable? | **Kept.** Formula fixed; steps need certification; {{end_rate}}% needs about {{g_lo_a}}× the base |
| Thesis: kept income is spent, so VAT revenue grows | Ashlynn (ab193f4; steering 2026-10-18) | How large is the effect? | **Central and modelled.** It adds about ${{dyn_gain_lo}}–{{dyn_gain_hi}}B at {{start_rate}}% (A); the MPC needed to close the gap alone is {{mpc_be_a}} |
| LVT, ${{lvt_claim_lo}}–{{lvt_claim_hi}}B | Ashlynn (ab193f4) | How much land does that need? | **Kept; amount flagged.** It needs ${{land_needed_lo}}–{{land_needed_hi}}T of taxable land; scenario ${{lvt_scenario}}–{{lvt_all_land}}B |
| Dividend fully offsets the bottom {{dividend_lo}}–{{dividend_hi}}% | Ashlynn (ab193f4) | What does it cost? | **Kept.** Floor ${{credit_hi}}B at {{start_rate}}%; full cost **unverified** |
| Fiscal board; referendum and supermajority | Ashlynn (ab193f4) | Does it conflict with a conditional path? | **Kept.** Certification is the mechanism that makes the path conditional |
| {{invest_lo}}–{{invest_hi}}% of incremental revenue to infrastructure | Ashlynn (ab193f4) | Is there incremental revenue early on? | **Kept, re-timed.** It applies once revenue exceeds need |
| "Neutral within a decade" | Ashlynn (ab193f4) | Does the arithmetic support it? | **Deleted as a claim.** Replaced by the certification rule |
| Velocity, investment multipliers, base elasticity | Ashlynn (ab193f4) | Is there a source? | **Deleted.** None found; replaced by the MPC sweep and Finance Canada multipliers ({{mult_cit}}–{{mult_pit}}) |
| Growth +{{growth_lo}}–{{growth_hi}} pp a year | Ashlynn (ab193f4) | Is there a source? | **What-if only, unverified.** It gives {{growth_f_lo}}–{{growth_f_hi}}× over {{path_years}} years |
| "Over 170 countries"; "Singapore: minimal income tax" | Ashlynn (ab193f4) | Still true? | **Corrected** to {{vat_countries}} [R4]; Singapore's top PIT rate is {{sg_pit_top}}% [R16] |
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

Each year the fiscal board compares realised federal revenue with the core need (${{need_core_b}}B, today's PIT + CIT + GST). Realised revenue is the VAT, plus the income tax still in force (share k), plus any growth in the land and property base.

1. **Hold the rate.** The VAT stays at {{start_rate}}% until k reaches zero. The step down to {{end_rate}}% (§5.5) waits until then: on today's base, a VAT of {{end_rate}}% would need k = {{v3_k_end}} under central values, so the step down cannot be debt-neutral while income tax remains.
2. **Cut only from realised surplus.** k falls by realised revenue above the need, after any balance owed in a compensation account is repaid, and never by more than the structural surplus. Structural revenue leaves out temporary demand from the investment build-out, as the Swiss rule leaves out the cycle.
3. **Pause.** If realised revenue is below the need, the shortfall is booked in the compensation account and k is not cut.
4. **Reverse.** If structural revenue is below the need, k rises by the shortfall. While the account is owed, k is raised enough to repay it over {{v3_repay_design}} years.
5. **Property and land.** The federal government levies no property tax, so this term is nil unless the LVT (§5.4) is adopted. LVT revenue then counts under the same test.

Without a shock, the deficit change against the baseline is zero or negative in every simulated year of every case (`transition.rs` test `deficit_never_rises_without_a_shock`). No rule can prevent a deficit in a year when the base falls unexpectedly. The rule's answer is the account and the reversal (§12.1).

### 10.2 Precedents for the rule

| Country | Rule | Relevance | Source |
|---|---|---|---|
| Canada, Program Review | Spending under review fell {{v3_pr_pct}}% relative to {{pr_base_year}}; it covered about ${{pr_scope_b}}B of spending, excluding major statutory transfers. Interim deficit target {{pr_deficit_target}}% of GDP (${{pr_deficit_target_b}}B), pursued mainly through ${{pr_cuts_b}}B of cumulative spending reductions: "{{pr_ratio}}" | Spending review at this scale has been done in Canada | Budget 1995 [R23] |
| Switzerland, debt brake | Ordinary spending is capped at receipts adjusted by the ratio of trend GDP to current GDP; shortfalls go to a compensation account that must be cleared in later years; a tax cut must be matched by spending cuts. Approved by {{ch_yes_vote}}% on {{ch_vote_date}}; debt ratio from {{ch_debt_2003}} to {{ch_debt_2019}} | Source of the structural test, the account and the matching rule | Federal Finance Administration [R24] |
| Germany, Schuldenbremse | Structural borrowing up to {{de_structural}}% of GDP; a control account whose negative balance may not exceed {{de_control}}% of GDP | Source of the control-account trigger | Article 115 Act [R25] |
| Sweden, fiscal framework | Surplus target over the business cycle and a debt anchor of {{se_debt_anchor}}% of GDP; the framework dates from the crisis of {{se_origin}} | A surplus rule can last decades | Government Offices of Sweden [R26] |
| New Zealand, GST | GST at a single rate of {{nz_gst_1986}}% from {{nz_date}}; personal income tax rates cut on the same date; new top rate {{nz_top_1986}}c | A broad VAT replaced part of income tax in one step | Inland Revenue [R27, R28] |

### 10.3 Result

{{table:transition}}

The rule keeps the transition debt-free, but the spin-up {{v3_verdict}}. Within {{v3_horizon}} years, k settles at {{v3_settle_low}} (low), {{v3_settle_central}} (central) and {{v3_settle_high}} (high), against {{v3_k_launch_low}}, {{v3_k_launch_central}} and {{v3_k_launch_high}} at launch. Years to full abolition within that horizon: {{v3_years_low}}, {{v3_years_central}} and {{v3_years_high}}. With k at zero, the gap left is ${{v3_gap_low}}B, ${{v3_gap_central}}B and ${{v3_gap_high}}B a year.

Central case, year by year:

{{table:transition_path}}

---

## 11. Industry spin-up scenario

This is a scenario, not a forecast. It asks how much new investment lower income tax could bring, and how much of the gap that investment's tax base would close.

### 11.1 Mechanism and inputs

- **Cost of capital.** Canada's marginal effective tax rate (METR) on new business investment is {{v3_metr}}%, or {{v3_metr_super}}% with the Budget 2025 super-deduction [R29]. The high case removes all of the latter. The low and central cases remove less, because part of the METR is provincial; the federal share is not in the sources used here.
- **Response.** The capital stock rises by {{v3_semi}}% per point of METR cut in the central case, less in the low case and more in the high case (table below). This semi-elasticity is **unverified**: it is the figure reported for an IMF working paper on Canadian capital stocks [R30], whose text could not be opened. Capital moves toward its new level at a set speed each year.
- **Base.** A larger capital stock raises output in proportion to the capital share of GDP, {{v3_alpha}} (gross operating surplus over GDP per dollar of final demand), and the VAT base and remaining income tax rise with it. The capital stock proxy is national wealth less residential real estate, ${{v3_capital_t}}T; it includes land and government assets, so it overstates business capital and makes the spin-up larger.
- **Build-out.** Investment during the build-out is demand: {{v3_io_gdp}} of GDP and {{v3_io_jobs}} jobs per $1M of final demand, from Statistics Canada's input-output multipliers, Table 36-10-0013-01 [R31]. These values are **unverified**: the Statistics Canada site was down on the access date, and they were read from a search index's copy of the table. The capital share above comes from the same table. This is temporary, so the rule does not count it as structural. In the central case it peaks at ${{v3_peak_inv_central}}B of investment in the first year, supporting about {{v3_peak_jobs_central}}k jobs.

{{table:spinup}}

### 11.2 Precedents

- **Ireland.** Low corporate tax drew large booked profits. Ireland's statistics office publishes modified GNI (GNI*) because globalisation effects distort GDP; GNI* was {{ie_gni_share}}% of GDP [R32]. Under this design no CIT is levied, so booked profits would raise measured GDP but add no federal revenue beyond the VAT on domestic spending. The spin-up above counts only domestic capital.
- **Estonia.** Estonia taxes company profits only when distributed, at {{ee_rate}} of the net distribution [R33]. It is a middle option: it removes the tax on retained, reinvested profit while keeping a tax on payouts. It is not modelled; its revenue for Canada is **unverified**.

### 11.3 Levers that would close the gap

{{table:levers}}

The arithmetic, central case (each figure is in the model output and reproduced by `cargo run --release`):

- **Rate.** The VAT that closes the gap alone, with no income tax and the long-run base, is {{v3_rate_alone}}.
- **Base broadening.** Taxing the household GST exemptions and zero-rating [R3] at the rule rate: their cost at today's rate, times the rule rate over today's rate.
- **Spending review.** A cut the size of the Program Review: {{v3_pr_pct}}% of today's direct program expenses (${{v3_direct}}B [R2]) is ${{v3_pr_cut}}B. Direct program expenses include transfers to Indigenous Peoples and other programs; which items to cut is a political choice this paper does not make.
- **LVT.** The scenario in §5.4, ${{lvt_scenario}}B; the land value is **unverified**.

With all of these levers, the VAT that meets the need with no income tax is {{v3_pkg_rate_low}} (low), {{v3_pkg_rate_central}} (central) and {{v3_pkg_rate_high}} (high). Under the rule at {{start_rate}}%, the central case reaches full abolition {{v3_pkg_years_central}}. The low case {{v3_pkg_verdict_low}}: k stays at about {{v3_pkg_k_low}} (years to zero: {{v3_pkg_years_low}}). The levers take time to enact; the Program Review cuts were phased in over several budget years [R23].

---

## 12. The system as infrastructure

Ashlynn's framing is that the tax system is infrastructure. The test for infrastructure finance is that borrowing is matched by a measured return, not used to fund operating deficits. Finance Canada is moving to a capital budget that "distinguishes its day-to-day operational spending from expenditure that stimulates public and private sector capital formation" [R2].

Applied here: abolishing the income tax at once, on borrowed money, would cost the gap every year. The spin-up's extra revenue covers {{v3_cover_central}} of that cost in the central case and {{v3_cover_high}} in the high case (${{v3_spinup_gain_high}}B a year), before interest. The average rate on federal unmatured debt is about {{v3_interest}}% (public debt charges over unmatured debt [R2]). Borrowing for the tax cut therefore fails the infrastructure test: the debt would compound, on top of today's deficit of ${{v3_deficit}}B. The rule in §10 is the infrastructure version of the transition: each step is paid for by revenue already measured.

### 12.1 Counter-arguments, with who bears them

{{table:counter_v3}}

- **Regressivity and the credit.** A VAT takes a larger share of income from low-income households, though roughly the same share of their spending [R6, R6b]. Scaling the GST credit to {{start_rate}}% costs ${{credit_hi}}B a year; the model already nets it out of revenue. A universal prebate paid to every household would cost more; its cost is **unverified**.
- **Evasion and the VAT gap.** The CRA estimates the net GST/HST gap at {{v3_gap_net}}% and the gross gap at {{v3_gap_gross}}% of revenue [R34]. If either applied to the new VAT revenue, k at launch would be higher (table above), and the shortfall falls on compliant firms and buyers.
- **Provincial harmonization.** In HST provinces the federal and provincial rates share one base [R3]. A federal rate of {{start_rate}}% on top of provincial sales taxes needs agreements with each province, and Quebec administers its own QST. Not modelled.
- **Capital flight.** Removing CIT and PIT lowers the incentive to move capital out. The risk runs the other way: profit booked in Canada without real activity, as in Ireland. Non-resident income tax stays outside the core need and is kept.
- **Recession.** A base {{v3_drop}}% lower for a year raises that year's deficit by ${{v3_recession_def}}B in the central case. The rule pauses cuts, raises k, and repays the account in {{v3_repay}} years (table below). The cost falls on income-tax payers through the higher k, and on households, since the consumption base falls with spending.

{{table:recession}}

---

## 13. Limitations

- **Accounting model, not a general-equilibrium model.** The MPC, taxable share and multipliers are fixed. There are no wage, price-level, exchange-rate or interest-rate responses beyond the stated incidence assumption.
- **Incidence.** Headline figures assume full pass-through with the real basket held (A). If nominal spending does not rise with prices (B), revenue at {{start_rate}}% is about ${{ab_gap}}B {{b_vs_a}} and k {{k_b_dir}} from {{k_a}} to {{k_b}}. Reality is likely between the two; it is not estimated here.
- **Federal only.** Provincial PIT, CIT and sales taxes are excluded, as is the provincial share of HST. A full Canadian reform would need provincial agreement. The constitutional route to referendum entrenchment in Canada is **unverified**.
- **Property tax total** is derived from a per-person figure (Nova Scotia Finance, from StatCan CGFS) times the 1 July 2024 population; it covers provincial and local property taxes.
- **One base year** (FY2024-25), which includes the GST holiday (${{holiday_b}}B).
- **Tax-expenditure costs** are not the same as revenue gained, and they are 2024 projections [R3].
- **Multipliers** are short-run estimates from 2009 [R8], with leakage built in. Long-run CIT effects may be larger.
- **The land value total is unverified.**
- **Compliance leakage** at higher rates is a what-if, not an estimate. The growth addition is a what-if, not an estimate.
- **Distribution by decile** and the full dividend cost are **unverified**.
- **Transition financing.** The rule in §10 keeps each year's deficit at or below the baseline, so no transition debt is modelled; federal debt is {{debt_pct_gdp}}% of GDP [R2]. The rule's result depends on the spin-up inputs below.
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
- [R15] Zakat, Tax and Customs Authority (Saudi Arabia), "About VAT" ({{sa_rate_2018}}% from {{sa_date_2018}}). https://zatca.gov.sa/en/RulesRegulations/VAT/Pages/About-Vat.aspx
- [R15b] Ministry of Finance (Saudi Arabia), "Additional Measures to Confront the Financial and Economic Impact of the Coronavirus Pandemic", 11 May 2020 ("VAT shall increase from {{sa_rate_2018}}% to {{sa_rate_2020}}% as of {{sa_date_2020}}"). https://www.mof.gov.sa/en/MediaCenter/news/Pages/News_11052020.aspx
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
