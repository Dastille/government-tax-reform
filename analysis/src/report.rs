//! Renders every table the whitepaper quotes, as Markdown. Each `table_*` function returns
//! the table body (no heading) so the same text can be embedded in the paper and in
//! `output/revenue_tables.md`. Source text is formatted from `data.rs`, never typed in.

use crate::data::{
    CIT, EI_PREMIUMS, END_RATE, GROWTH_ADDITION_PP, GST_BEFORE_REMISSION, GST_CREDIT,
    GST_GROSS_EXTERNAL, GST_HOLIDAY_COST, GST_NET, GST_RATE, GST_REMISSION, HFCE_PCT_GDP,
    IMPORTS_PCT_GDP, LAND_VALUE_UNVERIFIED, LVT_CLAIM_HIGH, LVT_CLAIM_LOW, LVT_RATES,
    LVT_TAXABLE_SHARE_SCENARIO, MULTIPLIER_CIT, MULTIPLIER_PIT, NATIONAL_WEALTH_Q2_2025,
    NON_RESIDENT, PATH_YEARS, PIT, POPULATION_2024, PROPERTY_TAX_PER_CAPITA_2024,
    RESIDENTIAL_REAL_ESTATE_Q2_2025, REVENUE_PCT_GDP, SAVING_RATE_Q2_2024, SAVING_RATE_Q2_2025,
    START_RATE, TE_HOUSEHOLD, TE_PUBLIC_AND_THRESHOLD, TOTAL_REVENUE, VRR_CANADA, VRR_NEW_ZEALAND,
    VRR_OECD_AVERAGE,
};
use crate::docs::{bc, thousands};
use crate::error::Result;
use crate::model::{
    annualised, apc, base0, broadening_factor, gdp, hfce, income_taxes, lvt_land_needed,
    lvt_rate_to_replace_property_tax, lvt_revenue, lvt_scenario, need_core, need_full, path_rate,
    property_tax_total, taxable_share_central, te_sum, v11_path_rate, vrr_factor, Incidence,
    Scenario,
};
use std::fmt::Write;

const A: Incidence = Incidence::RealBaseFixed;
const B: Incidence = Incidence::NominalSpendingFixed;

/// 2023 federal PIT bracket rates; see [`crate::data::BRACKETS_2023`].
pub use crate::data::BRACKETS_2023;

/// Rates at which the model reports scenario tables, per cent.
const TABLE_RATES: [f64; 6] = [5.0, 8.0, 10.0, 15.0, 20.0, 25.0];

/// $M -> $B with one decimal.
#[must_use]
pub fn b(x: f64) -> String {
    format!("{:.1}", x / 1000.0)
}

/// Per-cent value with one decimal and a % sign.
#[must_use]
pub fn pct(x: f64) -> String {
    format!("{x:.1}%")
}

