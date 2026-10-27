//! Basic income and robotics (paper §14.8). Four ways to pay people in the end state,
//! each run through the §14 wage-base shocks with full pass-through to spending:
//!
//! - (a) the §13 prebate only;
//! - (b) the prebate plus the PBO's income-tested guaranteed basic income on top;
//! - (c) a universal basic income at the Ontario pilot's single amount for every resident
//!   aged 15 and over, replacing the prebate, funded by the VAT;
//! - (d) option (c) with the land value tax and wealth tax multiplied by a scenario factor.
//!
//! Each option solves for the VAT rate that balances package C. `None` means no rate up to
//! [`MAX_RATE`] balances the budget.

use crate::data::{OBIP_SINGLE, PBO_GBI_GROSS_2024_25, UBI_LAND_WEALTH_MULTIPLE_SCENARIO};
use crate::endstate::{base_scenario, credit_scaled, prebate_net, prebate_people};
use crate::model::{need_core, Scenario};
use crate::nextgen::{ai_base_drop, package};

/// Highest VAT rate searched, per cent.
pub const MAX_RATE: f64 = 100.0;

/// A way to pay people in the end state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Payment {
    /// (a) The §13 prebate only.
    Prebate,
    /// (b) Prebate plus the PBO guaranteed basic income.
    PrebatePlusGbi,
    /// (c) Universal basic income replacing the prebate.
    Ubi,
    /// (d) Universal basic income with higher land and wealth taxes.
    UbiLandWealth,
}

impl Payment {
    /// The four options, in table order.
    #[must_use]
    pub const fn all() -> [Payment; 4] {
        [
            Self::Prebate,
            Self::PrebatePlusGbi,
            Self::Ubi,
            Self::UbiLandWealth,
        ]
    }

    /// Label used in the tables.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Prebate => "(a) prebate only",
            Self::PrebatePlusGbi => "(b) prebate + PBO basic income",
            Self::Ubi => "(c) universal basic income, VAT-funded",
            Self::UbiLandWealth => "(d) (c) + higher land and wealth taxes",
        }
    }

    /// Net cost of the payments at VAT rate `r`, $M.
    #[must_use]
    pub fn transfer(self, r: f64) -> f64 {
        match self {
            Self::Prebate => prebate_net(r),
            Self::PrebatePlusGbi => prebate_net(r) + PBO_GBI_GROSS_2024_25,
            Self::Ubi | Self::UbiLandWealth => ubi_gross() - credit_scaled(r),
        }
    }

    /// Non-VAT revenue and savings, $M.
    #[must_use]
    pub fn others(self) -> f64 {
        let p = package();
        match self {
            Self::UbiLandWealth => {
                p.spending + (p.lvt + p.wealth) * UBI_LAND_WEALTH_MULTIPLE_SCENARIO
            }
            _ => p.others(),
        }
    }

    /// Budget balance at VAT rate `r` with the wage base down `drop` %, $M.
    #[must_use]
    pub fn balance(self, r: f64, drop: f64) -> f64 {
        let s0 = base_scenario();
        let s = Scenario {
            base_factor: s0.base_factor * (1.0 - ai_base_drop(drop) / 100.0),
            ..s0
        };
        s.revenue(r) - self.transfer(r) + self.others() - need_core()
    }

    /// VAT rate (%) that balances the budget with the wage base down `drop` %, or `None`
    /// if even [`MAX_RATE`] leaves a deficit.
    #[must_use]
    pub fn rate(self, drop: f64) -> Option<f64> {
        if self.balance(MAX_RATE, drop) < 0.0 {
            return None;
        }
        let (mut lo, mut hi) = (0.0_f64, MAX_RATE);
        for _ in 0..200 {
            let mid = (lo + hi) / 2.0;
            if self.balance(mid, drop) < 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some((lo + hi) / 2.0)
    }
}

/// Gross cost of a universal basic income at the pilot's single amount for every
/// resident aged 15 and over, $M.
#[must_use]
pub fn ubi_gross() -> f64 {
    OBIP_SINGLE * prebate_people() / 1e6
}
