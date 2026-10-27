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
    // v3 precedents (debt-neutral transition, section 10 and 11 of the paper).
    ("ch_yes_vote", "85"),
    ("ch_debt_2003", "25.3% (2003)"),
    ("ch_debt_2019", "13.5% (2019)"),
    ("de_structural", "0.35"),
    ("de_control", "1.5"),
    ("se_debt_anchor", "35"),
    ("nz_gst_1986", "10"),
    ("nz_top_1986", "48"),
    ("ee_rate", "22/78"),
    ("pr_scope_b", "52"),
    ("pr_deficit_target", "3"),
    ("pr_deficit_target_b", "24.3"),
    ("pr_cuts_b", "25.3"),
    ("ie_gni_share", "57.1"),
    ("ch_vote_date", "2 December 2001"),
    ("nz_date", "1 October 1986"),
    ("se_origin", "the early 1990s"),
    ("pr_base_year", "1994-95"),
    (
        "pr_ratio",
        "nearly seven dollars of spending reduction for every dollar of tax revenue increase",
    ),
];

// ---------------------------------------------------------------------------
// v3: debt-neutral transition and industry spin-up (sections 10-12 of the paper).
// ---------------------------------------------------------------------------
/// Canada's marginal effective tax rate (METR) on new business investment, 2025, per cent,
/// before the Budget 2025 productivity super-deduction (Budget 2025, ch. 1, Chart 1.1).
/// <https://budget.canada.ca/2025/report-rapport/chap1-en.html>
pub const METR_2025: f64 = 15.6;
/// Canada's METR with the productivity super-deduction, per cent (same source). It is the
/// most a tax cut can remove from the METR, so it bounds the high spin-up case.
pub const METR_2025_SUPER: f64 = 13.2;

/// Statistics Canada Table 36-10-0013-01, "Input-output multipliers, summary level", simple
/// multiplier, total industries, Canada, 2022: output per dollar of output delivered to
/// final demand. **UNVERIFIED**: read from the search tool's index copy of the table page;
/// the live site returned an outage page on 2026-10-21, so no verbatim excerpt exists.
pub const IO_OUTPUT_2022: f64 = 1.560;
/// Same table: GDP at market prices per dollar of output to final demand, 2022.
pub const IO_GDP_2022: f64 = 0.833;
/// Same table: gross operating surplus per dollar of output to final demand, 2022.
pub const IO_GOS_2022: f64 = 0.348;
/// Same table: international imports per dollar of output to final demand, 2022.
pub const IO_IMPORTS_2022: f64 = 0.167;
/// Same table: jobs per million dollars of output to final demand, 2022.
pub const IO_JOBS_2022: f64 = 6.199;

/// Gross GST/HST gap, tax year 2022, per cent of federal GST/HST revenue (CRA, Overall
/// federal tax gap report 2026, Figure 10).
/// <https://www.canada.ca/en/revenue-agency/corporate/about-canada-revenue-agency-cra/tax-canada-a-conceptual-study/tax-gap-brief-overview/2026-overall-federal-tax-gap-report.html>
pub const GST_GAP_GROSS_2022: f64 = 13.5;
/// Net GST/HST gap (after CRA compliance and collection), tax year 2022, per cent (same).
pub const GST_GAP_NET_2022: f64 = 5.8;

/// Total direct program expenses, FY2024-25, $M (AFR 2024-25, Table 5).
pub const DIRECT_PROGRAM_EXPENSES: f64 = 237_594.0;
/// Public debt charges, FY2024-25, $M (AFR 2024-25, Table 5).
pub const PUBLIC_DEBT_CHARGES: f64 = 53_410.0;
/// Unmatured debt at March 31, 2025, $M (AFR 2024-25, statement of financial position).
pub const UNMATURED_DEBT: f64 = 1_485_887.0;
/// Annual operating deficit, FY2024-25, $M (AFR 2024-25, Table 6).
pub const OPERATING_DEFICIT: f64 = 36_348.0;

/// Spending covered by Canada's 1994-95 Program Review, $M ("about $52 billion worth of
/// spending, excluding only major statutory transfers", Budget 1995, Budget Plan).
/// <https://www.budget.canada.ca/archives/budget95/binb/budget1995-eng.pdf>
pub const PROGRAM_REVIEW_SCOPE: f64 = 52_000.0;
/// Decline in Program Review spending by 1997-98 relative to 1994-95, per cent (same).
pub const PROGRAM_REVIEW_DECLINE_PCT: f64 = 18.9;

/// Ireland, GDP at current market prices, 2024, EUR M (CSO, Annual National Accounts
/// 2024, Table 2.2). <https://www.cso.ie/en/releasesandpublications/ep/p-ana/annualnationalaccounts2024/gniandde-globalisedresults/>
pub const IE_GDP_2024: f64 = 562_794.0;
/// Ireland, modified gross national income (GNI*), 2024, EUR M (same table).
pub const IE_GNI_STAR_2024: f64 = 321_098.0;