/// A rate written without trailing zeros (20.0 -> "20", 0.5 -> "0.5").
#[must_use]
pub fn num(x: f64) -> String {
    let s = format!("{x:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.to_string()
}

/// Bracket schedule as "15/20.5/26/29/33".
fn brackets_text() -> String {
    BRACKETS_2023
        .iter()
        .map(|x| num(*x))
        .collect::<Vec<_>>()
        .join("/")
}

/// Named scenarios under one incidence assumption, used by Table 5b and the tests.
#[must_use]
pub fn named_scenarios(inc: Incidence) -> Vec<(String, Scenario)> {
    let st = Scenario::static_case(inc);
    let d = Scenario::dynamic(inc);
    let half = Scenario { mpc: 0.5, ..d };
    let leak = Scenario { leak: 0.2, ..st };
    let list: Vec<(String, Scenario)> = vec![
        ("Static (no spending effect)".into(), st),
        (format!("Dynamic, MPC {:.2}", half.mpc), half),
        (format!("Dynamic, central (MPC {:.2})", d.mpc), d),
        (
            "Dynamic, every new dollar taxable".into(),
            Scenario {
                taxable_share: 1.0,
                ..d
            },
        ),
        (
            "Dynamic + multiplier".into(),
            Scenario {
                multiplier: true,
                ..d
            },
        ),
        (
            format!("Static, {}% compliance leakage", num(100.0 * leak.leak)),
            leak,
        ),
        (
            "Static, all exemptions and rebates taxed".into(),
            Scenario {
                base_factor: broadening_factor(
                    te_sum(&TE_HOUSEHOLD) + te_sum(&TE_PUBLIC_AND_THRESHOLD),
                ),
                ..st
            },
        ),
        (
            "Static, New Zealand VRR".into(),
            Scenario {
                base_factor: vrr_factor(VRR_NEW_ZEALAND),
                ..st
            },
        ),
    ];
    list
}

/// Explanation of the two incidence assumptions (used in the output file).
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn incidence_note() -> Result<String> {
    let mut s = String::new();
    writeln!(s, "**Incidence assumption.** Every scenario, static included, is computed under one stated assumption about prices and nominal spending. **A (central): real base fixed** - the VAT is passed into prices and households keep buying the same tax-exclusive basket, so revenue on today's base = rate x base. **B (sensitivity): nominal spending fixed** - prices rise but nominal spending on today's base does not, so revenue = rate/(1+rate) x spending. \"Dynamic\" differs from \"static\" only by the extra spending from the income tax people keep; that new spending buys rate-inclusive goods under both A and B.")?;
    Ok(s)
}

/// Table 1: sourced baseline.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_baseline() -> Result<String> {
    let mut w = String::new();
    let it = income_taxes();
    writeln!(w, "| Item | $B | Source |\n|---|---:|---|")?;
    let gdp_src = format!("total revenue / {REVENUE_PCT_GDP}% of GDP (AFR 2024-25, derived)");
    let rows: [(&str, f64, &str); 12] = [
        ("Personal income tax (PIT)", PIT, "PA 2025 Vol I Table 3.2"),
        ("Corporate income tax (CIT)", CIT, "PA 2025 Vol I Table 3.2"),
        ("PIT + CIT", it, "sum"),
        (
            "Non-resident income tax",
            NON_RESIDENT,
            "PA 2025 Vol I Table 3.2",
        ),
        ("EI premiums (payroll)", EI_PREMIUMS, "AFR 2024-25 Table 4"),
        (
            "GST before remission order",
            GST_BEFORE_REMISSION,
            "PA 2025 Vol I Table 3.3",
        ),
        (
            "Less remission order (ministries)",
            GST_REMISSION,
            "PA 2025 Vol I Table 3.3",
        ),
        (
            "Gross GST from outside parties",
            GST_GROSS_EXTERNAL,
            "PA 2025 Vol I Table 3.3",
        ),
        (
            "Less quarterly GST credits",
            GST_CREDIT,
            "PA 2025 Vol I Table 3.3",
        ),
        (
            "Net GST from outside parties",
            GST_NET,
            "PA 2025 Vol I Table 3.3",
        ),
        (
            "Total federal revenues",
            TOTAL_REVENUE,
            "AFR 2024-25 Table 4",
        ),
        ("Nominal GDP", gdp(), &gdp_src),
    ];
    for (k, v, src) in rows {
        writeln!(w, "| {k} | {} | {src} |", b(v))?;
    }
    writeln!(
        w,
        "\nEffective (net) base = net GST / {}% = {}. Gross base = gross GST / {}% = {}. PIT + CIT = {} of GDP. Core need (PIT + CIT + net GST) = **{}**; full federal scope (adds non-resident tax and EI premiums) = **{}**.",
        num(GST_RATE),
        b(base0()),
        num(GST_RATE),
        b(GST_GROSS_EXTERNAL / (GST_RATE / 100.0)),
        pct(100.0 * it / gdp()),
        b(need_core()),
        b(need_full())
    )?;
    Ok(w)
}

/// Table 2: static revenue on today's base under A and B.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_static() -> Result<String> {
    let mut w = String::new();
    let (st_a, st_b) = (Scenario::static_case(A), Scenario::static_case(B));
    let (core, it) = (need_core(), income_taxes());
    writeln!(w, "| VAT rate | Revenue (A) | Revenue (B) | Net new vs today's GST (A) | Share of PIT+CIT replaced (A) | Gap to core need (A) |\n|---:|---:|---:|---:|---:|---:|")?;
    for r in TABLE_RATES {
        let rev = st_a.revenue(r);
        writeln!(
            w,
            "| {}% | {} | {} | {} | {} | {} |",
            num(r),
            b(rev),
            b(st_b.revenue(r)),
            b(rev - GST_NET),
            pct(100.0 * (rev - GST_NET) / it),
            b(core - rev)
        )?;
    }
    Ok(w)
}

