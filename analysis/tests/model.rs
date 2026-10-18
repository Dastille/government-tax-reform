use vat_revenue_model::data::*;
use vat_revenue_model::model::*;
use vat_revenue_model::report::{named_scenarios, render};

const A: Incidence = Incidence::RealBaseFixed;
const B: Incidence = Incidence::NominalSpendingFixed;

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn baseline_matches_public_accounts() {
    assert_eq!(PIT, 234_319.0);
    assert_eq!(CIT, 96_954.0);
    assert_eq!(GST_NET, 52_503.0);
    assert_eq!(GST_BEFORE_REMISSION - GST_REMISSION, GST_GROSS_EXTERNAL);
    assert_eq!(GST_GROSS_EXTERNAL - GST_CREDIT, GST_NET);
    assert!(close(income_taxes(), 331_273.0, 0.5));
    assert!(close(need_core(), 383_776.0, 0.5));
}

#[test]
fn both_incidence_assumptions_reproduce_todays_gst_at_5_percent() {
    for inc in [A, B] {
        assert!(close(
            Scenario::static_case(inc).revenue(5.0),
            GST_NET,
            0.01
        ));
    }
}

#[test]
fn audit_finding_twenty_percent_raises_about_210b() {
    let st = Scenario::static_case(A);
    assert!(close(st.revenue(20.0), 210_012.0, 1.0));
    assert!(close(static_revenue(20.0, 0.0), st.revenue(20.0), 1e-6));
    assert!(st.revenue(20.0) < income_taxes());
    assert!(close(st.required_rate(need_core()), 36.55, 0.01));
}

#[test]
fn rate_path_is_consistent() {
    assert!(close(path_rate(0), 20.0, 1e-9));
    assert!(close(path_rate(5), 14.0, 1e-9));
    assert!(close(path_rate(10), 8.0, 1e-9));
    assert!(close(v11_path_rate(10), 9.2, 1e-9));
}

#[test]
fn dynamic_differs_from_static_only_by_spending() {
    for inc in [A, B] {
        let st = Scenario::static_case(inc);
        let d = Scenario::dynamic(inc);
        assert!(close(
            Scenario { mpc: 0.0, ..d }.revenue(20.0),
            st.revenue(20.0),
            1e-9
        ));
        assert!(d.revenue(20.0) > st.revenue(20.0));
        assert!(d.required_rate(need_core()) < st.required_rate(need_core()));
    }
}

#[test]
fn central_values() {
    let c = Scenario::central();
    assert!(close(taxable_share_central(), 0.614, 0.002));
    assert!(close(c.revenue(20.0), 242_200.0, 100.0));
    assert!(close(c.required_rate(need_core()), 32.1, 0.1));
    assert!(
        c.breakeven_mpc(20.0, need_core()) > 1.0,
        "MPC alone cannot close the gap"
    );
    assert!(close(c.income_tax_kept(20.0, need_core()), 0.473, 0.002));
}

#[test]
fn required_rate_actually_meets_need() {
    for inc in [A, B] {
        for (_, s) in named_scenarios(inc) {
            let r = s.required_rate(need_core());
            assert!(close(s.revenue(r), need_core(), 0.5));
        }
    }
}

#[test]
fn partial_replacement_balances() {
    for inc in [A, B] {
        let c = Scenario::dynamic(inc);
        for r in [10.0, 20.0, 30.0] {
            let k = c.income_tax_kept(r, need_core());
            assert!(close(
                c.vat_at(r, 1.0 - k) + k * income_taxes(),
                need_core(),
                0.5
            ));
        }
    }
}

/// Auditor check: under one incidence assumption, for the same rate, a scenario that
/// raises more revenue must need a lower-or-equal rate to meet the core need.
#[test]
fn higher_revenue_implies_lower_or_equal_required_rate() {
    for inc in [A, B] {
        let mut all: Vec<(String, Scenario)> = named_scenarios(inc)
            .into_iter()
            .map(|(k, s)| (k.to_string(), s))
            .collect();
        let d = Scenario::dynamic(inc);
        for m in [0.0, 0.25, 0.5, 0.75, 0.95, 1.0] {
            for share in [taxable_share_central(), 1.0] {
                for mult in [false, true] {
                    all.push((
                        format!("mpc {m} share {share:.2} mult {mult}"),
                        Scenario {
                            mpc: m,
                            taxable_share: share,
                            multiplier: mult,
                            ..d
                        },
                    ));
                }
            }
        }
        for rate in [8.0, 10.0, 15.0, 20.0, 25.0] {
            for (ki, si) in &all {
                for (kj, sj) in &all {
                    if si.revenue(rate) > sj.revenue(rate) + 1e-6 {
                        assert!(
                            si.required_rate(need_core()) <= sj.required_rate(need_core()) + 1e-9,
                            "{inc:?} at {rate}%: '{ki}' raises more than '{kj}' but needs a higher rate"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn lvt_land_requirement() {
    assert!(close(lvt_land_needed(50_000.0, 1.0), 5_000_000.0, 1e-6));
    assert!(close(
        lvt_revenue(1.0, LAND_VALUE_UNVERIFIED),
        64_000.0,
        1e-6
    ));
}

#[test]
fn committed_output_is_current() {
    let committed = include_str!("../output/revenue_tables.md");
    assert_eq!(
        committed,
        render(),
        "run `cargo run --release > output/revenue_tables.md`"
    );
}
