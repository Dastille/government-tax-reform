//! Every input number, with its primary source. Units: millions of Canadian dollars
//! unless stated otherwise. Nothing in this file is estimated by us; derived values
//! live in `model.rs`.

// ---------------------------------------------------------------------------
// Public Accounts of Canada 2025, Volume I, Section 3, Table 3.2 "Tax revenues"
// and Table 3.3 "Goods and services tax" (fiscal year ended March 31, 2025).
// https://www.tpsgc-pwgsc.gc.ca/recgen/cpc-pac/2025/vol1/s3/rvnu-eng.html
// Same totals: Annual Financial Report 2024-25, Table 4 "Revenues".
// https://www.canada.ca/en/department-finance/services/publications/annual-financial-report/2025.html
// ---------------------------------------------------------------------------
/// Personal income tax, FY2024-25 (PA 2025 Table 3.2).
pub const PIT: f64 = 234_319.0;
/// Corporate income tax, FY2024-25 (PA 2025 Table 3.2).
pub const CIT: f64 = 96_954.0;
/// Non-resident income tax, FY2024-25 (PA 2025 Table 3.2).
pub const NON_RESIDENT: f64 = 13_528.0;
/// Employment Insurance premiums, FY2024-25 (AFR 2024-25 Table 4). A federal payroll tax.
pub const EI_PREMIUMS: f64 = 31_530.0;
/// Total federal revenues, FY2024-25 (AFR 2024-25 Table 4).
pub const TOTAL_REVENUE: f64 = 510_951.0;
/// Revenues as % of GDP, FY2024-25 (AFR 2024-25 Table 2, "Financial results").
pub const REVENUE_PCT_GDP: f64 = 16.6;

/// GST line before deductions (PA 2025 Table 3.3).
pub const GST_BEFORE_REMISSION: f64 = 62_334.0;
/// Remission order for GST paid by federal ministries (internal to government) (PA 2025 Table 3.3).
pub const GST_REMISSION: f64 = 3_565.0;
/// Gross GST from outside parties (PA 2025 Table 3.3).
pub const GST_GROSS_EXTERNAL: f64 = 58_769.0;
/// Quarterly GST credit payments deducted from GST revenue (PA 2025 Table 3.3).
pub const GST_CREDIT: f64 = 6_266.0;
/// Net GST from outside parties = the $52.5B headline (PA 2025 Table 3.3).
/// Federal 5% GST plus the federal 5-point portion of HST; provincial HST portions excluded.
pub const GST_NET: f64 = 52_503.0;
/// Statutory federal GST rate, per cent.
pub const GST_RATE: f64 = 5.0;