/// Table 3: static required rate under base broadening and VRR scenarios.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_broadening() -> Result<String> {
    let mut w = String::new();
    let (st_a, st_b) = (Scenario::static_case(A), Scenario::static_case(B));
    let (core, full) = (need_core(), need_full());
    let hh = te_sum(&TE_HOUSEHOLD);
    let pub_thr = te_sum(&TE_PUBLIC_AND_THRESHOLD);
    let groceries = TE_HOUSEHOLD[0].1;
    writeln!(w, "Tax-expenditure costs are calendar 2024 values at the {}% rate (Report on Federal Tax Expenditures 2026, Part 2). Finance Canada notes that removing a measure would not yield its full cost.\n", num(GST_RATE))?;
    writeln!(w, "| Base | Extra base, $B at {}% | Core need (A) | Core need (B) | Full scope (A) |\n|---|---:|---:|---:|---:|", num(GST_RATE))?;
    let oecd = format!("OECD-average VAT revenue ratio ({VRR_OECD_AVERAGE:.2})");
    let nz = format!("New Zealand VAT revenue ratio ({VRR_NEW_ZEALAND:.2})");
    let all = format!(
        "Tax all {} household zero-ratings/exemptions",
        TE_HOUSEHOLD.len()
    );
    let rows: [(&str, String, f64); 7] = [
        ("Today's base (FY2024-25)", b(0.0), 1.0),
        (
            "Add back the Dec 2024-Feb 2025 GST holiday",
            b(GST_HOLIDAY_COST),
            broadening_factor(GST_HOLIDAY_COST),
        ),
        (
            "Tax basic groceries",
            b(groceries),
            broadening_factor(groceries),
        ),
        (&all, b(hh), broadening_factor(hh)),
        (
            "Households + public-body rebates + small-supplier threshold",
            b(hh + pub_thr),
            broadening_factor(hh + pub_thr),
        ),
        (&oecd, "n/a".into(), vrr_factor(VRR_OECD_AVERAGE)),
        (&nz, "n/a".into(), vrr_factor(VRR_NEW_ZEALAND)),
    ];
    for (k, extra, f) in rows {
        let sa = Scenario {
            base_factor: f,
            ..st_a
        };
        let sb = Scenario {
            base_factor: f,
            ..st_b
        };
        writeln!(
            w,
            "| {k} | {extra} | {} | {} | {} |",
            pct(sa.required_rate(core)),
            pct(sb.required_rate(core)),
            pct(sa.required_rate(full))
        )?;
    }
    Ok(w)
}

/// Compliance leakage what-ifs.
const LEAKS: [f64; 4] = [0.0, 0.1, 0.2, 0.3];

/// Table 4: compliance leakage.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_leakage() -> Result<String> {
    let mut w = String::new();
    let (st_a, st_b) = (Scenario::static_case(A), Scenario::static_case(B));
    let core = need_core();
    writeln!(w, "Leakage = share of revenue above today's net GST lost to evasion or avoidance at higher rates. No Canadian estimate is used; these are what-ifs.\n")?;
    writeln!(w, "| Leakage | Revenue at {r}% (A) | Rate for core need (A) | Revenue at {r}% (B) | Rate for core need (B) |\n|---:|---:|---:|---:|---:|", r = num(START_RATE))?;
    for l in LEAKS {
        let sa = Scenario { leak: l, ..st_a };
        let sb = Scenario { leak: l, ..st_b };
        writeln!(
            w,
            "| {}% | {} | {} | {} | {} |",
            num(l * 100.0),
            b(sa.revenue(START_RATE)),
            pct(sa.required_rate(core)),
            b(sb.revenue(START_RATE)),
            pct(sb.required_rate(core))
        )?;
    }
    Ok(w)
}

/// MPC values swept in Table 5.
fn mpc_sweep() -> [f64; 8] {
    [
        0.0,
        0.50,
        0.60,
        0.70,
        0.80,
        0.90,
        apc(SAVING_RATE_Q2_2024),
        apc(SAVING_RATE_Q2_2025),
    ]
}

