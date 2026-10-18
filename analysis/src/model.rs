//! Revenue arithmetic. All money in $ millions; rates in per cent unless a name says `frac`.

use crate::data::*;

/// Nominal GDP implied by the AFR revenue ratio (revenues / (revenues as % of GDP)).
pub fn gdp() -> f64 {
    TOTAL_REVENUE / (REVENUE_PCT_GDP / 100.0)
}

/// PIT + CIT: the income taxes the proposal abolishes first.
pub fn income_taxes() -> f64 {
    PIT + CIT
}

/// Revenue a VAT that absorbs the GST must raise to keep federal revenue unchanged
/// when PIT and CIT are abolished: PIT + CIT + today's net GST.
pub fn need_core() -> f64 {
    PIT + CIT + GST_NET
}

/// Full federal scope of the original design: also non-resident income tax and EI premiums.
pub fn need_full() -> f64 {
    need_core() + NON_RESIDENT + EI_PREMIUMS
}

/// Sum of a list of tax expenditures.
pub fn te_sum(items: &[TaxExpenditure]) -> f64 {
    items.iter().map(|(_, v)| v).sum()
}

// --------------------------------------------------------------------------- static

/// Static revenue at `rate` on today's net base plus `extra_at_5` of broadening (valued at 5%).
/// Scaling the net figure keeps the GST credit at the same share of revenue.
pub fn static_revenue(rate: f64, extra_at_5: f64) -> f64 {
    (GST_NET + extra_at_5) * rate / GST_RATE
}

/// Base factor for broadening the base by `extra_at_5` (valued at today's 5% rate).
pub fn broadening_factor(extra_at_5: f64) -> f64 {
    (GST_NET + extra_at_5) / GST_NET
}

/// Base factor for collecting like a VAT with revenue ratio `vrr` instead of Canada's.
pub fn vrr_factor(vrr: f64) -> f64 {
    vrr / VRR_CANADA
}

// --------------------------------------------------------------------------- dynamic

/// Effective tax-exclusive base implied by today's net GST: GST_NET / 5%.
pub fn base0() -> f64 {
    GST_NET / (GST_RATE / 100.0)
}

/// Taxable spending including today's 5% tax.
pub fn spending0() -> f64 {
    base0() * (1.0 + GST_RATE / 100.0)
}

/// Household (and NPISH) final consumption, $M: HFCE share x GDP.
pub fn hfce() -> f64 {
    HFCE_PCT_GDP / 100.0 * gdp()
}

/// Share of household consumption that lands in the effective GST base today:
/// base0 / (HFCE share x GDP). Used as the central "taxable share" of new spending.
pub fn taxable_share_central() -> f64 {
    base0() / hfce()
}

/// Average propensity to consume implied by a saving rate. An upper bound for the
/// marginal propensity to consume out of a tax cut.
pub fn apc(saving_rate: f64) -> f64 {
    1.0 - saving_rate
}

/// What happens to prices and nominal spending when the VAT rate changes.
/// Every scenario (static included) is computed under the same assumption, so
/// "dynamic" differs from "static" only by the spending effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incidence {
    /// Central. The VAT is passed fully into prices and households keep buying the same
    /// real (tax-exclusive) basket; nominal spending rises with the tax. Revenue on the
    /// existing base is linear in the rate: rate x base. This is how official static
    /// costings scale a sales tax.
    RealBaseFixed,
    /// Sensitivity. Prices rise by the tax but nominal spending on the existing base does
    /// not, so a higher rate buys a smaller real basket. Revenue = rate/(1+rate) x spending.
    NominalSpendingFixed,
}

impl Incidence {
    pub fn label(self) -> &'static str {
        match self {
            Incidence::RealBaseFixed => "A: real base fixed (central)",
            Incidence::NominalSpendingFixed => "B: nominal spending fixed",
        }
    }
}

/// One revenue scenario. Static is `mpc = 0`, no multiplier.
#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    pub incidence: Incidence,
    /// Marginal propensity to consume out of the income tax people keep (0 = static).
    pub mpc: f64,
    /// Share of new spending that falls in the taxable base.
    pub taxable_share: f64,
    /// Add Finance Canada's short-run demand multipliers (PIT 1.0, CIT 0.3).
    pub multiplier: bool,
    /// Multiplier on the whole taxable base: growth beyond needs, broadening, or VRR.
    pub base_factor: f64,
    /// Share of revenue above today's net GST lost to non-compliance (assumption).
    pub leak: f64,
}

impl Scenario {
    pub fn static_case(incidence: Incidence) -> Self {
        Scenario {
            incidence,
            mpc: 0.0,
            taxable_share: taxable_share_central(),
            multiplier: false,
            base_factor: 1.0,
            leak: 0.0,
        }
    }