/// Long-run semi-elasticity of the machinery-and-equipment capital stock to the METR:
/// per cent of capital per percentage point of METR. **UNVERIFIED**: from the abstract of
/// IMF Working Paper WP/20/77, "Tax Elasticity Estimates for Capital Stocks in Canada"
/// (user-cost elasticity about -1.3, semi-elasticity about -0.2), as reported by a search
/// index. The IMF pages refused the fetch, so no verbatim excerpt is committed.
pub const USER_COST_SEMI_ELASTICITY_UNVERIFIED: f64 = 0.2;

/// Spin-up scenarios (low, central, high): multiplier on the semi-elasticity above.
/// Scenario parameters, not data.
pub const SPINUP_ELASTICITY_SCALE: [f64; 3] = [0.5, 1.0, 2.0];
/// METR cut, percentage points, from abolishing all federal CIT (low, central). The high
/// case uses [`METR_2025_SUPER`], the whole remaining METR. Scenario parameters: the
/// federal share of the METR is not published in the sources used here.
pub const SPINUP_METR_CUT_PP: [f64; 2] = [5.0, 8.0];
/// Share of the gap between the capital stock and its new target closed each year
/// (low, central, high). Scenario parameters.
pub const SPINUP_SPEED: [f64; 3] = [0.1, 0.2, 0.3];
/// Recession stress test: one-year fall in the taxable base, per cent. Scenario parameter.
pub const RECESSION_BASE_DROP_PCT: f64 = 3.0;
/// Year of the recession stress test (years after launch). Scenario parameter.
pub const RECESSION_YEAR: u32 = 5;
/// Years simulated under the debt-neutral rule. Design choice of this revision.
pub const HORIZON_YEARS: u32 = 100;
/// Years over which the rule repays a shortfall booked in the compensation account.
/// Design choice of this revision (the Swiss rule requires repayment "in subsequent
/// years" without a fixed count).
pub const REPAY_YEARS: u32 = 3;

// ---------------------------------------------------------------------------
// v3 end state: k = 0 with a balanced budget (section 13 of the paper).
// ---------------------------------------------------------------------------
/// Federal bracket thresholds, 2022 tax year, $ of taxable income (CRA, Individual Income
/// Tax Return Statistics, 2022 tax year, "Federal Income Tax Brackets").
/// <https://www.canada.ca/en/revenue-agency/programs/about-canada-revenue-agency-cra/income-statistics-gst-hst-statistics/t1-final-statistics/2022-tax-year.html>
pub const BRACKET_TOPS_2022: [f64; 4] = [50_197.0, 100_392.0, 155_625.0, 221_708.0];
/// Federal basic personal amount, 2022, $ (same page: "For 2022, this amount is $14,398").
pub const BPA_2022: f64 = 14_398.0;
/// Share of a realised capital gain that is taxable, per cent (same page: "only 50% of the
/// gains realized in 2022 are reported").
pub const CG_INCLUSION_PCT: f64 = 50.0;

/// Top 1% of Canadian taxfilers, 2022: total income (including capital gains) threshold, $
/// (Nova Scotia Finance, "High income trends and effective tax rates, 2022", from
/// Statistics Canada Tables 11-10-0054-01 and 11-10-0056-01).
/// <https://novascotia.ca/finance/statistics/archive_news.asp?df=&dg=&dti=3&dto=0&id=20388>
pub const TOP1_THRESHOLD_2022: f64 = 322_000.0;
/// Top 10% threshold, 2022, $ (same source).
pub const TOP10_THRESHOLD_2022: f64 = 115_000.0;
/// Top 1% share of total income (including capital gains), 2022, per cent (same source).
pub const TOP1_INCOME_SHARE_2022: f64 = 12.6;
/// Top 1% share of federal and provincial income taxes, 2022, per cent (same source).
pub const TOP1_TAX_SHARE_2022: f64 = 22.7;
/// Effective income tax rate, all taxfilers, Canada, 2022, per cent (same source).
pub const EFFECTIVE_RATE_2022: f64 = 20.1;

/// PBO costing of an annual 1% tax on family net wealth above $10M: revenue in 2024-25,
/// $M (PBO, September 2021 election proposal costing, "Implement annual net wealth tax";
/// PBO shows revenue as a negative cost). It already assumes a 35% behavioural reduction
/// of net wealth and administration costs of 2% of gross revenue.
/// <https://distribution-a617274656661637473.pbo-dpb.ca/e45d50992f5e3c2e474f60ce79b76664db4bdf21c2f2059901c3575c65f6c0fd>
pub const WEALTH_TAX_2024_25: f64 = 12_594.0;
/// Rate of the PBO wealth tax, per cent (same source).
pub const WEALTH_TAX_RATE_PCT: f64 = 1.0;
/// Exemption threshold of the PBO wealth tax, $M of family net wealth (same source).
pub const WEALTH_TAX_THRESHOLD: f64 = 10.0;
/// Behavioural reduction of net wealth in the PBO costing, per cent (same source).
pub const WEALTH_TAX_BEHAVIOUR_PCT: f64 = 35.0;
/// Administration costs in the PBO costing, per cent of gross revenue (same source).
pub const WEALTH_TAX_ADMIN_PCT: f64 = 2.0;