/// Table 5: dynamic scenario, MPC sweep.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_dynamic() -> Result<String> {
    let mut w = String::new();
    let (c, cb) = (Scenario::central(), Scenario::dynamic(B));
    let core = need_core();
    let r = START_RATE;
    writeln!(w, "All {} of PIT + CIT stays with households and firms. A share (MPC) is spent; a share of that spending (taxable share = {:.2}) lands in the VAT base and pays rate-inclusive prices. Imports ({IMPORTS_PCT_GDP}% of GDP) do not leak from a destination-based base; they matter only for the multiplier. \"+ mult\" adds Finance Canada's short-run multipliers (PIT {MULTIPLIER_PIT:.1}, CIT {MULTIPLIER_CIT:.1}). MPC 0.000 is the static case.\n", b(income_taxes()), taxable_share_central())?;
    writeln!(w, "| MPC | Rev. at {r}% (A) | Rate for core need (A) | Rev., share = 1 (A) | Rate (A) | Rev., + mult (A) | Rate (A) | Rev. at {r}% (B) | Rate for core need (B) |\n|---:|---:|---:|---:|---:|---:|---:|---:|---:|", r = num(r))?;
    for m in mpc_sweep() {
        let d1 = Scenario { mpc: m, ..c };
        let d2 = Scenario {
            mpc: m,
            taxable_share: 1.0,
            ..c
        };
        let d3 = Scenario {
            mpc: m,
            multiplier: true,
            ..c
        };
        let d4 = Scenario { mpc: m, ..cb };
        writeln!(
            w,
            "| {m:.3} | {} | {} | {} | {} | {} | {} | {} | {} |",
            b(d1.revenue(r)),
            pct(d1.required_rate(core)),
            b(d2.revenue(r)),
            pct(d2.required_rate(core)),
            b(d3.revenue(r)),
            pct(d3.required_rate(core)),
            b(d4.revenue(r)),
            pct(d4.required_rate(core))
        )?;
    }
    writeln!(
        w,
        "\nMPC rows {:.3} and {:.3} are 1 minus the household saving rate in Q2 2024 ({}) and Q2 2025 ({}); an average propensity is an upper bound for spending out of a tax cut. Real taxable consumption at {}%, central MPC: {} (A), {} (B), versus {} today.",
        apc(SAVING_RATE_Q2_2024),
        apc(SAVING_RATE_Q2_2025),
        pct(100.0 * SAVING_RATE_Q2_2024),
        pct(100.0 * SAVING_RATE_Q2_2025),
        num(r),
        b(c.real_base(r)),
        b(cb.real_base(r)),
        b(base0())
    )?;
    Ok(w)
}

/// Table 5b: every scenario under both incidence assumptions.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_sensitivity() -> Result<String> {
    let mut w = String::new();
    let core = need_core();
    let r = START_RATE;
    writeln!(w, "Within each assumption, higher revenue at a given rate always means a lower or equal required rate (tested). Comparisons across A and B are not like-for-like: B's revenue curve flattens as the rate rises.\n")?;
    writeln!(w, "| Scenario | Rev. at {r}% (A) | Rate for core need (A) | Rev. at {r}% (B) | Rate for core need (B) |\n|---|---:|---:|---:|---:|", r = num(r))?;
    for ((k, sa), (_, sb)) in named_scenarios(A).into_iter().zip(named_scenarios(B)) {
        writeln!(
            w,
            "| {k} | {} | {} | {} | {} |",
            b(sa.revenue(r)),
            pct(sa.required_rate(core)),
            b(sb.revenue(r)),
            pct(sb.required_rate(core))
        )?;
    }
    Ok(w)
}

/// Table 6: break-even conditions.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_breakeven() -> Result<String> {
    let mut w = String::new();
    let (c, cb) = (Scenario::central(), Scenario::dynamic(B));
    let mult = Scenario {
        multiplier: true,
        ..c
    };
    let core = need_core();
    let (hi, lo) = (START_RATE, END_RATE);
    writeln!(
        w,
        "| Condition | At {}% | At {}% |\n|---|---:|---:|",
        num(hi),
        num(lo)
    )?;
    let rows: [(&str, Scenario); 3] = [
        ("central share (A)", c),
        (
            "every new dollar taxable (A)",
            Scenario {
                taxable_share: 1.0,
                ..c
            },
        ),
        ("central share (B)", cb),
    ];
    for (k, d) in rows {
        writeln!(
            w,
            "| MPC needed, {k} (max possible 1.0) | {:.2} | {:.2} |",
            d.breakeven_mpc(hi, core),
            d.breakeven_mpc(lo, core)
        )?;
    }
    for (k, d) in [
        ("central (A)", c),
        ("central + multiplier (A)", mult),
        ("central (B)", cb),
    ] {
        let (g_hi, g_lo) = (
            d.breakeven_growth_factor(hi, core),
            d.breakeven_growth_factor(lo, core),
        );
        writeln!(
            w,
            "| Base growth beyond needs, {k}: cumulative | x{g_hi:.2} | x{g_lo:.2} |"
        )?;
        writeln!(
            w,
            "| ... same, per year over {PATH_YEARS} years | {} | {} |",
            pct(100.0 * annualised(g_hi, PATH_YEARS)),
            pct(100.0 * annualised(g_lo, PATH_YEARS))
        )?;
    }
    for g in GROWTH_ADDITION_PP {
        let f = growth_factor(g, PATH_YEARS);
        writeln!(w, "| What-if growth addition {} pp/yr (unsourced) gives, after {PATH_YEARS} years | x{f:.2} | x{f:.2} |", num(g))?;
    }
    Ok(w)
}

