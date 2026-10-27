//! §14 next-generation backbone: the shocks behave as the paper says.

use vat_revenue_model::data::{
    CHOAM_TIER1_PRICE, CHOAM_UNIT_COST_LOW, CHOAM_YIELD_KWH, GST_NET, NEXTGEN_SHOCK_PCT_SCENARIO,
    PIT,
};
use vat_revenue_model::endstate::Package;
use vat_revenue_model::nextgen::{
    ai_deficit, ai_rate, balance, base_rate, biotech_rate, choam_payback, quantum_gaps,
    quantum_rate, space_rate, today_loss,
};

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn unshocked_rate_is_package_c() {
    assert!(close(base_rate(), Package::all()[2].rate(), 1e-9));
    assert!(close(balance(base_rate(), 1.0, 0.0), 0.0, 1e-3));
}

#[test]
fn cheaper_power_lengthens_choam_payback_by_one_over_one_minus_x() {
    let p0 = choam_payback(0.0);
    // CHOAM row E4 prints 76.8 yr.
    assert_eq!(format!("{p0:.1}"), "76.8");
    assert!(close(
        p0,
        CHOAM_UNIT_COST_LOW / (CHOAM_YIELD_KWH * CHOAM_TIER1_PRICE),
        1e-9
    ));
    for x in NEXTGEN_SHOCK_PCT_SCENARIO {
        assert!(close(choam_payback(x), p0 / (1.0 - x / 100.0), 1e-9));
        assert!(choam_payback(x) > p0);
    }
}

#[test]
fn wage_shock_raises_the_rate_and_today_loses_pit_plus_gst() {
    let mut prev = base_rate();
    for x in NEXTGEN_SHOCK_PCT_SCENARIO {
        let r = ai_rate(x);
        assert!(r > prev, "rate must rise with the shock");
        prev = r;
        assert!(close(
            balance(
                r,
                1.0 - vat_revenue_model::nextgen::ai_base_drop(x) / 100.0,
                0.0
            ),
            0.0,
            1e-3
        ));
        assert!(ai_deficit(x) > 0.0);
        let d = vat_revenue_model::nextgen::ai_base_drop(x);
        assert!(close(
            today_loss(x),
            PIT * x / 100.0 + GST_NET * d / 100.0,
            1e-6
        ));
    }
}

#[test]
fn quantum_low_case_is_today_and_cases_rise() {
    let g = quantum_gaps();
    assert!(close(quantum_rate(g[0]), base_rate(), 1e-9));
    assert!(quantum_rate(g[1]) > quantum_rate(g[0]));
    assert!(quantum_rate(g[2]) > quantum_rate(g[1]));
}

#[test]
fn need_shocks_move_the_rate_in_their_direction() {
    assert!(space_rate(1_000.0) > base_rate());
    assert!(close(biotech_rate(0.0), base_rate(), 1e-9));
    assert!(biotech_rate(-2.0) < base_rate());
    assert!(biotech_rate(2.0) > base_rate());
}
