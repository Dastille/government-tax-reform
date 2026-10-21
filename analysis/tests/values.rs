//! Ties every filled-in placeholder to the model.
//!
//! `values_match_model` recomputes each placeholder from the model API with this file's own
//! formatters, independently of `src/docs.rs`, and asserts `docs::values()` gives the same
//! string for every name, with no missing or extra names. `placeholders_sit_in_their_context`
//! pins each placeholder occurrence to the text around it, so pointing a sentence at a
//! different value (for example `{{dyn_hi_a}}` → `{{mpc_half_rev}}`) fails.

#[path = "values/placements.rs"]
mod placements;

use std::collections::BTreeMap;
use vat_revenue_model::data::*;
use vat_revenue_model::docs::{values, CLAIMS_TEMPLATE, PAPER_TEMPLATE, README_TEMPLATE};
use vat_revenue_model::ledger::INPUTS;
use vat_revenue_model::model::{
    annualised, apc, base0, gdp, income_taxes, lvt_land_needed, lvt_rate_to_replace_property_tax,
    lvt_revenue, lvt_scenario, need_core, need_full, path_rate, property_tax_total,
    taxable_share_central, v11_path_rate, vrr_factor, Incidence, Scenario,
};
use vat_revenue_model::report;

// ---- This file's own formatters (deliberately not imported from the crate). ----

/// $M → $B, one decimal.
fn usd_b(m: f64) -> String {
    format!("{:.1}", m / 1000.0)
}
/// $M → whole $B.
fn usd_b0(m: f64) -> String {
    format!("{:.0}", m / 1000.0)
}
/// $M → $T, one decimal.
fn usd_t(m: f64) -> String {
    format!("{:.1}", m / 1_000_000.0)
}
/// Per cent, one decimal, with sign.
fn p1(x: f64) -> String {
    format!("{x:.1}%")
}
/// Per cent, whole, no sign.
fn p0(x: f64) -> String {
    format!("{x:.0}")
}
/// Two decimals.
fn d2(x: f64) -> String {
    format!("{x:.2}")
}
/// Shortest form up to three decimals ("20", "0.5", "12.5").
fn short(x: f64) -> String {
    let s = format!("{x:.3}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}
/// Thousands separators on the integer part.
fn commas(s: &str) -> String {
    let (int, frac) = match s.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (s, None),
    };
    let n = int.len();
    let mut out: String = int
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let sep = (i > 0 && (n - i) % 3 == 0).then_some(',');
            sep.into_iter().chain(std::iter::once(c))
        })
        .collect();
    if let Some(f) = frac {
        out = format!("{out}.{f}");
    }
    out
}
/// Words for a share near a simple fraction, else a whole per cent.
fn share_words(x: f64) -> String {
    let fr = [
        (0.25, "a quarter"),
        (1.0 / 3.0, "a third"),
        (0.5, "half"),
        (2.0 / 3.0, "two-thirds"),
        (0.75, "three-quarters"),
    ];
    let mut best = fr[0];
    for f in fr {
        if (x - f.0).abs() < (x - best.0).abs() {
            best = f;
        }
    }
    if (x - best.0).abs() <= 0.04 {
        format!("about {}", best.1)
    } else {
        format!("{:.0}%", 100.0 * x)
    }
}
fn cmp_words(a: f64, b: f64) -> &'static str {
    if a > b {
        "more than"
    } else if a < b {
        "less than"
    } else {
        "the same as"
    }
}

/// A table builder in `report.rs`.
type TableFn = fn() -> vat_revenue_model::error::Result<String>;

