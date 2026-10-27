//! The next-generation backbone (paper §14): five technology shocks run through the end
//! state of §13. Each shock is either a change in the VAT base (a factor on the model's
//! base) or a change in federal need ($M a year). Every shock solves package C (VAT, land
//! value tax, wealth tax, spending review, prebate) for the VAT rate that balances the
//! budget again, so each result is a rate in per cent, not a forecast.

use crate::data::{
    BIOTECH_HEALTH_SHIFT_PCT_SCENARIO, CHOAM_TIER1_PRICE, CHOAM_UNIT_COST_LOW, CHOAM_YIELD_KWH,
    GST_GAP_GROSS_2022, GST_GAP_NET_2022, GST_NET, HEALTH_SPENDING_2025, LABOUR_SHARE_2024, PIT,
    QUANTUM_GAP_MULTIPLE_SCENARIO, SPACE_PROCUREMENT_SCENARIO,
};
use crate::endstate::{base_scenario, prebate_net, Package};
use crate::model::{need_core, Scenario};

/// Package C of §13: the end-state package every shock is run through.
#[must_use]
pub fn package() -> Package {
    Package::all()[2]
}

/// Budget balance of package C at VAT rate `r`, $M, with the VAT base multiplied by
/// `base_scale` and federal need raised by `extra_need` $M. The prebate is per person, so
/// it does not move with the base.
#[must_use]
pub fn balance(r: f64, base_scale: f64, extra_need: f64) -> f64 {
    let s0 = base_scenario();
    let s = Scenario {
        base_factor: s0.base_factor * base_scale,
        ..s0
    };
    s.revenue(r) - prebate_net(r) + package().others() - need_core() - extra_need
}

/// VAT rate (%) that sets [`balance`] to zero. Bisection; balance rises with the rate.
#[must_use]
pub fn rate(base_scale: f64, extra_need: f64) -> f64 {
    let (mut lo, mut hi) = (0.0_f64, 200.0_f64);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if balance(mid, base_scale, extra_need) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// Package C's balancing rate with no shock (equals [`Package::rate`]).
#[must_use]
pub fn base_rate() -> f64 {
    rate(1.0, 0.0)
}

/// Energy: CHOAM energy-only payback, years, if the retail power price falls `drop` %.
/// Payback is unit cost over the value of the kWh, so it scales by `1 / (1 - drop)`.
#[must_use]
pub fn choam_payback(drop: f64) -> f64 {
    CHOAM_UNIT_COST_LOW / (CHOAM_YIELD_KWH * CHOAM_TIER1_PRICE * (1.0 - drop / 100.0))
}

/// AI: status-quo PIT lost if the wage base falls `drop` %, $M (PIT taken as
/// proportional to wages, which overstates the wage share of PIT).
#[must_use]
pub fn pit_loss(drop: f64) -> f64 {
    PIT * drop / 100.0
}

/// AI: consumption-base fall, per cent, if the wage base falls `drop` % and lost wages
/// are neither replaced nor saved differently: `drop × labour share`.
#[must_use]
pub fn ai_base_drop(drop: f64) -> f64 {
    drop * LABOUR_SHARE_2024 / 100.0
}

/// AI: package C rate when the wage base falls `drop` % with full pass-through to spending.
#[must_use]
pub fn ai_rate(drop: f64) -> f64 {
    rate(1.0 - ai_base_drop(drop) / 100.0, 0.0)
}

/// AI: package C deficit at the unshocked rate when the wage base falls `drop` %, $M.
#[must_use]
pub fn ai_deficit(drop: f64) -> f64 {
    -balance(base_rate(), 1.0 - ai_base_drop(drop) / 100.0, 0.0)
}

/// Quantum: the three GST/HST gaps, per cent: today's net gap, today's gross gap
/// (compliance and collection recover nothing), and a multiple of the gross gap.
#[must_use]
pub fn quantum_gaps() -> [f64; 3] {
    [
        GST_GAP_NET_2022,
        GST_GAP_GROSS_2022,
        GST_GAP_GROSS_2022 * QUANTUM_GAP_MULTIPLE_SCENARIO,
    ]
}

/// Quantum: package C rate when the gap rises to `gap` %. The model base already nets
/// today's net gap, so only the excess shrinks the base.
#[must_use]
pub fn quantum_rate(gap: f64) -> f64 {
    rate(1.0 - (gap - GST_GAP_NET_2022) / 100.0, 0.0)
}

/// Space: package C rate with `extra` $M a year of procurement added to need.
#[must_use]
pub fn space_rate(extra: f64) -> f64 {
    rate(1.0, extra)
}

/// Space: the three procurement cases, $M.
#[must_use]
pub fn space_cases() -> [f64; 3] {
    SPACE_PROCUREMENT_SCENARIO
}

/// Biotech: change in total health spending, $M, for a `shift` % change.
#[must_use]
pub fn health_change(shift: f64) -> f64 {
    HEALTH_SPENDING_2025 * shift / 100.0
}

/// Biotech: package C rate if the federal budget bore the whole health change (an upper
/// bound; provinces pay most health spending).
#[must_use]
pub fn biotech_rate(shift: f64) -> f64 {
    rate(1.0, health_change(shift))
}

/// Biotech: the three shifts, per cent.
#[must_use]
pub fn biotech_cases() -> [f64; 3] {
    BIOTECH_HEALTH_SHIFT_PCT_SCENARIO
}

/// AI: revenue today's system loses when the wage base falls `drop` % with full
/// pass-through, $M: the PIT loss plus net GST on the smaller spending base.
#[must_use]
pub fn today_loss(drop: f64) -> f64 {
    pit_loss(drop) + GST_NET * ai_base_drop(drop) / 100.0
}
