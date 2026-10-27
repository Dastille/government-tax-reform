//! Debt-neutral transition rule and industry spin-up scenario (paper sections 10-12).
//!
//! The rule: the VAT is held at [`RULE_RATE`] and the income-tax share `k` falls only by
//! revenue actually realised beyond the core need, after repaying any shortfall booked in
//! a compensation account, and never by more than the structural surplus (revenue without
//! the temporary build-out demand). If realised revenue falls short, cuts pause; if the
//! structural surplus is negative, `k` rises by the shortfall (reversal), and while the
//! account is owed, `k` is raised enough to repay it over [`REPAY_YEARS`] years. The VAT rate
//! steps down only after `k` reaches zero.
//!
//! The spin-up: lower income tax lowers the METR, the capital stock moves toward a higher
//! target at a set speed, and the larger capital stock raises the taxable base in
//! proportion to the capital share of GDP. Investment during the build-out adds temporary
//! demand. These are scenarios, not forecasts.

use crate::data::{
    DIRECT_PROGRAM_EXPENSES, GST_RATE, HORIZON_YEARS, IO_GDP_2022, IO_GOS_2022, IO_IMPORTS_2022,
    IO_JOBS_2022, METR_2025_SUPER, NATIONAL_WEALTH_Q2_2025, PROGRAM_REVIEW_DECLINE_PCT,
    PUBLIC_DEBT_CHARGES, REPAY_YEARS, RESIDENTIAL_REAL_ESTATE_Q2_2025, SPINUP_ELASTICITY_SCALE,
    SPINUP_METR_CUT_PP, SPINUP_SPEED, START_RATE, TE_HOUSEHOLD, UNMATURED_DEBT,
    USER_COST_SEMI_ELASTICITY_UNVERIFIED,
};
use crate::model::{
    broadening_factor, gdp, income_taxes, lvt_scenario, need_core, te_sum, Scenario,
};

/// VAT rate (%) held under the rule until the income tax is gone.
pub const RULE_RATE: f64 = START_RATE;

/// Capital share of GDP: gross operating surplus over GDP at market prices, per dollar
/// of final demand (Statistics Canada Table 36-10-0013-01, 2022). Derived.
#[must_use]
pub fn capital_share() -> f64 {
    IO_GOS_2022 / IO_GDP_2022
}

/// Non-residential capital proxy, $M: national wealth less residential real estate. It
/// includes land and government assets, so it overstates business capital; this makes
/// the spin-up larger, not smaller.
#[must_use]
pub fn capital_proxy() -> f64 {
    NATIONAL_WEALTH_Q2_2025 - RESIDENTIAL_REAL_ESTATE_Q2_2025
}

/// One spin-up case.
#[derive(Clone, Copy, Debug)]
pub struct Case {
    /// Label.
    pub name: &'static str,
    /// Spending scenario (MPC, multiplier) used for VAT revenue.
    pub spending: Scenario,
    /// Per cent rise in the capital stock per point of METR cut.
    pub semi_elasticity: f64,
    /// METR cut, points, when all federal income tax is gone.
    pub metr_cut: f64,
    /// Share of the remaining capital gap closed each year.
    pub speed: f64,
    /// Base factor from broadening (1.0 = today's base).
    pub broadening: f64,
    /// Reduction in the need from spending cuts and other revenue, $M.
    pub need_cut: f64,
}

impl Case {
    /// Low, central and high cases.
    #[must_use]
    pub fn all() -> [Case; 3] {
        let cen = Scenario::central();
        let half = Scenario { mpc: 0.5, ..cen };
        let mul = Scenario {
            multiplier: true,
            ..cen
        };
        let e = USER_COST_SEMI_ELASTICITY_UNVERIFIED;
        [
            Case {
                name: "Low",
                spending: half,
                semi_elasticity: e * SPINUP_ELASTICITY_SCALE[0],
                metr_cut: SPINUP_METR_CUT_PP[0],
                speed: SPINUP_SPEED[0],
                broadening: 1.0,
                need_cut: 0.0,
            },
            Case {
                name: "Central",
                spending: cen,
                semi_elasticity: e * SPINUP_ELASTICITY_SCALE[1],
                metr_cut: SPINUP_METR_CUT_PP[1],
                speed: SPINUP_SPEED[1],
                broadening: 1.0,
                need_cut: 0.0,
            },
            Case {
                name: "High",
                spending: mul,
                semi_elasticity: e * SPINUP_ELASTICITY_SCALE[2],
                metr_cut: METR_2025_SUPER,
                speed: SPINUP_SPEED[2],
                broadening: 1.0,
                need_cut: 0.0,
            },
        ]
    }

