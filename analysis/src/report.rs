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
    let sections: [Section; 25] = [
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
        (
            "Table 12. Debt-neutral transition: three spin-up cases",
            table_transition,
        ),
        (
            "Table 13. Central case under the rule, year by year",
            table_transition_path,
        ),
        (
            "Table 14. Recession stress test (central case)",
            table_recession,
        ),
        (
            "Table 15. Industry spin-up inputs and build-out",
            table_spinup,
        ),
        ("Table 16. Levers for the gap left with k = 0", table_levers),
        ("Table 17. Counter-arguments, measured", table_counter_v3),
        (
            "Table 18. End state: three balanced packages",
            table_endstate,
        ),
        (
            "Table 19. Who gains and who pays (federal, single filers, package B)",
            table_distribution,
        ),
        ("Table 20. Other revenue options", table_options),
        (
            "Table 21. Counter-arguments to each package",
            table_counter_endstate,
        ),
        ("Key figures", key_figures),
    ];
    for (title, f) in sections {
        writeln!(s, "## {title}\n\n{}", f()?)?;
    }
    Ok(s)
}

/// Headline numbers block.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn key_figures() -> Result<String> {
    let a = Incidence::RealBaseFixed;
    let bb = Incidence::NominalSpendingFixed;
    let st = Scenario::static_case(a);
    let c = Scenario::central();
    let m = Scenario {
        multiplier: true,
        ..c
    };
    let stb = Scenario::static_case(bb);
    let cb = Scenario::dynamic(bb);
    let nz = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..st
    };
    let core = need_core();
    let (hi, lo) = (START_RATE, END_RATE);
    let mut w = String::new();
    writeln!(
        w,
        "| Key figure (Canada FY2024-25, federal; incidence A unless stated) | Value |\n|---|---:|"
    )?;
    writeln!(w, "| Core need (PIT + CIT + net GST), $B | {} |", b(core))?;
    writeln!(w, "| VAT revenue at {}%: static / dynamic central (MPC {:.2}) / dynamic + multiplier, $B | {} / {} / {} |", num(hi), c.mpc, b(st.revenue(hi)), b(c.revenue(hi)), b(m.revenue(hi)))?;
    writeln!(
        w,
        "| Rate for core need: static / dynamic central / dynamic + multiplier | {} / {} / {} |",
        pct(st.required_rate(core)),
        pct(c.required_rate(core)),
        pct(m.required_rate(core))
    )?;
    writeln!(
        w,
        "| Income tax still needed at {}%, dynamic central (k) | {} |",
        num(hi),
        pct(100.0 * c.income_tax_kept(hi, core))
    )?;
    writeln!(
        w,
        "| MPC needed to close the gap at {}% (maximum possible 1.0) | {:.2} |",
        num(hi),
        c.breakeven_mpc(hi, core)
    )?;
    writeln!(
        w,
        "| Base growth beyond needs to abolish income tax at {}% / at {}% | x{:.2} / x{:.2} |",
        num(hi),
        num(lo),
        c.breakeven_growth_factor(hi, core),
        c.breakeven_growth_factor(lo, core)
    )?;
    writeln!(w, "| Rate for core need with a New Zealand-level base (VRR {VRR_NEW_ZEALAND:.2}), static | {} |", pct(nz.required_rate(core)))?;
    writeln!(
        w,
        "| Sensitivity B: revenue at {}% static / dynamic, $B | {} / {} |",
        num(hi),
        b(stb.revenue(hi)),
        b(cb.revenue(hi))
    )?;
    writeln!(
        w,
        "| Sensitivity B: rate for core need static / dynamic; k at {}% | {} / {}; {} |",
        num(hi),
        pct(stb.required_rate(core)),
        pct(cb.required_rate(core)),
        pct(100.0 * cb.income_tax_kept(hi, core))
    )?;
    writeln!(
        w,
        "| Recurrent property tax the LVT must replace first, $B | {} |",
        b(property_tax_total())
    )?;
    Ok(w)
}

