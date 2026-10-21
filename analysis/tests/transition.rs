//! Checks of the debt-neutral transition rule and the spin-up scenario (paper §10–12).

use vat_revenue_model::data::{
    HORIZON_YEARS, RECESSION_BASE_DROP_PCT, RECESSION_YEAR, REPAY_YEARS, START_RATE,
};
use vat_revenue_model::model::{income_taxes, need_core};
use vat_revenue_model::transition::{levers, simulate, years_to_abolition, years_to_repay, Case};

/// $1,000 tolerance on a $384B need, for floating-point rounding.
const TOL: f64 = 1e-3;

#[test]
fn deficit_never_rises_without_a_shock() {
    for c in Case::all()
        .iter()
        .chain(Case::all().map(|c| c.with_package()).iter())
    {
        for y in simulate(c, HORIZON_YEARS, None) {
            assert!(
                y.deficit_change <= TOL,
                "{} year {}: deficit change {} > 0",
                c.name,
                y.t,
                y.deficit_change
            );
        }
    }
}

#[test]
fn cuts_never_exceed_the_structural_surplus() {
    let it = income_taxes();
    for c in Case::all() {
        let s = simulate(&c, HORIZON_YEARS, None);
        for w in s.windows(2) {
            let cut = (w[0].k - w[1].k) * it * w[0].structural_factor;
            let surplus = (w[0].structural_revenue - c.need()).max(0.0);
            let realised = (w[0].revenue - c.need()).max(0.0);
            assert!(
                cut <= surplus + TOL && cut <= realised + TOL,
                "{} year {}: cut {cut} exceeds structural {surplus} or realised {realised}",
                c.name,
                w[0].t
            );
        }
    }
}

#[test]
fn recession_triggers_reversal_and_repayment() {
    let c = Case::all()[1];
    let s = simulate(
        &c,
        HORIZON_YEARS,
        Some((RECESSION_YEAR, RECESSION_BASE_DROP_PCT)),
    );
    let y = RECESSION_YEAR as usize;
    assert!(
        s[y].deficit_change > 0.0,
        "the shock year must show a deficit"
    );
    assert!(
        s[y + 1].k > s[y].k,
        "the rule must raise k after a shortfall"
    );
    let years = years_to_repay(&c, RECESSION_YEAR, RECESSION_BASE_DROP_PCT)
        .expect("account repaid within the horizon");
    assert!(years <= REPAY_YEARS + 1, "repaid in {years} years");
    // Every year after the shock is at or below the baseline again.
    for later in &s[y + 1..] {
        assert!(later.deficit_change <= TOL, "year {}", later.t);
    }
}

#[test]
fn no_case_reaches_full_abolition() {
    for c in Case::all() {
        assert_eq!(years_to_abolition(&c), None, "{}", c.name);
        assert!(c.residual_gap() > 0.0, "{}", c.name);
        assert!(
            c.settled_k() > 0.0 && c.settled_k() < c.launch_k(),
            "{}",
            c.name
        );
    }
}

#[test]
fn simulation_converges_to_settled_k() {
    for c in Case::all() {
        let s = simulate(&c, HORIZON_YEARS, None);
        let end = s.last().expect("years").k;
        assert!(
            (end - c.settled_k()).abs() < 1e-3,
            "{}: simulated {end}, settled {}",
            c.name,
            c.settled_k()
        );
    }
}

#[test]
fn launch_balances_the_need() {
    for c in Case::all() {
        let r = c.revenue(c.launch_k(), 1.0);
        assert!((r - need_core()).abs() < 1.0, "{}: {r}", c.name);
    }
}

#[test]
fn package_closes_the_central_case() {
    let cases = Case::all();
    let central = cases[1];
    assert_eq!(years_to_abolition(&central.with_package()), Some(0));
    assert!(levers(&central).package_rate <= START_RATE);
    // The low case is not closed by the package at the rule rate.
    assert_eq!(years_to_abolition(&cases[0].with_package()), None);
}

#[test]
fn levers_alone_rate_meets_the_need() {
    for c in Case::all() {
        let l = levers(&c);
        assert!(l.rate > START_RATE, "{}: rate {}", c.name, l.rate);
        assert!(
            l.gap > l.broadening,
            "{}: broadening alone closes it",
            c.name
        );
    }
}