/// Cumulative factor from `pp` percentage points a year over `years`.
fn growth_factor(pp: f64, years: u32) -> f64 {
    (1.0 + pp / 100.0).powf(f64::from(years))
}

/// Table 7: the rate path.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_path() -> Result<String> {
    let mut w = String::new();
    let (c, cb) = (Scenario::central(), Scenario::dynamic(B));
    let core = need_core();
    let step = (START_RATE - END_RATE) / f64::from(PATH_YEARS);
    let [g1, g2] = GROWTH_ADDITION_PP;
    writeln!(w, "rate(t) = {} - {} t, t = years since launch (0..{PATH_YEARS}). The v1.1 formula `20 - 1.2 x (year - 1)` for years 1-{PATH_YEARS} reaches {}% in year {PATH_YEARS}. Coverage = VAT revenue / core need, all income tax abolished, dynamic central. Growth columns are what-ifs (unsourced): the base grows that much faster than the revenue requirement every year.\n", num(START_RATE), num(step), num(v11_path_rate(PATH_YEARS)))?;
    writeln!(w, "| t | Rate | Revenue, no growth (A) | Coverage (A) | Coverage, +{} pp/yr (A) | Coverage, +{} pp/yr (A) | Coverage (B) |\n|---:|---:|---:|---:|---:|---:|---:|", num(g1), num(g2))?;
    for t in 0..=PATH_YEARS {
        let r = path_rate(t);
        let s1 = Scenario {
            base_factor: growth_factor(g1, t),
            ..c
        };
        let s2 = Scenario {
            base_factor: growth_factor(g2, t),
            ..c
        };
        writeln!(
            w,
            "| {t} | {r:.1}% | {} | {} | {} | {} | {} |",
            b(c.revenue(r)),
            pct(100.0 * c.revenue(r) / core),
            pct(100.0 * s1.revenue(r) / core),
            pct(100.0 * s2.revenue(r) / core),
            pct(100.0 * cb.revenue(r) / core)
        )?;
    }
    Ok(w)
}

/// Table 8: land value tax.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_lvt() -> Result<String> {
    let mut w = String::new();
    writeln!(
        w,
        "Taxable (unimproved) land value needed for the original ${}-{}B a year ($B of land):\n",
        b(LVT_CLAIM_LOW),
        b(LVT_CLAIM_HIGH)
    )?;
    writeln!(
        w,
        "| LVT rate | For ${}B | For ${}B |\n|---:|---:|---:|",
        b(LVT_CLAIM_LOW),
        b(LVT_CLAIM_HIGH)
    )?;
    for r in LVT_RATES {
        writeln!(
            w,
            "| {}% | {} | {} |",
            num(r),
            bc(lvt_land_needed(LVT_CLAIM_LOW, r)),
            bc(lvt_land_needed(LVT_CLAIM_HIGH, r))
        )?;
    }
    writeln!(w, "\nFor scale (verified): national wealth, all non-financial assets, Q2 2025 = ${}B; household residential real estate, land plus buildings, Q2 2025 = ${}B. Revenue ($B a year) if total land value is ${}B (**unverified**) and only part of it is taxable:\n", bc(NATIONAL_WEALTH_Q2_2025), bc(RESIDENTIAL_REAL_ESTATE_Q2_2025), bc(LAND_VALUE_UNVERIFIED))?;
    writeln!(
        w,
        "| Taxable share of land | At {}% | At {}% |\n|---:|---:|---:|",
        num(LVT_RATES[0]),
        num(LVT_RATES[1])
    )?;
    for sh in [0.10, 0.25, 0.50, 1.00] {
        let l = LAND_VALUE_UNVERIFIED * sh;
        writeln!(
            w,
            "| {}% | {} | {} |",
            num(sh * 100.0),
            b(lvt_revenue(LVT_RATES[0], l)),
            b(lvt_revenue(LVT_RATES[1], l))
        )?;
    }
    Ok(w)
}