/// README summary table.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn readme_table() -> Result<String> {
    let a = Incidence::RealBaseFixed;
    let bb = Incidence::NominalSpendingFixed;
    let (st, stb) = (Scenario::static_case(a), Scenario::static_case(bb));
    let (c, cb) = (Scenario::central(), Scenario::dynamic(bb));
    let core = need_core();
    let hi = START_RATE;
    let mut w = String::new();
    writeln!(w, "| | A | B | Source |\n|---|---:|---:|---|")?;
    writeln!(
        w,
        "| Personal income tax | {} | | Public Accounts 2025, Vol I, Table 3.2 |",
        b(PIT)
    )?;
    writeln!(
        w,
        "| Corporate income tax | {} | | Public Accounts 2025, Vol I, Table 3.2 |",
        b(CIT)
    )?;
    writeln!(
        w,
        "| Net GST ({}%) | {} | | Public Accounts 2025, Vol I, Table 3.3 |",
        num(GST_RATE),
        b(GST_NET)
    )?;
    writeln!(
        w,
        "| **Core need** (PIT + CIT + GST) | **{}** | | sum |",
        b(core)
    )?;
    writeln!(
        w,
        "| VAT at {}%, static | {} | {} | model |",
        num(hi),
        b(st.revenue(hi)),
        b(stb.revenue(hi))
    )?;
    writeln!(
        w,
        "| VAT at {}%, households spend the income tax they keep (MPC {:.2}) | {} | {} | model |",
        num(hi),
        c.mpc,
        b(c.revenue(hi)),
        b(cb.revenue(hi))
    )?;
    writeln!(
        w,
        "| Rate needed for the core need, static / dynamic | {} / {} | {} / {} | model |",
        pct(st.required_rate(core)),
        pct(c.required_rate(core)),
        pct(stb.required_rate(core)),
        pct(cb.required_rate(core))
    )?;
    writeln!(
        w,
        "| Share of income tax that must remain at {}% (k) | {} | {} | model |",
        num(hi),
        pct(100.0 * c.income_tax_kept(hi, core)),
        pct(100.0 * cb.income_tax_kept(hi, core))
    )?;
    writeln!(
        w,
        "| Recurrent property tax the LVT must replace first | {} | | StatCan CGFS (derived) |",
        b(property_tax_total())
    )?;
    Ok(w)
}

// ---------------------------------------------------------------------------
// v3: debt-neutral transition, industry spin-up, infrastructure (paper sections 10-12).
// ---------------------------------------------------------------------------

/// Like [`b`], but values that round to zero print as `0.0` (no `-0.0`).
#[must_use]
pub fn bz(x: f64) -> String {
    if x.abs() < 50.0 {
        "0.0".to_string()
    } else {
        b(x)
    }
}

/// Per-cent value with two decimals and a % sign.
#[must_use]
pub fn p2(x: f64) -> String {
    format!("{x:.2}%")
}

/// Years to `k = 0` under the rule within the horizon, or "never".
#[must_use]
pub fn years_text(case: &crate::transition::Case) -> String {
    match crate::transition::years_to_abolition(case) {
        Some(y) => format!("{y}"),
        None => "never".to_string(),
    }
}

/// "immediately once the levers are in force", "after N years", or "never".
#[must_use]
pub fn years_phrase(case: &crate::transition::Case) -> String {
    match crate::transition::years_to_abolition(case) {
        Some(0) => "immediately once the levers are in force".to_string(),
        Some(y) => format!("after {y} years"),
        None => "never".to_string(),
    }
}