    /// Ashlynn's mechanism with central, sourced values: MPC = 1 - saving rate (Q2 2025),
    /// taxable share calibrated to today's GST base, no multiplier, no growth.
    pub fn dynamic(incidence: Incidence) -> Self {
        Scenario {
            mpc: apc(SAVING_RATE_Q2_2025),
            ..Self::static_case(incidence)
        }
    }

    /// Central case used for headline figures: dynamic, incidence A.
    pub fn central() -> Self {
        Self::dynamic(Incidence::RealBaseFixed)
    }

    /// Extra nominal taxable spending when `removed` (0..1) of PIT + CIT is removed.
    pub fn extra_spending(&self, removed: f64) -> f64 {
        let first_round = self.mpc * self.taxable_share * income_taxes();
        let demand = if self.multiplier {
            (MULTIPLIER_PIT * PIT + MULTIPLIER_CIT * CIT) * spending0() / gdp()
        } else {
            0.0
        };
        removed * (first_round + demand)
    }

    /// Real (tax-exclusive) taxable base at `rate` (%) with `removed` of PIT + CIT gone.
    pub fn real_base_at(&self, rate: f64, removed: f64) -> f64 {
        let t = rate / 100.0;
        let new = self.extra_spending(removed) / (1.0 + t);
        let existing = match self.incidence {
            Incidence::RealBaseFixed => base0(),
            Incidence::NominalSpendingFixed => spending0() / (1.0 + t),
        };
        (existing + new) * self.base_factor
    }

    /// VAT revenue at `rate` (%) with `removed` share of PIT + CIT abolished.
    pub fn vat_at(&self, rate: f64, removed: f64) -> f64 {
        let gross = rate / 100.0 * self.real_base_at(rate, removed);
        if self.leak == 0.0 {
            gross
        } else {
            GST_NET + (gross - GST_NET) * (1.0 - self.leak)
        }
    }

    /// VAT revenue at `rate` with all of PIT + CIT abolished.
    pub fn revenue(&self, rate: f64) -> f64 {
        self.vat_at(rate, 1.0)
    }

    /// Real taxable base at `rate`, all of PIT + CIT abolished.
    pub fn real_base(&self, rate: f64) -> f64 {
        self.real_base_at(rate, 1.0)
    }

    /// Rate (%) at which revenue (all income tax abolished) equals `need`. Bisection;
    /// revenue is increasing in the rate under both incidence assumptions.
    pub fn required_rate(&self, need: f64) -> f64 {
        let (mut lo, mut hi) = (0.0_f64, 1000.0_f64);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if self.revenue(mid) < need {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    /// MPC that closes the gap at `rate` (revenue is linear in MPC). Can exceed 1.
    pub fn breakeven_mpc(&self, rate: f64, need: f64) -> f64 {
        let r0 = Scenario { mpc: 0.0, ..*self }.revenue(rate);
        let r1 = Scenario { mpc: 1.0, ..*self }.revenue(rate);
        (need - r0) / (r1 - r0)
    }

    /// Cumulative base factor that closes the gap at `rate` (revenue is linear in it
    /// when there is no leakage).
    pub fn breakeven_growth_factor(&self, rate: f64, need: f64) -> f64 {
        need / self.revenue(rate) * self.base_factor
    }

    /// Share k of today's PIT + CIT that must remain so that VAT at `rate` plus k x
    /// (PIT + CIT) equals `need`. VAT is linear in the removed share (1 - k).
    pub fn income_tax_kept(&self, rate: f64, need: f64) -> f64 {
        let all = self.vat_at(rate, 1.0);
        let none = self.vat_at(rate, 0.0);
        ((need - all) / (none - all + income_taxes())).max(0.0)
    }
}

/// Compound annual rate that produces cumulative factor `f` over `years`.
pub fn annualised(f: f64, years: u32) -> f64 {
    f.powf(1.0 / f64::from(years)) - 1.0
}

// --------------------------------------------------------------------------- rate path

/// Ashlynn's path, made internally consistent: 20% at launch (t = 0), falling
/// 1.2 points a year to 8% at t = 10.
pub fn path_rate(t: u32) -> f64 {
    START_RATE - (START_RATE - END_RATE) / f64::from(PATH_YEARS) * f64::from(t)
}

/// The formula committed in v1.1 (`20 - 1.2 x (year - 1)`), kept to document the bug.
pub fn v11_path_rate(year: u32) -> f64 {
    20.0 - 1.2 * (f64::from(year) - 1.0)
}

// --------------------------------------------------------------------------- land

/// Annual LVT revenue at `rate_pct` on `taxable_land` ($M).
pub fn lvt_revenue(rate_pct: f64, taxable_land: f64) -> f64 {
    rate_pct / 100.0 * taxable_land
}

/// Taxable land value needed to raise `revenue` at `rate_pct`.
pub fn lvt_land_needed(revenue: f64, rate_pct: f64) -> f64 {
    revenue / (rate_pct / 100.0)
}