/// Every placeholder, recomputed from the model API.
#[allow(clippy::too_many_lines)] // one line per placeholder
fn expected() -> BTreeMap<String, String> {
    let core = need_core();
    let (hi, lo) = (START_RATE, END_RATE);
    let st = Scenario::static_case(Incidence::RealBaseFixed);
    let stb = Scenario::static_case(Incidence::NominalSpendingFixed);
    let cen = Scenario::central();
    let cb = Scenario::dynamic(Incidence::NominalSpendingFixed);
    let mul = Scenario {
        multiplier: true,
        ..cen
    };
    let half = Scenario { mpc: 0.5, ..cen };
    let s1 = Scenario {
        taxable_share: 1.0,
        ..cen
    };
    let nz_a = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..st
    };
    let nz_b = Scenario {
        base_factor: vrr_factor(VRR_NEW_ZEALAND),
        ..stb
    };
    let leak20 = Scenario { leak: 0.2, ..st };
    let k_a = cen.income_tax_kept(hi, core);
    let k_b = cb.income_tax_kept(hi, core);
    let years = PATH_YEARS;
    let growth = |pp: f64| (1.0 + pp / 100.0f64).powi(i32::try_from(years).expect("years"));
    let ann = |f: f64| p1(100.0 * annualised(f, years));
    let path: Vec<String> = (0..=years)
        .map(|t| format!("{:.1}", path_rate(t)))
        .collect();
    let scen_land = LAND_VALUE_UNVERIFIED * LVT_TAXABLE_SHARE_SCENARIO;
    let groceries = TE_HOUSEHOLD[0].1;
    let join = |v: Vec<String>| v.join(" / ");
    let g_lo_pa = 100.0 * annualised(cen.breakeven_growth_factor(lo, core), years);
    let nz_needed = nz_a.required_rate(core);
    let lvt_ratio = lvt_rate_to_replace_property_tax(scen_land) / LVT_RATES[1];
    let shortfall = core - mul.revenue(hi);
    let land = |rev: f64, rate: f64| {
        usd_t(lvt_land_needed(rev, rate))
            .trim_end_matches(".0")
            .to_string()
    };

    let mut e: Vec<(&str, String)> = vec![
        // Design and path.
        ("start_rate", short(hi)),
        ("end_rate", short(lo)),
        ("path_years", years.to_string()),
        ("path_step", short((hi - lo) / f64::from(years))),
        ("path_mid", format!("{:.1}", path_rate(5))),
        ("path_list", path.join(", ")),
        ("v11_end", short(v11_path_rate(years))),
        ("lvt_rate_lo", short(LVT_RATES[0])),
        ("lvt_rate_hi", short(LVT_RATES[1])),
        ("lvt_claim_lo", usd_b0(LVT_CLAIM_LOW)),
        ("lvt_claim_hi", usd_b0(LVT_CLAIM_HIGH)),
        ("dividend_lo", short(DIVIDEND_BOTTOM_SHARE[0])),
        ("dividend_hi", short(DIVIDEND_BOTTOM_SHARE[1])),
        ("invest_lo", short(INVESTMENT_SHARE[0])),
        ("invest_hi", short(INVESTMENT_SHARE[1])),
        ("mpc_half", d2(0.5)),
        ("leak20_pct", short(20.0)),
        ("growth_lo", short(GROWTH_ADDITION_PP[0])),
        ("growth_hi", short(GROWTH_ADDITION_PP[1])),
        ("growth_f_lo", d2(growth(GROWTH_ADDITION_PP[0]))),
        ("growth_f_hi", d2(growth(GROWTH_ADDITION_PP[1]))),
        // Baseline.
        ("pit_m", commas(&short(PIT))),
        ("cit_m", commas(&short(CIT))),
        ("gst_m", commas(&short(GST_NET))),
        ("gst_b", usd_b(GST_NET)),
        ("gst_rate", short(GST_RATE)),
        ("it_b", usd_b(PIT + CIT)),
        ("nr_b", usd_b(NON_RESIDENT)),
        ("nr_m", commas(&short(NON_RESIDENT))),
        ("gst_before_m", commas(&short(GST_BEFORE_REMISSION))),
        ("gst_remission_m", commas(&short(GST_REMISSION))),
        ("gst_gross_m", commas(&short(GST_GROSS_EXTERNAL))),
        ("gst_credit_m", commas(&short(GST_CREDIT))),
        ("ei_m", commas(&short(EI_PREMIUMS))),
        ("total_rev_m", commas(&short(TOTAL_REVENUE))),
        ("holiday_2024_m", short(GST_HOLIDAY_COST_2024)),
        ("holiday_2025_m", short(GST_HOLIDAY_COST_2025)),
        ("property_pc", commas(&short(PROPERTY_TAX_PER_CAPITA_2024))),
        ("population", commas(&short(POPULATION_2024))),
        ("ei_b", usd_b(EI_PREMIUMS)),
        ("credit_b", usd_b(GST_CREDIT)),
        (
            "holiday_b",
            usd_b(GST_HOLIDAY_COST_2024 + GST_HOLIDAY_COST_2025),
        ),
        ("rev_pct_gdp", short(REVENUE_PCT_GDP)),
        (
            "gdp_b",
            commas(&format!(
                "{:.0}",
                TOTAL_REVENUE / (REVENUE_PCT_GDP / 100.0) / 1000.0
            )),
        ),
        ("base0_b", commas(&usd_b(GST_NET / (GST_RATE / 100.0)))),
        (
            "gross_base_b",
            commas(&usd_b(GST_GROSS_EXTERNAL / (GST_RATE / 100.0))),
        ),
        ("it_pct_gdp", p1(100.0 * income_taxes() / gdp())),
        ("need_core_b", usd_b(PIT + CIT + GST_NET)),
        ("need_full_b", usd_b(need_full())),
        // Headline.
        ("static_hi_a", usd_b(st.revenue(hi))),
        ("static_hi_b", usd_b(stb.revenue(hi))),
        ("dyn_hi_a", usd_b(cen.revenue(hi))),
        ("dyn_hi_b", usd_b(cb.revenue(hi))),
        ("mult_hi_a", usd_b(mul.revenue(hi))),
        ("rate_static_a", p1(st.required_rate(core))),
        ("rate_static_b", p1(stb.required_rate(core))),
        ("rate_dyn_a", p1(cen.required_rate(core))),
        ("rate_mult_a", p1(mul.required_rate(core))),
        ("rate_static_full_a", p1(st.required_rate(need_full()))),
        ("rate_static_a_round", p0(st.required_rate(core))),
        ("rate_dyn_a_round", p0(cen.required_rate(core))),
        ("rate_dyn_b_round", p0(cb.required_rate(core))),
        ("k_a", p1(100.0 * k_a)),
        ("k_b", p1(100.0 * k_b)),
        ("k_a_round", p0(100.0 * k_a)),
        ("k_b_round", p0(100.0 * k_b)),
        (
            "brackets_2023",
            join(BRACKETS_2023.iter().map(|x| short(*x)).collect()),
        ),
        (
            "brackets_k_a",
            join(
                BRACKETS_2023
                    .iter()
                    .map(|x| format!("{:.1}", x * k_a))
                    .collect(),
            ),
        ),
        ("bracket_lo", short(BRACKETS_2023[0])),
        ("bracket_hi", short(BRACKETS_2023[4])),
        ("mpc_be_a", d2(cen.breakeven_mpc(hi, core))),
        ("mpc_be_s1", d2(s1.breakeven_mpc(hi, core))),
        ("mpc_be_b", d2(cb.breakeven_mpc(hi, core))),
        ("g_hi_a", d2(cen.breakeven_growth_factor(hi, core))),
        ("g_lo_a", d2(cen.breakeven_growth_factor(lo, core))),
        ("g_hi_a_pa", ann(cen.breakeven_growth_factor(hi, core))),
        ("g_hi_mult", d2(mul.breakeven_growth_factor(hi, core))),
        ("g_hi_mult_pa", ann(mul.breakeven_growth_factor(hi, core))),
        ("g_hi_b", d2(cb.breakeven_growth_factor(hi, core))),
        ("g_hi_b_pa", ann(cb.breakeven_growth_factor(hi, core))),
        ("g_lo_a_pa", p1(g_lo_pa)),
        ("nz_rate_needed_a", p1(nz_a.required_rate(core))),
        ("nz_rate_needed_b", p1(nz_b.required_rate(core))),
        ("dyn_gain_lo", usd_b0(cen.revenue(hi) - st.revenue(hi))),
        ("dyn_gain_hi", usd_b0(mul.revenue(hi) - st.revenue(hi))),
        ("short_lo", usd_b0(core - mul.revenue(hi))),
        ("short_hi", usd_b0(core - cen.revenue(hi))),
        ("mpc_half_rev", usd_b(half.revenue(hi))),
        ("mpc_effect", usd_b0(cen.revenue(hi) - half.revenue(hi))),
        ("leak20_rate", p1(leak20.required_rate(core))),
        ("ab_gap", usd_b0(cen.revenue(hi) - cb.revenue(hi))),
        ("real_base_a", commas(&usd_b(cen.real_base(hi)))),
        ("real_base_b", commas(&usd_b(cb.real_base(hi)))),
        // Behaviour inputs.
        ("taxable_share", d2(taxable_share_central())),
        ("taxable_share_pct", p0(100.0 * taxable_share_central())),
        ("mpc_central", d2(apc(SAVING_RATE_Q2_2025))),
        ("apc_lo", format!("{:.3}", apc(SAVING_RATE_Q2_2024))),
        ("apc_hi", format!("{:.3}", apc(SAVING_RATE_Q2_2025))),
        ("saving_2024", p1(100.0 * SAVING_RATE_Q2_2024)),
        ("saving_2025", p1(100.0 * SAVING_RATE_Q2_2025)),
        ("hfce_pct", short(HFCE_PCT_GDP)),
        ("imports_pct", short(IMPORTS_PCT_GDP)),
        ("mult_pit", format!("{MULTIPLIER_PIT:.1}")),
        ("mult_cit", format!("{MULTIPLIER_CIT:.1}")),
        ("vrr_ca", d2(VRR_CANADA)),
        ("vrr_oecd", d2(VRR_OECD_AVERAGE)),
        ("vrr_nz", d2(VRR_NEW_ZEALAND)),
        // Broadening and protection.
        ("hh_count", TE_HOUSEHOLD.len().to_string()),
        ("credit_hi", usd_b(GST_CREDIT * hi / GST_RATE)),
        ("groceries_hi", usd_b(groceries * hi / GST_RATE)),
        // Land and property tax.
        ("land_needed_lo", land(LVT_CLAIM_LOW, LVT_RATES[1])),
        ("land_needed_hi", land(LVT_CLAIM_HIGH, LVT_RATES[0])),
        ("land_t", usd_t(LAND_VALUE_UNVERIFIED)),
        ("lvt_scenario", usd_b0(lvt_scenario())),
        (
            "lvt_all_land",
            usd_b0(lvt_revenue(LVT_RATES[1], LAND_VALUE_UNVERIFIED)),
        ),
        ("lvt_share_pct", short(100.0 * LVT_TAXABLE_SHARE_SCENARIO)),
        ("wealth_b", commas(&usd_b(NATIONAL_WEALTH_Q2_2025))),
        ("res_re_b", commas(&usd_b(RESIDENTIAL_REAL_ESTATE_Q2_2025))),
        (
            "property_b",
            usd_b(PROPERTY_TAX_PER_CAPITA_2024 * POPULATION_2024 / 1e6),
        ),
        (
            "lvt_replace_scen",
            p1(lvt_rate_to_replace_property_tax(scen_land)),
        ),
        // Comparison words.
        ("replaced_frac_a", share_words(1.0 - k_a)),
        ("base_frac_gdp", share_words(base0() / gdp())),
        (
            "mpc_verdict_a",
            if cen.breakeven_mpc(hi, core) > 1.0 {
                "no possible MPC closes the gap on its own"
            } else {
                "an MPC within the possible range closes the gap on its own"
            }
            .into(),
        ),
        (
            "g_hi_a_dir",
            if cen.breakeven_growth_factor(hi, core) > 1.0 {
                "larger"
            } else {
                "smaller"
            }
            .into(),
        ),
        (
            "property_vs_lvt",
            cmp_words(property_tax_total(), lvt_scenario()).into(),
        ),
        (
            "b_vs_a",
            if cb.revenue(hi) < cen.revenue(hi) {
                "lower"
            } else {
                "higher"
            }
            .into(),
        ),
        ("k_b_dir", if k_b > k_a { "rises" } else { "falls" }.into()),
        (
            "lvt_vs_design",
            if lvt_ratio >= 2.0 {
                "well above"
            } else if lvt_ratio > 1.0 {
                "above"
            } else {
                "within"
            }
            .into(),
        ),
        (
            "nz_condition",
            if nz_needed <= hi {
                "needs a VAT that performs like New Zealand's"
            } else {
                "needs a VAT that performs better than New Zealand's"
            }
            .into(),
        ),
        (
            "nz_verdict",
            if nz_needed <= hi {
                "is enough"
            } else if nz_needed <= hi + 2.0 {
                "comes close"
            } else {
                "is not enough"
            }
            .into(),
        ),
        (
            "growth_vs_original",
            if g_lo_pa > GROWTH_ADDITION_PP[1] {
                "faster than even the high end of"
            } else if g_lo_pa >= GROWTH_ADDITION_PP[0] {
                "within the range of"
            } else {
                "below"
            }
            .into(),
        ),
        (
            "mpc_effect_size",
            if cen.revenue(hi) - half.revenue(hi) < 0.25 * shortfall {
                "small"
            } else {
                "large"
            }
            .into(),
        ),
    ];
    // Sourced facts are filled in verbatim.
    for (k, v) in FACTS {
        e.push((k, (*v).to_string()));
    }
    // v3: debt-neutral transition (sections 10-12), recomputed from the transition API
    // and the raw constants with this file's formatters.
    {
        use vat_revenue_model::transition::{
            borrowing_test, levers, simulate, years_to_abolition, years_to_repay, Case,
        };
        let yrs =
            |c: &Case| years_to_abolition(c).map_or_else(|| "never".to_string(), |y| y.to_string());
        let cases = Case::all();
        let tags = ["low", "central", "high"];
        for (i, c) in cases.iter().enumerate() {
            let tag = tags[i];
            let leak: &'static str = Box::leak(format!("v3_settle_{tag}").into_boxed_str());
            e.push((leak, p1(100.0 * c.settled_k())));
            let n: &'static str = Box::leak(format!("v3_years_{tag}").into_boxed_str());
            e.push((n, yrs(c)));
            let n: &'static str = Box::leak(format!("v3_gap_{tag}").into_boxed_str());
            e.push((n, usd_b(c.residual_gap())));
            let n: &'static str = Box::leak(format!("v3_pkg_rate_{tag}").into_boxed_str());
            e.push((n, p1(levers(c).package_rate)));
            let n: &'static str = Box::leak(format!("v3_k_launch_{tag}").into_boxed_str());
            e.push((n, p1(100.0 * c.launch_k())));
        }
        let (lo_c, cen_c, hi_c) = (cases[0], cases[1], cases[2]);
        e.push(("v3_rate_alone", p1(levers(&cen_c).rate)));
        e.push(("v3_pkg_k_low", p1(100.0 * lo_c.with_package().launch_k())));
        e.push(("v3_pkg_years_low", yrs(&lo_c.with_package())));
        e.push((
            "v3_pkg_years_central",
            match years_to_abolition(&cen_c.with_package()) {
                Some(0) => "immediately once the levers are in force".into(),
                Some(y) => format!("after {y} years"),
                None => "never".into(),
            },
        ));
        e.push(("v3_cover_central", p1(100.0 * borrowing_test(&cen_c).2)));
        e.push(("v3_cover_high", p1(100.0 * borrowing_test(&hi_c).2)));
        e.push(("v3_spinup_gain_high", usd_b(borrowing_test(&hi_c).0)));
        e.push((
            "v3_interest",
            format!("{:.1}", 100.0 * PUBLIC_DEBT_CHARGES / UNMATURED_DEBT),
        ));
        e.push((
            "v3_pr_cut",
            usd_b(DIRECT_PROGRAM_EXPENSES * PROGRAM_REVIEW_DECLINE_PCT / 100.0),
        ));
        e.push(("v3_direct", usd_b(DIRECT_PROGRAM_EXPENSES)));
        e.push(("v3_pr_pct", short(PROGRAM_REVIEW_DECLINE_PCT)));
        e.push(("v3_k_end", p1(100.0 * cen.income_tax_kept(lo, core))));
        let shock = simulate(
            &cen_c,
            HORIZON_YEARS,
            Some((RECESSION_YEAR, RECESSION_BASE_DROP_PCT)),
        );
        e.push((
            "v3_recession_def",
            usd_b(core - shock[RECESSION_YEAR as usize].revenue),
        ));
        e.push((
            "v3_repay",
            years_to_repay(&cen_c, RECESSION_YEAR, RECESSION_BASE_DROP_PCT)
                .expect("repaid within the horizon")
                .to_string(),
        ));
        e.push(("v3_repay_design", REPAY_YEARS.to_string()));
        e.push(("v3_alpha", format!("{:.3}", IO_GOS_2022 / IO_GDP_2022)));
        e.push((
            "v3_capital_t",
            usd_t(NATIONAL_WEALTH_Q2_2025 - RESIDENTIAL_REAL_ESTATE_Q2_2025),
        ));
        e.push(("v3_metr", short(METR_2025)));
        e.push(("v3_metr_super", short(METR_2025_SUPER)));
        e.push(("v3_semi", short(USER_COST_SEMI_ELASTICITY_UNVERIFIED)));
        e.push(("v3_gap_net", short(GST_GAP_NET_2022)));
        e.push(("v3_gap_gross", short(GST_GAP_GROSS_2022)));
        e.push(("v3_deficit", usd_b(OPERATING_DEFICIT)));
        e.push(("v3_horizon", HORIZON_YEARS.to_string()));
        e.push(("v3_drop", short(RECESSION_BASE_DROP_PCT)));
        let path = simulate(&cen_c, HORIZON_YEARS, None);
        let inv = path.iter().map(|y| y.investment).fold(0.0_f64, f64::max);
        e.push(("v3_peak_inv_central", usd_b(inv)));
        e.push((
            "v3_peak_jobs_central",
            format!("{:.0}", inv * IO_JOBS_2022 / 1000.0),
        ));
        e.push(("v3_io_gdp", format!("{IO_GDP_2022:.3}")));
        e.push(("v3_io_jobs", format!("{IO_JOBS_2022:.3}")));
        let none = cases.iter().all(|c| years_to_abolition(c).is_none());
        e.push((
            "v3_verdict",
            if none {
                "does not close the gap in any case"
            } else {
                "closes the gap in at least one case"
            }
            .into(),
        ));
        e.push((
            "v3_pkg_verdict_low",
            if years_to_abolition(&lo_c.with_package()).is_none() {
                "still falls short"
            } else {
                "also closes"
            }
            .into(),
        ));
    }
    // Tables come straight from the table builders in `report.rs`, not from `docs.rs`.
    let tables: [(&str, TableFn); 22] = [
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
        ("table:transition", report::table_transition),
        ("table:transition_path", report::table_transition_path),
        ("table:recession", report::table_recession),
        ("table:spinup", report::table_spinup),
        ("table:levers", report::table_levers),
        ("table:counter_v3", report::table_counter_v3),
        ("table:key_figures", report::key_figures),
        ("table:readme", report::readme_table),
    ];
    for (k, f) in tables {
        e.push((k, f().expect("table").trim_end().to_string()));
    }
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in e {
        assert!(
            map.insert(k.to_string(), v).is_none(),
            "`{k}` listed twice in the test table"
        );
    }
    // The claims inputs table, rendered here from the ledger with the values above.
    let fill = |s: &str| -> String {
        let mut out = s.to_string();
        for (k, v) in &map {
            out = out.replace(&format!("{{{{{k}}}}}"), v);
        }
        assert!(
            !out.contains("{{"),
            "unfilled placeholder in ledger row: {out}"
        );
        out
    };
    let mut rows = vec![
        "| Claim | Status | Evidence |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for c in INPUTS {
        rows.push(format!(
            "| {} | {} | {} |",
            fill(c.claim),
            c.status.label(),
            fill(c.evidence)
        ));
    }
    map.insert("table:claims_inputs".into(), rows.join("\n"));
    map
}