/// Table 12: the three spin-up cases under the debt-neutral rule.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_transition() -> Result<String> {
    use crate::transition::{levers, simulate, Case};
    let mut w = String::new();
    writeln!(w, "| Case | Spending effect | Capital per point of METR cut | METR cut, points | Speed per year | k at launch | k after {} years | k settles at | Capital stock, long run | Years to k = 0 (within {}) | Gap left with k = 0, $B |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|", PATH_YEARS, crate::data::HORIZON_YEARS)?;
    for c in Case::all() {
        let s = simulate(&c, crate::data::HORIZON_YEARS, None);
        let spend = if c.spending.multiplier {
            format!("MPC {:.2} + multipliers", c.spending.mpc)
        } else {
            format!("MPC {:.2}", c.spending.mpc)
        };
        writeln!(
            w,
            "| {} | {} | {}% | {} | {} | {} | {} | {} | +{} | {} | {} |",
            c.name,
            spend,
            num(c.semi_elasticity),
            num(c.metr_cut),
            num(c.speed),
            pct(100.0 * c.launch_k()),
            pct(100.0 * s[PATH_YEARS as usize].k),
            pct(100.0 * c.settled_k()),
            pct(100.0 * c.capital_target(c.settled_k())),
            years_text(&c),
            b(levers(&c).gap)
        )?;
    }
    Ok(w)
}

/// Table 13: central case, year by year.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_transition_path() -> Result<String> {
    use crate::transition::{simulate, Case};
    let c = Case::all()[1];
    let s = simulate(&c, crate::data::HORIZON_YEARS, None);
    let mut w = String::new();
    writeln!(w, "| Year | k | Capital stock vs launch | Build-out investment, $B | Revenue, $B | Structural revenue, $B | Deficit change vs baseline, $B |\n|---:|---:|---:|---:|---:|---:|---:|")?;
    for t in [0_usize, 1, 2, 3, 5, 10, 20, 50] {
        let y = s[t];
        writeln!(
            w,
            "| {} | {} | +{} | {} | {} | {} | {} |",
            y.t,
            p2(100.0 * y.k),
            p2(100.0 * y.capital),
            b(y.investment),
            b(y.revenue),
            b(y.structural_revenue),
            bz(y.deficit_change)
        )?;
    }
    Ok(w)
}

/// Table 14: recession stress test on the central case.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_recession() -> Result<String> {
    use crate::data::{RECESSION_BASE_DROP_PCT, RECESSION_YEAR};
    use crate::transition::{simulate, Case};
    let c = Case::all()[1];
    let s = simulate(
        &c,
        crate::data::HORIZON_YEARS,
        Some((RECESSION_YEAR, RECESSION_BASE_DROP_PCT)),
    );
    let mut w = String::new();
    writeln!(w, "| Year | k | Revenue, $B | Deficit change vs baseline, $B | Compensation account owed, $B | Rule action for next year |\n|---:|---:|---:|---:|---:|---|")?;
    let first = RECESSION_YEAR as usize - 1;
    for y in &s[first..first + 6] {
        let next = s[y.t as usize + 1].k;
        let action = if next > y.k + 1e-12 {
            "reverse: raise k"
        } else if next < y.k - 1e-12 {
            "cut k"
        } else {
            "pause"
        };
        writeln!(
            w,
            "| {} | {} | {} | {} | {} | {} |",
            y.t,
            p2(100.0 * y.k),
            b(y.revenue),
            b(y.deficit_change),
            b(y.owed),
            action
        )?;
    }
    Ok(w)
}

/// Table 15: input-output inputs and the build-out each case implies.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_spinup() -> Result<String> {
    use crate::data::{IO_GDP_2022, IO_GOS_2022, IO_IMPORTS_2022, IO_JOBS_2022, IO_OUTPUT_2022};
    use crate::transition::{borrowing_test, capital_proxy, capital_share, peak_build, Case};
    let mut w = String::new();
    writeln!(w, "| Input | Value | Source |\n|---|---:|---|")?;
    let src = "StatCan Table 36-10-0013-01, simple multiplier, total industries, 2022 (**unverified**: search-index copy)";
    for (name, v) in [
        ("Output per $ of final demand", IO_OUTPUT_2022),
        ("GDP at market prices per $ of final demand", IO_GDP_2022),
        ("Gross operating surplus per $ of final demand", IO_GOS_2022),
        (
            "International imports per $ of final demand",
            IO_IMPORTS_2022,
        ),
        ("Jobs per $1M of final demand", IO_JOBS_2022),
    ] {
        writeln!(w, "| {name} | {v:.3} | {src} |")?;
    }
    writeln!(
        w,
        "| Capital share of GDP (surplus / GDP) | {:.3} | derived from the two rows above (**unverified**) |",
        capital_share()
    )?;
    writeln!(
        w,
        "| Capital proxy (national wealth less residential real estate), $T | {:.1} | StatCan national balance sheet (derived) |",
        capital_proxy() / 1e6
    )?;
    writeln!(w, "\n| Case | Peak build-out investment, $B/yr | GDP from it, $B | Imports, $B | Jobs supported (thousands) | Long-run base gain with k = 0 | Revenue gain with k = 0, $B/yr | Share of the cost of abolition it covers |\n|---|---:|---:|---:|---:|---:|---:|---:|")?;
    for c in Case::all() {
        let (inv, g, imp, jobs) = peak_build(&c);
        let (gain, gap, share) = borrowing_test(&c);
        let _ = gap;
        writeln!(
            w,
            "| {} | {} | {} | {} | {:.0} | +{} | {} | {} |",
            c.name,
            b(inv),
            b(g),
            b(imp),
            jobs / 1000.0,
            p2(100.0 * (c.long_run_factor(0.0) - 1.0)),
            b(gain),
            pct(100.0 * share)
        )?;
    }
    Ok(w)
}