/// Market Basket Measure threshold, family of four, "Ontario, population 500,000 and
/// over", 2024, $ (Statistics Canada Table 11-10-0066-01). **UNVERIFIED**: read from the
/// search tool's index copy; the Statistics Canada site was down on 2026-10-21.
pub const MBM_FAMILY4_UNVERIFIED: f64 = 54_369.0;
/// Share of the population aged 0 to 14 on 1 July 2024, per cent (Statistics Canada, The
/// Daily, 25 September 2024). **UNVERIFIED**: from the search tool's summary of the release;
/// the site was down on 2026-10-21.
pub const CHILD_SHARE_0_14_UNVERIFIED: f64 = 15.2;

/// Representative single filers for the distribution table: total income, $. The first
/// row is the MBM single-person line (half the family-of-four line); the third and fifth
/// are the top 10% and top 1% thresholds above. These three are scenario points.
pub const DIST_INCOME_SCENARIO: [f64; 3] = [50_000.0, 200_000.0, 1_000_000.0];
/// Consumption as a share of total income for the rows of the distribution table, in row
/// order (MBM line, 50k, top-10% line, 200k, top-1% line, 1M). Scenario parameters: the
/// Survey of Household Spending quintile tables were not retrievable on 2026-10-21.
pub const DIST_SPEND_SHARE_SCENARIO: [f64; 6] = [1.0, 0.9, 0.8, 0.7, 0.6, 0.4];
/// Consumption of the "asset-rich" row: a household with no realised income that spends
/// from unrealised gains or borrowing, $. Scenario parameter.
pub const ASSET_RICH_SPENDING_SCENARIO: f64 = 250_000.0;
/// Retiree row: total income and consumption, $ (spends savings on top of income).
/// Scenario parameters.
pub const RETIREE_SCENARIO: [f64; 2] = [30_000.0, 50_000.0];

// ---- §14 Next-generation backbone (v3.1, accessed 2026-10-27). ----

/// Labour share of GDP, Canada, 2024, per cent: compensation of employees over GDP (UN
/// Global SDG Database indicator 10.4.1, UNECE copy; Statistics Canada's own table
/// 36-10-0114-01 was not read).
/// <https://w3.unece.org/SDG/en/Indicator?id=30>
pub const LABOUR_SHARE_2024: f64 = 59.6;
/// Total health care spending, Canada, 2025 forecast, $M (CIHI, National health
/// expenditure trends, 2025).
/// <https://www.cihi.ca/en/national-health-expenditure-trends>
pub const HEALTH_SPENDING_2025: f64 = 399_000.0;
/// Health spending, per cent of GDP, 2025 forecast (same source).
pub const HEALTH_PCT_GDP_2025: f64 = 12.7;
/// CHOAM row E2: annual yield of a 5 kWp array in Toronto, kWh (Natural Resources Canada inputs, rendered by
/// the CHOAM model crate at d31a02a).
pub const CHOAM_YIELD_KWH: f64 = 5_815.0;
/// CHOAM row E7: OEB Tier-1 price, $/kWh (11.2 c/kWh).
pub const CHOAM_TIER1_PRICE: f64 = 0.112;
/// CHOAM row E4: unit cost of the low case, $.
pub const CHOAM_UNIT_COST_LOW: f64 = 50_000.0;
/// Low/central/high shocks of §14, per cent: fall in the retail power price (energy) and
/// fall in the wage base (AI and automation). Scenario parameters.
pub const NEXTGEN_SHOCK_PCT_SCENARIO: [f64; 3] = [10.0, 25.0, 40.0];
/// Quantum-era compliance high case: GST/HST gap twice today's gross gap, per cent.
/// Scenario parameter (multiplier on the CRA gross gap).
pub const QUANTUM_GAP_MULTIPLE_SCENARIO: f64 = 2.0;
/// Space procurement added to federal need, $M a year, low/central/high. Scenario
/// parameters; no sourced space-revenue figure is used.
pub const SPACE_PROCUREMENT_SCENARIO: [f64; 3] = [1_000.0, 2_500.0, 5_000.0];
/// Change in total health spending from biotech, per cent, low/central/high (negative =
/// saving). Scenario parameters; no medical efficacy claim is made.
pub const BIOTECH_HEALTH_SHIFT_PCT_SCENARIO: [f64; 3] = [-2.0, 0.0, 2.0];