/// Table 8b: recurrent property tax the LVT must replace first.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_property() -> Result<String> {
    let mut w = String::new();
    let scenario_land = LAND_VALUE_UNVERIFIED * LVT_TAXABLE_SHARE_SCENARIO;
    writeln!(w, "| Quantity | Value | From |\n|---|---:|---|")?;
    writeln!(w, "| Property tax per person, provincial + local, 2024 | ${} | StatCan CGFS Table 10-10-0147-01, via Nova Scotia Finance |", thousands(&thousands(&num(PROPERTY_TAX_PER_CAPITA_2024))))?;
    writeln!(
        w,
        "| Population, 1 July 2024 | {} | StatCan Daily 2024-09-25 |",
        thousands(&num(POPULATION_2024))
    )?;
    writeln!(
        w,
        "| Recurrent property tax revenue, $B | {} | per person x population (derived) |",
        b(property_tax_total())
    )?;
    writeln!(
        w,
        "| LVT scenario revenue, $B | {} | {}% on {}% of land (land value unverified) |",
        b(lvt_scenario()),
        num(LVT_RATES[1]),
        num(100.0 * LVT_TAXABLE_SHARE_SCENARIO)
    )?;
    writeln!(w, "| LVT rate needed to replace property tax, scenario land | {} | property tax / ({}% of land) |", pct(lvt_rate_to_replace_property_tax(scenario_land)), num(100.0 * LVT_TAXABLE_SHARE_SCENARIO))?;
    writeln!(
        w,
        "| LVT rate needed to replace property tax, all land | {} | property tax / all land |",
        pct(lvt_rate_to_replace_property_tax(LAND_VALUE_UNVERIFIED))
    )?;
    Ok(w)
}

/// Table 9: what would make the end rate reach the core need.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_stack() -> Result<String> {
    let mut w = String::new();
    let core = need_core();
    let r = END_RATE;
    let lvt = lvt_scenario();
    let broaden = broadening_factor(te_sum(&TE_HOUSEHOLD) + te_sum(&TE_PUBLIC_AND_THRESHOLD));
    let nz = vrr_factor(VRR_NEW_ZEALAND);
    writeln!(w, "Each row stacks one more lever on the dynamic central case. The LVT row assumes none of the LVT is needed to replace property taxes (optimistic; see Table 8b).\n")?;
    writeln!(w, "| Stack | Revenue at {r}%, no growth (A) | LVT | Base growth beyond needs over {PATH_YEARS} yrs (A) | Per year (A) | Over {PATH_YEARS} yrs (B) | Per year (B) |\n|---|---:|---:|---:|---:|---:|---:|", r = num(r))?;
    let stack = |inc: Incidence| -> Vec<(&'static str, Scenario, f64)> {
        let d = Scenario::dynamic(inc);
        let m = Scenario {
            multiplier: true,
            ..d
        };
        vec![
            ("Dynamic central (her mechanism)", d, 0.0),
            ("+ Finance Canada multiplier", m, 0.0),
            ("+ LVT", m, lvt),
            (
                "+ tax every exemption and rebate in Table 3",
                Scenario {
                    base_factor: broaden,
                    ..m
                },
                lvt,
            ),
            (
                "+ New Zealand-level base and compliance instead",
                Scenario {
                    base_factor: nz,
                    ..m
                },
                lvt,
            ),
        ]
    };
    for ((k, da, l), (_, db, _)) in stack(A).into_iter().zip(stack(B)) {
        let ga = (core - l) / da.revenue(r);
        let gb = (core - l) / db.revenue(r);
        writeln!(
            w,
            "| {k} | {} | {} | x{ga:.2} | {} | x{gb:.2} | {} |",
            b(da.revenue(r)),
            b(l),
            pct(100.0 * annualised(ga.max(1.0), PATH_YEARS)),
            pct(100.0 * annualised(gb.max(1.0), PATH_YEARS))
        )?;
    }
    Ok(w)
}