/// Table 16: levers that would close the gap left with k = 0, per case.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_levers() -> Result<String> {
    use crate::transition::{levers, Case};
    let mut w = String::new();
    writeln!(w, "| Case | Gap, $B | VAT rate that closes it alone | Tax household GST exemptions at the rule rate, $B | Program-Review-sized spending cut, $B | LVT scenario, $B (land value unverified) | VAT rate needed with all three |\n|---|---:|---:|---:|---:|---:|---:|")?;
    for c in Case::all() {
        let l = levers(&c);
        writeln!(
            w,
            "| {} | {} | {} | {} | {} | {} | {} |",
            c.name,
            b(l.gap),
            pct(l.rate),
            b(l.broadening),
            b(l.spending),
            b(l.lvt),
            pct(l.package_rate)
        )?;
    }
    writeln!(w, "\n| Case, with all three levers | k at launch | Years to k = 0 under the rule (within {}) | VAT rate that then meets the need |\n|---|---:|---|---:|", crate::data::HORIZON_YEARS)?;
    for c in Case::all() {
        let p = c.with_package();
        writeln!(
            w,
            "| {} | {} | {} | {} |",
            c.name,
            pct(100.0 * p.launch_k()),
            years_text(&p),
            pct(levers(&c).package_rate)
        )?;
    }
    Ok(w)
}

/// Table 17: counter-arguments with numbers (central case unless stated).
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_counter_v3() -> Result<String> {
    use crate::data::{
        GST_GAP_GROSS_2022, GST_GAP_NET_2022, IE_GDP_2024, IE_GNI_STAR_2024,
        RECESSION_BASE_DROP_PCT, RECESSION_YEAR,
    };
    use crate::transition::{simulate, years_to_repay, Case, RULE_RATE};
    let c = Case::all()[1];
    let core = need_core();
    let cen = Scenario::central();
    let mut w = String::new();
    writeln!(
        w,
        "| Risk | Measure | Value | Who bears it |\n|---|---|---:|---|"
    )?;
    writeln!(
        w,
        "| Regressivity | GST credit scaled to {}%, $B/yr (already netted out of the model's base) | {} | low-income households if the credit is not scaled |",
        num(RULE_RATE),
        b(GST_CREDIT * RULE_RATE / GST_RATE)
    )?;
    writeln!(
        w,
        "| Regressivity | Same, share of VAT revenue at {}% | {} | |",
        num(RULE_RATE),
        pct(100.0 * GST_CREDIT * RULE_RATE / GST_RATE / cen.revenue(RULE_RATE))
    )?;
    for (label, gap) in [("net", GST_GAP_NET_2022), ("gross", GST_GAP_GROSS_2022)] {
        let s = Scenario {
            leak: gap / 100.0,
            ..cen
        };
        writeln!(
            w,
            "| Evasion | k at launch if the {label} GST/HST gap ({}%) applies to new VAT revenue | {} | compliant firms and buyers |",
            num(gap),
            pct(100.0 * s.income_tax_kept(RULE_RATE, core))
        )?;
    }
    let shock = simulate(
        &c,
        crate::data::HORIZON_YEARS,
        Some((RECESSION_YEAR, RECESSION_BASE_DROP_PCT)),
    );
    writeln!(
        w,
        "| Recession | Deficit change in a year the base falls {}%, $B | {} | taxpayers, through the reversal |",
        num(RECESSION_BASE_DROP_PCT),
        b(shock[RECESSION_YEAR as usize].deficit_change)
    )?;
    writeln!(
        w,
        "| Recession | Years to repay it under the rule | {} | |",
        years_to_repay(&c, RECESSION_YEAR, RECESSION_BASE_DROP_PCT)
            .map_or_else(|| "not within the horizon".to_string(), |y| y.to_string())
    )?;
    writeln!(
        w,
        "| Profit shifting | Ireland 2024: GNI* / GDP | {} | measured GDP overstates domestic income |",
        pct(100.0 * IE_GNI_STAR_2024 / IE_GDP_2024)
    )?;
    Ok(w)
}

