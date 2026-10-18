//! Generates the paper, README and claims ledger from templates. Every model-derived
//! number in those documents comes from [`values`]; templates hold only prose, sourced
//! literals listed in the test allowlist, and `{{placeholders}}`.

use crate::data::{
    CIT, DIVIDEND_BOTTOM_SHARE, EI_PREMIUMS, END_RATE, FACTS, GROWTH_ADDITION_PP,
    GST_BEFORE_REMISSION, GST_CREDIT, GST_GROSS_EXTERNAL, GST_HOLIDAY_COST, GST_HOLIDAY_COST_2024,
    GST_HOLIDAY_COST_2025, GST_NET, GST_RATE, GST_REMISSION, HFCE_PCT_GDP, IMPORTS_PCT_GDP,
    INVESTMENT_SHARE, LAND_VALUE_UNVERIFIED, LVT_CLAIM_HIGH, LVT_CLAIM_LOW, LVT_RATES,
    LVT_TAXABLE_SHARE_SCENARIO, MULTIPLIER_CIT, MULTIPLIER_PIT, NATIONAL_WEALTH_Q2_2025,
    NON_RESIDENT, PATH_YEARS, PIT, POPULATION_2024, PROPERTY_TAX_PER_CAPITA_2024,
    RESIDENTIAL_REAL_ESTATE_Q2_2025, REVENUE_PCT_GDP, SAVING_RATE_Q2_2024, SAVING_RATE_Q2_2025,
    START_RATE, TE_HOUSEHOLD, TE_PUBLIC_AND_THRESHOLD, TOTAL_REVENUE, VRR_CANADA, VRR_NEW_ZEALAND,
    VRR_OECD_AVERAGE,
};
use crate::error::{Error, Result};
use crate::model::{
    apc, base0, gdp, income_taxes, lvt_land_needed, lvt_rate_to_replace_property_tax, lvt_revenue,
    lvt_scenario, need_core, need_full, path_rate, property_tax_total, taxable_share_central,
    te_sum, v11_path_rate, vrr_factor, Incidence, Scenario,
};
use crate::report::{self, b, num, pct, BRACKETS_2023};
use std::collections::BTreeMap;

/// Template for the whitepaper.
pub const PAPER_TEMPLATE: &str = include_str!("../templates/tax-whitepaper.md");
/// Template for the README.
pub const README_TEMPLATE: &str = include_str!("../templates/README.md");
/// Template for the claims ledger.
pub const CLAIMS_TEMPLATE: &str = include_str!("../templates/CLAIMS.md");

/// Generated documents: (template, path relative to the repository root).
pub const DOCUMENTS: [(&str, &str); 3] = [
    (PAPER_TEMPLATE, "tax-whitepaper.md"),
    (README_TEMPLATE, "README.md"),
    (CLAIMS_TEMPLATE, "CLAIMS.md"),
];

/// $M -> "$B" with thousands separators, one decimal ("1,050.1").
#[must_use]
pub fn bc(x: f64) -> String {
    thousands(&b(x))
}

/// Insert thousands separators into the integer part of a plain decimal string.
#[must_use]
pub fn thousands(s: &str) -> String {
    let (int, frac) = s.split_once('.').map_or((s, None), |(i, f)| (i, Some(f)));
    let mut out = String::new();
    for (i, ch) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    if let Some(f) = frac {
        out.push('.');
        out.push_str(f);
    }
    out
}

/// $M -> whole $B, rounded ("32").
fn bi(x: f64) -> String {
    format!("{:.0}", x / 1000.0)
}

/// $M -> $T with one decimal ("6.4").
fn t1(x: f64) -> String {
    format!("{:.1}", x / 1_000_000.0)
}

/// Per cent rounded to a whole number, without the sign ("47").
fn pi(x: f64) -> String {
    format!("{x:.0}")
}

/// "more than" / "less than" / "the same as", comparing `a` with `b`.
#[must_use]
pub fn more_or_less(a: f64, b: f64) -> &'static str {
    if a > b {
        "more than"
    } else if a < b {
        "less than"
    } else {
        "the same as"
    }
}

/// A share in words when it is within 4 points of a simple fraction ("about half",
/// "about a third"); otherwise a whole per cent ("41%").
#[must_use]
pub fn fraction_words(x: f64) -> String {
    const FRACTIONS: [(f64, &str); 5] = [
        (0.25, "a quarter"),
        (1.0 / 3.0, "a third"),
        (0.5, "half"),
        (2.0 / 3.0, "two-thirds"),
        (0.75, "three-quarters"),
    ];
    let nearest = FRACTIONS
        .iter()
        .min_by(|p, q| (x - p.0).abs().total_cmp(&(x - q.0).abs()));
    match nearest {
        Some((f, w)) if (x - f).abs() <= 0.04 => format!("about {w}"),
        _ => format!("{:.0}%", 100.0 * x),
    }
}