/// Table 10: share of income tax that must remain (k) and scaled bracket rates.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_k() -> Result<String> {
    let mut w = String::new();
    let (c, cb) = (Scenario::central(), Scenario::dynamic(B));
    let (core, it) = (need_core(), income_taxes());
    writeln!(w, "Dynamic central case, no growth, no LVT. k = share of today's PIT + CIT still needed; the removed share (1 - k) drives the extra spending. Bracket rates scale the 2023 federal schedule ({}%) by k.\n", brackets_text())?;
    writeln!(w, "| VAT rate | VAT revenue (A) | k (A) | Kept, $B (A) | Federal bracket rates (A) | k (B) |\n|---:|---:|---:|---:|---|---:|")?;
    for r in [10.0, 15.0, 20.0, 25.0, 30.0] {
        let k = c.income_tax_kept(r, core);
        let rev = c.vat_at(r, 1.0 - k);
        let br: Vec<String> = BRACKETS_2023
            .iter()
            .map(|x| format!("{:.1}", x * k))
            .collect();
        writeln!(
            w,
            "| {}% | {} | {} | {} | {} | {} |",
            num(r),
            b(rev),
            pct(100.0 * k),
            b(k * it),
            br.join(" / "),
            pct(100.0 * cb.income_tax_kept(r, core))
        )?;
    }
    Ok(w)
}

/// Table 11: cost of protecting low-income households.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_protect() -> Result<String> {
    let mut w = String::new();
    let groceries = TE_HOUSEHOLD[0].1;
    writeln!(w, "Static scaling (A) of two existing measures: the GST credit (PA 2025 Table 3.3) and zero-rating of basic groceries (Tax Expenditures 2026).\n")?;
    writeln!(
        w,
        "| VAT rate | GST-credit equivalent | Cost of zero-rating groceries |\n|---:|---:|---:|"
    )?;
    for r in [5.0, 10.0, 15.0, 20.0, 25.0] {
        writeln!(
            w,
            "| {}% | {} | {} |",
            num(r),
            b(GST_CREDIT * r / GST_RATE),
            b(groceries * r / GST_RATE)
        )?;
    }
    Ok(w)
}

