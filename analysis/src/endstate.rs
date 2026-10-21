//! The end state (paper §13): federal PIT and CIT at k = 0, budget balanced, no transition
//! path. Every package solves for the VAT rate at which
//!
//! `VAT(r) - net prebate cost(r) + other revenue + spending cut = core need`,
//!
//! on the central spin-up case's long-run base with the household GST exemptions taxed.
//! The prebate pays each resident aged 15 and over the VAT on the single-person Market
//! Basket Measure line (half the family-of-four line), and replaces the GST credit that the
//! model's base already nets out, so its net cost is the prebate less the scaled credit.

use crate::data::{
    BPA_2022, BRACKETS_2023, BRACKET_TOPS_2022, CHILD_SHARE_0_14_UNVERIFIED, GST_CREDIT, GST_RATE,
    MBM_FAMILY4_UNVERIFIED, POPULATION_2024, TE_HOUSEHOLD, WEALTH_TAX_2024_25,
};
use crate::model::Scenario;
use crate::model::{broadening_factor, lvt_scenario, need_core, taxable_share_central, te_sum};
use crate::transition::{program_review_cut, Case};

/// The VAT scenario of the end state: central case, long-run base at k = 0, household
/// exemptions taxed.
#[must_use]
pub fn base_scenario() -> Scenario {
    let c = Case::all()[1];
    Scenario {
        base_factor: c.long_run_factor(0.0) * broadening_factor(te_sum(&TE_HOUSEHOLD)),
        ..c.spending
    }
}

/// Share of household spending that is taxable once the household exemptions are taxed
/// (capped at 1).
#[must_use]
pub fn taxable_share_wide() -> f64 {
    (taxable_share_central() * broadening_factor(te_sum(&TE_HOUSEHOLD))).min(1.0)
}

/// MBM single-person line, $: half the family-of-four line (square-root scale).
#[must_use]
pub fn mbm_single() -> f64 {
    MBM_FAMILY4_UNVERIFIED / 2.0
}

/// Residents aged 15 and over.
#[must_use]
pub fn prebate_people() -> f64 {
    POPULATION_2024 * (1.0 - CHILD_SHARE_0_14_UNVERIFIED / 100.0)
}

/// Prebate per eligible person at VAT rate `r` (%), $: the VAT on spending up to the line.
#[must_use]
pub fn prebate_each(r: f64) -> f64 {
    r / 100.0 * taxable_share_wide() * mbm_single()
}

/// Gross prebate cost at `r`, $M.
#[must_use]
pub fn prebate_gross(r: f64) -> f64 {
    prebate_each(r) * prebate_people() / 1e6
}

/// GST credit scaled to `r`, $M: already netted out of the model's base.
#[must_use]
pub fn credit_scaled(r: f64) -> f64 {
    GST_CREDIT * r / GST_RATE
}

/// Net prebate cost at `r`, $M.
#[must_use]
pub fn prebate_net(r: f64) -> f64 {
    prebate_gross(r) - credit_scaled(r)
}

/// A funding package: levers besides the VAT, each $M per year.
#[derive(Clone, Copy, Debug)]
pub struct Package {
    /// Name used in the tables.
    pub name: &'static str,
    /// Program-Review-sized cut to direct program expenses.
    pub spending: f64,
    /// Land value tax.
    pub lvt: f64,
    /// Annual net wealth tax.
    pub wealth: f64,
}

impl Package {
    /// The three packages of §13.
    #[must_use]
    pub fn all() -> [Package; 3] {
        let s = program_review_cut();
        [
            Package {
                name: "A: VAT and spending review",
                spending: s,
                lvt: 0.0,
                wealth: 0.0,
            },
            Package {
                name: "B: A plus land value tax",
                spending: s,
                lvt: lvt_scenario(),
                wealth: 0.0,
            },
            Package {
                name: "C: B plus wealth tax",
                spending: s,
                lvt: lvt_scenario(),
                wealth: WEALTH_TAX_2024_25,
            },
        ]
    }