    /// The same case with the three levers in place: household GST exemptions taxed, a
    /// Program-Review-sized spending cut, and the LVT scenario.
    #[must_use]
    pub fn with_package(&self) -> Case {
        Case {
            broadening: broadening_factor(te_sum(&TE_HOUSEHOLD)),
            need_cut: program_review_cut() + lvt_scenario(),
            ..*self
        }
    }

    /// Need the rule must cover, $M.
    #[must_use]
    pub fn need(&self) -> f64 {
        need_core() - self.need_cut
    }

    /// Long-run capital gain (fraction) when share `k` of income tax remains.
    #[must_use]
    pub fn capital_target(&self, k: f64) -> f64 {
        self.semi_elasticity / 100.0 * self.metr_cut * (1.0 - k)
    }

    /// Long-run base factor at share `k`.
    #[must_use]
    pub fn long_run_factor(&self, k: f64) -> f64 {
        1.0 + capital_share() * self.capital_target(k)
    }

    /// Federal revenue replacing PIT + CIT + GST: VAT at [`RULE_RATE`] with `1 - k` of
    /// the income tax removed, plus `k` of the income tax, both on a base scaled by
    /// `factor`.
    #[must_use]
    pub fn revenue(&self, k: f64, factor: f64) -> f64 {
        Scenario {
            base_factor: factor * self.broadening,
            ..self.spending
        }
        .vat_at(RULE_RATE, 1.0 - k)
            + k * income_taxes() * factor
    }

    /// Share `k` that must remain at launch.
    #[must_use]
    pub fn launch_k(&self) -> f64 {
        Scenario {
            base_factor: self.broadening,
            ..self.spending
        }
        .income_tax_kept(RULE_RATE, self.need())
    }