/// Table 0: chain from raw official quantities to the headline.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_first_principles() -> Result<String> {
    let c = Scenario::central();
    let it = income_taxes();
    let new_spend = c.extra_spending(1.0);
    let r = START_RATE;
    let tau = r / 100.0;
    let k = c.income_tax_kept(r, need_core());
    let mut w = String::new();
    writeln!(w, "| Step | Quantity | Value | From |\n|---:|---|---:|---|")?;
    // Step numbers referenced in the "From" column, so the chain text cannot drift.
    let (hfce_s, base_s, share_s, it_s, new_s, extra_s, need_s) = (2, 3, 4, 6, 7, 8, 10);
    let rows: Vec<(String, String, String)> = vec![
        ("GDP, $B".into(), b(gdp()), format!("total revenue {} / revenue share of GDP {REVENUE_PCT_GDP}% (AFR 2024-25)", b(TOTAL_REVENUE))),
        ("Household final consumption (HFCE), $B".into(), b(hfce()), format!("{HFCE_PCT_GDP}% x GDP (World Bank WDI 4.8)")),
        ("Effective taxed base, $B".into(), b(base0()), format!("net GST {} / statutory rate {}% (PA 2025, Table 3.3)", b(GST_NET), num(GST_RATE))),
        ("Coverage-and-compliance ratio".into(), format!("{:.3}", taxable_share_central()), format!("step {base_s} / step {hfce_s} (measured; OECD VRR for Canada, on a wider denominator: {VRR_CANADA:.2})")),
        (format!("Static VAT revenue at {}%, $B", num(r)), b(Scenario::static_case(A).revenue(r)), format!("step {hfce_s} x step {share_s} x {}% (prices rise by the tax; same real basket)", num(r))),
        ("Income tax kept by households and firms, $B".into(), b(it), format!("PIT {} + CIT {} (PA 2025, Table 3.2)", b(PIT), b(CIT))),
        ("New taxable spending, $B".into(), b(new_spend), format!("MPC {:.2} (1 - saving rate {}, StatCan) x step {share_s} x step {it_s}", c.mpc, pct(100.0 * SAVING_RATE_Q2_2025))),
        (format!("Extra real base at {}%, $B", num(r)), b(new_spend / (1.0 + tau)), format!("step {new_s} / {:.2} (new spending pays the VAT-inclusive price)", 1.0 + tau)),
        (format!("Dynamic VAT revenue at {}%, $B", num(r)), b(c.revenue(r)), format!("{}% x (step {base_s} + step {extra_s})", num(r))),
        ("Core need, $B".into(), b(need_core()), "PIT + CIT + net GST".into()),
        ("Rate that meets the core need".into(), pct(c.required_rate(need_core())), format!("solves rate x (step {base_s} + step {new_s} / (1 + rate)) = step {need_s}")),
        (format!("Share of income tax still needed at {}% (k)", num(r)), pct(100.0 * k), format!("solves VAT({}%, 1-k) + k x step {it_s} = step {need_s}", num(r))),
        ("LVT scenario, $B".into(), b(lvt_scenario()), format!("land {} (unverified) x {}% taxable x {}%", b(LAND_VALUE_UNVERIFIED), num(100.0 * LVT_TAXABLE_SHARE_SCENARIO), num(LVT_RATES[1]))),
        ("Recurrent property tax to replace first, $B".into(), b(property_tax_total()), format!("${} per person x {} people (StatCan)", thousands(&num(PROPERTY_TAX_PER_CAPITA_2024)), thousands(&num(POPULATION_2024)))),
    ];
    debug_assert!(rows[hfce_s - 1]
        .0
        .starts_with("Household final consumption"));
    debug_assert!(rows[base_s - 1].0.starts_with("Effective taxed base"));
    debug_assert!(rows[share_s - 1].0.starts_with("Coverage"));
    debug_assert!(rows[it_s - 1].0.starts_with("Income tax kept"));
    debug_assert!(rows[new_s - 1].0.starts_with("New taxable spending"));
    debug_assert!(rows[extra_s - 1].0.starts_with("Extra real base"));
    debug_assert!(rows[need_s - 1].0.starts_with("Core need"));
    for (i, (q, v, f)) in rows.into_iter().enumerate() {
        writeln!(w, "| {} | {q} | {v} | {f} |", i + 1)?;
    }
    Ok(w)
}

/// A named table: (key or title, function that renders its body).
pub type Section = (&'static str, fn() -> Result<String>);

/// The full model output (`output/revenue_tables.md`).
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn render() -> Result<String> {
    let mut s = String::new();
    writeln!(
        s,
        "<!-- Generated by `cargo run --release` in analysis/. Do not edit by hand. -->"
    )?;
    writeln!(
        s,
        "# Revenue model output (Canada, FY2024-25 worked example)\n"
    )?;
    writeln!(
        s,
        "All amounts are $ billions, federal government, fiscal year April 2024 to March 2025.\n"
    )?;
    writeln!(s, "{}", incidence_note()?)?;
    let sections: [Section; 15] = [
        (
            "Table 0. First-principles derivation (incidence A)",
            table_first_principles,
        ),
        ("Table 1. Baseline (sourced)", table_baseline),
        (
            "Table 2. Static: today's GST base, no spending effect",
            table_static,
        ),
        (
            "Table 3. Rate needed (static) under base broadening",
            table_broadening,
        ),
        (
            "Table 4. Compliance leakage (assumption sweep, static)",
            table_leakage,
        ),
        (
            "Table 5. Dynamic scenario: households keep the income tax and spend it",
            table_dynamic,
        ),
        (
            "Table 5b. Incidence sensitivity for every scenario",
            table_sensitivity,
        ),
        (
            "Table 6. Break-even conditions (dynamic, central values unless stated)",
            table_breakeven,
        ),
        ("Table 7. The rate path, made consistent", table_path),
        ("Table 8. Land value tax", table_lvt),
        (
            "Table 8b. Recurrent property tax the LVT must replace first",
            table_property,
        ),
        (
            "Table 9. What would make the end rate reach the core need",
            table_stack,
        ),
        (
            "Table 10. Income tax that must remain while the VAT phases in",
            table_k,
        ),
        (
            "Table 11. Cost of protecting low-income households",
            table_protect,
        ),
        ("Key figures", crate::docs::key_figures),
    ];
    for (title, f) in sections {
        writeln!(s, "## {title}\n\n{}", f()?)?;
    }
    Ok(s)
}