/// Names of every `{{placeholder}}` used in the templates and the ledger rows.
fn used_names() -> std::collections::BTreeSet<String> {
    let mut text = format!("{PAPER_TEMPLATE}{README_TEMPLATE}{CLAIMS_TEMPLATE}");
    for c in INPUTS {
        text.push_str(c.claim);
        text.push_str(c.evidence);
    }
    occurrences_in(&text)
        .into_iter()
        .map(|(n, _, _)| n)
        .collect()
}

/// `(name, text before, text after)` for every `{{name}}` in `text`, line by line. The
/// context is the rest of the line, cut to 28 characters before and 16 after.
fn occurrences_in(text: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        let mut offset = 0;
        while let Some(i) = rest.find("{{") {
            let Some(j) = rest[i..].find("}}") else { break };
            let name = rest[i + 2..i + j].to_string();
            let start = offset + i;
            let end = offset + i + j + 2;
            let before: String = line[..start]
                .chars()
                .rev()
                .take(28)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            let after: String = line[end..].chars().take(16).collect();
            out.push((name, before, after));
            offset = end;
            rest = &line[end..];
        }
    }
    out
}

#[test]
fn values_match_model() {
    let got = values().expect("values");
    let want = expected();
    let missing: Vec<_> = got.keys().filter(|k| !want.contains_key(*k)).collect();
    let extra: Vec<_> = want.keys().filter(|k| !got.contains_key(*k)).collect();
    assert!(
        missing.is_empty(),
        "docs::values() names with no model expression in tests/values.rs: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "names in tests/values.rs that docs::values() lacks: {extra:?}"
    );
    let wrong: Vec<String> = want
        .iter()
        .filter(|(k, v)| got.get(*k) != Some(v))
        .map(|(k, v)| {
            format!(
                "{k}: docs.rs gives {:?}, the model gives {v:?}",
                got.get(k)
                    .map_or("", String::as_str)
                    .chars()
                    .take(120)
                    .collect::<String>()
            )
        })
        .collect();
    assert!(
        wrong.is_empty(),
        "values that do not match the model:\n{}",
        wrong.join("\n")
    );
}

