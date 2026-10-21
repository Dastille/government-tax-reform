//! Checks of the end state (paper §13): k = 0, every package balances, the prebate
//! offsets the VAT on spending up to the MBM line, and the distribution signs.

use vat_revenue_model::data::{GST_CREDIT, GST_RATE, MBM_FAMILY4_UNVERIFIED};
use vat_revenue_model::endstate::{
    base_scenario, distribution, federal_pit_2022, prebate_each, prebate_net, taxable_share_wide,
    Household, Package,
};
use vat_revenue_model::model::{income_taxes, need_core};

/// $1,000 tolerance on a $384B need ($M units).
const TOL: f64 = 1e-3;

#[test]
fn every_package_balances_with_no_income_tax() {
    for p in Package::all() {
        let r = p.rate();
        // Rebuild the balance by hand: VAT with all PIT and CIT removed (k = 0), less the
        // prebate, plus the scaled GST credit the base already nets out, plus other levers.
        let vat = base_scenario().vat_at(r, 1.0);
        let prebate = prebate_each(r) * vat_revenue_model::endstate::prebate_people() / 1e6;
        let bal = vat - prebate + GST_CREDIT * r / GST_RATE + p.others() - need_core();
        assert!(bal.abs() < TOL, "{}: balance {bal}", p.name);
        assert!(
            prebate_net(r) > 0.0,
            "{}: the prebate must cost money",
            p.name
        );
    }
}

#[test]
fn more_levers_mean_a_lower_rate() {
    let [a, b, c] = Package::all();
    assert!(a.rate() > b.rate() && b.rate() > c.rate());
}

#[test]
fn prebate_cancels_vat_at_the_line() {
    let line = MBM_FAMILY4_UNVERIFIED / 2.0;
    for p in Package::all() {
        let r = p.rate();
        let h = Household::new(line, line, r);
        assert!(
            h.end.abs() < 1e-6,
            "{}: net VAT at the line {}",
            p.name,
            h.end
        );
        let vat_on_line = r / 100.0 * taxable_share_wide() * line;
        assert!((prebate_each(r) - vat_on_line).abs() < 1e-9);
    }
}

#[test]
fn income_tax_is_zero_in_the_end_state() {
    // The end-state balance does not depend on income taxes: doubling them would change a
    // k > 0 system, but VAT at k = 0 already counts the full removal.
    let s = base_scenario();
    let r = Package::all()[1].rate();
    assert!((s.revenue(r) - s.vat_at(r, 1.0)).abs() < TOL);
    assert!(income_taxes() > 0.0);
}

#[test]
fn federal_pit_matches_hand_computed_2022_values() {
    // $60,000: 15% of 50,197 + 20.5% of 9,803 - 15% of 14,398.
    let want = 0.15 * 50_197.0 + 0.205 * (60_000.0 - 50_197.0) - 0.15 * 14_398.0;
    assert!((federal_pit_2022(60_000.0) - want).abs() < 1e-6);
    assert!(federal_pit_2022(10_000.0).abs() < 1e-9);
}

#[test]
fn asset_rich_pays_and_income_earners_gain() {
    let r = Package::all()[1].rate();
    let d = distribution(r);
    let rich = d.last().copied().expect("rows");
    assert!(rich.gain() < 0.0, "asset-rich spender must pay more");
    for h in &d[..6] {
        assert!(h.gain() > 0.0, "income {}: gain {}", h.income, h.gain());
    }
}