    /// Long-run share `k` where revenue at the long-run base equals the need (bisection);
    /// zero if the need is met with no income tax.
    #[must_use]
    pub fn settled_k(&self) -> f64 {
        let n = self.need();
        if self.revenue(0.0, self.long_run_factor(0.0)) >= n {
            return 0.0;
        }
        let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
        for _ in 0..200 {
            let mid = (lo + hi) / 2.0;
            if self.revenue(mid, self.long_run_factor(mid)) < n {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    }

    /// Annual gap, $M, with all income tax gone at the long-run base.
    #[must_use]
    pub fn residual_gap(&self) -> f64 {
        (self.need() - self.revenue(0.0, self.long_run_factor(0.0))).max(0.0)
    }
}

/// One simulated year.
#[derive(Clone, Copy, Debug)]
pub struct Year {
    /// Years after launch.
    pub t: u32,
    /// Share of today's PIT + CIT in force this year.
    pub k: f64,
    /// Capital stock gain over the launch level (fraction).
    pub capital: f64,
    /// Base factor from the larger capital stock (permanent).
    pub structural_factor: f64,
    /// Realised revenue, $M.
    pub revenue: f64,
    /// Structural revenue (no build-out demand, no shock), $M.
    pub structural_revenue: f64,
    /// Deficit change against the baseline, $M (positive = higher deficit).
    pub deficit_change: f64,
    /// Compensation account balance owed at year end, $M.
    pub owed: f64,
    /// Build-out investment this year, $M.
    pub investment: f64,
}

/// Simulate `years` years of the rule for `case`. `shock` is `(year, base drop %)`.
#[must_use]
pub fn simulate(case: &Case, years: u32, shock: Option<(u32, f64)>) -> Vec<Year> {
    let n = case.need();
    let it = income_taxes();
    let k0 = case.launch_k();
    let mut k = k0;
    let (mut cap, mut prev_cap, mut owed) = (0.0_f64, 0.0_f64, 0.0_f64);
    let mut out = Vec::new();
    for t in 0..=years {
        let fs = 1.0 + capital_share() * cap;
        let investment = (cap - prev_cap) * capital_proxy();
        let build = investment * IO_GDP_2022 / gdp();
        let mut f = fs + build;
        if let Some((year, drop)) = shock {
            if t == year {
                f *= 1.0 - drop / 100.0;
            }
        }
        let revenue = case.revenue(k, f);
        let structural_revenue = case.revenue(k, fs);
        let deficit_change = n - revenue;
        let mut next = k;
        if deficit_change > 0.0 {
            owed += deficit_change;
            let structural = structural_revenue - n;
            if structural < 0.0 {
                next = (k - structural / (it * fs)).min(1.0);
            }
        } else {
            let mut spare = -deficit_change;
            let repay = spare.min(owed);
            owed -= repay;
            spare -= repay;
            let cut = spare.min((structural_revenue - n).max(0.0));
            next = (k - cut / (it * fs)).max(0.0);
        }
        if owed > 0.0 {
            next = (next + owed / f64::from(REPAY_YEARS) / (it * fs)).min(1.0);
        }
        out.push(Year {
            t,
            k,
            capital: cap,
            structural_factor: fs,
            revenue,
            structural_revenue,
            deficit_change,
            owed,
            investment,
        });
        prev_cap = cap;
        cap += case.speed * (case.capital_target(next) - cap);
        k = next;
    }
    out
}

/// First year in which `k` is zero within the horizon, if any.
#[must_use]
pub fn years_to_abolition(case: &Case) -> Option<u32> {
    simulate(case, HORIZON_YEARS, None)
        .iter()
        .find(|y| y.k <= 0.0)
        .map(|y| y.t)
}

/// Years after the shock year until the compensation account is back to zero.
#[must_use]
pub fn years_to_repay(case: &Case, year: u32, drop: f64) -> Option<u32> {
    simulate(case, HORIZON_YEARS, Some((year, drop)))
        .iter()
        .find(|y| y.t > year && y.owed <= 0.0)
        .map(|y| y.t - year)
}

/// Program-Review-sized cut: the 1994-95 Program Review's percentage decline applied to
/// today's direct program expenses, $M.
#[must_use]
pub fn program_review_cut() -> f64 {
    PROGRAM_REVIEW_DECLINE_PCT / 100.0 * DIRECT_PROGRAM_EXPENSES
}

/// Levers that could close the residual gap of `case`, each in $M per year.
#[derive(Clone, Copy, Debug)]
pub struct Levers {
    /// Residual gap with no income tax, long-run base, $M.
    pub gap: f64,
    /// VAT rate (%) that closes the gap alone.
    pub rate: f64,
    /// Revenue from taxing the household GST tax expenditures at the rule rate.
    pub broadening: f64,
    /// Program-Review-sized cut to direct program expenses.
    pub spending: f64,
    /// LVT scenario revenue (land value unverified).
    pub lvt: f64,
    /// VAT rate (%) needed after broadening, the spending cut and the LVT.
    pub package_rate: f64,
}

/// Lever arithmetic for `case`.
#[must_use]
pub fn levers(case: &Case) -> Levers {
    let n = need_core();
    let f0 = case.long_run_factor(0.0);
    let s = Scenario {
        base_factor: f0,
        ..case.spending
    };
    let extra = te_sum(&TE_HOUSEHOLD);
    let broadening = extra * RULE_RATE / GST_RATE * f0;
    let spending = program_review_cut();
    let lvt = lvt_scenario();
    let wide = Scenario {
        base_factor: f0 * broadening_factor(extra),
        ..case.spending
    };
    Levers {
        gap: case.residual_gap(),
        rate: s.required_rate(n),
        broadening,
        spending,
        lvt,
        package_rate: wide.required_rate(n - spending - lvt),
    }
}

/// Average effective interest rate on unmatured debt, per cent (derived, AFR 2024-25).
#[must_use]
pub fn interest_rate() -> f64 {
    100.0 * PUBLIC_DEBT_CHARGES / UNMATURED_DEBT
}

/// Peak-year build-out of `case`: (investment $M, GDP $M, imports $M, jobs).
#[must_use]
pub fn peak_build(case: &Case) -> (f64, f64, f64, f64) {
    let inv = simulate(case, HORIZON_YEARS, None)
        .iter()
        .map(|y| y.investment)
        .fold(0.0_f64, f64::max);
    (
        inv,
        inv * IO_GDP_2022,
        inv * IO_IMPORTS_2022,
        inv * IO_JOBS_2022,
    )
}

/// Borrow-to-abolish test at the long-run base of `case`: (annual spin-up revenue $M,
/// annual gap $M, ratio of the two). A ratio below one means the cut does not pay for
/// itself even before interest.
#[must_use]
pub fn borrowing_test(case: &Case) -> (f64, f64, f64) {
    let gain = case.revenue(0.0, case.long_run_factor(0.0)) - case.revenue(0.0, 1.0);
    let gap = case.residual_gap();
    (gain, gap, gain / (gain + gap))
}
