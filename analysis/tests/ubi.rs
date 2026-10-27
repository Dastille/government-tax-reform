//! §14.7 basic income and robotics: the options behave as the paper says.

use vat_revenue_model::data::{NEXTGEN_SHOCK_PCT_SCENARIO, OBIP_SINGLE, PBO_GBI_GROSS_2024_25};
use vat_revenue_model::endstate::prebate_people;
use vat_revenue_model::nextgen::ai_rate;
use vat_revenue_model::ubi::{ubi_gross, Payment};

fn rate(o: Payment, d: f64) -> f64 {
    o.rate(d).expect("balances below the search cap")
}

#[test]
fn prebate_option_matches_the_section_14_ai_rows() {
    for d in [0.0, 10.0, 25.0, 40.0] {
        assert!((rate(Payment::Prebate, d) - ai_rate(d)).abs() < 1e-6);
    }
}

#[test]
fn universal_cost_is_pilot_amount_times_eligible_people() {
    assert!((ubi_gross() - OBIP_SINGLE * prebate_people() / 1e6).abs() < 1e-6);
    assert!(ubi_gross() > PBO_GBI_GROSS_2024_25);
}

#[test]
fn basic_income_raises_the_rate_and_land_wealth_lowers_it_again() {
    for d in std::iter::once(0.0).chain(NEXTGEN_SHOCK_PCT_SCENARIO) {
        let a = rate(Payment::Prebate, d);
        let b = rate(Payment::PrebatePlusGbi, d);
        let c = rate(Payment::Ubi, d);
        let e = rate(Payment::UbiLandWealth, d);
        assert!(b > a && c > a, "a basic income costs more than the prebate");
        assert!(e < c, "more land and wealth tax lowers the VAT rate");
        assert!(Payment::Ubi.balance(c, d).abs() < 1e-3);
    }
}

#[test]
fn every_option_rises_with_the_robotics_shock() {
    for o in Payment::all() {
        let mut prev = rate(o, 0.0);
        for d in NEXTGEN_SHOCK_PCT_SCENARIO {
            let r = rate(o, d);
            assert!(r > prev, "{}: rate must rise with the shock", o.label());
            prev = r;
        }
    }
}