/// Dollars with thousands separators, no decimals; negative values with a minus sign.
#[must_use]
pub fn dollars(x: f64) -> String {
    let r = x.round();
    let s = thousands(&format!("{:.0}", r.abs()));
    if r < 0.0 {
        format!("−${s}")
    } else {
        format!("${s}")
    }
}

/// Value of a key in [`crate::data::FACTS`] (empty if absent).
fn fact(key: &str) -> &'static str {
    crate::data::FACTS
        .iter()
        .find(|(k, _)| *k == key)
        .map_or("", |(_, v)| *v)
}

/// Table 18: the three balanced end-state packages (k = 0).
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_endstate() -> Result<String> {
    use crate::endstate::{
        base_scenario, credit_scaled, prebate_each, prebate_gross, prebate_net,
        rate_without_prebate, Package,
    };
    let mut w = String::new();
    writeln!(w, "| Package | Spending review, $B | LVT, $B (land value unverified) | Wealth tax, $B | VAT revenue, $B | Prebate, $B | Less GST credit it replaces, $B | Balance, $B | **VAT rate** | Rate if the GST credit is scaled instead of a prebate | Prebate per person aged 15+ |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")?;
    for p in Package::all() {
        let r = p.rate();
        writeln!(
            w,
            "| {} | {} | {} | {} | {} | {} | {} | {} | **{}** | {} | {} |",
            p.name,
            b(p.spending),
            b(p.lvt),
            b(p.wealth),
            b(base_scenario().revenue(r)),
            b(prebate_gross(r)),
            b(credit_scaled(r)),
            bz(p.balance(r)),
            pct(r),
            pct(rate_without_prebate(&p)),
            dollars(prebate_each(r))
        )?;
    }
    let nets = Package::all()
        .iter()
        .map(|p| format!("${}B", b(prebate_net(p.rate()))))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(
        w,
        "\nNeed: ${}B (PIT + CIT + net GST). Net prebate cost at each package's rate: {nets}.",
        b(need_core())
    )?;
    Ok(w)
}

/// Table 19: who gains and who pays, federal taxes per single filer, package B.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_distribution() -> Result<String> {
    use crate::endstate::{distribution, Package};
    let r = Package::all()[1].rate();
    let labels = [
        "Single at the MBM line",
        "Wage earner",
        "Top 10% threshold",
        "Higher earner",
        "Top 1% threshold",
        "High earner",
        "Retiree drawing savings",
        "Asset-rich, no realised income",
    ];
    let mut w = String::new();
    writeln!(w, "| Filer | Total income | Consumption (scenario) | Today: federal PIT + GST | End state: VAT at {} less prebate | Gain (+) or loss (−) | Change, % of income |\n|---|---:|---:|---:|---:|---:|---:|", pct(r))?;
    for (l, h) in labels.iter().zip(distribution(r)) {
        let share = if h.income > 0.0 {
            pct(100.0 * h.gain() / h.income).replace('-', "−")
        } else {
            "no income".to_string()
        };
        writeln!(
            w,
            "| {l} | {} | {} | {} | {} | {} | {share} |",
            dollars(h.income),
            dollars(h.spending),
            dollars(h.today),
            dollars(h.end),
            dollars(h.gain())
        )?;
    }
    Ok(w)
}