/// Every named value the templates can use.
///
/// # Errors
/// Only if rendering a table fails.
#[allow(clippy::too_many_lines)] // a flat list of named values; splitting it hides nothing
pub fn values() -> Result<BTreeMap<String, String>> {
    let inc_a = Incidence::RealBaseFixed;
    let inc_b = Incidence::NominalSpendingFixed;
    let core = need_core();
    let st = Scenario::static_case(inc_a);
    let stb = Scenario::static_case(inc_b);
    let cen = Scenario::central();
    let cb = Scenario::dynamic(inc_b);
    let mul = Scenario {
        multiplier: true,
        ..cen
    };
    let half = Scenario { mpc: 0.5, ..cen };
    let nz_a = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..st
    };
    let nz_b = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..stb
    };
    let leak20 = Scenario { leak: 0.2, ..st };
    let (hi, lo) = (START_RATE, END_RATE);
    let k_a = cen.income_tax_kept(hi, core);
    let k_b = cb.income_tax_kept(hi, core);
    let [g_lo, g_hi] = GROWTH_ADDITION_PP;
    let gf = |pp: f64| (1.0 + pp / 100.0f64).powf(f64::from(PATH_YEARS));
    let hh = te_sum(&TE_HOUSEHOLD);
    let pub_thr = te_sum(&TE_PUBLIC_AND_THRESHOLD);
    let groceries = TE_HOUSEHOLD[0].1;
    let path: Vec<String> = (0..=PATH_YEARS)
        .map(|t| format!("{:.1}", path_rate(t)))
        .collect();
    let scen_land = LAND_VALUE_UNVERIFIED * LVT_TAXABLE_SHARE_SCENARIO;
    let brackets_k: Vec<String> = BRACKETS_2023
        .iter()
        .map(|x| format!("{:.1}", x * k_a))
        .collect();

    let mut vals: BTreeMap<String, String> = BTreeMap::new();
    let mut put = |k: &str, val: String| {
        vals.insert(k.to_string(), val);
    };
    // Design parameters and path.
    put("start_rate", num(hi));
    put("end_rate", num(lo));
    put("path_years", PATH_YEARS.to_string());
    put("path_step", num((hi - lo) / f64::from(PATH_YEARS)));
    put("path_mid", path[5].clone());
    put("path_list", path.join(", "));
    put("v11_end", num(v11_path_rate(PATH_YEARS)));
    put("lvt_rate_lo", num(LVT_RATES[0]));
    put("lvt_rate_hi", num(LVT_RATES[1]));
    put("lvt_claim_lo", bi(LVT_CLAIM_LOW));
    put("lvt_claim_hi", bi(LVT_CLAIM_HIGH));
    put("dividend_lo", num(DIVIDEND_BOTTOM_SHARE[0]));
    put("dividend_hi", num(DIVIDEND_BOTTOM_SHARE[1]));
    put("invest_lo", num(INVESTMENT_SHARE[0]));
    put("invest_hi", num(INVESTMENT_SHARE[1]));
    put("mpc_half", format!("{:.2}", half.mpc));
    put("leak20_pct", num(100.0 * leak20.leak));
    put("growth_lo", num(g_lo));
    put("growth_hi", num(g_hi));
    put("growth_f_lo", format!("{:.2}", gf(g_lo)));
    put("growth_f_hi", format!("{:.2}", gf(g_hi)));
    // Baseline.
    put("pit_b", b(PIT));
    put("cit_b", b(CIT));
    put("pit_m", thousands(&num(PIT)));
    put("cit_m", thousands(&num(CIT)));
    put("gst_m", thousands(&num(GST_NET)));
    put("gst_b", b(GST_NET));
    put("gst_rate", num(GST_RATE));
    put("it_b", b(income_taxes()));
    put("nr_b", b(NON_RESIDENT));
    put("nr_m", thousands(&num(NON_RESIDENT)));
    put("gst_before_m", thousands(&num(GST_BEFORE_REMISSION)));
    put("gst_remission_m", thousands(&num(GST_REMISSION)));
    put("gst_gross_m", thousands(&num(GST_GROSS_EXTERNAL)));
    put("gst_credit_m", thousands(&num(GST_CREDIT)));
    put("ei_m", thousands(&num(EI_PREMIUMS)));
    put("total_rev_m", thousands(&num(TOTAL_REVENUE)));
    put("holiday_2024_m", num(GST_HOLIDAY_COST_2024));
    put("holiday_2025_m", num(GST_HOLIDAY_COST_2025));
    put("property_pc", thousands(&num(PROPERTY_TAX_PER_CAPITA_2024)));
    put("population", thousands(&num(POPULATION_2024)));
    put("ei_b", b(EI_PREMIUMS));
    put("credit_b", b(GST_CREDIT));
    put("holiday_b", b(GST_HOLIDAY_COST));
    put("total_rev_b", b(TOTAL_REVENUE));
    put("rev_pct_gdp", num(REVENUE_PCT_GDP));
    put("gdp_b", thousands(&format!("{:.0}", gdp() / 1000.0)));
    put("base0_b", bc(base0()));
    put("gross_base_b", bc(GST_GROSS_EXTERNAL / (GST_RATE / 100.0)));
    put("it_pct_gdp", pct(100.0 * income_taxes() / gdp()));
    put("need_core_b", b(core));
    put("need_full_b", b(need_full()));
    // Static and dynamic headline.
    put("static_hi_a", b(st.revenue(hi)));
    put("static_hi_b", b(stb.revenue(hi)));
    put("dyn_hi_a", b(cen.revenue(hi)));
    put("dyn_hi_b", b(cb.revenue(hi)));
    put("mult_hi_a", b(mul.revenue(hi)));
    put("rate_static_a", pct(st.required_rate(core)));
    put("rate_static_b", pct(stb.required_rate(core)));
    put("rate_dyn_a", pct(cen.required_rate(core)));
    put("rate_dyn_b", pct(cb.required_rate(core)));
    put("rate_mult_a", pct(mul.required_rate(core)));
    put("rate_static_full_a", pct(st.required_rate(need_full())));
    put("rate_static_a_round", pi(st.required_rate(core)));
    put("rate_dyn_a_round", pi(cen.required_rate(core)));
    put("rate_dyn_b_round", pi(cb.required_rate(core)));
    put("k_a", pct(100.0 * k_a));
    put("k_b", pct(100.0 * k_b));
    put("k_a_round", pi(100.0 * k_a));
    put("k_b_round", pi(100.0 * k_b));
    put(
        "brackets_2023",
        BRACKETS_2023
            .iter()
            .map(|x| num(*x))
            .collect::<Vec<_>>()
            .join(" / "),
    );
    put("brackets_k_a", brackets_k.join(" / "));
    put("bracket_lo", num(BRACKETS_2023[0]));
    put("bracket_hi", num(BRACKETS_2023[4]));
    put("mpc_be_a", format!("{:.2}", cen.breakeven_mpc(hi, core)));
    put(
        "mpc_be_s1",
        format!(
            "{:.2}",
            Scenario {
                taxable_share: 1.0,
                ..cen
            }
            .breakeven_mpc(hi, core)
        ),
    );
    put("mpc_be_b", format!("{:.2}", cb.breakeven_mpc(hi, core)));
    put(
        "g_hi_a",
        format!("{:.2}", cen.breakeven_growth_factor(hi, core)),
    );
    put(
        "g_lo_a",
        format!("{:.2}", cen.breakeven_growth_factor(lo, core)),
    );
    put(
        "g_hi_a_pa",
        pct(100.0 * crate::model::annualised(cen.breakeven_growth_factor(hi, core), PATH_YEARS)),
    );
    put(
        "g_hi_mult",
        format!("{:.2}", mul.breakeven_growth_factor(hi, core)),
    );
    put(
        "g_hi_mult_pa",
        pct(100.0 * crate::model::annualised(mul.breakeven_growth_factor(hi, core), PATH_YEARS)),
    );
    put(
        "g_hi_b",
        format!("{:.2}", cb.breakeven_growth_factor(hi, core)),
    );
    put(
        "g_hi_b_pa",
        pct(100.0 * crate::model::annualised(cb.breakeven_growth_factor(hi, core), PATH_YEARS)),
    );
    put("nz_rate_needed_a", pct(nz_a.required_rate(core)));
    put("nz_rate_needed_b", pct(nz_b.required_rate(core)));
    put("dyn_gain_lo", bi(cen.revenue(hi) - st.revenue(hi)));
    put("dyn_gain_hi", bi(mul.revenue(hi) - st.revenue(hi)));
    put("short_lo", bi(core - mul.revenue(hi)));
    put("short_hi", bi(core - cen.revenue(hi)));
    put("mpc_half_rev", b(half.revenue(hi)));
    put("mpc_effect", bi(cen.revenue(hi) - half.revenue(hi)));
    put("leak20_rate", pct(leak20.required_rate(core)));
    put("ab_gap", bi(cen.revenue(hi) - cb.revenue(hi)));
    put("real_base_a", bc(cen.real_base(hi)));
    put("real_base_b", bc(cb.real_base(hi)));
    // Behaviour inputs.
    put("taxable_share", format!("{:.2}", taxable_share_central()));
    put("taxable_share_pct", pi(100.0 * taxable_share_central()));
    put("mpc_central", format!("{:.2}", cen.mpc));
    put("apc_lo", format!("{:.3}", apc(SAVING_RATE_Q2_2024)));
    put("apc_hi", format!("{:.3}", apc(SAVING_RATE_Q2_2025)));
    put("saving_2024", pct(100.0 * SAVING_RATE_Q2_2024));
    put("saving_2025", pct(100.0 * SAVING_RATE_Q2_2025));
    put("hfce_pct", num(HFCE_PCT_GDP));
    put("imports_pct", num(IMPORTS_PCT_GDP));
    put("mult_pit", format!("{MULTIPLIER_PIT:.1}"));
    put("mult_cit", format!("{MULTIPLIER_CIT:.1}"));
    put("vrr_ca", format!("{VRR_CANADA:.2}"));
    put("vrr_oecd", format!("{VRR_OECD_AVERAGE:.2}"));
    put("vrr_nz", format!("{VRR_NEW_ZEALAND:.2}"));
    // Broadening and protection.
    put("groceries_extra", b(groceries));
    put("hh_extra", b(hh));
    put("all_extra", b(hh + pub_thr));
    put("hh_count", TE_HOUSEHOLD.len().to_string());
    put("credit_hi", b(GST_CREDIT * hi / GST_RATE));
    put("groceries_hi", b(groceries * hi / GST_RATE));
    // Land and property tax.
    put(
        "land_needed_lo",
        t1(lvt_land_needed(LVT_CLAIM_LOW, LVT_RATES[1]))
            .trim_end_matches(".0")
            .to_string(),
    );
    put(
        "land_needed_hi",
        t1(lvt_land_needed(LVT_CLAIM_HIGH, LVT_RATES[0]))
            .trim_end_matches(".0")
            .to_string(),
    );
    put("land_t", t1(LAND_VALUE_UNVERIFIED));
    put("lvt_scenario", bi(lvt_scenario()));
    put(
        "lvt_all_land",
        bi(lvt_revenue(LVT_RATES[1], LAND_VALUE_UNVERIFIED)),
    );
    put("lvt_share_pct", num(100.0 * LVT_TAXABLE_SHARE_SCENARIO));
    put("wealth_b", bc(NATIONAL_WEALTH_Q2_2025));
    put("res_re_b", bc(RESIDENTIAL_REAL_ESTATE_Q2_2025));
    put("property_b", b(property_tax_total()));
    put(
        "lvt_replace_scen",
        pct(lvt_rate_to_replace_property_tax(scen_land)),
    );
    put(
        "lvt_replace_all",
        pct(lvt_rate_to_replace_property_tax(LAND_VALUE_UNVERIFIED)),
    );
    for (k, val) in FACTS {
        put(k, (*val).to_string());
    }
    // Comparison words, generated from the values they compare.
    let g_factor_hi = cen.breakeven_growth_factor(hi, core);
    let g_lo_pa =
        100.0 * crate::model::annualised(cen.breakeven_growth_factor(lo, core), PATH_YEARS);
    let nz_needed = nz_a.required_rate(core);
    let lvt_ratio = lvt_rate_to_replace_property_tax(scen_land) / LVT_RATES[1];
    put("g_lo_a_pa", pct(g_lo_pa));
    put("replaced_frac_a", fraction_words(1.0 - k_a));
    put("base_frac_gdp", fraction_words(base0() / gdp()));
    put(
        "mpc_verdict_a",
        if cen.breakeven_mpc(hi, core) > 1.0 {
            "no possible MPC closes the gap on its own"
        } else {
            "an MPC within the possible range closes the gap on its own"
        }
        .into(),
    );
    put(
        "g_hi_a_dir",
        if g_factor_hi > 1.0 {
            "larger"
        } else {
            "smaller"
        }
        .into(),
    );
    put(
        "property_vs_lvt",
        more_or_less(property_tax_total(), lvt_scenario()).into(),
    );
    put(
        "b_vs_a",
        if cb.revenue(hi) < cen.revenue(hi) {
            "lower"
        } else {
            "higher"
        }
        .into(),
    );
    put("k_b_dir", if k_b > k_a { "rises" } else { "falls" }.into());
    put(
        "lvt_vs_design",
        if lvt_ratio >= 2.0 {
            "well above"
        } else if lvt_ratio > 1.0 {
            "above"
        } else {
            "within"
        }
        .into(),
    );
    put(
        "nz_condition",
        if nz_needed <= hi {
            "needs a VAT that performs like New Zealand's"
        } else {
            "needs a VAT that performs better than New Zealand's"
        }
        .into(),
    );
    put(
        "nz_verdict",
        if nz_needed <= hi {
            "is enough"
        } else if nz_needed <= hi + 2.0 {
            "comes close"
        } else {
            "is not enough"
        }
        .into(),
    );
    put(
        "growth_vs_original",
        if g_lo_pa > g_hi {
            "faster than even the high end of"
        } else if g_lo_pa >= g_lo {
            "within the range of"
        } else {
            "below"
        }
        .into(),
    );
    let short = core - mul.revenue(hi);
    put(
        "mpc_effect_size",
        if cen.revenue(hi) - half.revenue(hi) < 0.25 * short {
            "small"
        } else {
            "large"
        }
        .into(),
    );
    // Tables and blocks.
    let tables: [report::Section; 16] = [
        ("table:first_principles", report::table_first_principles),
        ("table:baseline", report::table_baseline),
        ("table:static", report::table_static),
        ("table:broadening", report::table_broadening),
        ("table:leakage", report::table_leakage),
        ("table:dynamic", report::table_dynamic),
        ("table:sensitivity", report::table_sensitivity),
        ("table:breakeven", report::table_breakeven),
        ("table:path", report::table_path),
        ("table:lvt", report::table_lvt),
        ("table:property", report::table_property),
        ("table:stack", report::table_stack),
        ("table:k", report::table_k),
        ("table:protect", report::table_protect),
        ("table:key_figures", key_figures),
        ("table:readme", readme_table),
    ];
    for (k, f) in tables {
        let t = f()?;
        put(k, t.trim_end().to_string());
    }
    let ledger = claims_inputs_table(&vals)?;
    vals.insert("table:claims_inputs".into(), ledger);
    Ok(vals)
}

