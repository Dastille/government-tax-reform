//! Every input number, with its primary source. Each sourced value is checked against a
//! committed verbatim excerpt in `tests/sources/` by `tests/sources.rs`. Units: millions of Canadian dollars
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
/// Net GST from outside parties = the $52.5B headline (PA 2025 Table 3.3): GST after the
/// ministries' remission order and after quarterly GST credits. PA does not split GST from
/// the federal part of HST; HST = GST rate + a provincial component, and "GST" means the
/// federal portion only (Report on Federal Tax Expenditures, Part 1, benchmark note).
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

/// Targeted Temporary GST/HST Relief (Dec 14, 2024 to Feb 15, 2025), calendar 2024 cost.
pub const GST_HOLIDAY_COST_2024: f64 = 445.0;
/// Targeted Temporary GST/HST Relief, calendar 2025 cost.
pub const GST_HOLIDAY_COST_2025: f64 = 840.0;
/// Whole GST holiday cost. It fell inside FY2024-25, so it depressed the `GST_NET` figure.
pub const GST_HOLIDAY_COST: f64 = GST_HOLIDAY_COST_2024 + GST_HOLIDAY_COST_2025;

/// Federal PIT bracket rates, 2023 taxation year, per cent (TE 2026 Part 2, background
/// table "Federal Personal Income Tax Brackets and Rates, Tax Filers and Taxes Paid, 2023").
pub const BRACKETS_2023: [f64; 5] = [15.0, 20.5, 26.0, 29.0, 33.0];

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
/// <https://www150.statcan.gc.ca/n1/daily-quotidien/250911/dq250911a-eng.htm>
pub const SAVING_RATE_Q2_2025: f64 = 0.050;
/// Household saving rate, Q2 2024 (Statistics Canada, The Daily, 2024-09-12): 7.2%.
/// <https://www150.statcan.gc.ca/n1/daily-quotidien/240912/dq240912a-eng.htm>
pub const SAVING_RATE_Q2_2024: f64 = 0.072;
/// Household + NPISH final consumption, % of GDP, Canada 2024 (World Bank WDI table 4.8).
/// <https://wdi.worldbank.org/table/4.8>
pub const HFCE_PCT_GDP: f64 = 55.6;
/// Imports of goods and services, % of GDP, Canada 2024 (World Bank WDI table 4.8).
pub const IMPORTS_PCT_GDP: f64 = 32.7;
/// Finance Canada short-run multipliers, Budget 2009 Annex 1, Table A1.1, "2010Q4" column:
/// personal income tax measures 1.0, corporate income tax measures 0.3.
/// <https://budget.canada.ca/2009/plan/bpa1-eng.html>
pub const MULTIPLIER_PIT: f64 = 1.0;
/// Finance Canada short-run multiplier for corporate income tax measures (same table).
pub const MULTIPLIER_CIT: f64 = 0.3;

// ---------------------------------------------------------------------------
// Land. Verified: national wealth (all non-financial assets) Q2 2025 = $17,267.4B and
// household residential real estate (dwellings + land) Q2 2025 = $8,375.9B
// (Statistics Canada, The Daily, 2025-09-11; Table 36-10-0580-01).
// UNVERIFIED: the land-only total. A secondary source quotes ~$6.4T for Q2 2025 from
// Table 36-10-0580-01; we could not download the table to confirm it. It is used only
// as a labelled scenario.
// ---------------------------------------------------------------------------
/// National wealth (all non-financial assets), Q2 2025, $M (`StatCan` Daily 2025-09-11).
pub const NATIONAL_WEALTH_Q2_2025: f64 = 17_267_400.0;
/// Household residential real estate (land plus buildings), Q2 2025, $M (same source).
pub const RESIDENTIAL_REAL_ESTATE_Q2_2025: f64 = 8_375_900.0;
/// Land-only value, $M. **UNVERIFIED** (secondary source; see above).
pub const LAND_VALUE_UNVERIFIED: f64 = 6_400_000.0;
/// Share of land value assumed taxable after the design's exemptions (scenario, not data).
pub const LVT_TAXABLE_SHARE_SCENARIO: f64 = 0.5;

// ---------------------------------------------------------------------------
// Recurrent property taxes that the design abolishes.
// Consolidated provincial-local property tax revenue per capita, Canada 2024: $2,242.
// Nova Scotia Department of Finance, "Government Financial Statistics, 2024" (2025-11-21),
// computed from Statistics Canada Table 10-10-0147-01 (CGFS) and Table 17-10-0009-01.
// https://novascotia.ca/finance/statistics/archive_news.asp?df=&dg=&dti=3&dto=0&id=21517
// Population of Canada, 1 July 2024: 41,288,599 (StatCan Daily 2024-09-25).
// https://www150.statcan.gc.ca/n1/daily-quotidien/240925/dq240925d-eng.htm
// ---------------------------------------------------------------------------
/// Consolidated provincial-local property tax revenue per capita, Canada 2024, dollars.
pub const PROPERTY_TAX_PER_CAPITA_2024: f64 = 2_242.0;
/// Population of Canada, 1 July 2024 (preliminary estimate).
pub const POPULATION_2024: f64 = 41_288_599.0;