// ---------------------------------------------------------------------------
// Report on Federal Tax Expenditures - Concepts, Estimates and Evaluations 2026, Part 2,
// calendar-year 2024 column (a projection in that edition). "GST" = federal component only.
// https://www.canada.ca/en/department-finance/services/publications/federal-tax-expenditures/2026/part-2.html
// ---------------------------------------------------------------------------
/// One base-broadening item: (label, 2024 cost at the 5% rate, $M).
pub type TaxExpenditure = (&'static str, f64);

/// Zero-rated or exempt household purchases.
pub const TE_HOUSEHOLD: [TaxExpenditure; 11] = [
    ("Zero-rating of basic groceries", 6_620.0),
    ("Exemption: certain residential rent", 2_590.0),
    ("Exemption: certain supplies by charities and NPOs", 1_900.0),
    ("Zero-rating of prescription drugs", 1_260.0),
    ("Exemption: tuition and educational services", 1_210.0),
    ("Exemption: health care services", 1_200.0),
    ("Zero-rating of medical and assistive devices", 905.0),
    ("Exemption: water, sewage and basic garbage", 480.0),
    ("Exemption: municipal transit", 230.0),
    ("Exemption: child care", 200.0),
    ("Zero-rating of feminine hygiene products", 55.0),
];

/// Rebates to public bodies, plus the small-supplier threshold. Removing the public-body
/// rebates moves money between governments rather than raising it from households.
pub const TE_PUBLIC_AND_THRESHOLD: [TaxExpenditure; 6] = [
    ("Rebate for municipalities", 3_875.0),
    ("Rebate for hospitals, facility operators", 1_290.0),
    ("Rebate for schools, colleges and universities", 1_190.0),
    ("Rebate for registered charities", 465.0),
    ("Small suppliers' threshold", 260.0),
    ("Rebate for qualifying NPOs", 115.0),
];

/// Targeted Temporary GST/HST Relief (Dec 14, 2024 to Feb 15, 2025): 445 (2024) + 840 (2025).
/// The whole holiday fell inside FY2024-25, so it depressed the GST_NET figure.
pub const GST_HOLIDAY_COST: f64 = 445.0 + 840.0;

// ---------------------------------------------------------------------------
// OECD (2024), Consumption Tax Trends 2024, https://doi.org/10.1787/dcd4dd36-en
// Annex Table 2.A.7 (p. 111): VAT Revenue Ratio 2022. Canada's VRR covers federal GST only.
// ---------------------------------------------------------------------------
/// Canada VRR 2022.
pub const VRR_CANADA: f64 = 0.50;
/// New Zealand VRR 2022 (single 15% rate, very broad base, public services taxable).
pub const VRR_NEW_ZEALAND: f64 = 0.96;
/// OECD unweighted average VRR 2022.
pub const VRR_OECD_AVERAGE: f64 = 0.58;

// ---------------------------------------------------------------------------
// Behaviour and macro parameters.
// ---------------------------------------------------------------------------
/// Household saving rate, Q2 2025 (Statistics Canada, The Daily, 2025-09-11): 5.0%.
/// https://www150.statcan.gc.ca/n1/daily-quotidien/250911/dq250911a-eng.htm
pub const SAVING_RATE_Q2_2025: f64 = 0.050;
/// Household saving rate, Q2 2024 (Statistics Canada, The Daily, 2024-09-12): 7.2%.
/// https://www150.statcan.gc.ca/n1/daily-quotidien/240912/dq240912a-eng.htm
pub const SAVING_RATE_Q2_2024: f64 = 0.072;
/// Household + NPISH final consumption, % of GDP, Canada 2024 (World Bank WDI table 4.8).
/// https://wdi.worldbank.org/table/4.8
pub const HFCE_PCT_GDP: f64 = 55.6;
/// Imports of goods and services, % of GDP, Canada 2024 (World Bank WDI table 4.8).
pub const IMPORTS_PCT_GDP: f64 = 32.7;
/// Finance Canada short-run multipliers, Budget 2009 Annex 1, Table A1.1, "2010Q4" column:
/// personal income tax measures 1.0, corporate income tax measures 0.3.
/// https://budget.canada.ca/2009/plan/bpa1-eng.html
pub const MULTIPLIER_PIT: f64 = 1.0;
pub const MULTIPLIER_CIT: f64 = 0.3;

// ---------------------------------------------------------------------------
// Land. Verified: national wealth (all non-financial assets) Q2 2025 = $17,267.4B and
// household residential real estate (dwellings + land) Q2 2025 = $8,375.9B
// (Statistics Canada, The Daily, 2025-09-11; Table 36-10-0580-01).
// UNVERIFIED: the land-only total. A secondary source quotes ~$6.4T for Q2 2025 from
// Table 36-10-0580-01; we could not download the table to confirm it. It is used only
// as a labelled scenario.
// ---------------------------------------------------------------------------
pub const NATIONAL_WEALTH_Q2_2025: f64 = 17_267_400.0;
pub const RESIDENTIAL_REAL_ESTATE_Q2_2025: f64 = 8_375_900.0;
pub const LAND_VALUE_UNVERIFIED: f64 = 6_400_000.0;

// ---------------------------------------------------------------------------
// Ashlynn's design parameters (ab193f4).
// ---------------------------------------------------------------------------
pub const START_RATE: f64 = 20.0;
pub const END_RATE: f64 = 8.0;
pub const PATH_YEARS: u32 = 10;
pub const LVT_RATES: [f64; 2] = [0.5, 1.0];
pub const LVT_CLAIM_LOW: f64 = 50_000.0;
pub const LVT_CLAIM_HIGH: f64 = 100_000.0;
pub const GROWTH_ADDITION_PP: [f64; 2] = [1.5, 2.5];