/// Table 20: other options and why each is in or out of the packages.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_options() -> Result<String> {
    use crate::data::{
        WEALTH_TAX_2024_25, WEALTH_TAX_ADMIN_PCT, WEALTH_TAX_BEHAVIOUR_PCT, WEALTH_TAX_RATE_PCT,
        WEALTH_TAX_THRESHOLD,
    };
    let mut w = String::new();
    writeln!(
        w,
        "| Option | Rate | Revenue, $B/yr | Status | In packages |\n|---|---|---:|---|---|"
    )?;
    writeln!(
        w,
        "| Annual net wealth tax (PBO costing) | {}% above ${}M of family net wealth | {} | sourced; assumes {}% behavioural reduction and {}% administration cost | C |",
        num(WEALTH_TAX_RATE_PCT),
        num(WEALTH_TAX_THRESHOLD),
        b(WEALTH_TAX_2024_25),
        num(WEALTH_TAX_BEHAVIOUR_PCT),
        num(WEALTH_TAX_ADMIN_PCT)
    )?;
    writeln!(
        w,
        "| Land value tax | as in §5.4 | {} | land value unverified | B, C |",
        b(lvt_scenario())
    )?;
    writeln!(w, "| Distributed-profit tax (Estonia) | {} of the net distribution | not computed | Canadian distribution base unverified | no |", fact("ee_rate"))?;
    writeln!(w, "| Resource royalties | set by each province | 0 federal | resources belong to the provinces (Constitution Act, 1867, s. 109) | no |")?;
    writeln!(w, "| Inheritance or estate tax | not set | not computed | no Canadian costing found; capital gains are taxed at death today only through the income tax | no |")?;
    Ok(w)
}

/// Table 21: strongest counter-arguments to each package.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn table_counter_endstate() -> Result<String> {
    use crate::data::{GST_GAP_GROSS_2022, WEALTH_TAX_BEHAVIOUR_PCT};
    use crate::endstate::{prebate_net, Package};
    let p = Package::all();
    let (ra, rb, rc) = (p[0].rate(), p[1].rate(), p[2].rate());
    let mut w = String::new();
    writeln!(
        w,
        "| Package | Strongest counter-argument | Measure |\n|---|---|---|"
    )?;
    writeln!(w, "| A | The rate is above Hungary's, the highest standard VAT rate in the OECD, so evasion rises | VAT {} against {}% [R4]; the gross GST/HST gap is already {}% at {}% [R34] |", pct(ra), fact("hu_rate"), num(GST_GAP_GROSS_2022), num(GST_RATE))?;
    writeln!(
        w,
        "| A | The prebate costs about as much as all of today's direct program expenses | net ${}B a year against ${}B [R2] |",
        b(prebate_net(ra)),
        b(crate::data::DIRECT_PROGRAM_EXPENSES)
    )?;
    writeln!(w, "| B | The land value is not verified, so the LVT may raise much less | if the LVT raised nothing, B's rate would be A's: {} |", pct(ra))?;
    writeln!(w, "| B | Provinces and municipalities already tax property | the LVT must first replace ${}B of property tax (§5.4) |", b(property_tax_total()))?;
    writeln!(w, "| C | Wealth is hard to value and mobile; the PBO already cuts the base by {}% for avoidance | VAT {} with it, {} without it |", num(WEALTH_TAX_BEHAVIOUR_PCT), pct(rc), pct(rb))?;
    writeln!(w, "| All | With PIT at zero, the deemed disposition at death goes with it, so gains never realised are never taxed | only the VAT on what the owners spend, and package C's wealth tax, reach them |")?;
    writeln!(w, "| All | Retirees who saved out of taxed income pay VAT again when they spend | see the retiree row in §13.4 |")?;
    Ok(w)
}