    /// Non-VAT contribution, $M.
    #[must_use]
    pub fn others(&self) -> f64 {
        self.spending + self.lvt + self.wealth
    }

    /// Budget balance at VAT rate `r`, $M (zero when balanced).
    #[must_use]
    pub fn balance(&self, r: f64) -> f64 {
        base_scenario().revenue(r) - prebate_net(r) + self.others() - need_core()
    }

    /// VAT rate (%) that balances the budget. Bisection; balance rises with the rate.
    #[must_use]
    pub fn rate(&self) -> f64 {
        let (mut lo, mut hi) = (0.0_f64, 200.0_f64);
        for _ in 0..200 {
            let mid = (lo + hi) / 2.0;
            if self.balance(mid) < 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    }
}

/// VAT rate (%) that balances package `p` with no prebate (the GST credit scaled instead).
#[must_use]
pub fn rate_without_prebate(p: &Package) -> f64 {
    base_scenario().required_rate(need_core() - p.others())
}

/// Federal income tax of a single filer, 2022 brackets and basic personal amount, $.
/// Ignores every credit except the basic personal amount.
#[must_use]
pub fn federal_pit_2022(income: f64) -> f64 {
    let mut tax = 0.0;
    let mut lower = 0.0;
    for (i, rate) in BRACKETS_2023.iter().enumerate() {
        let upper = BRACKET_TOPS_2022.get(i).copied().unwrap_or(f64::INFINITY);
        if income > lower {
            tax += (income.min(upper) - lower) * rate / 100.0;
        }
        lower = upper;
    }
    (tax - BRACKETS_2023[0] / 100.0 * BPA_2022).max(0.0)
}

/// One row of the distribution table, $ per year.
#[derive(Clone, Copy, Debug)]
pub struct Household {
    /// Total income.
    pub income: f64,
    /// Consumption.
    pub spending: f64,
    /// Today: federal PIT plus 5% GST on the taxable share.
    pub today: f64,
    /// End state: VAT at `rate` on the wider taxable share, less the prebate.
    pub end: f64,
}

impl Household {
    /// A household with `income` and `spending` under VAT rate `rate`.
    #[must_use]
    pub fn new(income: f64, spending: f64, rate: f64) -> Self {
        let today =
            federal_pit_2022(income) + GST_RATE / 100.0 * taxable_share_central() * spending;
        let end = rate / 100.0 * taxable_share_wide() * spending - prebate_each(rate);
        Self {
            income,
            spending,
            today,
            end,
        }
    }

    /// Gain (positive) or loss under the end state, $.
    #[must_use]
    pub fn gain(&self) -> f64 {
        self.today - self.end
    }
}

/// Incomes of the distribution rows, $: MBM line, 50k, top-10% line, 200k, top-1% line, 1M.
#[must_use]
pub fn dist_incomes() -> [f64; 6] {
    use crate::data::{DIST_INCOME_SCENARIO as S, TOP10_THRESHOLD_2022, TOP1_THRESHOLD_2022};
    [
        mbm_single(),
        S[0],
        TOP10_THRESHOLD_2022,
        S[1],
        TOP1_THRESHOLD_2022,
        S[2],
    ]
}

/// The distribution rows at VAT rate `rate`, then the retiree and asset-rich rows.
#[must_use]
pub fn distribution(rate: f64) -> Vec<Household> {
    use crate::data::{ASSET_RICH_SPENDING_SCENARIO, DIST_SPEND_SHARE_SCENARIO, RETIREE_SCENARIO};
    let mut v: Vec<Household> = dist_incomes()
        .iter()
        .zip(DIST_SPEND_SHARE_SCENARIO)
        .map(|(y, s)| Household::new(*y, y * s, rate))
        .collect();
    v.push(Household::new(
        RETIREE_SCENARIO[0],
        RETIREE_SCENARIO[1],
        rate,
    ));
    v.push(Household::new(0.0, ASSET_RICH_SPENDING_SCENARIO, rate));
    v
}
