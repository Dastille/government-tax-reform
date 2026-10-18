use vat_revenue_model::data::*;
use vat_revenue_model::model::*;
use vat_revenue_model::report::render;

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
fn audit_finding_twenty_percent_raises_about_210b() {
    assert!(close(static_revenue(20.0, 0.0), 210_012.0, 1.0));
    assert!(static_revenue(20.0, 0.0) < income_taxes());
    assert!(close(static_required_rate(need_core(), 0.0), 36.55, 0.01));
}

#[test]
fn rate_path_is_consistent() {
    assert!(close(path_rate(0), 20.0, 1e-9));
    assert!(close(path_rate(5), 14.0, 1e-9));
    assert!(close(path_rate(10), 8.0, 1e-9));
    // the v1.1 bug: year 10 gives 9.2%
    assert!(close(v11_path_rate(10), 9.2, 1e-9));
}

#[test]
fn dynamic_with_full_spending_reproduces_static_neutral_rate() {
    let d = Dynamic {
        mpc: 1.0,
        taxable_share: 1.0,
        ..Dynamic::central()
    };
    assert!(close(d.required_rate(need_core()), 36.55, 0.01));
}

#[test]
fn dynamic_central_values() {
    let c = Dynamic::central();
    assert!(close(taxable_share_central(), 0.614, 0.002));
    assert!(close(c.required_rate(need_core()), 42.1, 0.1));
    assert!(
        c.breakeven_mpc(20.0, need_core()) > 1.0,
        "MPC alone cannot close the gap"
    );
    assert!(close(c.income_tax_kept(20.0, need_core()), 0.561, 0.002));
}

#[test]
fn partial_replacement_balances() {
    let c = Dynamic::central();
    for r in [10.0, 20.0, 30.0] {
        let k = c.income_tax_kept(r, need_core());
        let t = r / 100.0;
        let vat = t / (1.0 + t) * (spending0() + (1.0 - k) * (c.spending(1.0) - spending0()));
        assert!(close(vat + k * income_taxes(), need_core(), 0.5));
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