// ---------------------------------------------------------------------------
// Ashlynn's design parameters (ab193f4). These are policy choices, not data.
// ---------------------------------------------------------------------------
/// VAT rate at launch, per cent.
pub const START_RATE: f64 = 20.0;
/// VAT rate at the end of the path, per cent.
pub const END_RATE: f64 = 8.0;
/// Length of the rate path, years.
pub const PATH_YEARS: u32 = 10;
/// LVT rate range, per cent of unimproved land value.
pub const LVT_RATES: [f64; 2] = [0.5, 1.0];
/// Original LVT revenue claim, low end, $M.
pub const LVT_CLAIM_LOW: f64 = 50_000.0;
/// Original LVT revenue claim, high end, $M.
pub const LVT_CLAIM_HIGH: f64 = 100_000.0;
/// Original growth addition, percentage points a year. **UNVERIFIED**: no source was given
/// in ab193f4 and none was found; used only as a what-if.
pub const GROWTH_ADDITION_PP: [f64; 2] = [1.5, 2.5];
/// Share of households (bottom, per cent) whose VAT burden the dividend should fully offset.
pub const DIVIDEND_BOTTOM_SHARE: [f64; 2] = [40.0, 50.0];
/// Share of incremental revenue committed to high-multiplier infrastructure, per cent.
pub const INVESTMENT_SHARE: [f64; 2] = [60.0, 70.0];

// ---------------------------------------------------------------------------
// Facts cited in the precedents and prior-work sections.
// ---------------------------------------------------------------------------
/// VAT / GST standard rates and dates cited in Section 8. Each entry: (key, value, source).
/// Sources:
/// - NZ: Inland Revenue, "GST rate increase" (Taxation (Budget Measures) Act 2010): 12.5% to
///   15% from 1 October 2010.
///   <https://www.taxtechnical.ird.govt.nz/new-legislation/act-articles/taxation-budget-measures-act-2010/gst-rate-increase>
/// - DK, HU standard rates 2022: OECD CTT 2024 Annex Table 2.A.7.
/// - VAT % of GDP / % of total taxation 2022: OECD CTT 2024 Annex Table 1.A.4 (p. 32 of the PDF).
/// - Saudi Arabia: ZATCA "About VAT" (5% from 1 January 2018); Saudi Ministry of Finance,
///   11 May 2020, "VAT shall increase from 5% to 15% as of July 2020".
///   <https://www.mof.gov.sa/en/MediaCenter/news/Pages/News_11052020.aspx>
/// - Singapore: IRAS ("The current highest personal income tax rate is at 24%"; "The current
///   GST rate in Singapore is 9%").
/// - Canada 1991: TE 2017 Part 10 (FST 13.5%); Library of Parliament PRB 00-03E (GST 7% from
///   1 January 1991).
/// - 175 countries and territories with a VAT on 1 July 2024: OECD CTT 2024.
/// - CGEB +25% for 2026-2031: CRA CGEB page.
/// - `FairTax`: H.R. 25 (119th Congress), proposed sec. 101(b): 23% of gross payments in 2027.
/// - Federal debt 41.2% of GDP: AFR 2024-25.
pub const FACTS: &[(&str, &str)] = &[
    ("nz_rate", "15"),
    ("nz_prev_rate", "12.5"),
    ("nz_rate_date", "1 October 2010"),
    ("nz_vat_gdp", "10.0"),
    ("nz_vat_taxes", "30.3"),
    ("dk_rate", "25"),
    ("dk_vat_gdp", "9.2"),
    ("dk_vat_taxes", "22.0"),
    ("hu_rate", "27"),
    ("hu_vat_gdp", "10.1"),
    ("hu_vat_taxes", "28.9"),
    ("sa_rate_2018", "5"),
    ("sa_rate_2020", "15"),
    ("sa_date_2018", "1 January 2018"),
    ("sa_date_2020", "July 2020"),
    ("sg_pit_top", "24"),
    ("sg_gst", "9"),
    ("fst_rate", "13.5"),
    ("gst_1991_rate", "7"),
    ("vat_countries", "175"),
    ("cgeb_increase", "25"),
    ("fairtax_rate", "23"),
    ("debt_pct_gdp", "41.2"),
];
