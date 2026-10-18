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

/// Rate that raises `need` on today's base plus `extra_at_5`, static.
pub fn static_required_rate(need: f64, extra_at_5: f64) -> f64 {
    need / ((GST_NET + extra_at_5) / GST_RATE)
}

/// Static required rate if the base performed like a VAT with revenue ratio `vrr`.
pub fn vrr_required_rate(need: f64, vrr: f64) -> f64 {
    static_required_rate(need, 0.0) * VRR_CANADA / vrr
}

/// Static revenue when a fraction `leak` of the *incremental* revenue is lost to
/// non-compliance or avoidance. `leak` is an assumption, not a measured value.
pub fn leaky_revenue(rate: f64, leak: f64) -> f64 {
    GST_NET + (static_revenue(rate, 0.0) - GST_NET) * (1.0 - leak)
}

/// Rate needed under leakage `leak`.
pub fn leaky_required_rate(need: f64, leak: f64) -> f64 {
    GST_RATE * (1.0 + (need / GST_NET - 1.0) / (1.0 - leak))
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

/// The behavioural assumptions of the dynamic scenario.
#[derive(Clone, Copy, Debug)]
pub struct Dynamic {
    /// Marginal propensity to consume out of the money households and firms keep.
    pub mpc: f64,
    /// Share of that new spending that falls in the taxable base.
    pub taxable_share: f64,
    /// Add Finance Canada's short-run demand multipliers (PIT 1.0, CIT 0.3).
    pub multiplier: bool,
    /// Cumulative real growth of the base relative to the revenue requirement (1.0 = none).
    pub growth_factor: f64,
}

impl Dynamic {
    /// Ashlynn's mechanism with central, sourced values: MPC = 1 - saving rate (Q2 2025),
    /// taxable share calibrated to today's GST base, no multiplier, no growth.
    pub fn central() -> Self {
        Dynamic {
            mpc: apc(SAVING_RATE_Q2_2025),
            taxable_share: taxable_share_central(),
            multiplier: false,
            growth_factor: 1.0,
        }
    }

    /// Extra taxable spending per dollar of income tax removed (first round, plus
    /// multiplier-driven GDP times today's spending-to-GDP ratio).
    fn extra_spending(&self, removed_share: f64) -> f64 {
        let first_round = self.mpc * self.taxable_share * income_taxes();
        let demand = if self.multiplier {
            (MULTIPLIER_PIT * PIT + MULTIPLIER_CIT * CIT) * spending0() / gdp()
        } else {
            0.0
        };
        removed_share * (first_round + demand)
    }

    /// Tax-inclusive taxable spending after removing `removed_share` of PIT + CIT.
    pub fn spending(&self, removed_share: f64) -> f64 {
        (spending0() + self.extra_spending(removed_share)) * self.growth_factor
    }

    /// VAT revenue at `rate` (%) when all of PIT + CIT is removed. The VAT is fully
    /// passed into prices, so revenue = tau / (1 + tau) x tax-inclusive spending.
    pub fn revenue(&self, rate: f64) -> f64 {
        let t = rate / 100.0;
        t / (1.0 + t) * self.spending(1.0)
    }

    /// Real (tax-exclusive) taxable consumption at `rate`, all of PIT + CIT removed.
    pub fn real_base(&self, rate: f64) -> f64 {
        self.spending(1.0) / (1.0 + rate / 100.0)
    }

    /// Rate (%) that raises `need` when all of PIT + CIT is removed.
    pub fn required_rate(&self, need: f64) -> f64 {
        let a = need / self.spending(1.0);
        100.0 * a / (1.0 - a)
    }

    /// MPC that closes the gap at `rate` (other parameters fixed). Can exceed 1.
    pub fn breakeven_mpc(&self, rate: f64, need: f64) -> f64 {
        let t = rate / 100.0;
        let x_needed = need * (1.0 + t) / t / self.growth_factor;
        let demand = if self.multiplier {
            (MULTIPLIER_PIT * PIT + MULTIPLIER_CIT * CIT) * spending0() / gdp()
        } else {
            0.0
        };
        (x_needed - spending0() - demand) / (self.taxable_share * income_taxes())
    }

    /// Cumulative base growth (relative to needs) that closes the gap at `rate`.
    pub fn breakeven_growth_factor(&self, rate: f64, need: f64) -> f64 {
        need / self.revenue(rate) * self.growth_factor
    }

    /// Share of today's PIT + CIT that must stay in place so that VAT at `rate`
    /// plus the remaining income tax raises `need`. The removed share (1 - kept)
    /// drives the extra spending. Returns a value that may exceed 1.
    pub fn income_tax_kept(&self, rate: f64, need: f64) -> f64 {
        let a = (rate / 100.0) / (1.0 + rate / 100.0);
        let x0 = spending0() * self.growth_factor;
        let d = self.extra_spending(1.0) * self.growth_factor;
        let y = income_taxes();
        // a * (x0 + (1 - k) d) + k y = need  =>  k = (need - a x0 - a d) / (y - a d)
        ((need - a * x0 - a * d) / (y - a * d)).max(0.0)
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