/// The inputs table of `CLAIMS.md`, rendered from [`crate::ledger::INPUTS`].
///
/// # Errors
/// If a ledger row names an unknown placeholder.
pub fn claims_inputs_table(vals: &BTreeMap<String, String>) -> Result<String> {
    let mut rows = vec![
        "| Claim | Status | Evidence |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for row in crate::ledger::INPUTS {
        rows.push(format!(
            "| {} | {} | {} |",
            render_template(row.claim, vals)?,
            row.status.label(),
            render_template(row.evidence, vals)?
        ));
    }
    Ok(rows.join("\n"))
}

/// Headline numbers block.
///
/// # Errors
/// Only if formatting into a `String` fails.
pub fn key_figures() -> Result<String> {
    use std::fmt::Write;
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
    use std::fmt::Write;
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

/// Replace every `{{name}}` in `template` with its value.
///
/// # Errors
/// [`Error::UnknownPlaceholder`] for a name with no value, [`Error::UnclosedPlaceholder`]
/// for a `{{` without `}}`.
pub fn render_template(template: &str, values: &BTreeMap<String, String>) -> Result<String> {
    let mut out = String::with_capacity(template.len() * 2);
    let mut rest = template;
    let mut offset = 0;
    while let Some(i) = rest.find("{{") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 2..];
        let j = after
            .find("}}")
            .ok_or(Error::UnclosedPlaceholder(offset + i))?;
        let key = after[..j].trim();
        let val = values
            .get(key)
            .ok_or_else(|| Error::UnknownPlaceholder(key.to_string()))?;
        out.push_str(val);
        let consumed = i + 2 + j + 2;
        offset += consumed;
        rest = &rest[consumed..];
    }
    out.push_str(rest);
    Ok(out)
}

/// Render every generated document: (path relative to repo root, contents).
///
/// # Errors
/// If a table fails to render or a template has an unknown placeholder.
pub fn render_documents() -> Result<Vec<(&'static str, String)>> {
    let v = values()?;
    DOCUMENTS
        .iter()
        .map(|(t, p)| Ok((*p, render_template(t, &v)?)))
        .collect()
}