#[test]
fn every_value_is_used_and_every_placeholder_has_a_value() {
    let got = values().expect("values");
    let used = used_names();
    // `{{placeholders}}` is the word used in doc text about the template syntax.
    let unknown: Vec<_> = used
        .iter()
        .filter(|n| !got.contains_key(*n) && n.as_str() != "placeholders")
        .collect();
    let unused: Vec<_> = got.keys().filter(|k| !used.contains(*k)).collect();
    assert!(
        unknown.is_empty(),
        "placeholders with no value: {unknown:?}"
    );
    assert!(
        unused.is_empty(),
        "values no template or ledger row uses: {unused:?}"
    );
}

#[test]
fn placeholders_sit_in_their_context() {
    let mut actual: Vec<(String, String, String, String)> = Vec::new();
    for (file, text) in [
        ("tax-whitepaper.md", PAPER_TEMPLATE),
        ("README.md", README_TEMPLATE),
        ("CLAIMS.md", CLAIMS_TEMPLATE),
    ] {
        for (n, b, a) in occurrences_in(text) {
            actual.push((file.into(), n, b, a));
        }
    }
    for c in INPUTS {
        for (n, b, a) in occurrences_in(&format!("{}\n{}", c.claim, c.evidence)) {
            actual.push(("ledger.rs".into(), n, b, a));
        }
    }
    actual.retain(|(_, n, _, _)| n != "placeholders");
    let pinned: Vec<(String, String, String, String)> = placements::PLACEMENTS
        .iter()
        .map(|(f, n, b, a)| ((*f).into(), (*n).into(), (*b).into(), (*a).into()))
        .collect();
    let mut problems = Vec::new();
    let mut left = pinned.clone();
    for occ in &actual {
        if let Some(i) = left.iter().position(|p| p == occ) {
            left.remove(i);
        } else {
            problems.push(format!(
                "{}: `{{{{{}}}}}` is not pinned to this context: …{}⟨{}⟩{}…",
                occ.0, occ.1, occ.2, occ.1, occ.3
            ));
        }
    }
    for p in left {
        problems.push(format!(
            "{}: pinned `{{{{{}}}}}` no longer appears in its context: …{}⟨{}⟩{}…",
            p.0, p.1, p.2, p.1, p.3
        ));
    }
    assert!(
        problems.is_empty(),
        "placeholder placement changed:\n{}",
        problems.join("\n")
    );
}

/// Prints the current placements in `placements.rs` form, to review a deliberate change.
#[test]
#[ignore = "helper: cargo test --test values dump_placements -- --ignored --nocapture"]
fn dump_placements() {
    for (file, text) in [
        ("tax-whitepaper.md", PAPER_TEMPLATE),
        ("README.md", README_TEMPLATE),
        ("CLAIMS.md", CLAIMS_TEMPLATE),
    ] {
        for (n, b, a) in occurrences_in(text) {
            println!("PLACE\t{file}\t{n}\t{b}\t{a}");
        }
    }
    for c in INPUTS {
        for (n, b, a) in occurrences_in(&format!("{}\n{}", c.claim, c.evidence)) {
            println!("PLACE\tledger.rs\t{n}\t{b}\t{a}");
        }
    }
}
